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
    pub u_gameplay_cue_notify_static_while_active: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_notify_static_on_remove: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_notify_static_on_execute: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_notify_static_on_active: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_notify_static_k2_handle_gameplay_cue: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_end_action: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_attribute_changed_wait_for_attribute_changed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_attribute_changed_async_wait_attribute_changed_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_effect_applied_wait_gameplay_effect_applied_to_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_effect_applied_on_applied_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_event_wait_gameplay_event_to_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_event_event_received_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_tag_async_wait_gameplay_tag_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_tag_added_wait_gameplay_tag_add_to_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_tag_removed_wait_gameplay_tag_remove_from_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_changed_on_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_tag_count_changed_async_wait_gameplay_tag_count_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_ability_async_wait_gameplay_tag_query_wait_gameplay_tag_query_on_actor: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_set_should_block_other_abilities: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_set_can_be_canceled: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_send_gameplay_event: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_remove_granted_by_effect: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_montage_stop: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_montage_set_next_section_name: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_montage_jump_to_section: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_make_target_location_info_from_owner_skeletal_mesh_component: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_make_target_location_info_from_owner_actor: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_make_outgoing_gameplay_effect_spec: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_should_ability_respond_to_event: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_remove_gameplay_cue: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_on_end_ability: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_has_authority: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_execute_gameplay_cue_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_execute_gameplay_cue: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_end_ability_locally: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_end_ability: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_commit_execute: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_commit_ability_cost: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_commit_ability_cooldown: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_commit_ability: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_check_ability_cost: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_check_ability_cooldown: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_cancel_ability: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_can_activate_ability: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_apply_gameplay_effect_spec_to_target: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_apply_gameplay_effect_spec_to_owner: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_add_gameplay_cue_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_add_gameplay_cue: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_activate_ability_from_event: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_k2_activate_ability: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_is_locally_controlled: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_invalidate_client_prediction_key: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_source_object_bp: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_owning_component_from_actor_info: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_owning_actor_from_actor_info: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_granted_by_effect_context: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_current_source_object: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_current_montage: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_cooldown_time_remaining: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_context_from_owner: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_avatar_actor_from_actor_info: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_actor_info: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_ability_system_component_from_actor_info: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_ability_level_bp: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_get_ability_level: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_end_task_by_instance_name: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_end_ability_state: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_confirm_task_by_instance_name: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_cancel_task_by_instance_name: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_handle: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_granted_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_asset_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_bp_apply_gameplay_effect_to_target: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_ability_bp_apply_gameplay_effect_to_owner: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_target_actor_confirm_targeting: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_target_actor_cancel_targeting: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_world_reticle_set_reticle_material_param_vector: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_world_reticle_set_reticle_material_param_float: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_world_reticle_on_valid_target_changed: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_world_reticle_on_targeting_an_actor: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_world_reticle_on_parameters_initialized: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_ability_world_reticle_face_toward_source: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_constant_force_apply_root_motion_constant_force: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_jump_force_on_landed_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_jump_force_finish: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_jump_force_apply_root_motion_jump_force: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_move_to_actor_force_on_target_actor_swapped: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_move_to_actor_force_on_rep_target_location: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_target_data_actor_force: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_component_force: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_actor_force: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_move_to_force_apply_root_motion_move_to_force: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_apply_root_motion_radial_force_apply_root_motion_radial_force: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_move_to_location_move_to_location: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_network_sync_point_wait_net_sync: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_network_sync_point_on_signal_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_anim_and_wait_on_montage_interrupted: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_anim_and_wait_on_montage_ended: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_anim_and_wait_on_montage_blending_out: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_anim_and_wait_on_montage_blended_in: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_anim_and_wait_create_play_anim_and_wait_proxy: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_montage_and_wait_on_montage_interrupted: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_montage_and_wait_on_montage_ended: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_montage_and_wait_on_montage_blending_out: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_montage_and_wait_on_montage_blended_in: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_montage_and_wait_on_gameplay_ability_cancelled: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_play_montage_and_wait_create_play_montage_and_wait_proxy: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_repeat_repeat_action: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_spawn_actor_spawn_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_spawn_actor_finish_spawning_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_spawn_actor_begin_spawning_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_start_ability_state_start_ability_state: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_visualize_targeting_visualize_targeting_using_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_visualize_targeting_visualize_targeting: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_visualize_targeting_finish_spawning_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_visualize_targeting_begin_spawning_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_ability_activate_wait_for_ability_activate_with_tag_requirements: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_ability_activate_wait_for_ability_activate_query: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_ability_activate_wait_for_ability_activate: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_ability_activate_on_ability_activate: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_ability_commit_wait_for_ability_commit_query: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_ability_commit_wait_for_ability_commit: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_ability_commit_on_ability_commit: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_attribute_change_wait_for_attribute_change_with_comparison: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_attribute_change_wait_for_attribute_change: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_attribute_change_ratio_threshold_wait_for_attribute_change_ratio_threshold: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_attribute_change_threshold_wait_for_attribute_change_threshold: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_cancel_wait_cancel: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_cancel_on_local_cancel_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_cancel_on_cancel_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_confirm_wait_confirm: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_confirm_on_confirm_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_confirm_cancel_wait_confirm_cancel: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_confirm_cancel_on_local_confirm_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_confirm_cancel_on_local_cancel_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_confirm_cancel_on_confirm_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_confirm_cancel_on_cancel_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_delay_wait_delay: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_applied_on_apply_gameplay_effect_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self_query: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target_query: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_blocked_immunity_wait_gameplay_effect_blocked_by_immunity: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_removed_wait_for_gameplay_effect_removed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_removed_on_gameplay_effect_removed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_stack_change_wait_for_gameplay_effect_stack_change: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_effect_stack_change_on_gameplay_effect_stack_change: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_event_wait_gameplay_event: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_tag_gameplay_tag_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_tag_added_wait_gameplay_tag_add: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_tag_removed_wait_gameplay_tag_remove: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_change: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_tag_query_wait_gameplay_tag_query: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_gameplay_tag_query_update_target_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_input_press_wait_input_press: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_input_press_on_press_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_input_release_wait_input_release: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_input_release_on_release_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_movement_mode_change_on_movement_mode_change: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_movement_mode_change_create_wait_movement_mode_change: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_overlap_wait_for_overlap: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_overlap_on_hit_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_wait_target_data_using_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_wait_target_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_on_target_data_replicated_cancelled_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_on_target_data_replicated_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_on_target_data_ready_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_on_target_data_cancelled_callback: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_finish_spawning_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_target_data_begin_spawning_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_task_wait_velocity_change_create_wait_velocity_change: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_unbind_gameplay_tag_changed_event_wrapper_for_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_unbind_all_gameplay_tag_changed_event_wrappers_for_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_target_data_has_origin: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_target_data_has_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_target_data_has_end_point: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_target_data_has_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_set_stack_count_to_max: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_set_stack_count: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_set_duration: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_send_gameplay_event_to_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_remove_loose_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_remove_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_not_equal_gameplay_attribute_gameplay_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_not_equal_gameplay_ability_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_not_equal_active_gameplay_effect_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_make_spec_handle_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_make_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_make_gameplay_cue_parameters: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_make_filter_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_is_instigator_locally_controlled_player: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_is_instigator_locally_controlled: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_is_gameplay_ability_active: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_is_duration_gameplay_effect_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_is_active_gameplay_effect_handle_valid: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_is_active_gameplay_effect_handle_active: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_has_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_has_any_abilities_with_asset_tag: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_target_data_origin: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_target_data_end_point_transform: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_target_data_end_point: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_origin: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_modified_attribute_magnitude: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_instigator_transform: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_instigator_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_hit_result_from_target_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_granted_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_effect_ui_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_effect_granted_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_effect_from_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_effect_from_active_effect_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_effect_asset_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_cue_end_location_and_normal: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_cue_direction: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_gameplay_ability_from_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_float_attribute_from_ability_system_component: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_float_attribute_base_from_ability_system_component: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_float_attribute_base: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_float_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_effect_context: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_duration_policy_from_gameplay_effect_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_debug_string_from_gameplay_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_data_count_from_target_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_asset_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_all_linked_gameplay_effect_spec_handles: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_all_actors_from_target_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_actors_from_target_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_actor_count: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_actor_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_active_gameplay_effect_total_duration: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_active_gameplay_effect_start_time: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_active_gameplay_effect_stack_limit_count: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_active_gameplay_effect_stack_count: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_active_gameplay_effect_remaining_duration: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_active_gameplay_effect_expected_end_time: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_active_gameplay_effect_debug_string: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_ability_system_component_from_active_gameplay_effect_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_get_ability_system_component: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_forward_gameplay_cue_to_target: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_filter_target_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_evaluate_attribute_value_with_tags_and_base: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_evaluate_attribute_value_with_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_equal_equal_gameplay_attribute_gameplay_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_equal_equal_gameplay_ability_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_equal_equal_active_gameplay_effect_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_set_origin: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_is_instigator_locally_controlled: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_has_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_get_source_object: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_get_original_instigator_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_get_origin: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_get_instigator_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_get_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_get_effect_causer: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_effect_context_add_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_does_target_data_contain_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_does_gameplay_cue_meet_tag_requirements: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_conv_scalable_float_to_float: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_conv_scalable_float_to_double: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_clone_spec_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_break_gameplay_cue_parameters: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_bind_event_wrapper_to_gameplay_tag_changed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tags_changed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tag_container_changed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_assign_tag_set_by_caller_magnitude: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_assign_set_by_caller_magnitude: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_append_target_data_handle: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_loose_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_linked_gameplay_effect_spec: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_linked_gameplay_effect: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_granted_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_granted_tag: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_asset_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_add_asset_tag: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_ability_target_data_from_locations: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_ability_target_data_from_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_ability_target_data_from_actor_array: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_blueprint_library_ability_target_data_from_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_cheat_manager_extension_effect_remove: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_cheat_manager_extension_effect_list_active: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_cheat_manager_extension_effect_apply: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_cheat_manager_extension_ability_list_granted: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_cheat_manager_extension_ability_grant: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_cheat_manager_extension_ability_cancel: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_cheat_manager_extension_ability_activate: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitudes: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitude: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_try_activate_ability_by_class: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_try_activate_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_try_activate_abilities_by_tag: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_target_confirm: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_target_cancel: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_set_user_ability_activation_inhibited: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_set_active_gameplay_effect_level_using_query: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_set_active_gameplay_effect_level: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_try_activate_ability_with_event_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_try_activate_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_set_replicated_target_data_cancelled: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_set_replicated_target_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_set_replicated_event_with_payload: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_set_replicated_event: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_set_input_released: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_set_input_pressed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_print_debug_request_with_strings: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_print_debug_request: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_end_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_current_montage_set_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_current_montage_set_next_section_name: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_current_montage_jump_to_section_name: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_cancel_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_server_ability_rpc_batch: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_remove_active_gameplay_effect_by_source_effect: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_remove_active_gameplay_effect: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_remove_active_effects_with_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_remove_active_effects_with_source_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_remove_active_effects_with_granted_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_remove_active_effects_with_applied_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_release_input_id: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_press_input_id: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_spawned_attributes_end_played: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_rep_spawned_attributes: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_rep_server_debug_string: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_rep_replicated_anim_montage: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_rep_owning_actor: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_rep_client_debug_string: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_rep_activate_abilities: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_owner_actor_destroyed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_on_avatar_actor_destroyed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cues_executed_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cues_executed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cues_added_and_while_active_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cue_executed_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cue_executed_from_spec: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cue_executed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cue_added_and_while_active_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cue_added_and_while_active_from_spec: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cue_added_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_net_multicast_invoke_gameplay_cue_added: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_make_outgoing_spec: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_make_effect_context: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_k2_init_stats: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_k2_give_ability_and_activate_once: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_k2_give_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_is_gameplay_cue_active: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_input_confirm: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_input_cancel: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_user_ability_activation_inhibited: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_gameplay_tag_count: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_gameplay_effect_magnitude: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_gameplay_effect_count_if_loaded: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_gameplay_effect_count: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_gameplay_attribute_value: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_attribute_set: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_all_attributes: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_all_abilities: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_active_effects_with_all_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_get_active_effects: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_find_all_abilities_with_tags: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_find_all_abilities_with_input_id: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_find_all_abilities_matching_query: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_try_activate_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_set_replicated_event: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_print_debug_response: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_end_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_cancel_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_activate_ability_succeed_with_event_data: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_activate_ability_succeed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_client_activate_ability_failed: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_clear_all_abilities_with_input_id: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_clear_all_abilities: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_clear_ability: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_bp_apply_gameplay_effect_to_target: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_bp_apply_gameplay_effect_to_self: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_bp_apply_gameplay_effect_spec_to_target: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_bp_apply_gameplay_effect_spec_to_self: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_ability_confirm_or_cancel_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_ability_system_component_ability_ability_key_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_function_library_remove_gameplay_cue_on_actor: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_function_library_make_gameplay_cue_parameters_from_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_function_library_execute_gameplay_cue_on_actor: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_function_library_add_gameplay_cue_on_actor: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_interface_forward_gameplay_cue_to_parent: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_interface_blueprint_custom_handler: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_actor_while_active: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_actor_on_remove: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_actor_on_owner_destroyed: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_actor_on_execute: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_actor_on_active: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_actor_k2_handle_gameplay_cue: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_actor_k2_end_gameplay_cue: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_cue_notify_burst_on_burst: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_burst_latent_on_burst: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_looping_on_removal: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_looping_on_recurring: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_looping_on_looping_start: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cue_notify_looping_on_application: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_effect_custom_application_requirement_can_apply_gameplay_effect: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_effect_execution_calculation_execute: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_k2_get_captured_attribute_magnitude: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_target_spec_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_target_aggregated_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_target_actor_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_source_spec_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_source_aggregated_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_source_actor_tags: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_tag: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_mod_magnitude_calculation_calculate_base_magnitude: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_tag_reponse_table_tag_response_event: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_gameplay_cue_track_set_sequencer_track_handler: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_gameplay_cue_notify_static_while_active: std::ptr::null_mut(),
            u_gameplay_cue_notify_static_on_remove: std::ptr::null_mut(),
            u_gameplay_cue_notify_static_on_execute: std::ptr::null_mut(),
            u_gameplay_cue_notify_static_on_active: std::ptr::null_mut(),
            u_gameplay_cue_notify_static_k2_handle_gameplay_cue: std::ptr::null_mut(),
            u_ability_async_end_action: std::ptr::null_mut(),
            u_ability_async_wait_attribute_changed_wait_for_attribute_changed: std::ptr::null_mut(),
            u_ability_async_wait_attribute_changed_async_wait_attribute_changed_delegate_delegate_signature: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_effect_applied_wait_gameplay_effect_applied_to_actor: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_effect_applied_on_applied_delegate_delegate_signature: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_event_wait_gameplay_event_to_actor: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_event_event_received_delegate_delegate_signature: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_tag_async_wait_gameplay_tag_delegate_delegate_signature: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_tag_added_wait_gameplay_tag_add_to_actor: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_tag_removed_wait_gameplay_tag_remove_from_actor: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_changed_on_actor: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_tag_count_changed_async_wait_gameplay_tag_count_delegate_delegate_signature: std::ptr::null_mut(),
            u_ability_async_wait_gameplay_tag_query_wait_gameplay_tag_query_on_actor: std::ptr::null_mut(),
            u_gameplay_ability_set_should_block_other_abilities: std::ptr::null_mut(),
            u_gameplay_ability_set_can_be_canceled: std::ptr::null_mut(),
            u_gameplay_ability_send_gameplay_event: std::ptr::null_mut(),
            u_gameplay_ability_remove_granted_by_effect: std::ptr::null_mut(),
            u_gameplay_ability_montage_stop: std::ptr::null_mut(),
            u_gameplay_ability_montage_set_next_section_name: std::ptr::null_mut(),
            u_gameplay_ability_montage_jump_to_section: std::ptr::null_mut(),
            u_gameplay_ability_make_target_location_info_from_owner_skeletal_mesh_component: std::ptr::null_mut(),
            u_gameplay_ability_make_target_location_info_from_owner_actor: std::ptr::null_mut(),
            u_gameplay_ability_make_outgoing_gameplay_effect_spec: std::ptr::null_mut(),
            u_gameplay_ability_k2_should_ability_respond_to_event: std::ptr::null_mut(),
            u_gameplay_ability_k2_remove_gameplay_cue: std::ptr::null_mut(),
            u_gameplay_ability_k2_on_end_ability: std::ptr::null_mut(),
            u_gameplay_ability_k2_has_authority: std::ptr::null_mut(),
            u_gameplay_ability_k2_execute_gameplay_cue_with_params: std::ptr::null_mut(),
            u_gameplay_ability_k2_execute_gameplay_cue: std::ptr::null_mut(),
            u_gameplay_ability_k2_end_ability_locally: std::ptr::null_mut(),
            u_gameplay_ability_k2_end_ability: std::ptr::null_mut(),
            u_gameplay_ability_k2_commit_execute: std::ptr::null_mut(),
            u_gameplay_ability_k2_commit_ability_cost: std::ptr::null_mut(),
            u_gameplay_ability_k2_commit_ability_cooldown: std::ptr::null_mut(),
            u_gameplay_ability_k2_commit_ability: std::ptr::null_mut(),
            u_gameplay_ability_k2_check_ability_cost: std::ptr::null_mut(),
            u_gameplay_ability_k2_check_ability_cooldown: std::ptr::null_mut(),
            u_gameplay_ability_k2_cancel_ability: std::ptr::null_mut(),
            u_gameplay_ability_k2_can_activate_ability: std::ptr::null_mut(),
            u_gameplay_ability_k2_apply_gameplay_effect_spec_to_target: std::ptr::null_mut(),
            u_gameplay_ability_k2_apply_gameplay_effect_spec_to_owner: std::ptr::null_mut(),
            u_gameplay_ability_k2_add_gameplay_cue_with_params: std::ptr::null_mut(),
            u_gameplay_ability_k2_add_gameplay_cue: std::ptr::null_mut(),
            u_gameplay_ability_k2_activate_ability_from_event: std::ptr::null_mut(),
            u_gameplay_ability_k2_activate_ability: std::ptr::null_mut(),
            u_gameplay_ability_is_locally_controlled: std::ptr::null_mut(),
            u_gameplay_ability_invalidate_client_prediction_key: std::ptr::null_mut(),
            u_gameplay_ability_get_source_object_bp: std::ptr::null_mut(),
            u_gameplay_ability_get_owning_component_from_actor_info: std::ptr::null_mut(),
            u_gameplay_ability_get_owning_actor_from_actor_info: std::ptr::null_mut(),
            u_gameplay_ability_get_granted_by_effect_context: std::ptr::null_mut(),
            u_gameplay_ability_get_current_source_object: std::ptr::null_mut(),
            u_gameplay_ability_get_current_montage: std::ptr::null_mut(),
            u_gameplay_ability_get_cooldown_time_remaining: std::ptr::null_mut(),
            u_gameplay_ability_get_context_from_owner: std::ptr::null_mut(),
            u_gameplay_ability_get_avatar_actor_from_actor_info: std::ptr::null_mut(),
            u_gameplay_ability_get_actor_info: std::ptr::null_mut(),
            u_gameplay_ability_get_ability_system_component_from_actor_info: std::ptr::null_mut(),
            u_gameplay_ability_get_ability_level_bp: std::ptr::null_mut(),
            u_gameplay_ability_get_ability_level: std::ptr::null_mut(),
            u_gameplay_ability_end_task_by_instance_name: std::ptr::null_mut(),
            u_gameplay_ability_end_ability_state: std::ptr::null_mut(),
            u_gameplay_ability_confirm_task_by_instance_name: std::ptr::null_mut(),
            u_gameplay_ability_cancel_task_by_instance_name: std::ptr::null_mut(),
            u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_handle: std::ptr::null_mut(),
            u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_granted_tags: std::ptr::null_mut(),
            u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_asset_tags: std::ptr::null_mut(),
            u_gameplay_ability_bp_apply_gameplay_effect_to_target: std::ptr::null_mut(),
            u_gameplay_ability_bp_apply_gameplay_effect_to_owner: std::ptr::null_mut(),
            a_gameplay_ability_target_actor_confirm_targeting: std::ptr::null_mut(),
            a_gameplay_ability_target_actor_cancel_targeting: std::ptr::null_mut(),
            a_gameplay_ability_world_reticle_set_reticle_material_param_vector: std::ptr::null_mut(),
            a_gameplay_ability_world_reticle_set_reticle_material_param_float: std::ptr::null_mut(),
            a_gameplay_ability_world_reticle_on_valid_target_changed: std::ptr::null_mut(),
            a_gameplay_ability_world_reticle_on_targeting_an_actor: std::ptr::null_mut(),
            a_gameplay_ability_world_reticle_on_parameters_initialized: std::ptr::null_mut(),
            a_gameplay_ability_world_reticle_face_toward_source: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_constant_force_apply_root_motion_constant_force: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_jump_force_on_landed_callback: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_jump_force_finish: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_jump_force_apply_root_motion_jump_force: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_move_to_actor_force_on_target_actor_swapped: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_move_to_actor_force_on_rep_target_location: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_target_data_actor_force: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_component_force: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_actor_force: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_move_to_force_apply_root_motion_move_to_force: std::ptr::null_mut(),
            u_ability_task_apply_root_motion_radial_force_apply_root_motion_radial_force: std::ptr::null_mut(),
            u_ability_task_move_to_location_move_to_location: std::ptr::null_mut(),
            u_ability_task_network_sync_point_wait_net_sync: std::ptr::null_mut(),
            u_ability_task_network_sync_point_on_signal_callback: std::ptr::null_mut(),
            u_ability_task_play_anim_and_wait_on_montage_interrupted: std::ptr::null_mut(),
            u_ability_task_play_anim_and_wait_on_montage_ended: std::ptr::null_mut(),
            u_ability_task_play_anim_and_wait_on_montage_blending_out: std::ptr::null_mut(),
            u_ability_task_play_anim_and_wait_on_montage_blended_in: std::ptr::null_mut(),
            u_ability_task_play_anim_and_wait_create_play_anim_and_wait_proxy: std::ptr::null_mut(),
            u_ability_task_play_montage_and_wait_on_montage_interrupted: std::ptr::null_mut(),
            u_ability_task_play_montage_and_wait_on_montage_ended: std::ptr::null_mut(),
            u_ability_task_play_montage_and_wait_on_montage_blending_out: std::ptr::null_mut(),
            u_ability_task_play_montage_and_wait_on_montage_blended_in: std::ptr::null_mut(),
            u_ability_task_play_montage_and_wait_on_gameplay_ability_cancelled: std::ptr::null_mut(),
            u_ability_task_play_montage_and_wait_create_play_montage_and_wait_proxy: std::ptr::null_mut(),
            u_ability_task_repeat_repeat_action: std::ptr::null_mut(),
            u_ability_task_spawn_actor_spawn_actor: std::ptr::null_mut(),
            u_ability_task_spawn_actor_finish_spawning_actor: std::ptr::null_mut(),
            u_ability_task_spawn_actor_begin_spawning_actor: std::ptr::null_mut(),
            u_ability_task_start_ability_state_start_ability_state: std::ptr::null_mut(),
            u_ability_task_visualize_targeting_visualize_targeting_using_actor: std::ptr::null_mut(),
            u_ability_task_visualize_targeting_visualize_targeting: std::ptr::null_mut(),
            u_ability_task_visualize_targeting_finish_spawning_actor: std::ptr::null_mut(),
            u_ability_task_visualize_targeting_begin_spawning_actor: std::ptr::null_mut(),
            u_ability_task_wait_ability_activate_wait_for_ability_activate_with_tag_requirements: std::ptr::null_mut(),
            u_ability_task_wait_ability_activate_wait_for_ability_activate_query: std::ptr::null_mut(),
            u_ability_task_wait_ability_activate_wait_for_ability_activate: std::ptr::null_mut(),
            u_ability_task_wait_ability_activate_on_ability_activate: std::ptr::null_mut(),
            u_ability_task_wait_ability_commit_wait_for_ability_commit_query: std::ptr::null_mut(),
            u_ability_task_wait_ability_commit_wait_for_ability_commit: std::ptr::null_mut(),
            u_ability_task_wait_ability_commit_on_ability_commit: std::ptr::null_mut(),
            u_ability_task_wait_attribute_change_wait_for_attribute_change_with_comparison: std::ptr::null_mut(),
            u_ability_task_wait_attribute_change_wait_for_attribute_change: std::ptr::null_mut(),
            u_ability_task_wait_attribute_change_ratio_threshold_wait_for_attribute_change_ratio_threshold: std::ptr::null_mut(),
            u_ability_task_wait_attribute_change_threshold_wait_for_attribute_change_threshold: std::ptr::null_mut(),
            u_ability_task_wait_cancel_wait_cancel: std::ptr::null_mut(),
            u_ability_task_wait_cancel_on_local_cancel_callback: std::ptr::null_mut(),
            u_ability_task_wait_cancel_on_cancel_callback: std::ptr::null_mut(),
            u_ability_task_wait_confirm_wait_confirm: std::ptr::null_mut(),
            u_ability_task_wait_confirm_on_confirm_callback: std::ptr::null_mut(),
            u_ability_task_wait_confirm_cancel_wait_confirm_cancel: std::ptr::null_mut(),
            u_ability_task_wait_confirm_cancel_on_local_confirm_callback: std::ptr::null_mut(),
            u_ability_task_wait_confirm_cancel_on_local_cancel_callback: std::ptr::null_mut(),
            u_ability_task_wait_confirm_cancel_on_confirm_callback: std::ptr::null_mut(),
            u_ability_task_wait_confirm_cancel_on_cancel_callback: std::ptr::null_mut(),
            u_ability_task_wait_delay_wait_delay: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_applied_on_apply_gameplay_effect_callback: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self_query: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target_query: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_blocked_immunity_wait_gameplay_effect_blocked_by_immunity: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_removed_wait_for_gameplay_effect_removed: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_removed_on_gameplay_effect_removed: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_stack_change_wait_for_gameplay_effect_stack_change: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_effect_stack_change_on_gameplay_effect_stack_change: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_event_wait_gameplay_event: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_tag_gameplay_tag_callback: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_tag_added_wait_gameplay_tag_add: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_tag_removed_wait_gameplay_tag_remove: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_change: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_tag_query_wait_gameplay_tag_query: std::ptr::null_mut(),
            u_ability_task_wait_gameplay_tag_query_update_target_tags: std::ptr::null_mut(),
            u_ability_task_wait_input_press_wait_input_press: std::ptr::null_mut(),
            u_ability_task_wait_input_press_on_press_callback: std::ptr::null_mut(),
            u_ability_task_wait_input_release_wait_input_release: std::ptr::null_mut(),
            u_ability_task_wait_input_release_on_release_callback: std::ptr::null_mut(),
            u_ability_task_wait_movement_mode_change_on_movement_mode_change: std::ptr::null_mut(),
            u_ability_task_wait_movement_mode_change_create_wait_movement_mode_change: std::ptr::null_mut(),
            u_ability_task_wait_overlap_wait_for_overlap: std::ptr::null_mut(),
            u_ability_task_wait_overlap_on_hit_callback: std::ptr::null_mut(),
            u_ability_task_wait_target_data_wait_target_data_using_actor: std::ptr::null_mut(),
            u_ability_task_wait_target_data_wait_target_data: std::ptr::null_mut(),
            u_ability_task_wait_target_data_on_target_data_replicated_cancelled_callback: std::ptr::null_mut(),
            u_ability_task_wait_target_data_on_target_data_replicated_callback: std::ptr::null_mut(),
            u_ability_task_wait_target_data_on_target_data_ready_callback: std::ptr::null_mut(),
            u_ability_task_wait_target_data_on_target_data_cancelled_callback: std::ptr::null_mut(),
            u_ability_task_wait_target_data_finish_spawning_actor: std::ptr::null_mut(),
            u_ability_task_wait_target_data_begin_spawning_actor: std::ptr::null_mut(),
            u_ability_task_wait_velocity_change_create_wait_velocity_change: std::ptr::null_mut(),
            u_ability_system_blueprint_library_unbind_gameplay_tag_changed_event_wrapper_for_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_unbind_all_gameplay_tag_changed_event_wrappers_for_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_target_data_has_origin: std::ptr::null_mut(),
            u_ability_system_blueprint_library_target_data_has_hit_result: std::ptr::null_mut(),
            u_ability_system_blueprint_library_target_data_has_end_point: std::ptr::null_mut(),
            u_ability_system_blueprint_library_target_data_has_actor: std::ptr::null_mut(),
            u_ability_system_blueprint_library_set_stack_count_to_max: std::ptr::null_mut(),
            u_ability_system_blueprint_library_set_stack_count: std::ptr::null_mut(),
            u_ability_system_blueprint_library_set_duration: std::ptr::null_mut(),
            u_ability_system_blueprint_library_send_gameplay_event_to_actor: std::ptr::null_mut(),
            u_ability_system_blueprint_library_remove_loose_gameplay_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_remove_gameplay_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_not_equal_gameplay_attribute_gameplay_attribute: std::ptr::null_mut(),
            u_ability_system_blueprint_library_not_equal_gameplay_ability_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_not_equal_active_gameplay_effect_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_make_spec_handle_by_class: std::ptr::null_mut(),
            u_ability_system_blueprint_library_make_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_make_gameplay_cue_parameters: std::ptr::null_mut(),
            u_ability_system_blueprint_library_make_filter_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_is_valid: std::ptr::null_mut(),
            u_ability_system_blueprint_library_is_instigator_locally_controlled_player: std::ptr::null_mut(),
            u_ability_system_blueprint_library_is_instigator_locally_controlled: std::ptr::null_mut(),
            u_ability_system_blueprint_library_is_gameplay_ability_active: std::ptr::null_mut(),
            u_ability_system_blueprint_library_is_duration_gameplay_effect_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_is_active_gameplay_effect_handle_valid: std::ptr::null_mut(),
            u_ability_system_blueprint_library_is_active_gameplay_effect_handle_active: std::ptr::null_mut(),
            u_ability_system_blueprint_library_has_hit_result: std::ptr::null_mut(),
            u_ability_system_blueprint_library_has_any_abilities_with_asset_tag: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_target_data_origin: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_target_data_end_point_transform: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_target_data_end_point: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_origin: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_modified_attribute_magnitude: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_instigator_transform: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_instigator_actor: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_hit_result_from_target_data: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_hit_result: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_granted_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_effect_ui_data: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_effect_granted_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_effect_from_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_effect_from_active_effect_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_effect_asset_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_cue_end_location_and_normal: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_cue_direction: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_gameplay_ability_from_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_float_attribute_from_ability_system_component: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_float_attribute_base_from_ability_system_component: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_float_attribute_base: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_float_attribute: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_effect_context: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_duration_policy_from_gameplay_effect_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_debug_string_from_gameplay_attribute: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_data_count_from_target_data: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_asset_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_all_linked_gameplay_effect_spec_handles: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_all_actors_from_target_data: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_actors_from_target_data: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_actor_count: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_actor_by_index: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_active_gameplay_effect_total_duration: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_active_gameplay_effect_start_time: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_active_gameplay_effect_stack_limit_count: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_active_gameplay_effect_stack_count: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_active_gameplay_effect_remaining_duration: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_active_gameplay_effect_expected_end_time: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_active_gameplay_effect_debug_string: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_ability_system_component_from_active_gameplay_effect_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_get_ability_system_component: std::ptr::null_mut(),
            u_ability_system_blueprint_library_forward_gameplay_cue_to_target: std::ptr::null_mut(),
            u_ability_system_blueprint_library_filter_target_data: std::ptr::null_mut(),
            u_ability_system_blueprint_library_evaluate_attribute_value_with_tags_and_base: std::ptr::null_mut(),
            u_ability_system_blueprint_library_evaluate_attribute_value_with_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_equal_equal_gameplay_attribute_gameplay_attribute: std::ptr::null_mut(),
            u_ability_system_blueprint_library_equal_equal_gameplay_ability_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_equal_equal_active_gameplay_effect_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_set_origin: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_is_valid: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_is_instigator_locally_controlled: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_has_hit_result: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_get_source_object: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_get_original_instigator_actor: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_get_origin: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_get_instigator_actor: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_get_hit_result: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_get_effect_causer: std::ptr::null_mut(),
            u_ability_system_blueprint_library_effect_context_add_hit_result: std::ptr::null_mut(),
            u_ability_system_blueprint_library_does_target_data_contain_actor: std::ptr::null_mut(),
            u_ability_system_blueprint_library_does_gameplay_cue_meet_tag_requirements: std::ptr::null_mut(),
            u_ability_system_blueprint_library_conv_scalable_float_to_float: std::ptr::null_mut(),
            u_ability_system_blueprint_library_conv_scalable_float_to_double: std::ptr::null_mut(),
            u_ability_system_blueprint_library_clone_spec_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_break_gameplay_cue_parameters: std::ptr::null_mut(),
            u_ability_system_blueprint_library_bind_event_wrapper_to_gameplay_tag_changed: std::ptr::null_mut(),
            u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tags_changed: std::ptr::null_mut(),
            u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tag_container_changed: std::ptr::null_mut(),
            u_ability_system_blueprint_library_assign_tag_set_by_caller_magnitude: std::ptr::null_mut(),
            u_ability_system_blueprint_library_assign_set_by_caller_magnitude: std::ptr::null_mut(),
            u_ability_system_blueprint_library_append_target_data_handle: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_loose_gameplay_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_linked_gameplay_effect_spec: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_linked_gameplay_effect: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_granted_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_granted_tag: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_gameplay_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_asset_tags: std::ptr::null_mut(),
            u_ability_system_blueprint_library_add_asset_tag: std::ptr::null_mut(),
            u_ability_system_blueprint_library_ability_target_data_from_locations: std::ptr::null_mut(),
            u_ability_system_blueprint_library_ability_target_data_from_hit_result: std::ptr::null_mut(),
            u_ability_system_blueprint_library_ability_target_data_from_actor_array: std::ptr::null_mut(),
            u_ability_system_blueprint_library_ability_target_data_from_actor: std::ptr::null_mut(),
            u_ability_system_cheat_manager_extension_effect_remove: std::ptr::null_mut(),
            u_ability_system_cheat_manager_extension_effect_list_active: std::ptr::null_mut(),
            u_ability_system_cheat_manager_extension_effect_apply: std::ptr::null_mut(),
            u_ability_system_cheat_manager_extension_ability_list_granted: std::ptr::null_mut(),
            u_ability_system_cheat_manager_extension_ability_grant: std::ptr::null_mut(),
            u_ability_system_cheat_manager_extension_ability_cancel: std::ptr::null_mut(),
            u_ability_system_cheat_manager_extension_ability_activate: std::ptr::null_mut(),
            u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitudes: std::ptr::null_mut(),
            u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitude: std::ptr::null_mut(),
            u_ability_system_component_try_activate_ability_by_class: std::ptr::null_mut(),
            u_ability_system_component_try_activate_ability: std::ptr::null_mut(),
            u_ability_system_component_try_activate_abilities_by_tag: std::ptr::null_mut(),
            u_ability_system_component_target_confirm: std::ptr::null_mut(),
            u_ability_system_component_target_cancel: std::ptr::null_mut(),
            u_ability_system_component_set_user_ability_activation_inhibited: std::ptr::null_mut(),
            u_ability_system_component_set_active_gameplay_effect_level_using_query: std::ptr::null_mut(),
            u_ability_system_component_set_active_gameplay_effect_level: std::ptr::null_mut(),
            u_ability_system_component_server_try_activate_ability_with_event_data: std::ptr::null_mut(),
            u_ability_system_component_server_try_activate_ability: std::ptr::null_mut(),
            u_ability_system_component_server_set_replicated_target_data_cancelled: std::ptr::null_mut(),
            u_ability_system_component_server_set_replicated_target_data: std::ptr::null_mut(),
            u_ability_system_component_server_set_replicated_event_with_payload: std::ptr::null_mut(),
            u_ability_system_component_server_set_replicated_event: std::ptr::null_mut(),
            u_ability_system_component_server_set_input_released: std::ptr::null_mut(),
            u_ability_system_component_server_set_input_pressed: std::ptr::null_mut(),
            u_ability_system_component_server_print_debug_request_with_strings: std::ptr::null_mut(),
            u_ability_system_component_server_print_debug_request: std::ptr::null_mut(),
            u_ability_system_component_server_end_ability: std::ptr::null_mut(),
            u_ability_system_component_server_current_montage_set_play_rate: std::ptr::null_mut(),
            u_ability_system_component_server_current_montage_set_next_section_name: std::ptr::null_mut(),
            u_ability_system_component_server_current_montage_jump_to_section_name: std::ptr::null_mut(),
            u_ability_system_component_server_cancel_ability: std::ptr::null_mut(),
            u_ability_system_component_server_ability_rpc_batch: std::ptr::null_mut(),
            u_ability_system_component_remove_active_gameplay_effect_by_source_effect: std::ptr::null_mut(),
            u_ability_system_component_remove_active_gameplay_effect: std::ptr::null_mut(),
            u_ability_system_component_remove_active_effects_with_tags: std::ptr::null_mut(),
            u_ability_system_component_remove_active_effects_with_source_tags: std::ptr::null_mut(),
            u_ability_system_component_remove_active_effects_with_granted_tags: std::ptr::null_mut(),
            u_ability_system_component_remove_active_effects_with_applied_tags: std::ptr::null_mut(),
            u_ability_system_component_release_input_id: std::ptr::null_mut(),
            u_ability_system_component_press_input_id: std::ptr::null_mut(),
            u_ability_system_component_on_spawned_attributes_end_played: std::ptr::null_mut(),
            u_ability_system_component_on_rep_spawned_attributes: std::ptr::null_mut(),
            u_ability_system_component_on_rep_server_debug_string: std::ptr::null_mut(),
            u_ability_system_component_on_rep_replicated_anim_montage: std::ptr::null_mut(),
            u_ability_system_component_on_rep_owning_actor: std::ptr::null_mut(),
            u_ability_system_component_on_rep_client_debug_string: std::ptr::null_mut(),
            u_ability_system_component_on_rep_activate_abilities: std::ptr::null_mut(),
            u_ability_system_component_on_owner_actor_destroyed: std::ptr::null_mut(),
            u_ability_system_component_on_avatar_actor_destroyed: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cues_executed_with_params: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cues_executed: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cues_added_and_while_active_with_params: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cue_executed_with_params: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cue_executed_from_spec: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cue_executed: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cue_added_and_while_active_with_params: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cue_added_and_while_active_from_spec: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cue_added_with_params: std::ptr::null_mut(),
            u_ability_system_component_net_multicast_invoke_gameplay_cue_added: std::ptr::null_mut(),
            u_ability_system_component_make_outgoing_spec: std::ptr::null_mut(),
            u_ability_system_component_make_effect_context: std::ptr::null_mut(),
            u_ability_system_component_k2_init_stats: std::ptr::null_mut(),
            u_ability_system_component_k2_give_ability_and_activate_once: std::ptr::null_mut(),
            u_ability_system_component_k2_give_ability: std::ptr::null_mut(),
            u_ability_system_component_is_gameplay_cue_active: std::ptr::null_mut(),
            u_ability_system_component_input_confirm: std::ptr::null_mut(),
            u_ability_system_component_input_cancel: std::ptr::null_mut(),
            u_ability_system_component_get_user_ability_activation_inhibited: std::ptr::null_mut(),
            u_ability_system_component_get_gameplay_tag_count: std::ptr::null_mut(),
            u_ability_system_component_get_gameplay_effect_magnitude: std::ptr::null_mut(),
            u_ability_system_component_get_gameplay_effect_count_if_loaded: std::ptr::null_mut(),
            u_ability_system_component_get_gameplay_effect_count: std::ptr::null_mut(),
            u_ability_system_component_get_gameplay_attribute_value: std::ptr::null_mut(),
            u_ability_system_component_get_attribute_set: std::ptr::null_mut(),
            u_ability_system_component_get_all_attributes: std::ptr::null_mut(),
            u_ability_system_component_get_all_abilities: std::ptr::null_mut(),
            u_ability_system_component_get_active_effects_with_all_tags: std::ptr::null_mut(),
            u_ability_system_component_get_active_effects: std::ptr::null_mut(),
            u_ability_system_component_find_all_abilities_with_tags: std::ptr::null_mut(),
            u_ability_system_component_find_all_abilities_with_input_id: std::ptr::null_mut(),
            u_ability_system_component_find_all_abilities_matching_query: std::ptr::null_mut(),
            u_ability_system_component_client_try_activate_ability: std::ptr::null_mut(),
            u_ability_system_component_client_set_replicated_event: std::ptr::null_mut(),
            u_ability_system_component_client_print_debug_response: std::ptr::null_mut(),
            u_ability_system_component_client_end_ability: std::ptr::null_mut(),
            u_ability_system_component_client_cancel_ability: std::ptr::null_mut(),
            u_ability_system_component_client_activate_ability_succeed_with_event_data: std::ptr::null_mut(),
            u_ability_system_component_client_activate_ability_succeed: std::ptr::null_mut(),
            u_ability_system_component_client_activate_ability_failed: std::ptr::null_mut(),
            u_ability_system_component_clear_all_abilities_with_input_id: std::ptr::null_mut(),
            u_ability_system_component_clear_all_abilities: std::ptr::null_mut(),
            u_ability_system_component_clear_ability: std::ptr::null_mut(),
            u_ability_system_component_bp_apply_gameplay_effect_to_target: std::ptr::null_mut(),
            u_ability_system_component_bp_apply_gameplay_effect_to_self: std::ptr::null_mut(),
            u_ability_system_component_bp_apply_gameplay_effect_spec_to_target: std::ptr::null_mut(),
            u_ability_system_component_bp_apply_gameplay_effect_spec_to_self: std::ptr::null_mut(),
            u_ability_system_component_ability_confirm_or_cancel_delegate_signature: std::ptr::null_mut(),
            u_ability_system_component_ability_ability_key_delegate_signature: std::ptr::null_mut(),
            u_gameplay_cue_function_library_remove_gameplay_cue_on_actor: std::ptr::null_mut(),
            u_gameplay_cue_function_library_make_gameplay_cue_parameters_from_hit_result: std::ptr::null_mut(),
            u_gameplay_cue_function_library_execute_gameplay_cue_on_actor: std::ptr::null_mut(),
            u_gameplay_cue_function_library_add_gameplay_cue_on_actor: std::ptr::null_mut(),
            u_gameplay_cue_interface_forward_gameplay_cue_to_parent: std::ptr::null_mut(),
            u_gameplay_cue_interface_blueprint_custom_handler: std::ptr::null_mut(),
            a_gameplay_cue_notify_actor_while_active: std::ptr::null_mut(),
            a_gameplay_cue_notify_actor_on_remove: std::ptr::null_mut(),
            a_gameplay_cue_notify_actor_on_owner_destroyed: std::ptr::null_mut(),
            a_gameplay_cue_notify_actor_on_execute: std::ptr::null_mut(),
            a_gameplay_cue_notify_actor_on_active: std::ptr::null_mut(),
            a_gameplay_cue_notify_actor_k2_handle_gameplay_cue: std::ptr::null_mut(),
            a_gameplay_cue_notify_actor_k2_end_gameplay_cue: std::ptr::null_mut(),
            u_gameplay_cue_notify_burst_on_burst: std::ptr::null_mut(),
            a_gameplay_cue_notify_burst_latent_on_burst: std::ptr::null_mut(),
            a_gameplay_cue_notify_looping_on_removal: std::ptr::null_mut(),
            a_gameplay_cue_notify_looping_on_recurring: std::ptr::null_mut(),
            a_gameplay_cue_notify_looping_on_looping_start: std::ptr::null_mut(),
            a_gameplay_cue_notify_looping_on_application: std::ptr::null_mut(),
            u_gameplay_effect_custom_application_requirement_can_apply_gameplay_effect: std::ptr::null_mut(),
            u_gameplay_effect_execution_calculation_execute: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_k2_get_captured_attribute_magnitude: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_target_spec_tags: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_target_aggregated_tags: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_target_actor_tags: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_source_spec_tags: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_source_aggregated_tags: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_source_actor_tags: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_tag: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_name: std::ptr::null_mut(),
            u_gameplay_mod_magnitude_calculation_calculate_base_magnitude: std::ptr::null_mut(),
            u_gameplay_tag_reponse_table_tag_response_event: std::ptr::null_mut(),
            u_movie_scene_gameplay_cue_track_set_sequencer_track_handler: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCueNotify_Static::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WhileActive"),
                &raw mut __FUNCTION_PTRS.u_gameplay_cue_notify_static_while_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRemove"),
                &raw mut __FUNCTION_PTRS.u_gameplay_cue_notify_static_on_remove,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnExecute"),
                &raw mut __FUNCTION_PTRS.u_gameplay_cue_notify_static_on_execute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnActive"),
                &raw mut __FUNCTION_PTRS.u_gameplay_cue_notify_static_on_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_HandleGameplayCue"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_k2_handle_gameplay_cue,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EndAction"),
                &raw mut __FUNCTION_PTRS.u_ability_async_end_action,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitAttributeChanged::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAttributeChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_attribute_changed_wait_for_attribute_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "AsyncWaitAttributeChangedDelegate__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_attribute_changed_async_wait_attribute_changed_delegate_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitGameplayEffectApplied::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEffectAppliedToActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_effect_applied_wait_gameplay_effect_applied_to_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnAppliedDelegate__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_effect_applied_on_applied_delegate_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitGameplayEvent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEventToActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_event_wait_gameplay_event_to_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EventReceivedDelegate__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_event_event_received_delegate_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitGameplayTag::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "AsyncWaitGameplayTagDelegate__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_async_wait_gameplay_tag_delegate_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitGameplayTagAdded::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagAddToActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_added_wait_gameplay_tag_add_to_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitGameplayTagRemoved::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagRemoveFromActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_removed_wait_gameplay_tag_remove_from_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitGameplayTagCountChanged::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagCountChangedOnActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_changed_on_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "AsyncWaitGameplayTagCountDelegate__DelegateSignature",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_count_changed_async_wait_gameplay_tag_count_delegate_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityAsync_WaitGameplayTagQuery::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagQueryOnActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_query_wait_gameplay_tag_query_on_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayAbility::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetShouldBlockOtherAbilities"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_set_should_block_other_abilities,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCanBeCanceled"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_set_can_be_canceled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SendGameplayEvent"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_send_gameplay_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveGrantedByEffect"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_remove_granted_by_effect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MontageStop"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_montage_stop,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MontageSetNextSectionName"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_montage_set_next_section_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MontageJumpToSection"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_montage_jump_to_section,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "MakeTargetLocationInfoFromOwnerSkeletalMeshComponent",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_make_target_location_info_from_owner_skeletal_mesh_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeTargetLocationInfoFromOwnerActor"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_make_target_location_info_from_owner_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeOutgoingGameplayEffectSpec"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_make_outgoing_gameplay_effect_spec,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ShouldAbilityRespondToEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_k2_should_ability_respond_to_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_RemoveGameplayCue"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_remove_gameplay_cue,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnEndAbility"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_on_end_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_HasAuthority"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_has_authority,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ExecuteGameplayCueWithParams"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_k2_execute_gameplay_cue_with_params,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ExecuteGameplayCue"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_execute_gameplay_cue,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_EndAbilityLocally"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_end_ability_locally,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_EndAbility"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_end_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CommitExecute"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_commit_execute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CommitAbilityCost"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_commit_ability_cost,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CommitAbilityCooldown"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_commit_ability_cooldown,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CommitAbility"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_commit_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CheckAbilityCost"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_check_ability_cost,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CheckAbilityCooldown"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_check_ability_cooldown,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CancelAbility"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_cancel_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_CanActivateAbility"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_can_activate_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ApplyGameplayEffectSpecToTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_k2_apply_gameplay_effect_spec_to_target,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ApplyGameplayEffectSpecToOwner"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_k2_apply_gameplay_effect_spec_to_owner,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_AddGameplayCueWithParams"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_k2_add_gameplay_cue_with_params,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_AddGameplayCue"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_add_gameplay_cue,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ActivateAbilityFromEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_k2_activate_ability_from_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ActivateAbility"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_k2_activate_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLocallyControlled"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_is_locally_controlled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InvalidateClientPredictionKey"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_invalidate_client_prediction_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceObject_BP"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_source_object_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOwningComponentFromActorInfo"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_get_owning_component_from_actor_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOwningActorFromActorInfo"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_get_owning_actor_from_actor_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGrantedByEffectContext"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_granted_by_effect_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentSourceObject"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_current_source_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentMontage"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_current_montage,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCooldownTimeRemaining"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_cooldown_time_remaining,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetContextFromOwner"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_context_from_owner,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAvatarActorFromActorInfo"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_get_avatar_actor_from_actor_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorInfo"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_actor_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAbilitySystemComponentFromActorInfo"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_get_ability_system_component_from_actor_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAbilityLevel_BP"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_ability_level_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAbilityLevel"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_get_ability_level,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EndTaskByInstanceName"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_end_task_by_instance_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EndAbilityState"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_end_ability_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConfirmTaskByInstanceName"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_confirm_task_by_instance_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelTaskByInstanceName"),
                &raw mut __FUNCTION_PTRS.u_gameplay_ability_cancel_task_by_instance_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_RemoveGameplayEffectFromOwnerWithHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "BP_RemoveGameplayEffectFromOwnerWithGrantedTags",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_granted_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "BP_RemoveGameplayEffectFromOwnerWithAssetTags",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_asset_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ApplyGameplayEffectToTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_bp_apply_gameplay_effect_to_target,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ApplyGameplayEffectToOwner"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_ability_bp_apply_gameplay_effect_to_owner,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayAbilityTargetActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConfirmTargeting"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_ability_target_actor_confirm_targeting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelTargeting"),
                &raw mut __FUNCTION_PTRS.a_gameplay_ability_target_actor_cancel_targeting,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayAbilityWorldReticle::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetReticleMaterialParamVector"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_set_reticle_material_param_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetReticleMaterialParamFloat"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_set_reticle_material_param_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnValidTargetChanged"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_valid_target_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTargetingAnActor"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_targeting_an_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnParametersInitialized"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_parameters_initialized,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FaceTowardSource"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_face_toward_source,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_ApplyRootMotionConstantForce::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyRootMotionConstantForce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_constant_force_apply_root_motion_constant_force,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_ApplyRootMotionJumpForce::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnLandedCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_jump_force_on_landed_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Finish"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_jump_force_finish,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyRootMotionJumpForce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_jump_force_apply_root_motion_jump_force,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_ApplyRootMotionMoveToActorForce::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTargetActorSwapped"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_on_target_actor_swapped,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_TargetLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_on_rep_target_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyRootMotionMoveToTargetDataActorForce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_target_data_actor_force,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyRootMotionMoveToComponentForce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_component_force,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyRootMotionMoveToActorForce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_actor_force,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_ApplyRootMotionMoveToForce::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyRootMotionMoveToForce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_force_apply_root_motion_move_to_force,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_ApplyRootMotionRadialForce::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyRootMotionRadialForce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_radial_force_apply_root_motion_radial_force,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_MoveToLocation::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveToLocation"),
                &raw mut __FUNCTION_PTRS.u_ability_task_move_to_location_move_to_location,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_NetworkSyncPoint::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitNetSync"),
                &raw mut __FUNCTION_PTRS.u_ability_task_network_sync_point_wait_net_sync,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnSignalCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_network_sync_point_on_signal_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_PlayAnimAndWait::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageInterrupted"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_anim_and_wait_on_montage_interrupted,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageEnded"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_anim_and_wait_on_montage_ended,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageBlendingOut"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_anim_and_wait_on_montage_blending_out,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageBlendedIn"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_anim_and_wait_on_montage_blended_in,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreatePlayAnimAndWaitProxy"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_anim_and_wait_create_play_anim_and_wait_proxy,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_PlayMontageAndWait::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageInterrupted"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_on_montage_interrupted,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageEnded"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_on_montage_ended,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageBlendingOut"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_on_montage_blending_out,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMontageBlendedIn"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_on_montage_blended_in,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnGameplayAbilityCancelled"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_on_gameplay_ability_cancelled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreatePlayMontageAndWaitProxy"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_create_play_montage_and_wait_proxy,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_Repeat::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RepeatAction"),
                &raw mut __FUNCTION_PTRS.u_ability_task_repeat_repeat_action,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_SpawnActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnActor"),
                &raw mut __FUNCTION_PTRS.u_ability_task_spawn_actor_spawn_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FinishSpawningActor"),
                &raw mut __FUNCTION_PTRS.u_ability_task_spawn_actor_finish_spawning_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BeginSpawningActor"),
                &raw mut __FUNCTION_PTRS.u_ability_task_spawn_actor_begin_spawning_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_StartAbilityState::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartAbilityState"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_start_ability_state_start_ability_state,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_VisualizeTargeting::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("VisualizeTargetingUsingActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_visualize_targeting_using_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("VisualizeTargeting"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_visualize_targeting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FinishSpawningActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_finish_spawning_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BeginSpawningActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_begin_spawning_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitAbilityActivate::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAbilityActivateWithTagRequirements"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate_with_tag_requirements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAbilityActivate_Query"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate_query,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAbilityActivate"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnAbilityActivate"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_on_ability_activate,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitAbilityCommit::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAbilityCommit_Query"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_ability_commit_wait_for_ability_commit_query,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAbilityCommit"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_ability_commit_wait_for_ability_commit,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnAbilityCommit"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_ability_commit_on_ability_commit,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitAttributeChange::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAttributeChangeWithComparison"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_wait_for_attribute_change_with_comparison,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAttributeChange"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_wait_for_attribute_change,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitAttributeChangeRatioThreshold::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAttributeChangeRatioThreshold"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_ratio_threshold_wait_for_attribute_change_ratio_threshold,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitAttributeChangeThreshold::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForAttributeChangeThreshold"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_threshold_wait_for_attribute_change_threshold,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitCancel::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitCancel"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_cancel_wait_cancel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnLocalCancelCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_cancel_on_local_cancel_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnCancelCallback"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_cancel_on_cancel_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitConfirm::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitConfirm"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_confirm_wait_confirm,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnConfirmCallback"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_confirm_on_confirm_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitConfirmCancel::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitConfirmCancel"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_confirm_cancel_wait_confirm_cancel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnLocalConfirmCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_confirm_cancel_on_local_confirm_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnLocalCancelCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_confirm_cancel_on_local_cancel_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnConfirmCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_confirm_cancel_on_confirm_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnCancelCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_confirm_cancel_on_cancel_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitDelay::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitDelay"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_delay_wait_delay,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayEffectApplied::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnApplyGameplayEffectCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_on_apply_gameplay_effect_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayEffectApplied_Self::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEffectAppliedToSelf_Query"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self_query,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEffectAppliedToSelf"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayEffectApplied_Target::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEffectAppliedToTarget_Query"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target_query,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEffectAppliedToTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayEffectBlockedImmunity::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEffectBlockedByImmunity"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_blocked_immunity_wait_gameplay_effect_blocked_by_immunity,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayEffectRemoved::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForGameplayEffectRemoved"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_removed_wait_for_gameplay_effect_removed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnGameplayEffectRemoved"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_removed_on_gameplay_effect_removed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayEffectStackChange::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForGameplayEffectStackChange"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_stack_change_wait_for_gameplay_effect_stack_change,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnGameplayEffectStackChange"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_stack_change_on_gameplay_effect_stack_change,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayEvent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_event_wait_gameplay_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayTag::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GameplayTagCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_gameplay_tag_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayTagAdded::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagAdd"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_added_wait_gameplay_tag_add,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayTagRemoved::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagRemove"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_removed_wait_gameplay_tag_remove,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayTagCountChanged::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagCountChange"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_change,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitGameplayTagQuery::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitGameplayTagQuery"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_query_wait_gameplay_tag_query,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateTargetTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_query_update_target_tags,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitInputPress::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitInputPress"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_input_press_wait_input_press,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPressCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_input_press_on_press_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitInputRelease::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitInputRelease"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_input_release_wait_input_release,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnReleaseCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_input_release_on_release_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitMovementModeChange::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMovementModeChange"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_movement_mode_change_on_movement_mode_change,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateWaitMovementModeChange"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_movement_mode_change_create_wait_movement_mode_change,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitOverlap::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitForOverlap"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_overlap_wait_for_overlap,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnHitCallback"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_overlap_on_hit_callback,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitTargetData::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitTargetDataUsingActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_target_data_wait_target_data_using_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WaitTargetData"),
                &raw mut __FUNCTION_PTRS.u_ability_task_wait_target_data_wait_target_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTargetDataReplicatedCancelledCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_target_data_on_target_data_replicated_cancelled_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTargetDataReplicatedCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_target_data_on_target_data_replicated_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTargetDataReadyCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_target_data_on_target_data_ready_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTargetDataCancelledCallback"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_target_data_on_target_data_cancelled_callback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FinishSpawningActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_target_data_finish_spawning_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BeginSpawningActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_target_data_begin_spawning_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilityTask_WaitVelocityChange::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateWaitVelocityChange"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_task_wait_velocity_change_create_wait_velocity_change,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilitySystemBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "UnbindGameplayTagChangedEventWrapperForHandle",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_unbind_gameplay_tag_changed_event_wrapper_for_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "UnbindAllGameplayTagChangedEventWrappersForHandle",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_unbind_all_gameplay_tag_changed_event_wrappers_for_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TargetDataHasOrigin"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TargetDataHasHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TargetDataHasEndPoint"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_end_point,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TargetDataHasActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStackCountToMax"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_stack_count_to_max,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStackCount"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_stack_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDuration"),
                &raw mut __FUNCTION_PTRS.u_ability_system_blueprint_library_set_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SendGameplayEventToActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_send_gameplay_event_to_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveLooseGameplayTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_remove_loose_gameplay_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveGameplayTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_remove_gameplay_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotEqual_GameplayAttributeGameplayAttribute"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_gameplay_attribute_gameplay_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotEqual_GameplayAbilitySpecHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_gameplay_ability_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotEqual_ActiveGameplayEffectHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_active_gameplay_effect_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeSpecHandleByClass"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_spec_handle_by_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeSpecHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeGameplayCueParameters"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_gameplay_cue_parameters,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeFilterHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_filter_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValid"),
                &raw mut __FUNCTION_PTRS.u_ability_system_blueprint_library_is_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInstigatorLocallyControlledPlayer"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_instigator_locally_controlled_player,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInstigatorLocallyControlled"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_instigator_locally_controlled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsGameplayAbilityActive"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_gameplay_ability_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDurationGameplayEffectSpecHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_duration_gameplay_effect_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActiveGameplayEffectHandleValid"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_active_gameplay_effect_handle_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsActiveGameplayEffectHandleActive"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_active_gameplay_effect_handle_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_has_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasAnyAbilitiesWithAssetTag"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_has_any_abilities_with_asset_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetDataOrigin"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetDataEndPointTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_end_point_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetDataEndPoint"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_end_point,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOrigin"),
                &raw mut __FUNCTION_PTRS.u_ability_system_blueprint_library_get_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModifiedAttributeMagnitude"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_modified_attribute_magnitude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstigatorTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_instigator_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInstigatorActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_instigator_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHitResultFromTargetData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_hit_result_from_target_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGrantedTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_granted_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectUIData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_ui_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectGrantedTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_granted_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectFromSpecHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_from_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectFromActiveEffectHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_from_active_effect_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectAssetTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_asset_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayCueEndLocationAndNormal"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_cue_end_location_and_normal,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayCueDirection"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_cue_direction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayAbilityFromSpecHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_ability_from_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatAttributeFromAbilitySystemComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_from_ability_system_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "GetFloatAttributeBaseFromAbilitySystemComponent",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_base_from_ability_system_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatAttributeBase"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_base,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatAttribute"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEffectContext"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_effect_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "GetDurationPolicyFromGameplayEffectSpecHandle",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_duration_policy_from_gameplay_effect_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDebugStringFromGameplayAttribute"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_debug_string_from_gameplay_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDataCountFromTargetData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_data_count_from_target_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssetTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_asset_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllLinkedGameplayEffectSpecHandles"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_all_linked_gameplay_effect_spec_handles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllActorsFromTargetData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_all_actors_from_target_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorsFromTargetData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actors_from_target_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorCount"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actor_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actor_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveGameplayEffectTotalDuration"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_total_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveGameplayEffectStartTime"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_start_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveGameplayEffectStackLimitCount"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_stack_limit_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveGameplayEffectStackCount"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_stack_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveGameplayEffectRemainingDuration"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_remaining_duration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveGameplayEffectExpectedEndTime"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_expected_end_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveGameplayEffectDebugString"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_debug_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "GetAbilitySystemComponentFromActiveGameplayEffectHandle",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_ability_system_component_from_active_gameplay_effect_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAbilitySystemComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_ability_system_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForwardGameplayCueToTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_forward_gameplay_cue_to_target,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FilterTargetData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_filter_target_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateAttributeValueWithTagsAndBase"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_evaluate_attribute_value_with_tags_and_base,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EvaluateAttributeValueWithTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_evaluate_attribute_value_with_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "EqualEqual_GameplayAttributeGameplayAttribute",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_gameplay_attribute_gameplay_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EqualEqual_GameplayAbilitySpecHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_gameplay_ability_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EqualEqual_ActiveGameplayEffectHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_active_gameplay_effect_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextSetOrigin"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_set_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextIsValid"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_is_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextIsInstigatorLocallyControlled"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_is_instigator_locally_controlled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextHasHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_has_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextGetSourceObject"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_source_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextGetOriginalInstigatorActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_original_instigator_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextGetOrigin"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextGetInstigatorActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_instigator_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextGetHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextGetEffectCauser"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_effect_causer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectContextAddHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_add_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoesTargetDataContainActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_does_target_data_contain_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoesGameplayCueMeetTagRequirements"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_does_gameplay_cue_meet_tag_requirements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_ScalableFloatToFloat"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_conv_scalable_float_to_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Conv_ScalableFloatToDouble"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_conv_scalable_float_to_double,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CloneSpecHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_clone_spec_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BreakGameplayCueParameters"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_break_gameplay_cue_parameters,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BindEventWrapperToGameplayTagChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_gameplay_tag_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BindEventWrapperToAnyOfGameplayTagsChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tags_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "BindEventWrapperToAnyOfGameplayTagContainerChanged",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tag_container_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AssignTagSetByCallerMagnitude"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_assign_tag_set_by_caller_magnitude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AssignSetByCallerMagnitude"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_assign_set_by_caller_magnitude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AppendTargetDataHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_append_target_data_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLooseGameplayTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_loose_gameplay_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLinkedGameplayEffectSpec"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_linked_gameplay_effect_spec,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLinkedGameplayEffect"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_linked_gameplay_effect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGrantedTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_granted_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGrantedTag"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_granted_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGameplayTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_gameplay_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddAssetTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_asset_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddAssetTag"),
                &raw mut __FUNCTION_PTRS.u_ability_system_blueprint_library_add_asset_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityTargetDataFromLocations"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_locations,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityTargetDataFromHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityTargetDataFromActorArray"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_actor_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityTargetDataFromActor"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilitySystemCheatManagerExtension::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectRemove"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_cheat_manager_extension_effect_remove,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectListActive"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_cheat_manager_extension_effect_list_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EffectApply"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_cheat_manager_extension_effect_apply,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityListGranted"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_cheat_manager_extension_ability_list_granted,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityGrant"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_cheat_manager_extension_ability_grant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityCancel"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_cheat_manager_extension_ability_cancel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityActivate"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_cheat_manager_extension_ability_activate,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAbilitySystemComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "UpdateActiveGameplayEffectSetByCallerMagnitudes",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitudes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "UpdateActiveGameplayEffectSetByCallerMagnitude",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryActivateAbilityByClass"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_try_activate_ability_by_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryActivateAbility"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_try_activate_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryActivateAbilitiesByTag"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_try_activate_abilities_by_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TargetConfirm"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_target_confirm,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TargetCancel"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_target_cancel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUserAbilityActivationInhibited"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_set_user_ability_activation_inhibited,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActiveGameplayEffectLevelUsingQuery"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_set_active_gameplay_effect_level_using_query,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActiveGameplayEffectLevel"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_set_active_gameplay_effect_level,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerTryActivateAbilityWithEventData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_try_activate_ability_with_event_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerTryActivateAbility"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_try_activate_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetReplicatedTargetDataCancelled"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_set_replicated_target_data_cancelled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetReplicatedTargetData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_set_replicated_target_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetReplicatedEventWithPayload"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_set_replicated_event_with_payload,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetReplicatedEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_set_replicated_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetInputReleased"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_set_input_released,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerSetInputPressed"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_set_input_pressed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerPrintDebug_RequestWithStrings"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_print_debug_request_with_strings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerPrintDebug_Request"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_print_debug_request,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerEndAbility"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_server_end_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerCurrentMontageSetPlayRate"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_current_montage_set_play_rate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerCurrentMontageSetNextSectionName"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_current_montage_set_next_section_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerCurrentMontageJumpToSectionName"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_current_montage_jump_to_section_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerCancelAbility"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_server_cancel_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ServerAbilityRPCBatch"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_server_ability_rpc_batch,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActiveGameplayEffectBySourceEffect"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_remove_active_gameplay_effect_by_source_effect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActiveGameplayEffect"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_remove_active_gameplay_effect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActiveEffectsWithTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActiveEffectsWithSourceTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_source_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActiveEffectsWithGrantedTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_granted_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveActiveEffectsWithAppliedTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_applied_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReleaseInputID"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_release_input_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PressInputID"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_press_input_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnSpawnedAttributesEndPlayed"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_spawned_attributes_end_played,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_SpawnedAttributes"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_rep_spawned_attributes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_ServerDebugString"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_rep_server_debug_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_ReplicatedAnimMontage"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_rep_replicated_anim_montage,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_OwningActor"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_on_rep_owning_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_ClientDebugString"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_rep_client_debug_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRep_ActivateAbilities"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_rep_activate_abilities,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnOwnerActorDestroyed"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_owner_actor_destroyed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnAvatarActorDestroyed"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_on_avatar_actor_destroyed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NetMulticast_InvokeGameplayCuesExecuted_WithParams",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cues_executed_with_params,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NetMulticast_InvokeGameplayCuesExecuted"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cues_executed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NetMulticast_InvokeGameplayCuesAddedAndWhileActive_WithParams",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cues_added_and_while_active_with_params,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NetMulticast_InvokeGameplayCueExecuted_WithParams",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cue_executed_with_params,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NetMulticast_InvokeGameplayCueExecuted_FromSpec",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cue_executed_from_spec,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NetMulticast_InvokeGameplayCueExecuted"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cue_executed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NetMulticast_InvokeGameplayCueAddedAndWhileActive_WithParams",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cue_added_and_while_active_with_params,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NetMulticast_InvokeGameplayCueAddedAndWhileActive_FromSpec",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cue_added_and_while_active_from_spec,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "NetMulticast_InvokeGameplayCueAdded_WithParams",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cue_added_with_params,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NetMulticast_InvokeGameplayCueAdded"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_net_multicast_invoke_gameplay_cue_added,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeOutgoingSpec"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_make_outgoing_spec,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeEffectContext"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_make_effect_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_InitStats"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_k2_init_stats,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GiveAbilityAndActivateOnce"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_k2_give_ability_and_activate_once,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GiveAbility"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_k2_give_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsGameplayCueActive"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_is_gameplay_cue_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InputConfirm"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_input_confirm,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InputCancel"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_input_cancel,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserAbilityActivationInhibited"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_get_user_ability_activation_inhibited,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayTagCount"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_tag_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectMagnitude"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_magnitude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectCount_IfLoaded"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_count_if_loaded,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayEffectCount"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGameplayAttributeValue"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_attribute_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAttributeSet"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_get_attribute_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllAttributes"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_get_all_attributes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllAbilities"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_get_all_abilities,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveEffectsWithAllTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_get_active_effects_with_all_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveEffects"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_get_active_effects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindAllAbilitiesWithTags"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_with_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindAllAbilitiesWithInputID"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_with_input_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindAllAbilitiesMatchingQuery"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_matching_query,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientTryActivateAbility"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_client_try_activate_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientSetReplicatedEvent"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_client_set_replicated_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientPrintDebug_Response"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_client_print_debug_response,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientEndAbility"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_client_end_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientCancelAbility"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_client_cancel_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientActivateAbilitySucceedWithEventData"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_client_activate_ability_succeed_with_event_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientActivateAbilitySucceed"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_client_activate_ability_succeed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClientActivateAbilityFailed"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_client_activate_ability_failed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearAllAbilitiesWithInputID"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_clear_all_abilities_with_input_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearAllAbilities"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_clear_all_abilities,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearAbility"),
                &raw mut __FUNCTION_PTRS.u_ability_system_component_clear_ability,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ApplyGameplayEffectToTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_to_target,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ApplyGameplayEffectToSelf"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_to_self,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ApplyGameplayEffectSpecToTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_spec_to_target,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_ApplyGameplayEffectSpecToSelf"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_spec_to_self,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityConfirmOrCancel__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_ability_confirm_or_cancel_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AbilityAbilityKey__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_ability_system_component_ability_ability_key_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCueFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveGameplayCueOnActor"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_cue_function_library_remove_gameplay_cue_on_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeGameplayCueParametersFromHitResult"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_cue_function_library_make_gameplay_cue_parameters_from_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExecuteGameplayCueOnActor"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_cue_function_library_execute_gameplay_cue_on_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGameplayCueOnActor"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_cue_function_library_add_gameplay_cue_on_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCueInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForwardGameplayCueToParent"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_cue_interface_forward_gameplay_cue_to_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlueprintCustomHandler"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_cue_interface_blueprint_custom_handler,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayCueNotify_Actor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("WhileActive"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_actor_while_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRemove"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_actor_on_remove,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnOwnerDestroyed"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_actor_on_owner_destroyed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnExecute"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_actor_on_execute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnActive"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_actor_on_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_HandleGameplayCue"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_k2_handle_gameplay_cue,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_EndGameplayCue"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_actor_k2_end_gameplay_cue,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCueNotify_Burst::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnBurst"),
                &raw mut __FUNCTION_PTRS.u_gameplay_cue_notify_burst_on_burst,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayCueNotify_BurstLatent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnBurst"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_burst_latent_on_burst,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayCueNotify_Looping::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRemoval"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_looping_on_removal,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnRecurring"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_looping_on_recurring,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnLoopingStart"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_looping_on_looping_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnApplication"),
                &raw mut __FUNCTION_PTRS.a_gameplay_cue_notify_looping_on_application,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayEffectCustomApplicationRequirement::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanApplyGameplayEffect"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_effect_custom_application_requirement_can_apply_gameplay_effect,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayEffectExecutionCalculation::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Execute"),
                &raw mut __FUNCTION_PTRS.u_gameplay_effect_execution_calculation_execute,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayModMagnitudeCalculation::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetCapturedAttributeMagnitude"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_k2_get_captured_attribute_magnitude,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetSpecTags"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_spec_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetAggregatedTags"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_aggregated_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetActorTags"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_actor_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceSpecTags"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_spec_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceAggregatedTags"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_aggregated_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceActorTags"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_actor_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSetByCallerMagnitudeByTag"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSetByCallerMagnitudeByName"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CalculateBaseMagnitude"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_calculate_base_magnitude,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayTagReponseTable::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TagResponseEvent"),
                &raw mut __FUNCTION_PTRS.u_gameplay_tag_reponse_table_tag_response_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovieSceneGameplayCueTrack::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSequencerTrackHandler"),
                &raw mut __FUNCTION_PTRS
                    .u_movie_scene_gameplay_cue_track_set_sequencer_track_handler,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FActiveGameplayEffect {
    pub(crate) __padding_end: [u8; 904],
}
impl FActiveGameplayEffect {}
#[repr(C, align(4))]
pub struct FGameplayAbilitySpecHandle {
    pub(crate) __padding_end: [u8; 4],
}
impl FGameplayAbilitySpecHandle {}
#[repr(C, align(8))]
pub struct FGameplayEffectSpec {
    pub(crate) __padding_end: [u8; 664],
}
impl FGameplayEffectSpec {}
#[repr(C, align(8))]
pub struct FGameplayEffectContextHandle {
    pub(crate) __padding_end: [u8; 24],
}
impl FGameplayEffectContextHandle {}
#[repr(C, align(8))]
pub struct FGameplayAbilitySpecDef {
    pub(crate) __padding_end: [u8; 168],
}
impl FGameplayAbilitySpecDef {}
#[repr(C, align(8))]
pub struct FScalableFloat {
    pub value: f32,
    pub curve: crate::bindings::engine::FCurveTableRowHandle,
    pub registry_type: crate::bindings::data_registry::FDataRegistryType,
    pub(crate) __padding_end: [u8; 12],
}
impl FScalableFloat {}
#[repr(C, align(8))]
pub struct FGameplayEffectAttributeCaptureDefinition {
    pub(crate) __padding_end: [u8; 80],
}
impl FGameplayEffectAttributeCaptureDefinition {}
#[repr(C, align(8))]
pub struct FGameplayAttribute {
    pub attribute_name: FString,
    pub(crate) __padding_end: [u8; 56],
}
impl FGameplayAttribute {}
#[repr(C, align(4))]
pub struct FActiveGameplayEffectHandle {
    pub(crate) __padding_end: [u8; 8],
}
impl FActiveGameplayEffectHandle {}
#[repr(C, align(8))]
pub struct FGameplayEffectSpecHandle {
    pub(crate) __padding_end: [u8; 16],
}
impl FGameplayEffectSpecHandle {}
#[repr(C, align(8))]
pub struct FGameplayCueParameters {
    pub normalized_magnitude: f32,
    pub raw_magnitude: f32,
    pub effect_context: FGameplayEffectContextHandle,
    pub matched_tag_name: crate::bindings::gameplay_tags::FGameplayTag,
    pub original_tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub aggregated_source_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub aggregated_target_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub location: crate::bindings::engine::FVector_NetQuantize10,
    pub normal: crate::bindings::engine::FVector_NetQuantizeNormal,
    pub instigator: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub effect_causer: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub source_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub physical_material: TWeakObjectPtr<
        crate::bindings::physics_core::UPhysicalMaterial,
    >,
    pub gameplay_effect_level: i32,
    pub ability_level: i32,
    pub target_attach_component: TWeakObjectPtr<
        crate::bindings::engine::USceneComponent,
    >,
    pub b_replicate_location_when_using_minimal_rep_proxy: bool,
}
impl FGameplayCueParameters {}
#[repr(C, align(8))]
pub struct FGameplayEffectRemovalInfo {
    pub b_premature_removal: bool,
    pub b_prediction_rejected: bool,
    pub stack_count: i32,
    pub effect_context: FGameplayEffectContextHandle,
    pub(crate) __padding_end: [u8; 8],
}
impl FGameplayEffectRemovalInfo {}
#[repr(C, align(8))]
pub struct FGameplayEventData {
    pub event_tag: crate::bindings::gameplay_tags::FGameplayTag,
    pub instigator: UPtr<crate::bindings::engine::AActor>,
    pub target: UPtr<crate::bindings::engine::AActor>,
    pub optional_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub optional_object2: UPtr<crate::bindings::core_u_object::UObject>,
    pub context_handle: FGameplayEffectContextHandle,
    pub instigator_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub target_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub event_magnitude: f32,
    pub target_data: FGameplayAbilityTargetDataHandle,
}
impl FGameplayEventData {}
#[repr(C, align(8))]
pub struct FGameplayAbilityTargetDataHandle {
    pub(crate) __padding_end: [u8; 40],
}
impl FGameplayAbilityTargetDataHandle {}
#[repr(C, align(8))]
pub struct FGameplayTagRequirements {
    pub require_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub ignore_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
}
impl FGameplayTagRequirements {}
#[repr(C, align(8))]
pub struct FGameplayTargetDataFilterHandle {
    pub(crate) __padding_end: [u8; 16],
}
impl FGameplayTargetDataFilterHandle {}
#[repr(C, align(4))]
pub struct FGameplayAbilityActivationInfo {
    pub activation_mode: EGameplayAbilityActivationMode,
    pub(crate) __padding_end: [u8; 19],
}
impl FGameplayAbilityActivationInfo {}
#[repr(C, align(8))]
pub struct FGameplayEffectQuery {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub custom_match_delegate_bp: FGameplayEffectQuery_CustomMatchDelegate_BP,
    pub owning_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub effect_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub source_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub source_aggregate_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    pub modifying_attribute: FGameplayAttribute,
    pub effect_source: UPtr<crate::bindings::core_u_object::UObject>,
    pub effect_definition: TSubclassOf<UGameplayEffect>,
    pub(crate) __padding_end: [u8; 16],
}
impl FGameplayEffectQuery {}
#[repr(C, align(8))]
pub struct FActiveGameplayCueContainer {
    pub(crate) __padding_end: [u8; 304],
}
impl FActiveGameplayCueContainer {}
#[repr(C, align(8))]
pub struct FActiveGameplayCue {
    pub(crate) __padding_end: [u8; 272],
}
impl FActiveGameplayCue {}
#[repr(C, align(8))]
pub struct FGameplayAbilitySpecContainer {
    pub(crate) __padding_end: [u8; 296],
}
impl FGameplayAbilitySpecContainer {}
#[repr(C, align(8))]
pub struct FGameplayAbilitySpec {
    pub(crate) __padding_end: [u8; 240],
}
impl FGameplayAbilitySpec {}
#[repr(C, align(8))]
pub struct FGameplayTargetDataFilter {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub required_actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    pub self_filter: ETargetDataFilterSelf,
    pub b_reverse_filter: bool,
}
impl FGameplayTargetDataFilter {}
#[repr(C, align(8))]
pub struct FWorldReticleParameters {
    pub aoe_scale: crate::bindings::core_u_object::FVector,
}
impl FWorldReticleParameters {}
#[repr(C, align(8))]
pub struct FGameplayTagChangedEventWrapperSpecHandle {
    pub(crate) __padding_end: [u8; 16],
}
impl FGameplayTagChangedEventWrapperSpecHandle {}
#[repr(C, align(8))]
pub struct FGameplayAttributeData {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub base_value: f32,
    pub current_value: f32,
}
impl FGameplayAttributeData {}
#[repr(C, align(8))]
pub struct FAttributeMetaData {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub base_value: f32,
    pub min_value: f32,
    pub max_value: f32,
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 20],
    pub b_can_stack: bool,
}
impl FAttributeMetaData {}
#[repr(C, align(16))]
pub struct FGameplayAbilityTargetingLocationInfo {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub source_actor: UPtr<crate::bindings::engine::AActor>,
    pub source_component: UPtr<crate::bindings::engine::UMeshComponent>,
    pub source_ability: UPtr<UGameplayAbility>,
    pub literal_transform: crate::bindings::core_u_object::FTransform,
    pub source_socket_name: FName,
    pub location_type: EGameplayAbilityTargetingLocationType,
}
impl FGameplayAbilityTargetingLocationInfo {}
#[repr(C, align(16))]
pub struct FGameplayAbilityTargetData_LocationInfo {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub source_location: FGameplayAbilityTargetingLocationInfo,
    pub target_location: FGameplayAbilityTargetingLocationInfo,
}
impl FGameplayAbilityTargetData_LocationInfo {}
#[repr(C, align(16))]
pub struct FGameplayAbilityTargetData_ActorArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub source_location: FGameplayAbilityTargetingLocationInfo,
    pub(crate) __padding_end: [u8; 16],
}
impl FGameplayAbilityTargetData_ActorArray {}
#[repr(C, align(8))]
pub struct FGameplayAbilityTargetData_SingleTargetHit {
    pub(crate) __padding_end: [u8; 280],
}
impl FGameplayAbilityTargetData_SingleTargetHit {}
#[repr(C, align(8))]
pub struct FGameplayAbilityActorInfo {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub owner_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub avatar_actor: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub player_controller: TWeakObjectPtr<crate::bindings::engine::APlayerController>,
    pub ability_system_component: TWeakObjectPtr<UAbilitySystemComponent>,
    pub skeletal_mesh_component: TWeakObjectPtr<
        crate::bindings::engine::USkeletalMeshComponent,
    >,
    pub anim_instance: TWeakObjectPtr<crate::bindings::engine::UAnimInstance>,
    pub movement_component: TWeakObjectPtr<crate::bindings::engine::UMovementComponent>,
    pub affected_anim_instance_tag: FName,
}
impl FGameplayAbilityActorInfo {}
#[repr(C, align(8))]
pub struct FAbilityEndedData {
    pub(crate) __padding_end: [u8; 16],
}
impl FAbilityEndedData {}
#[repr(C, align(4))]
pub struct FGameplayCueTag {
    pub gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
}
impl FGameplayCueTag {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_SpawnCondition {
    pub locally_controlled_source: EGameplayCueNotify_LocallyControlledSource,
    pub locally_controlled_policy: EGameplayCueNotify_LocallyControlledPolicy,
    pub chance_to_play: f32,
    pub allowed_surface_types: TArray<crate::bindings::physics_core::EPhysicalSurface>,
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 8],
    pub rejected_surface_types: TArray<crate::bindings::physics_core::EPhysicalSurface>,
    pub(crate) __padding_end: [u8; 8],
}
impl FGameplayCueNotify_SpawnCondition {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_PlacementInfo {
    pub socket_name: FName,
    pub attach_policy: EGameplayCueNotify_AttachPolicy,
    pub attachment_rule: crate::bindings::engine::EAttachmentRule,
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 2],
    pub flags_16: u8,
    pub rotation_override: crate::bindings::core_u_object::FRotator,
    pub scale_override: crate::bindings::core_u_object::FVector,
}
impl FGameplayCueNotify_PlacementInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_SpawnResult {
    pub fx_system_components: TArray<UPtr<crate::bindings::engine::UFXSystemComponent>>,
    pub audio_components: TArray<UPtr<crate::bindings::engine::UAudioComponent>>,
    pub camera_shakes: TArray<UPtr<crate::bindings::engine::UCameraShakeBase>>,
    pub camera_lens_effects: TArray<
        TScriptInterface<crate::bindings::engine::UCameraLensEffectInterface>,
    >,
    pub force_feedback_component: UPtr<crate::bindings::engine::UForceFeedbackComponent>,
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 8],
    pub decal_component: UPtr<crate::bindings::engine::UDecalComponent>,
}
impl FGameplayCueNotify_SpawnResult {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_ParticleInfo {
    pub spawn_condition_override: FGameplayCueNotify_SpawnCondition,
    pub placement_info_override: FGameplayCueNotify_PlacementInfo,
    pub niagara_system: UPtr<crate::bindings::niagara::UNiagaraSystem>,
    pub flags_136: u8,
}
impl FGameplayCueNotify_ParticleInfo {}
#[repr(C, align(4))]
pub struct FGameplayCueNotify_SoundParameterInterfaceInfo {
    pub stop_trigger_name: FName,
}
impl FGameplayCueNotify_SoundParameterInterfaceInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_SoundInfo {
    pub spawn_condition_override: FGameplayCueNotify_SpawnCondition,
    pub placement_info_override: FGameplayCueNotify_PlacementInfo,
    pub sound: UPtr<crate::bindings::engine::USoundBase>,
    #[doc(hidden)]
    pub(crate) __padding_144: [u8; 8],
    pub looping_fade_out_duration: f32,
    pub looping_fade_volume_level: f32,
    pub sound_parameter_interface_info: FGameplayCueNotify_SoundParameterInterfaceInfo,
    pub flags_164: u8,
}
impl FGameplayCueNotify_SoundInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_CameraShakeInfo {
    pub spawn_condition_override: FGameplayCueNotify_SpawnCondition,
    pub placement_info_override: FGameplayCueNotify_PlacementInfo,
    pub camera_shake: TSubclassOf<crate::bindings::engine::UCameraShakeBase>,
    pub shake_scale: f32,
    pub play_space: EGameplayCueNotify_EffectPlaySpace,
    #[doc(hidden)]
    pub(crate) __padding_144: [u8; 3],
    pub flags_144: u8,
    pub world_inner_radius: f32,
    pub world_outer_radius: f32,
    pub world_falloff_exponent: f32,
}
impl FGameplayCueNotify_CameraShakeInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_CameraLensEffectInfo {
    pub spawn_condition_override: FGameplayCueNotify_SpawnCondition,
    pub placement_info_override: FGameplayCueNotify_PlacementInfo,
    pub camera_lens_effect: TSubclassOf<crate::bindings::engine::AActor>,
    pub flags_136: u8,
    pub world_inner_radius: f32,
    pub world_outer_radius: f32,
}
impl FGameplayCueNotify_CameraLensEffectInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_ForceFeedbackInfo {
    pub spawn_condition_override: FGameplayCueNotify_SpawnCondition,
    pub placement_info_override: FGameplayCueNotify_PlacementInfo,
    pub force_feedback_effect: UPtr<crate::bindings::engine::UForceFeedbackEffect>,
    pub force_feedback_tag: FName,
    pub flags_148: u8,
    pub world_intensity: f32,
    pub world_attenuation: UPtr<crate::bindings::engine::UForceFeedbackAttenuation>,
}
impl FGameplayCueNotify_ForceFeedbackInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_InputDevicePropertyInfo {
    pub device_properties: TArray<
        TSubclassOf<crate::bindings::engine::UInputDeviceProperty>,
    >,
}
impl FGameplayCueNotify_InputDevicePropertyInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_DecalInfo {
    pub spawn_condition_override: FGameplayCueNotify_SpawnCondition,
    pub placement_info_override: FGameplayCueNotify_PlacementInfo,
    pub decal_material: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub decal_size: crate::bindings::core_u_object::FVector,
    pub flags_160: u8,
    pub fade_out_start_delay: f32,
    pub fade_out_duration: f32,
}
impl FGameplayCueNotify_DecalInfo {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_BurstEffects {
    pub burst_particles: TArray<FGameplayCueNotify_ParticleInfo>,
    pub burst_sounds: TArray<FGameplayCueNotify_SoundInfo>,
    pub burst_camera_shake: FGameplayCueNotify_CameraShakeInfo,
    pub burst_camera_lens_effect: FGameplayCueNotify_CameraLensEffectInfo,
    pub burst_force_feedback: FGameplayCueNotify_ForceFeedbackInfo,
    pub burst_device_property_effect: FGameplayCueNotify_InputDevicePropertyInfo,
    pub burst_decal: FGameplayCueNotify_DecalInfo,
}
impl FGameplayCueNotify_BurstEffects {}
#[repr(C, align(8))]
pub struct FGameplayCueNotify_LoopingEffects {
    pub looping_particles: TArray<FGameplayCueNotify_ParticleInfo>,
    pub looping_sounds: TArray<FGameplayCueNotify_SoundInfo>,
    pub looping_camera_shake: FGameplayCueNotify_CameraShakeInfo,
    pub looping_camera_lens_effect: FGameplayCueNotify_CameraLensEffectInfo,
    pub looping_force_feedback: FGameplayCueNotify_ForceFeedbackInfo,
    pub looping_input_device_property_effect: FGameplayCueNotify_InputDevicePropertyInfo,
}
impl FGameplayCueNotify_LoopingEffects {}
#[repr(C, align(8))]
pub struct FGameplayEffectExecutionScopedModifierInfo {
    pub(crate) __padding_end: [u8; 1008],
}
impl FGameplayEffectExecutionScopedModifierInfo {}
#[repr(C, align(8))]
pub struct FConditionalGameplayEffect {
    pub effect_class: TSubclassOf<UGameplayEffect>,
    pub required_source_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
}
impl FConditionalGameplayEffect {}
#[repr(C, align(8))]
pub struct FGameplayEffectExecutionDefinition {
    pub(crate) __padding_end: [u8; 72],
}
impl FGameplayEffectExecutionDefinition {}
#[repr(C, align(8))]
pub struct FGameplayModifierInfo {
    pub(crate) __padding_end: [u8; 992],
}
impl FGameplayModifierInfo {}
#[repr(C, align(8))]
pub struct FGameplayEffectCue {
    pub(crate) __padding_end: [u8; 112],
}
impl FGameplayEffectCue {}
#[repr(C, align(8))]
pub struct FInheritedTagContainer {
    pub combined_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub added: crate::bindings::gameplay_tags::FGameplayTagContainer,
    pub removed: crate::bindings::gameplay_tags::FGameplayTagContainer,
}
impl FInheritedTagContainer {}
#[repr(C, align(8))]
pub struct FGameplayEffectCustomExecutionParameters {
    pub(crate) __padding_end: [u8; 240],
}
impl FGameplayEffectCustomExecutionParameters {}
#[repr(C, align(8))]
pub struct FGameplayEffectCustomExecutionOutput {
    pub(crate) __padding_end: [u8; 24],
}
impl FGameplayEffectCustomExecutionOutput {}
#[repr(C, align(8))]
pub struct FGameplayModifierEvaluatedData {
    pub(crate) __padding_end: [u8; 96],
}
impl FGameplayModifierEvaluatedData {}
#[repr(C, align(8))]
pub struct AAbilitySystemDebugHUD {
    __padding_end: [u8; 1376],
}
impl AAbilitySystemDebugHUD {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAbilitySystemDebugHUD")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAbilitySystemDebugHUD")
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
pub struct UGameplayCueNotify_Static {
    __padding_end: [u8; 80],
}
impl UGameplayCueNotify_Static {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_Static")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_Static")
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
    pub fn while_active(
        &self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_while_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_while_active,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn on_remove(
        &self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_on_remove,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_on_remove,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn on_execute(
        &self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_on_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_on_execute,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn on_active(
        &self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_on_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_on_active,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn handle_gameplay_cue(
        &self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        event_type: EGameplayCueEvent,
        parameters: &FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_k2_handle_gameplay_cue,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_type,
                __buffer.add(8).cast::<EGameplayCueEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(16).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_static_k2_handle_gameplay_cue,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        std::mem::forget(event_type);
    }
}
#[repr(C, align(8))]
pub struct UGameplayCueNotify_UnitTest {
    __padding_end: [u8; 96],
}
impl UGameplayCueNotify_UnitTest {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_UnitTest")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_UnitTest")
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
pub struct UGameplayEffect {
    #[doc(hidden)]
    pub(crate) __padding_1328: [u8; 1328],
    pub period: FScalableFloat,
    pub b_execute_periodic_effect_on_application: bool,
    pub periodic_inhibition_policy: EGameplayEffectPeriodInhibitionRemovedPolicy,
    pub modifiers: TArray<FGameplayModifierInfo>,
    pub executions: TArray<FGameplayEffectExecutionDefinition>,
    #[doc(hidden)]
    pub(crate) __padding_1496: [u8; 72],
    pub conditional_gameplay_effects: TArray<FConditionalGameplayEffect>,
    pub overflow_effects: TArray<TSubclassOf<UGameplayEffect>>,
    pub b_deny_overflow_application: bool,
    pub b_clear_stack_on_overflow: bool,
    pub premature_expiration_effect_classes: TArray<TSubclassOf<UGameplayEffect>>,
    pub routine_expiration_effect_classes: TArray<TSubclassOf<UGameplayEffect>>,
    pub b_require_modifier_success_to_trigger_cues: bool,
    pub gameplay_cues: TArray<FGameplayEffectCue>,
    pub ui_data: UPtr<UGameplayEffectUIData>,
    pub inheritable_gameplay_effect_tags: FInheritedTagContainer,
    pub inheritable_owned_tags_container: FInheritedTagContainer,
    pub inheritable_blocked_ability_tags_container: FInheritedTagContainer,
    pub ongoing_tag_requirements: FGameplayTagRequirements,
    pub application_tag_requirements: FGameplayTagRequirements,
    pub removal_tag_requirements: FGameplayTagRequirements,
    pub remove_gameplay_effects_with_tags: FInheritedTagContainer,
    pub granted_application_immunity_tags: FGameplayTagRequirements,
    pub granted_application_immunity_query: FGameplayEffectQuery,
    #[doc(hidden)]
    pub(crate) __padding_2984: [u8; 8],
    pub remove_gameplay_effect_query: FGameplayEffectQuery,
    #[doc(hidden)]
    pub(crate) __padding_3433: [u8; 1],
    pub stacking_type: EGameplayEffectStackingType,
    pub stack_limit_count: i32,
    pub stack_duration_refresh_policy: EGameplayEffectStackingDurationPolicy,
    pub stack_period_reset_policy: EGameplayEffectStackingPeriodPolicy,
    pub stack_expiration_policy: EGameplayEffectStackingExpirationPolicy,
    pub b_factor_in_stack_count: bool,
    pub granted_abilities: TArray<FGameplayAbilitySpecDef>,
    #[doc(hidden)]
    pub(crate) __padding_3560: [u8; 96],
    pub ge_components: TArray<UPtr<UGameplayEffectComponent>>,
    __padding_end: [u8; 24],
}
impl UGameplayEffect {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffect")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffect")
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
pub struct UGameplayEffectComponent {
    __padding_end: [u8; 64],
}
impl UGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectComponent")
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
pub struct UGameplayEffectUIData {
    __padding_end: [u8; 64],
}
impl UGameplayEffectUIData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectUIData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectUIData")
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
pub struct UGameplayEffectUIData_TextOnly {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub description: FText,
}
impl UGameplayEffectUIData_TextOnly {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectUIData_TextOnly")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectUIData_TextOnly")
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
pub struct UAbilityAsync {
    __padding_end: [u8; 64],
}
impl UAbilityAsync {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync")
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
    pub fn end_action(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_end_action,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_end_action,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAbilityAsync_WaitAttributeChanged {
    __padding_end: [u8; 176],
}
impl UAbilityAsync_WaitAttributeChanged {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitAttributeChanged")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitAttributeChanged")
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
    pub fn wait_for_attribute_changed(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        attribute: FGameplayAttribute,
        only_trigger_once: bool,
    ) -> UPtr<UAbilityAsync_WaitAttributeChanged> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_attribute_changed_wait_for_attribute_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityAsync_WaitAttributeChanged::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_attribute_changed_wait_for_attribute_changed,
                __buffer,
            )
        };
        std::mem::forget(target_actor);
        std::mem::forget(attribute);
        std::mem::forget(only_trigger_once);
        unsafe {
            __buffer.add(88).cast::<UPtr<UAbilityAsync_WaitAttributeChanged>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityAsync_WaitGameplayEffectApplied {
    __padding_end: [u8; 688],
}
impl UAbilityAsync_WaitGameplayEffectApplied {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayEffectApplied")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayEffectApplied")
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
    pub fn wait_gameplay_effect_applied_to_actor(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        source_filter: FGameplayTargetDataFilterHandle,
        source_tag_requirements: FGameplayTagRequirements,
        target_tag_requirements: FGameplayTagRequirements,
        asset_tag_requirements: FGameplayTagRequirements,
        granted_tag_requirements: FGameplayTagRequirements,
        trigger_once: bool,
        listen_for_periodic_effect: bool,
    ) -> UPtr<UAbilityAsync_WaitGameplayEffectApplied> {
        let mut __stack = crate::core_data::StackAlloc::<584>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_effect_applied_wait_gameplay_effect_applied_to_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filter,
                __buffer.add(8).cast::<FGameplayTargetDataFilterHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_tag_requirements,
                __buffer.add(24).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_tag_requirements,
                __buffer.add(160).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_tag_requirements,
                __buffer.add(296).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &granted_tag_requirements,
                __buffer.add(432).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(568).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &listen_for_periodic_effect,
                __buffer.add(569).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityAsync_WaitGameplayEffectApplied::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_effect_applied_wait_gameplay_effect_applied_to_actor,
                __buffer,
            )
        };
        std::mem::forget(target_actor);
        std::mem::forget(source_filter);
        std::mem::forget(source_tag_requirements);
        std::mem::forget(target_tag_requirements);
        std::mem::forget(asset_tag_requirements);
        std::mem::forget(granted_tag_requirements);
        std::mem::forget(trigger_once);
        std::mem::forget(listen_for_periodic_effect);
        unsafe {
            __buffer
                .add(576)
                .cast::<UPtr<UAbilityAsync_WaitGameplayEffectApplied>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityAsync_WaitGameplayEvent {
    __padding_end: [u8; 112],
}
impl UAbilityAsync_WaitGameplayEvent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayEvent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayEvent")
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
    pub fn wait_gameplay_event_to_actor(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        event_tag: crate::bindings::gameplay_tags::FGameplayTag,
        only_trigger_once: bool,
        only_match_exact: bool,
    ) -> UPtr<UAbilityAsync_WaitGameplayEvent> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_event_wait_gameplay_event_to_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_match_exact,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityAsync_WaitGameplayEvent::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_event_wait_gameplay_event_to_actor,
                __buffer,
            )
        };
        std::mem::forget(target_actor);
        std::mem::forget(event_tag);
        std::mem::forget(only_trigger_once);
        std::mem::forget(only_match_exact);
        unsafe {
            __buffer.add(24).cast::<UPtr<UAbilityAsync_WaitGameplayEvent>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityAsync_WaitGameplayTag {
    __padding_end: [u8; 96],
}
impl UAbilityAsync_WaitGameplayTag {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTag")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTag")
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
pub struct UAbilityAsync_WaitGameplayTagAdded {
    __padding_end: [u8; 120],
}
impl UAbilityAsync_WaitGameplayTagAdded {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagAdded")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagAdded")
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
    pub fn wait_gameplay_tag_add_to_actor(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        tag: crate::bindings::gameplay_tags::FGameplayTag,
        only_trigger_once: bool,
    ) -> UPtr<UAbilityAsync_WaitGameplayTagAdded> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_added_wait_gameplay_tag_add_to_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityAsync_WaitGameplayTagAdded::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_added_wait_gameplay_tag_add_to_actor,
                __buffer,
            )
        };
        std::mem::forget(target_actor);
        std::mem::forget(tag);
        std::mem::forget(only_trigger_once);
        unsafe {
            __buffer.add(24).cast::<UPtr<UAbilityAsync_WaitGameplayTagAdded>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityAsync_WaitGameplayTagRemoved {
    __padding_end: [u8; 120],
}
impl UAbilityAsync_WaitGameplayTagRemoved {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagRemoved")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagRemoved")
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
    pub fn wait_gameplay_tag_remove_from_actor(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        tag: crate::bindings::gameplay_tags::FGameplayTag,
        only_trigger_once: bool,
    ) -> UPtr<UAbilityAsync_WaitGameplayTagRemoved> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_removed_wait_gameplay_tag_remove_from_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityAsync_WaitGameplayTagRemoved::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_removed_wait_gameplay_tag_remove_from_actor,
                __buffer,
            )
        };
        std::mem::forget(target_actor);
        std::mem::forget(tag);
        std::mem::forget(only_trigger_once);
        unsafe {
            __buffer.add(24).cast::<UPtr<UAbilityAsync_WaitGameplayTagRemoved>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityAsync_WaitGameplayTagCountChanged {
    __padding_end: [u8; 112],
}
impl UAbilityAsync_WaitGameplayTagCountChanged {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagCountChanged")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagCountChanged")
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
    pub fn wait_gameplay_tag_count_changed_on_actor(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> UPtr<UAbilityAsync_WaitGameplayTagCountChanged> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_changed_on_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityAsync_WaitGameplayTagCountChanged::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_changed_on_actor,
                __buffer,
            )
        };
        std::mem::forget(target_actor);
        std::mem::forget(tag);
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<UAbilityAsync_WaitGameplayTagCountChanged>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityAsync_WaitGameplayTagQuery {
    __padding_end: [u8; 280],
}
impl UAbilityAsync_WaitGameplayTagQuery {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagQuery")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityAsync_WaitGameplayTagQuery")
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
    pub fn wait_gameplay_tag_query_on_actor(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        trigger_condition: EWaitGameplayTagQueryTriggerCondition,
        b_only_trigger_once: bool,
    ) -> UPtr<UAbilityAsync_WaitGameplayTagQuery> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_query_wait_gameplay_tag_query_on_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_query,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_condition,
                __buffer.add(80).cast::<EWaitGameplayTagQueryTriggerCondition>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_trigger_once,
                __buffer.add(81).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityAsync_WaitGameplayTagQuery::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_async_wait_gameplay_tag_query_wait_gameplay_tag_query_on_actor,
                __buffer,
            )
        };
        std::mem::forget(target_actor);
        std::mem::forget(tag_query);
        std::mem::forget(trigger_condition);
        std::mem::forget(b_only_trigger_once);
        unsafe {
            __buffer.add(88).cast::<UPtr<UAbilityAsync_WaitGameplayTagQuery>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UGameplayAbility {
    #[doc(hidden)]
    pub(crate) __padding_268: [u8; 268],
    pub current_activation_info: FGameplayAbilityActivationInfo,
    pub current_event_data: FGameplayEventData,
    #[doc(hidden)]
    pub(crate) __padding_992: [u8; 520],
    pub b_mark_pending_kill_on_ability_end: bool,
}
impl UGameplayAbility {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbility")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbility")
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
    pub fn set_should_block_other_abilities(&mut self, b_should_block_abilities: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_set_should_block_other_abilities,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_block_abilities,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_set_should_block_other_abilities,
                __buffer,
            )
        };
        std::mem::forget(b_should_block_abilities);
    }
    pub fn set_can_be_canceled(&mut self, b_can_be_canceled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_set_can_be_canceled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_can_be_canceled,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_set_can_be_canceled,
                __buffer,
            )
        };
        std::mem::forget(b_can_be_canceled);
    }
    pub fn send_gameplay_event(
        &mut self,
        event_tag: crate::bindings::gameplay_tags::FGameplayTag,
        payload: FGameplayEventData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<200>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_send_gameplay_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload,
                __buffer.add(16).cast::<FGameplayEventData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_send_gameplay_event,
                __buffer,
            )
        };
        std::mem::forget(event_tag);
        std::mem::forget(payload);
    }
    pub fn remove_granted_by_effect(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_remove_granted_by_effect,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_remove_granted_by_effect,
                __buffer,
            )
        };
    }
    pub fn montage_stop(&mut self, override_blend_out_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_montage_stop,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_blend_out_time,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_montage_stop,
                __buffer,
            )
        };
        std::mem::forget(override_blend_out_time);
    }
    pub fn montage_set_next_section_name(
        &mut self,
        from_section_name: FName,
        to_section_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_montage_set_next_section_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_section_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_section_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_montage_set_next_section_name,
                __buffer,
            )
        };
        std::mem::forget(from_section_name);
        std::mem::forget(to_section_name);
    }
    pub fn montage_jump_to_section(&mut self, section_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_montage_jump_to_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section_name,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_montage_jump_to_section,
                __buffer,
            )
        };
        std::mem::forget(section_name);
    }
    pub fn make_target_location_info_from_owner_skeletal_mesh_component(
        &mut self,
        socket_name: FName,
    ) -> FGameplayAbilityTargetingLocationInfo {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_make_target_location_info_from_owner_skeletal_mesh_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_name,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_make_target_location_info_from_owner_skeletal_mesh_component,
                __buffer,
            )
        };
        std::mem::forget(socket_name);
        unsafe {
            __buffer.add(16).cast::<FGameplayAbilityTargetingLocationInfo>().read()
        }
    }
    pub fn make_target_location_info_from_owner_actor(
        &mut self,
    ) -> FGameplayAbilityTargetingLocationInfo {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_make_target_location_info_from_owner_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_make_target_location_info_from_owner_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FGameplayAbilityTargetingLocationInfo>().read() }
    }
    pub fn make_outgoing_gameplay_effect_spec(
        &self,
        gameplay_effect_class: TSubclassOf<UGameplayEffect>,
        level: f32,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_make_outgoing_gameplay_effect_spec,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_make_outgoing_gameplay_effect_spec,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect_class);
        std::mem::forget(level);
        unsafe { __buffer.add(16).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn should_ability_respond_to_event(
        &self,
        actor_info: FGameplayAbilityActorInfo,
        payload: FGameplayEventData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<265>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_should_ability_respond_to_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_info,
                __buffer.add(0).cast::<FGameplayAbilityActorInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload,
                __buffer.add(80).cast::<FGameplayEventData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_should_ability_respond_to_event,
                __buffer,
            )
        };
        std::mem::forget(actor_info);
        std::mem::forget(payload);
        unsafe { __buffer.add(264).cast::<bool>().read() }
    }
    pub fn remove_gameplay_cue(
        &mut self,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_remove_gameplay_cue,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_remove_gameplay_cue,
                __buffer,
            )
        };
        std::mem::forget(gameplay_cue_tag);
    }
    pub fn on_end_ability(&mut self, b_was_cancelled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_on_end_ability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_was_cancelled,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_on_end_ability,
                __buffer,
            )
        };
        std::mem::forget(b_was_cancelled);
    }
    pub fn k2_has_authority(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_has_authority,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_has_authority,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn execute_gameplay_cue_with_params(
        &mut self,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
        gameplay_cue_parameters: &FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_execute_gameplay_cue_with_params,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_cue_parameters,
                __buffer.add(16).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_execute_gameplay_cue_with_params,
                __buffer,
            )
        };
        std::mem::forget(gameplay_cue_tag);
    }
    pub fn execute_gameplay_cue(
        &mut self,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
        context: FGameplayEffectContextHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_execute_gameplay_cue,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(16).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_execute_gameplay_cue,
                __buffer,
            )
        };
        std::mem::forget(gameplay_cue_tag);
        std::mem::forget(context);
    }
    pub fn end_ability_locally(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_end_ability_locally,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_end_ability_locally,
                __buffer,
            )
        };
    }
    pub fn end_ability(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_end_ability,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_end_ability,
                __buffer,
            )
        };
    }
    pub fn commit_execute(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_execute,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_execute,
                __buffer,
            )
        };
    }
    pub fn commit_ability_cost(&mut self, broadcast_commit_event: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_ability_cost,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &broadcast_commit_event,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_ability_cost,
                __buffer,
            )
        };
        std::mem::forget(broadcast_commit_event);
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn commit_ability_cooldown(
        &mut self,
        broadcast_commit_event: bool,
        force_cooldown: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_ability_cooldown,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &broadcast_commit_event,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &force_cooldown,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_ability_cooldown,
                __buffer,
            )
        };
        std::mem::forget(broadcast_commit_event);
        std::mem::forget(force_cooldown);
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn commit_ability(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_ability,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_commit_ability,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn check_ability_cost(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_check_ability_cost,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_check_ability_cost,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn check_ability_cooldown(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_check_ability_cooldown,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_check_ability_cooldown,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn cancel_ability(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_cancel_ability,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_cancel_ability,
                __buffer,
            )
        };
    }
    pub fn can_activate_ability(
        &self,
        actor_info: FGameplayAbilityActorInfo,
        handle: FGameplayAbilitySpecHandle,
        relevant_tags: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<121>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_can_activate_ability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_info,
                __buffer.add(0).cast::<FGameplayAbilityActorInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(80).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                relevant_tags,
                __buffer
                    .add(88)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_can_activate_ability,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(relevant_tags);
        }
        std::mem::forget(actor_info);
        std::mem::forget(handle);
        unsafe { __buffer.add(120).cast::<bool>().read() }
    }
    pub fn apply_gameplay_effect_spec_to_target(
        &mut self,
        effect_spec_handle: FGameplayEffectSpecHandle,
        target_data: FGameplayAbilityTargetDataHandle,
    ) -> TArray<FActiveGameplayEffectHandle> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_apply_gameplay_effect_spec_to_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_data,
                __buffer.add(16).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_apply_gameplay_effect_spec_to_target,
                __buffer,
            )
        };
        std::mem::forget(effect_spec_handle);
        std::mem::forget(target_data);
        unsafe { __buffer.add(56).cast::<TArray<FActiveGameplayEffectHandle>>().read() }
    }
    pub fn apply_gameplay_effect_spec_to_owner(
        &mut self,
        effect_spec_handle: FGameplayEffectSpecHandle,
    ) -> FActiveGameplayEffectHandle {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_apply_gameplay_effect_spec_to_owner,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_apply_gameplay_effect_spec_to_owner,
                __buffer,
            )
        };
        std::mem::forget(effect_spec_handle);
        unsafe { __buffer.add(16).cast::<FActiveGameplayEffectHandle>().read() }
    }
    pub fn add_gameplay_cue_with_params(
        &mut self,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
        gameplay_cue_parameter: &FGameplayCueParameters,
        b_remove_on_ability_end: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<241>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_add_gameplay_cue_with_params,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_cue_parameter,
                __buffer.add(16).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_on_ability_end,
                __buffer.add(240).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_add_gameplay_cue_with_params,
                __buffer,
            )
        };
        std::mem::forget(gameplay_cue_tag);
        std::mem::forget(b_remove_on_ability_end);
    }
    pub fn add_gameplay_cue(
        &mut self,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
        context: FGameplayEffectContextHandle,
        b_remove_on_ability_end: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_add_gameplay_cue,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(16).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_on_ability_end,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_add_gameplay_cue,
                __buffer,
            )
        };
        std::mem::forget(gameplay_cue_tag);
        std::mem::forget(context);
        std::mem::forget(b_remove_on_ability_end);
    }
    pub fn activate_ability_from_event(&mut self, event_data: &FGameplayEventData) {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_activate_ability_from_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                event_data,
                __buffer.add(0).cast::<FGameplayEventData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_activate_ability_from_event,
                __buffer,
            )
        };
    }
    pub fn activate_ability(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_activate_ability,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_k2_activate_ability,
                __buffer,
            )
        };
    }
    pub fn is_locally_controlled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_is_locally_controlled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_is_locally_controlled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn invalidate_client_prediction_key(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_invalidate_client_prediction_key,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_invalidate_client_prediction_key,
                __buffer,
            )
        };
    }
    pub fn get_source_object_bp(
        &self,
        handle: FGameplayAbilitySpecHandle,
        actor_info: &FGameplayAbilityActorInfo,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_source_object_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actor_info,
                __buffer.add(8).cast::<FGameplayAbilityActorInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_source_object_bp,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe {
            __buffer
                .add(88)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_owning_component_from_actor_info(
        &self,
    ) -> UPtr<crate::bindings::engine::USkeletalMeshComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_owning_component_from_actor_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_owning_component_from_actor_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>()
                .read()
        }
    }
    pub fn get_owning_actor_from_actor_info(
        &self,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_owning_actor_from_actor_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_owning_actor_from_actor_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
    pub fn get_granted_by_effect_context(&self) -> FGameplayEffectContextHandle {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_granted_by_effect_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_granted_by_effect_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FGameplayEffectContextHandle>().read() }
    }
    pub fn get_current_source_object(
        &self,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_current_source_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_current_source_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_current_montage(&self) -> UPtr<crate::bindings::engine::UAnimMontage> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_current_montage,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_current_montage,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimMontage>>().read()
        }
    }
    pub fn get_cooldown_time_remaining(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_cooldown_time_remaining,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_cooldown_time_remaining,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_context_from_owner(
        &self,
        optional_target_data: FGameplayAbilityTargetDataHandle,
    ) -> FGameplayEffectContextHandle {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_context_from_owner,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_context_from_owner,
                __buffer,
            )
        };
        std::mem::forget(optional_target_data);
        unsafe { __buffer.add(40).cast::<FGameplayEffectContextHandle>().read() }
    }
    pub fn get_avatar_actor_from_actor_info(
        &self,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_avatar_actor_from_actor_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_avatar_actor_from_actor_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
    pub fn get_actor_info(&self) -> FGameplayAbilityActorInfo {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_actor_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_actor_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FGameplayAbilityActorInfo>().read() }
    }
    pub fn get_ability_system_component_from_actor_info(
        &self,
    ) -> UPtr<UAbilitySystemComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_ability_system_component_from_actor_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_ability_system_component_from_actor_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>().read() }
    }
    pub fn get_ability_level_bp(
        &self,
        handle: FGameplayAbilitySpecHandle,
        actor_info: &FGameplayAbilityActorInfo,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_ability_level_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                actor_info,
                __buffer.add(8).cast::<FGameplayAbilityActorInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_ability_level_bp,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe { __buffer.add(88).cast::<i32>().read() }
    }
    pub fn get_ability_level(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_ability_level,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_get_ability_level,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn end_task_by_instance_name(&mut self, instance_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_end_task_by_instance_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_name,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_end_task_by_instance_name,
                __buffer,
            )
        };
        std::mem::forget(instance_name);
    }
    pub fn end_ability_state(&mut self, optional_state_name_to_end: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_end_ability_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_state_name_to_end,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_end_ability_state,
                __buffer,
            )
        };
        std::mem::forget(optional_state_name_to_end);
    }
    pub fn confirm_task_by_instance_name(
        &mut self,
        instance_name: FName,
        b_end_task: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_confirm_task_by_instance_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_end_task,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_confirm_task_by_instance_name,
                __buffer,
            )
        };
        std::mem::forget(instance_name);
        std::mem::forget(b_end_task);
    }
    pub fn cancel_task_by_instance_name(&mut self, instance_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_cancel_task_by_instance_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_name,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_cancel_task_by_instance_name,
                __buffer,
            )
        };
        std::mem::forget(instance_name);
    }
    pub fn remove_gameplay_effect_from_owner_with_handle(
        &mut self,
        handle: FActiveGameplayEffectHandle,
        stacks_to_remove: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stacks_to_remove,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_handle,
                __buffer,
            )
        };
        std::mem::forget(handle);
        std::mem::forget(stacks_to_remove);
    }
    pub fn remove_gameplay_effect_from_owner_with_granted_tags(
        &mut self,
        with_granted_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
        stacks_to_remove: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_granted_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &with_granted_tags,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stacks_to_remove,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_granted_tags,
                __buffer,
            )
        };
        std::mem::forget(with_granted_tags);
        std::mem::forget(stacks_to_remove);
    }
    pub fn remove_gameplay_effect_from_owner_with_asset_tags(
        &mut self,
        with_asset_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
        stacks_to_remove: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_asset_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &with_asset_tags,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stacks_to_remove,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_remove_gameplay_effect_from_owner_with_asset_tags,
                __buffer,
            )
        };
        std::mem::forget(with_asset_tags);
        std::mem::forget(stacks_to_remove);
    }
    pub fn apply_gameplay_effect_to_target(
        &mut self,
        target_data: FGameplayAbilityTargetDataHandle,
        gameplay_effect_class: TSubclassOf<UGameplayEffect>,
        gameplay_effect_level: i32,
        stacks: i32,
    ) -> TArray<FActiveGameplayEffectHandle> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_apply_gameplay_effect_to_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_class,
                __buffer.add(40).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_level,
                __buffer.add(48).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&stacks, __buffer.add(52).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_apply_gameplay_effect_to_target,
                __buffer,
            )
        };
        std::mem::forget(target_data);
        std::mem::forget(gameplay_effect_class);
        std::mem::forget(gameplay_effect_level);
        std::mem::forget(stacks);
        unsafe { __buffer.add(56).cast::<TArray<FActiveGameplayEffectHandle>>().read() }
    }
    pub fn apply_gameplay_effect_to_owner(
        &mut self,
        gameplay_effect_class: TSubclassOf<UGameplayEffect>,
        gameplay_effect_level: i32,
        stacks: i32,
    ) -> FActiveGameplayEffectHandle {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_apply_gameplay_effect_to_owner,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_level,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&stacks, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_ability_bp_apply_gameplay_effect_to_owner,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect_class);
        std::mem::forget(gameplay_effect_level);
        std::mem::forget(stacks);
        unsafe { __buffer.add(16).cast::<FActiveGameplayEffectHandle>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGameplayAbilitySet {
    __padding_end: [u8; 72],
}
impl UGameplayAbilitySet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilitySet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilitySet")
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
#[repr(C, align(16))]
pub struct AGameplayAbilityTargetActor {
    #[doc(hidden)]
    pub(crate) __padding_1152: [u8; 1152],
    pub start_location: FGameplayAbilityTargetingLocationInfo,
    #[doc(hidden)]
    pub(crate) __padding_1368: [u8; 72],
    pub primary_pc: UPtr<crate::bindings::engine::APlayerController>,
    #[doc(hidden)]
    pub(crate) __padding_1384: [u8; 8],
    pub b_destroy_on_confirmation: bool,
    pub source_actor: UPtr<crate::bindings::engine::AActor>,
    pub reticle_params: FWorldReticleParameters,
    pub reticle_class: TSubclassOf<AGameplayAbilityWorldReticle>,
    pub filter: FGameplayTargetDataFilterHandle,
    pub b_debug: bool,
    __padding_end: [u8; 39],
}
impl AGameplayAbilityTargetActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor")
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
#[repr(C, align(16))]
pub struct AGameplayAbilityTargetActor_Trace {
    #[doc(hidden)]
    pub(crate) __padding_1480: [u8; 1480],
    pub max_range: f32,
    pub trace_profile: crate::bindings::engine::FCollisionProfileName,
    pub b_trace_affects_aim_pitch: bool,
    __padding_end: [u8; 23],
}
impl AGameplayAbilityTargetActor_Trace {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_Trace")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_Trace")
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
#[repr(C, align(16))]
pub struct AGameplayAbilityTargetActor_GroundTrace {
    #[doc(hidden)]
    pub(crate) __padding_1512: [u8; 1512],
    pub collision_radius: f32,
    pub collision_height: f32,
    __padding_end: [u8; 32],
}
impl AGameplayAbilityTargetActor_GroundTrace {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_GroundTrace")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_GroundTrace")
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
#[repr(C, align(16))]
pub struct AGameplayAbilityTargetActor_ActorPlacement {
    #[doc(hidden)]
    pub(crate) __padding_1544: [u8; 1544],
    pub placed_actor_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub placed_actor_material: UPtr<crate::bindings::engine::UMaterialInterface>,
}
impl AGameplayAbilityTargetActor_ActorPlacement {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_ActorPlacement")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_ActorPlacement")
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
#[repr(C, align(16))]
pub struct AGameplayAbilityTargetActor_Radius {
    #[doc(hidden)]
    pub(crate) __padding_1480: [u8; 1480],
    pub radius: f32,
}
impl AGameplayAbilityTargetActor_Radius {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_Radius")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_Radius")
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
#[repr(C, align(16))]
pub struct AGameplayAbilityTargetActor_SingleLineTrace {
    __padding_end: [u8; 1520],
}
impl AGameplayAbilityTargetActor_SingleLineTrace {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_SingleLineTrace")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityTargetActor_SingleLineTrace")
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
pub struct AGameplayAbilityWorldReticle {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub parameters: FWorldReticleParameters,
    pub b_face_owner_flat: bool,
    pub b_snap_to_targeted_actor: bool,
    pub b_is_target_valid: bool,
    pub b_is_target_an_actor: bool,
    #[doc(hidden)]
    pub(crate) __padding_1176: [u8; 8],
    pub primary_pc: UPtr<crate::bindings::engine::APlayerController>,
    pub targeting_actor: UPtr<AGameplayAbilityTargetActor>,
}
impl AGameplayAbilityWorldReticle {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityWorldReticle")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityWorldReticle")
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
    pub fn set_reticle_material_param_vector(
        &mut self,
        param_name: FName,
        value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_set_reticle_material_param_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_set_reticle_material_param_vector,
                __buffer,
            )
        };
        std::mem::forget(param_name);
        std::mem::forget(value);
    }
    pub fn set_reticle_material_param_float(&mut self, param_name: FName, value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_set_reticle_material_param_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &param_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_set_reticle_material_param_float,
                __buffer,
            )
        };
        std::mem::forget(param_name);
        std::mem::forget(value);
    }
    pub fn on_valid_target_changed(&mut self, b_new_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_valid_target_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_value,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_valid_target_changed,
                __buffer,
            )
        };
        std::mem::forget(b_new_value);
    }
    pub fn on_targeting_an_actor(&mut self, b_new_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_targeting_an_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_new_value,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_targeting_an_actor,
                __buffer,
            )
        };
        std::mem::forget(b_new_value);
    }
    pub fn on_parameters_initialized(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_parameters_initialized,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_on_parameters_initialized,
                __buffer,
            )
        };
    }
    pub fn face_toward_source(&mut self, b_face_in2_d: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_face_toward_source,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_face_in2_d,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_ability_world_reticle_face_toward_source,
                __buffer,
            )
        };
        std::mem::forget(b_face_in2_d);
    }
}
#[repr(C, align(8))]
pub struct AGameplayAbilityWorldReticle_ActorVisualization {
    __padding_end: [u8; 1216],
}
impl AGameplayAbilityWorldReticle_ActorVisualization {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityWorldReticle_ActorVisualization")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayAbilityWorldReticle_ActorVisualization")
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
pub struct UGameplayAbility_CharacterJump {
    __padding_end: [u8; 1000],
}
impl UGameplayAbility_CharacterJump {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbility_CharacterJump")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbility_CharacterJump")
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
pub struct UGameplayAbility_Montage {
    __padding_end: [u8; 1056],
}
impl UGameplayAbility_Montage {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbility_Montage")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbility_Montage")
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
pub struct UAbilityTask {
    __padding_end: [u8; 152],
}
impl UAbilityTask {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask")
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
pub struct UAbilityTask_ApplyRootMotion_Base {
    __padding_end: [u8; 224],
}
impl UAbilityTask_ApplyRootMotion_Base {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotion_Base")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotion_Base")
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
pub struct UAbilityTask_ApplyRootMotionConstantForce {
    __padding_end: [u8; 304],
}
impl UAbilityTask_ApplyRootMotionConstantForce {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionConstantForce")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionConstantForce")
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
    pub fn apply_root_motion_constant_force(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        world_direction: crate::bindings::core_u_object::FVector,
        strength: f32,
        duration: f32,
        b_is_additive: bool,
        strength_over_time: UPtr<crate::bindings::engine::UCurveFloat>,
        velocity_on_finish_mode: crate::bindings::engine::ERootMotionFinishVelocityMode,
        set_velocity_on_finish: crate::bindings::core_u_object::FVector,
        clamp_velocity_on_finish: f32,
        b_enable_gravity: bool,
    ) -> UPtr<UAbilityTask_ApplyRootMotionConstantForce> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_constant_force_apply_root_motion_constant_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_direction,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&strength, __buffer.add(48).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(52).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_additive,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &strength_over_time,
                __buffer.add(64).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_on_finish_mode,
                __buffer
                    .add(72)
                    .cast::<crate::bindings::engine::ERootMotionFinishVelocityMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_velocity_on_finish,
                __buffer.add(80).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_velocity_on_finish,
                __buffer.add(104).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_gravity,
                __buffer.add(108).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_ApplyRootMotionConstantForce::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_constant_force_apply_root_motion_constant_force,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(world_direction);
        std::mem::forget(strength);
        std::mem::forget(duration);
        std::mem::forget(b_is_additive);
        std::mem::forget(strength_over_time);
        std::mem::forget(velocity_on_finish_mode);
        std::mem::forget(set_velocity_on_finish);
        std::mem::forget(clamp_velocity_on_finish);
        std::mem::forget(b_enable_gravity);
        unsafe {
            __buffer
                .add(112)
                .cast::<UPtr<UAbilityTask_ApplyRootMotionConstantForce>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_ApplyRootMotionJumpForce {
    __padding_end: [u8; 344],
}
impl UAbilityTask_ApplyRootMotionJumpForce {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionJumpForce")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionJumpForce")
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
    pub fn finish(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_jump_force_finish,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_jump_force_finish,
                __buffer,
            )
        };
    }
    pub fn apply_root_motion_jump_force(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        rotation: crate::bindings::core_u_object::FRotator,
        distance: f32,
        height: f32,
        duration: f32,
        minimum_landed_trigger_time: f32,
        b_finish_on_landed: bool,
        velocity_on_finish_mode: crate::bindings::engine::ERootMotionFinishVelocityMode,
        set_velocity_on_finish: crate::bindings::core_u_object::FVector,
        clamp_velocity_on_finish: f32,
        path_offset_curve: UPtr<crate::bindings::engine::UCurveVector>,
        time_mapping_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    ) -> UPtr<UAbilityTask_ApplyRootMotionJumpForce> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_jump_force_apply_root_motion_jump_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&distance, __buffer.add(48).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&height, __buffer.add(52).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(56).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &minimum_landed_trigger_time,
                __buffer.add(60).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_finish_on_landed,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_on_finish_mode,
                __buffer
                    .add(65)
                    .cast::<crate::bindings::engine::ERootMotionFinishVelocityMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_velocity_on_finish,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_velocity_on_finish,
                __buffer.add(96).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_offset_curve,
                __buffer.add(104).cast::<UPtr<crate::bindings::engine::UCurveVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_mapping_curve,
                __buffer.add(112).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_ApplyRootMotionJumpForce::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_jump_force_apply_root_motion_jump_force,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(rotation);
        std::mem::forget(distance);
        std::mem::forget(height);
        std::mem::forget(duration);
        std::mem::forget(minimum_landed_trigger_time);
        std::mem::forget(b_finish_on_landed);
        std::mem::forget(velocity_on_finish_mode);
        std::mem::forget(set_velocity_on_finish);
        std::mem::forget(clamp_velocity_on_finish);
        std::mem::forget(path_offset_curve);
        std::mem::forget(time_mapping_curve);
        unsafe {
            __buffer
                .add(120)
                .cast::<UPtr<UAbilityTask_ApplyRootMotionJumpForce>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_ApplyRootMotionMoveToActorForce {
    __padding_end: [u8; 432],
}
impl UAbilityTask_ApplyRootMotionMoveToActorForce {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionMoveToActorForce")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionMoveToActorForce")
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
    pub fn apply_root_motion_move_to_target_data_actor_force(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        target_data_handle: FGameplayAbilityTargetDataHandle,
        target_data_index: i32,
        target_actor_index: i32,
        target_location_offset: crate::bindings::core_u_object::FVector,
        offset_alignment: ERootMotionMoveToActorTargetOffsetType,
        duration: f32,
        target_lerp_speed_horizontal: UPtr<crate::bindings::engine::UCurveFloat>,
        target_lerp_speed_vertical: UPtr<crate::bindings::engine::UCurveFloat>,
        b_set_new_movement_mode: bool,
        movement_mode: crate::bindings::engine::EMovementMode,
        b_restrict_speed_to_expected: bool,
        path_offset_curve: UPtr<crate::bindings::engine::UCurveVector>,
        time_mapping_curve: UPtr<crate::bindings::engine::UCurveFloat>,
        velocity_on_finish_mode: crate::bindings::engine::ERootMotionFinishVelocityMode,
        set_velocity_on_finish: crate::bindings::core_u_object::FVector,
        clamp_velocity_on_finish: f32,
        b_disable_destination_reached_interrupt: bool,
        reached_destination_distance: f32,
    ) -> UPtr<UAbilityTask_ApplyRootMotionMoveToActorForce> {
        let mut __stack = crate::core_data::StackAlloc::<200>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_target_data_actor_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_data_handle,
                __buffer.add(24).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_data_index,
                __buffer.add(64).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor_index,
                __buffer.add(68).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_location_offset,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_alignment,
                __buffer.add(96).cast::<ERootMotionMoveToActorTargetOffsetType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(100).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_lerp_speed_horizontal,
                __buffer.add(104).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_lerp_speed_vertical,
                __buffer.add(112).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_new_movement_mode,
                __buffer.add(120).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode,
                __buffer.add(121).cast::<crate::bindings::engine::EMovementMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_restrict_speed_to_expected,
                __buffer.add(122).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_offset_curve,
                __buffer.add(128).cast::<UPtr<crate::bindings::engine::UCurveVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_mapping_curve,
                __buffer.add(136).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_on_finish_mode,
                __buffer
                    .add(144)
                    .cast::<crate::bindings::engine::ERootMotionFinishVelocityMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_velocity_on_finish,
                __buffer.add(152).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_velocity_on_finish,
                __buffer.add(176).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_disable_destination_reached_interrupt,
                __buffer.add(180).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reached_destination_distance,
                __buffer.add(184).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_ApplyRootMotionMoveToActorForce::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_target_data_actor_force,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(target_data_handle);
        std::mem::forget(target_data_index);
        std::mem::forget(target_actor_index);
        std::mem::forget(target_location_offset);
        std::mem::forget(offset_alignment);
        std::mem::forget(duration);
        std::mem::forget(target_lerp_speed_horizontal);
        std::mem::forget(target_lerp_speed_vertical);
        std::mem::forget(b_set_new_movement_mode);
        std::mem::forget(movement_mode);
        std::mem::forget(b_restrict_speed_to_expected);
        std::mem::forget(path_offset_curve);
        std::mem::forget(time_mapping_curve);
        std::mem::forget(velocity_on_finish_mode);
        std::mem::forget(set_velocity_on_finish);
        std::mem::forget(clamp_velocity_on_finish);
        std::mem::forget(b_disable_destination_reached_interrupt);
        std::mem::forget(reached_destination_distance);
        unsafe {
            __buffer
                .add(192)
                .cast::<UPtr<UAbilityTask_ApplyRootMotionMoveToActorForce>>()
                .read()
        }
    }
    pub fn apply_root_motion_move_to_component_force(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        target_component: UPtr<crate::bindings::engine::USceneComponent>,
        target_component_relative_location: crate::bindings::core_u_object::FVector,
        target_location_offset: crate::bindings::core_u_object::FVector,
        offset_alignment: ERootMotionMoveToActorTargetOffsetType,
        duration: f32,
        target_lerp_speed_horizontal: UPtr<crate::bindings::engine::UCurveFloat>,
        target_lerp_speed_vertical: UPtr<crate::bindings::engine::UCurveFloat>,
        b_set_new_movement_mode: bool,
        movement_mode: crate::bindings::engine::EMovementMode,
        b_restrict_speed_to_expected: bool,
        path_offset_curve: UPtr<crate::bindings::engine::UCurveVector>,
        time_mapping_curve: UPtr<crate::bindings::engine::UCurveFloat>,
        velocity_on_finish_mode: crate::bindings::engine::ERootMotionFinishVelocityMode,
        set_velocity_on_finish: crate::bindings::core_u_object::FVector,
        clamp_velocity_on_finish: f32,
        b_disable_destination_reached_interrupt: bool,
        reached_destination_distance: f32,
    ) -> UPtr<UAbilityTask_ApplyRootMotionMoveToActorForce> {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_component_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_component,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_component_relative_location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_location_offset,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_alignment,
                __buffer.add(80).cast::<ERootMotionMoveToActorTargetOffsetType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(84).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_lerp_speed_horizontal,
                __buffer.add(88).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_lerp_speed_vertical,
                __buffer.add(96).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_new_movement_mode,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode,
                __buffer.add(105).cast::<crate::bindings::engine::EMovementMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_restrict_speed_to_expected,
                __buffer.add(106).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_offset_curve,
                __buffer.add(112).cast::<UPtr<crate::bindings::engine::UCurveVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_mapping_curve,
                __buffer.add(120).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_on_finish_mode,
                __buffer
                    .add(128)
                    .cast::<crate::bindings::engine::ERootMotionFinishVelocityMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_velocity_on_finish,
                __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_velocity_on_finish,
                __buffer.add(160).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_disable_destination_reached_interrupt,
                __buffer.add(164).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reached_destination_distance,
                __buffer.add(168).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_ApplyRootMotionMoveToActorForce::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_component_force,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(target_component);
        std::mem::forget(target_component_relative_location);
        std::mem::forget(target_location_offset);
        std::mem::forget(offset_alignment);
        std::mem::forget(duration);
        std::mem::forget(target_lerp_speed_horizontal);
        std::mem::forget(target_lerp_speed_vertical);
        std::mem::forget(b_set_new_movement_mode);
        std::mem::forget(movement_mode);
        std::mem::forget(b_restrict_speed_to_expected);
        std::mem::forget(path_offset_curve);
        std::mem::forget(time_mapping_curve);
        std::mem::forget(velocity_on_finish_mode);
        std::mem::forget(set_velocity_on_finish);
        std::mem::forget(clamp_velocity_on_finish);
        std::mem::forget(b_disable_destination_reached_interrupt);
        std::mem::forget(reached_destination_distance);
        unsafe {
            __buffer
                .add(176)
                .cast::<UPtr<UAbilityTask_ApplyRootMotionMoveToActorForce>>()
                .read()
        }
    }
    pub fn apply_root_motion_move_to_actor_force(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        target_actor: UPtr<crate::bindings::engine::AActor>,
        target_location_offset: crate::bindings::core_u_object::FVector,
        offset_alignment: ERootMotionMoveToActorTargetOffsetType,
        duration: f32,
        target_lerp_speed_horizontal: UPtr<crate::bindings::engine::UCurveFloat>,
        target_lerp_speed_vertical: UPtr<crate::bindings::engine::UCurveFloat>,
        b_set_new_movement_mode: bool,
        movement_mode: crate::bindings::engine::EMovementMode,
        b_restrict_speed_to_expected: bool,
        path_offset_curve: UPtr<crate::bindings::engine::UCurveVector>,
        time_mapping_curve: UPtr<crate::bindings::engine::UCurveFloat>,
        velocity_on_finish_mode: crate::bindings::engine::ERootMotionFinishVelocityMode,
        set_velocity_on_finish: crate::bindings::core_u_object::FVector,
        clamp_velocity_on_finish: f32,
        b_disable_destination_reached_interrupt: bool,
        reached_destination_distance: f32,
    ) -> UPtr<UAbilityTask_ApplyRootMotionMoveToActorForce> {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_actor_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_location_offset,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_alignment,
                __buffer.add(56).cast::<ERootMotionMoveToActorTargetOffsetType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(60).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_lerp_speed_horizontal,
                __buffer.add(64).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_lerp_speed_vertical,
                __buffer.add(72).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_new_movement_mode,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode,
                __buffer.add(81).cast::<crate::bindings::engine::EMovementMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_restrict_speed_to_expected,
                __buffer.add(82).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_offset_curve,
                __buffer.add(88).cast::<UPtr<crate::bindings::engine::UCurveVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_mapping_curve,
                __buffer.add(96).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_on_finish_mode,
                __buffer
                    .add(104)
                    .cast::<crate::bindings::engine::ERootMotionFinishVelocityMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_velocity_on_finish,
                __buffer.add(112).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_velocity_on_finish,
                __buffer.add(136).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_disable_destination_reached_interrupt,
                __buffer.add(140).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reached_destination_distance,
                __buffer.add(144).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_ApplyRootMotionMoveToActorForce::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_actor_force_apply_root_motion_move_to_actor_force,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(target_actor);
        std::mem::forget(target_location_offset);
        std::mem::forget(offset_alignment);
        std::mem::forget(duration);
        std::mem::forget(target_lerp_speed_horizontal);
        std::mem::forget(target_lerp_speed_vertical);
        std::mem::forget(b_set_new_movement_mode);
        std::mem::forget(movement_mode);
        std::mem::forget(b_restrict_speed_to_expected);
        std::mem::forget(path_offset_curve);
        std::mem::forget(time_mapping_curve);
        std::mem::forget(velocity_on_finish_mode);
        std::mem::forget(set_velocity_on_finish);
        std::mem::forget(clamp_velocity_on_finish);
        std::mem::forget(b_disable_destination_reached_interrupt);
        std::mem::forget(reached_destination_distance);
        unsafe {
            __buffer
                .add(152)
                .cast::<UPtr<UAbilityTask_ApplyRootMotionMoveToActorForce>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_ApplyRootMotionMoveToForce {
    __padding_end: [u8; 344],
}
impl UAbilityTask_ApplyRootMotionMoveToForce {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionMoveToForce")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionMoveToForce")
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
    pub fn apply_root_motion_move_to_force(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        target_location: crate::bindings::core_u_object::FVector,
        duration: f32,
        b_set_new_movement_mode: bool,
        movement_mode: crate::bindings::engine::EMovementMode,
        b_restrict_speed_to_expected: bool,
        path_offset_curve: UPtr<crate::bindings::engine::UCurveVector>,
        velocity_on_finish_mode: crate::bindings::engine::ERootMotionFinishVelocityMode,
        set_velocity_on_finish: crate::bindings::core_u_object::FVector,
        clamp_velocity_on_finish: f32,
    ) -> UPtr<UAbilityTask_ApplyRootMotionMoveToForce> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_force_apply_root_motion_move_to_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(48).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_new_movement_mode,
                __buffer.add(52).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode,
                __buffer.add(53).cast::<crate::bindings::engine::EMovementMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_restrict_speed_to_expected,
                __buffer.add(54).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_offset_curve,
                __buffer.add(56).cast::<UPtr<crate::bindings::engine::UCurveVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_on_finish_mode,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::engine::ERootMotionFinishVelocityMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_velocity_on_finish,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_velocity_on_finish,
                __buffer.add(96).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_ApplyRootMotionMoveToForce::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_move_to_force_apply_root_motion_move_to_force,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(target_location);
        std::mem::forget(duration);
        std::mem::forget(b_set_new_movement_mode);
        std::mem::forget(movement_mode);
        std::mem::forget(b_restrict_speed_to_expected);
        std::mem::forget(path_offset_curve);
        std::mem::forget(velocity_on_finish_mode);
        std::mem::forget(set_velocity_on_finish);
        std::mem::forget(clamp_velocity_on_finish);
        unsafe {
            __buffer
                .add(104)
                .cast::<UPtr<UAbilityTask_ApplyRootMotionMoveToForce>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_ApplyRootMotionRadialForce {
    __padding_end: [u8; 344],
}
impl UAbilityTask_ApplyRootMotionRadialForce {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionRadialForce")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_ApplyRootMotionRadialForce")
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
    pub fn apply_root_motion_radial_force(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        location: crate::bindings::core_u_object::FVector,
        location_actor: UPtr<crate::bindings::engine::AActor>,
        strength: f32,
        duration: f32,
        radius: f32,
        b_is_push: bool,
        b_is_additive: bool,
        b_no_z_force: bool,
        strength_distance_falloff: UPtr<crate::bindings::engine::UCurveFloat>,
        strength_over_time: UPtr<crate::bindings::engine::UCurveFloat>,
        b_use_fixed_world_direction: bool,
        fixed_world_direction: crate::bindings::core_u_object::FRotator,
        velocity_on_finish_mode: crate::bindings::engine::ERootMotionFinishVelocityMode,
        set_velocity_on_finish: crate::bindings::core_u_object::FVector,
        clamp_velocity_on_finish: f32,
    ) -> UPtr<UAbilityTask_ApplyRootMotionRadialForce> {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_radial_force_apply_root_motion_radial_force,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location_actor,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&strength, __buffer.add(56).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(60).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius, __buffer.add(64).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_push,
                __buffer.add(68).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_additive,
                __buffer.add(69).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_no_z_force,
                __buffer.add(70).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &strength_distance_falloff,
                __buffer.add(72).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &strength_over_time,
                __buffer.add(80).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_fixed_world_direction,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fixed_world_direction,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &velocity_on_finish_mode,
                __buffer
                    .add(120)
                    .cast::<crate::bindings::engine::ERootMotionFinishVelocityMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_velocity_on_finish,
                __buffer.add(128).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &clamp_velocity_on_finish,
                __buffer.add(152).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_ApplyRootMotionRadialForce::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_apply_root_motion_radial_force_apply_root_motion_radial_force,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(location);
        std::mem::forget(location_actor);
        std::mem::forget(strength);
        std::mem::forget(duration);
        std::mem::forget(radius);
        std::mem::forget(b_is_push);
        std::mem::forget(b_is_additive);
        std::mem::forget(b_no_z_force);
        std::mem::forget(strength_distance_falloff);
        std::mem::forget(strength_over_time);
        std::mem::forget(b_use_fixed_world_direction);
        std::mem::forget(fixed_world_direction);
        std::mem::forget(velocity_on_finish_mode);
        std::mem::forget(set_velocity_on_finish);
        std::mem::forget(clamp_velocity_on_finish);
        unsafe {
            __buffer
                .add(160)
                .cast::<UPtr<UAbilityTask_ApplyRootMotionRadialForce>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_MoveToLocation {
    __padding_end: [u8; 272],
}
impl UAbilityTask_MoveToLocation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_MoveToLocation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_MoveToLocation")
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
    pub fn move_to_location(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        location: crate::bindings::core_u_object::FVector,
        duration: f32,
        optional_interpolation_curve: UPtr<crate::bindings::engine::UCurveFloat>,
        optional_vector_interpolation_curve: UPtr<crate::bindings::engine::UCurveVector>,
    ) -> UPtr<UAbilityTask_MoveToLocation> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_move_to_location_move_to_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(48).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_interpolation_curve,
                __buffer.add(56).cast::<UPtr<crate::bindings::engine::UCurveFloat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_vector_interpolation_curve,
                __buffer.add(64).cast::<UPtr<crate::bindings::engine::UCurveVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_MoveToLocation::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_move_to_location_move_to_location,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(location);
        std::mem::forget(duration);
        std::mem::forget(optional_interpolation_curve);
        std::mem::forget(optional_vector_interpolation_curve);
        unsafe { __buffer.add(72).cast::<UPtr<UAbilityTask_MoveToLocation>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_NetworkSyncPoint {
    __padding_end: [u8; 184],
}
impl UAbilityTask_NetworkSyncPoint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_NetworkSyncPoint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_NetworkSyncPoint")
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
    pub fn wait_net_sync(
        owning_ability: UPtr<UGameplayAbility>,
        sync_type: EAbilityTaskNetSyncType,
    ) -> UPtr<UAbilityTask_NetworkSyncPoint> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_network_sync_point_wait_net_sync,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sync_type,
                __buffer.add(8).cast::<EAbilityTaskNetSyncType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_NetworkSyncPoint::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_network_sync_point_wait_net_sync,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(sync_type);
        unsafe { __buffer.add(16).cast::<UPtr<UAbilityTask_NetworkSyncPoint>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_PlayAnimAndWait {
    __padding_end: [u8; 400],
}
impl UAbilityTask_PlayAnimAndWait {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_PlayAnimAndWait")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_PlayAnimAndWait")
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
    pub fn create_play_anim_and_wait_proxy(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        slot_name: FName,
        blend_in_time: f32,
        blend_out_time: f32,
        in_play_rate: f32,
        start_time_seconds: f32,
        b_stop_when_ability_ends: bool,
        anim_root_motion_translation_scale: f32,
        in_play_count: f32,
    ) -> UPtr<UAbilityTask_PlayAnimAndWait> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_play_anim_and_wait_create_play_anim_and_wait_proxy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_in_time,
                __buffer.add(44).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_out_time,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_play_rate,
                __buffer.add(52).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_time_seconds,
                __buffer.add(56).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_stop_when_ability_ends,
                __buffer.add(60).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_root_motion_translation_scale,
                __buffer.add(64).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_play_count,
                __buffer.add(68).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_PlayAnimAndWait::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_play_anim_and_wait_create_play_anim_and_wait_proxy,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(anim_sequence);
        std::mem::forget(slot_name);
        std::mem::forget(blend_in_time);
        std::mem::forget(blend_out_time);
        std::mem::forget(in_play_rate);
        std::mem::forget(start_time_seconds);
        std::mem::forget(b_stop_when_ability_ends);
        std::mem::forget(anim_root_motion_translation_scale);
        std::mem::forget(in_play_count);
        unsafe { __buffer.add(72).cast::<UPtr<UAbilityTask_PlayAnimAndWait>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_PlayMontageAndWait {
    __padding_end: [u8; 392],
}
impl UAbilityTask_PlayMontageAndWait {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_PlayMontageAndWait")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_PlayMontageAndWait")
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
    pub fn create_play_montage_and_wait_proxy(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        montage_to_play: UPtr<crate::bindings::engine::UAnimMontage>,
        rate: f32,
        start_section: FName,
        b_stop_when_ability_ends: bool,
        anim_root_motion_translation_scale: f32,
        start_time_seconds: f32,
        b_allow_interrupt_after_blend_out: bool,
    ) -> UPtr<UAbilityTask_PlayMontageAndWait> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_create_play_montage_and_wait_proxy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &montage_to_play,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UAnimMontage>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&rate, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_section,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_stop_when_ability_ends,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_root_motion_translation_scale,
                __buffer.add(52).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_time_seconds,
                __buffer.add(56).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_interrupt_after_blend_out,
                __buffer.add(60).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_PlayMontageAndWait::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_play_montage_and_wait_create_play_montage_and_wait_proxy,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(montage_to_play);
        std::mem::forget(rate);
        std::mem::forget(start_section);
        std::mem::forget(b_stop_when_ability_ends);
        std::mem::forget(anim_root_motion_translation_scale);
        std::mem::forget(start_time_seconds);
        std::mem::forget(b_allow_interrupt_after_blend_out);
        unsafe {
            __buffer.add(64).cast::<UPtr<UAbilityTask_PlayMontageAndWait>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_Repeat {
    __padding_end: [u8; 224],
}
impl UAbilityTask_Repeat {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_Repeat")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_Repeat")
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
    pub fn repeat_action(
        owning_ability: UPtr<UGameplayAbility>,
        time_between_actions: f32,
        total_action_count: i32,
    ) -> UPtr<UAbilityTask_Repeat> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_repeat_repeat_action,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_between_actions,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &total_action_count,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_Repeat::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_repeat_repeat_action,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(time_between_actions);
        std::mem::forget(total_action_count);
        unsafe { __buffer.add(16).cast::<UPtr<UAbilityTask_Repeat>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_SpawnActor {
    __padding_end: [u8; 240],
}
impl UAbilityTask_SpawnActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_SpawnActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_SpawnActor")
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
    pub fn spawn_actor(
        owning_ability: UPtr<UGameplayAbility>,
        target_data: FGameplayAbilityTargetDataHandle,
        class: TSubclassOf<crate::bindings::engine::AActor>,
    ) -> UPtr<UAbilityTask_SpawnActor> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_spawn_actor_spawn_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_data,
                __buffer.add(8).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer.add(48).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_SpawnActor::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_spawn_actor_spawn_actor,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(target_data);
        std::mem::forget(class);
        unsafe { __buffer.add(56).cast::<UPtr<UAbilityTask_SpawnActor>>().read() }
    }
    pub fn finish_spawning_actor(
        &mut self,
        owning_ability: UPtr<UGameplayAbility>,
        target_data: FGameplayAbilityTargetDataHandle,
        spawned_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_spawn_actor_finish_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_data,
                __buffer.add(8).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spawned_actor,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_spawn_actor_finish_spawning_actor,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(target_data);
        std::mem::forget(spawned_actor);
    }
    pub fn begin_spawning_actor(
        &mut self,
        owning_ability: UPtr<UGameplayAbility>,
        target_data: FGameplayAbilityTargetDataHandle,
        class: TSubclassOf<crate::bindings::engine::AActor>,
        spawned_actor: &mut UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_spawn_actor_begin_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_data,
                __buffer.add(8).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer.add(48).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawned_actor,
                __buffer.add(56).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_spawn_actor_begin_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(spawned_actor);
        }
        std::mem::forget(owning_ability);
        std::mem::forget(target_data);
        std::mem::forget(class);
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_StartAbilityState {
    __padding_end: [u8; 224],
}
impl UAbilityTask_StartAbilityState {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_StartAbilityState")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_StartAbilityState")
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
    pub fn start_ability_state(
        owning_ability: UPtr<UGameplayAbility>,
        state_name: FName,
        b_end_current_state: bool,
    ) -> UPtr<UAbilityTask_StartAbilityState> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_start_ability_state_start_ability_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &state_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_end_current_state,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_StartAbilityState::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_start_ability_state_start_ability_state,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(state_name);
        std::mem::forget(b_end_current_state);
        unsafe { __buffer.add(24).cast::<UPtr<UAbilityTask_StartAbilityState>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_VisualizeTargeting {
    __padding_end: [u8; 200],
}
impl UAbilityTask_VisualizeTargeting {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_VisualizeTargeting")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_VisualizeTargeting")
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
    pub fn visualize_targeting_using_actor(
        owning_ability: UPtr<UGameplayAbility>,
        target_actor: UPtr<AGameplayAbilityTargetActor>,
        task_instance_name: FName,
        duration: f32,
    ) -> UPtr<UAbilityTask_VisualizeTargeting> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_visualize_targeting_using_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(8).cast::<UPtr<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(28).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_VisualizeTargeting::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_visualize_targeting_using_actor,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(target_actor);
        std::mem::forget(task_instance_name);
        std::mem::forget(duration);
        unsafe {
            __buffer.add(32).cast::<UPtr<UAbilityTask_VisualizeTargeting>>().read()
        }
    }
    pub fn visualize_targeting(
        owning_ability: UPtr<UGameplayAbility>,
        class: TSubclassOf<AGameplayAbilityTargetActor>,
        task_instance_name: FName,
        duration: f32,
    ) -> UPtr<UAbilityTask_VisualizeTargeting> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_visualize_targeting,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer.add(8).cast::<TSubclassOf<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(28).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_VisualizeTargeting::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_visualize_targeting,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(class);
        std::mem::forget(task_instance_name);
        std::mem::forget(duration);
        unsafe {
            __buffer.add(32).cast::<UPtr<UAbilityTask_VisualizeTargeting>>().read()
        }
    }
    pub fn finish_spawning_actor(
        &mut self,
        owning_ability: UPtr<UGameplayAbility>,
        spawned_actor: UPtr<AGameplayAbilityTargetActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_finish_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spawned_actor,
                __buffer.add(8).cast::<UPtr<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_finish_spawning_actor,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(spawned_actor);
    }
    pub fn begin_spawning_actor(
        &mut self,
        owning_ability: UPtr<UGameplayAbility>,
        class: TSubclassOf<AGameplayAbilityTargetActor>,
        spawned_actor: &mut UPtr<AGameplayAbilityTargetActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_begin_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer.add(8).cast::<TSubclassOf<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawned_actor,
                __buffer.add(16).cast::<UPtr<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_visualize_targeting_begin_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<AGameplayAbilityTargetActor>>()
                .swap(spawned_actor);
        }
        std::mem::forget(owning_ability);
        std::mem::forget(class);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitAbilityActivate {
    __padding_end: [u8; 424],
}
impl UAbilityTask_WaitAbilityActivate {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAbilityActivate")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAbilityActivate")
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
    pub fn wait_for_ability_activate_with_tag_requirements(
        owning_ability: UPtr<UGameplayAbility>,
        tag_requirements: FGameplayTagRequirements,
        include_triggered_abilities: bool,
        trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitAbilityActivate> {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate_with_tag_requirements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_requirements,
                __buffer.add(8).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &include_triggered_abilities,
                __buffer.add(144).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(145).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAbilityActivate::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate_with_tag_requirements,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(tag_requirements);
        std::mem::forget(include_triggered_abilities);
        std::mem::forget(trigger_once);
        unsafe {
            __buffer.add(152).cast::<UPtr<UAbilityTask_WaitAbilityActivate>>().read()
        }
    }
    pub fn wait_for_ability_activate_query(
        owning_ability: UPtr<UGameplayAbility>,
        query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        include_triggered_abilities: bool,
        trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitAbilityActivate> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &query,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &include_triggered_abilities,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(81).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAbilityActivate::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate_query,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(query);
        std::mem::forget(include_triggered_abilities);
        std::mem::forget(trigger_once);
        unsafe {
            __buffer.add(88).cast::<UPtr<UAbilityTask_WaitAbilityActivate>>().read()
        }
    }
    pub fn wait_for_ability_activate(
        owning_ability: UPtr<UGameplayAbility>,
        with_tag: crate::bindings::gameplay_tags::FGameplayTag,
        without_tag: crate::bindings::gameplay_tags::FGameplayTag,
        include_triggered_abilities: bool,
        trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitAbilityActivate> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &with_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &without_tag,
                __buffer.add(20).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &include_triggered_abilities,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAbilityActivate::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_activate_wait_for_ability_activate,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(with_tag);
        std::mem::forget(without_tag);
        std::mem::forget(include_triggered_abilities);
        std::mem::forget(trigger_once);
        unsafe {
            __buffer.add(40).cast::<UPtr<UAbilityTask_WaitAbilityActivate>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitAbilityCommit {
    __padding_end: [u8; 288],
}
impl UAbilityTask_WaitAbilityCommit {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAbilityCommit")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAbilityCommit")
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
    pub fn wait_for_ability_commit_query(
        owning_ability: UPtr<UGameplayAbility>,
        query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitAbilityCommit> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_commit_wait_for_ability_commit_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &query,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAbilityCommit::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_commit_wait_for_ability_commit_query,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(query);
        std::mem::forget(trigger_once);
        unsafe { __buffer.add(88).cast::<UPtr<UAbilityTask_WaitAbilityCommit>>().read() }
    }
    pub fn wait_for_ability_commit(
        owning_ability: UPtr<UGameplayAbility>,
        with_tag: crate::bindings::gameplay_tags::FGameplayTag,
        without_tage: crate::bindings::gameplay_tags::FGameplayTag,
        trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitAbilityCommit> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_commit_wait_for_ability_commit,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &with_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &without_tage,
                __buffer.add(20).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAbilityCommit::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_ability_commit_wait_for_ability_commit,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(with_tag);
        std::mem::forget(without_tage);
        std::mem::forget(trigger_once);
        unsafe { __buffer.add(40).cast::<UPtr<UAbilityTask_WaitAbilityCommit>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitAttributeChange {
    __padding_end: [u8; 304],
}
impl UAbilityTask_WaitAttributeChange {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAttributeChange")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAttributeChange")
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
    pub fn wait_for_attribute_change_with_comparison(
        owning_ability: UPtr<UGameplayAbility>,
        in_attribute: FGameplayAttribute,
        in_with_tag: crate::bindings::gameplay_tags::FGameplayTag,
        in_without_tag: crate::bindings::gameplay_tags::FGameplayTag,
        in_comparison_type: EWaitAttributeChangeComparison,
        in_comparison_value: f32,
        trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UAbilityTask_WaitAttributeChange> {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_wait_for_attribute_change_with_comparison,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_with_tag,
                __buffer.add(80).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_without_tag,
                __buffer.add(92).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comparison_type,
                __buffer.add(104).cast::<EWaitAttributeChangeComparison>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comparison_value,
                __buffer.add(108).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(120).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAttributeChange::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_wait_for_attribute_change_with_comparison,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(in_attribute);
        std::mem::forget(in_with_tag);
        std::mem::forget(in_without_tag);
        std::mem::forget(in_comparison_type);
        std::mem::forget(in_comparison_value);
        std::mem::forget(trigger_once);
        std::mem::forget(optional_external_owner);
        unsafe {
            __buffer.add(128).cast::<UPtr<UAbilityTask_WaitAttributeChange>>().read()
        }
    }
    pub fn wait_for_attribute_change(
        owning_ability: UPtr<UGameplayAbility>,
        attribute: FGameplayAttribute,
        with_src_tag: crate::bindings::gameplay_tags::FGameplayTag,
        without_src_tag: crate::bindings::gameplay_tags::FGameplayTag,
        trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UAbilityTask_WaitAttributeChange> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_wait_for_attribute_change,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &with_src_tag,
                __buffer.add(80).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &without_src_tag,
                __buffer.add(92).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(112).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAttributeChange::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_wait_for_attribute_change,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(attribute);
        std::mem::forget(with_src_tag);
        std::mem::forget(without_src_tag);
        std::mem::forget(trigger_once);
        std::mem::forget(optional_external_owner);
        unsafe {
            __buffer.add(120).cast::<UPtr<UAbilityTask_WaitAttributeChange>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitAttributeChangeRatioThreshold {
    __padding_end: [u8; 384],
}
impl UAbilityTask_WaitAttributeChangeRatioThreshold {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAttributeChangeRatioThreshold")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAttributeChangeRatioThreshold")
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
    pub fn wait_for_attribute_change_ratio_threshold(
        owning_ability: UPtr<UGameplayAbility>,
        attribute_numerator: FGameplayAttribute,
        attribute_denominator: FGameplayAttribute,
        comparison_type: EWaitAttributeChangeComparison,
        comparison_value: f32,
        b_trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UAbilityTask_WaitAttributeChangeRatioThreshold> {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_ratio_threshold_wait_for_attribute_change_ratio_threshold,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_numerator,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_denominator,
                __buffer.add(80).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_type,
                __buffer.add(152).cast::<EWaitAttributeChangeComparison>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_value,
                __buffer.add(156).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_trigger_once,
                __buffer.add(160).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(168).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAttributeChangeRatioThreshold::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_ratio_threshold_wait_for_attribute_change_ratio_threshold,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(attribute_numerator);
        std::mem::forget(attribute_denominator);
        std::mem::forget(comparison_type);
        std::mem::forget(comparison_value);
        std::mem::forget(b_trigger_once);
        std::mem::forget(optional_external_owner);
        unsafe {
            __buffer
                .add(176)
                .cast::<UPtr<UAbilityTask_WaitAttributeChangeRatioThreshold>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitAttributeChangeThreshold {
    __padding_end: [u8; 288],
}
impl UAbilityTask_WaitAttributeChangeThreshold {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAttributeChangeThreshold")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitAttributeChangeThreshold")
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
    pub fn wait_for_attribute_change_threshold(
        owning_ability: UPtr<UGameplayAbility>,
        attribute: FGameplayAttribute,
        comparison_type: EWaitAttributeChangeComparison,
        comparison_value: f32,
        b_trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UAbilityTask_WaitAttributeChangeThreshold> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_threshold_wait_for_attribute_change_threshold,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_type,
                __buffer.add(80).cast::<EWaitAttributeChangeComparison>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &comparison_value,
                __buffer.add(84).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_trigger_once,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(96).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitAttributeChangeThreshold::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_attribute_change_threshold_wait_for_attribute_change_threshold,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(attribute);
        std::mem::forget(comparison_type);
        std::mem::forget(comparison_value);
        std::mem::forget(b_trigger_once);
        std::mem::forget(optional_external_owner);
        unsafe {
            __buffer
                .add(104)
                .cast::<UPtr<UAbilityTask_WaitAttributeChangeThreshold>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitCancel {
    __padding_end: [u8; 184],
}
impl UAbilityTask_WaitCancel {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitCancel")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitCancel")
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
    pub fn wait_cancel(
        owning_ability: UPtr<UGameplayAbility>,
    ) -> UPtr<UAbilityTask_WaitCancel> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_cancel_wait_cancel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitCancel::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_cancel_wait_cancel,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        unsafe { __buffer.add(8).cast::<UPtr<UAbilityTask_WaitCancel>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitConfirm {
    __padding_end: [u8; 192],
}
impl UAbilityTask_WaitConfirm {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitConfirm")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitConfirm")
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
    pub fn wait_confirm(
        owning_ability: UPtr<UGameplayAbility>,
    ) -> UPtr<UAbilityTask_WaitConfirm> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_confirm_wait_confirm,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitConfirm::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_confirm_wait_confirm,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        unsafe { __buffer.add(8).cast::<UPtr<UAbilityTask_WaitConfirm>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitConfirmCancel {
    __padding_end: [u8; 208],
}
impl UAbilityTask_WaitConfirmCancel {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitConfirmCancel")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitConfirmCancel")
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
    pub fn wait_confirm_cancel(
        owning_ability: UPtr<UGameplayAbility>,
    ) -> UPtr<UAbilityTask_WaitConfirmCancel> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_confirm_cancel_wait_confirm_cancel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitConfirmCancel::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_confirm_cancel_wait_confirm_cancel,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        unsafe { __buffer.add(8).cast::<UPtr<UAbilityTask_WaitConfirmCancel>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitDelay {
    __padding_end: [u8; 184],
}
impl UAbilityTask_WaitDelay {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitDelay")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitDelay")
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
    pub fn wait_delay(
        owning_ability: UPtr<UGameplayAbility>,
        time: f32,
    ) -> UPtr<UAbilityTask_WaitDelay> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_delay_wait_delay,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitDelay::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_delay_wait_delay,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(time);
        unsafe { __buffer.add(16).cast::<UPtr<UAbilityTask_WaitDelay>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayEffectApplied {
    __padding_end: [u8; 1032],
}
impl UAbilityTask_WaitGameplayEffectApplied {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectApplied")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectApplied")
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
pub struct UAbilityTask_WaitGameplayEffectApplied_Self {
    __padding_end: [u8; 1072],
}
impl UAbilityTask_WaitGameplayEffectApplied_Self {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectApplied_Self")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectApplied_Self")
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
    pub fn wait_gameplay_effect_applied_to_self_query(
        owning_ability: UPtr<UGameplayAbility>,
        source_filter: FGameplayTargetDataFilterHandle,
        source_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        target_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        asset_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        granted_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
        listen_for_periodic_effect: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayEffectApplied_Self> {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filter,
                __buffer.add(8).cast::<FGameplayTargetDataFilterHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_tag_query,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_tag_query,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_tag_query,
                __buffer
                    .add(168)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &granted_tag_query,
                __buffer
                    .add(240)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(312).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(320).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &listen_for_periodic_effect,
                __buffer.add(328).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEffectApplied_Self::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self_query,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(source_filter);
        std::mem::forget(source_tag_query);
        std::mem::forget(target_tag_query);
        std::mem::forget(asset_tag_query);
        std::mem::forget(granted_tag_query);
        std::mem::forget(trigger_once);
        std::mem::forget(optional_external_owner);
        std::mem::forget(listen_for_periodic_effect);
        unsafe {
            __buffer
                .add(336)
                .cast::<UPtr<UAbilityTask_WaitGameplayEffectApplied_Self>>()
                .read()
        }
    }
    pub fn wait_gameplay_effect_applied_to_self(
        owning_ability: UPtr<UGameplayAbility>,
        source_filter: FGameplayTargetDataFilterHandle,
        source_tag_requirements: FGameplayTagRequirements,
        target_tag_requirements: FGameplayTagRequirements,
        asset_tag_requirements: FGameplayTagRequirements,
        granted_tag_requirements: FGameplayTagRequirements,
        trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
        listen_for_periodic_effect: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayEffectApplied_Self> {
        let mut __stack = crate::core_data::StackAlloc::<600>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filter,
                __buffer.add(8).cast::<FGameplayTargetDataFilterHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_tag_requirements,
                __buffer.add(24).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_tag_requirements,
                __buffer.add(160).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_tag_requirements,
                __buffer.add(296).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &granted_tag_requirements,
                __buffer.add(432).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(568).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(576).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &listen_for_periodic_effect,
                __buffer.add(584).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEffectApplied_Self::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_self_wait_gameplay_effect_applied_to_self,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(source_filter);
        std::mem::forget(source_tag_requirements);
        std::mem::forget(target_tag_requirements);
        std::mem::forget(asset_tag_requirements);
        std::mem::forget(granted_tag_requirements);
        std::mem::forget(trigger_once);
        std::mem::forget(optional_external_owner);
        std::mem::forget(listen_for_periodic_effect);
        unsafe {
            __buffer
                .add(592)
                .cast::<UPtr<UAbilityTask_WaitGameplayEffectApplied_Self>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayEffectApplied_Target {
    __padding_end: [u8; 1072],
}
impl UAbilityTask_WaitGameplayEffectApplied_Target {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectApplied_Target")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectApplied_Target")
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
    pub fn wait_gameplay_effect_applied_to_target_query(
        owning_ability: UPtr<UGameplayAbility>,
        source_filter: FGameplayTargetDataFilterHandle,
        source_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        target_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        asset_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        granted_tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
        listen_for_periodic_effect: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayEffectApplied_Target> {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filter,
                __buffer.add(8).cast::<FGameplayTargetDataFilterHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_tag_query,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_tag_query,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_tag_query,
                __buffer
                    .add(168)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &granted_tag_query,
                __buffer
                    .add(240)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(312).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(320).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &listen_for_periodic_effect,
                __buffer.add(328).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEffectApplied_Target::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target_query,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(source_filter);
        std::mem::forget(source_tag_query);
        std::mem::forget(target_tag_query);
        std::mem::forget(asset_tag_query);
        std::mem::forget(granted_tag_query);
        std::mem::forget(trigger_once);
        std::mem::forget(optional_external_owner);
        std::mem::forget(listen_for_periodic_effect);
        unsafe {
            __buffer
                .add(336)
                .cast::<UPtr<UAbilityTask_WaitGameplayEffectApplied_Target>>()
                .read()
        }
    }
    pub fn wait_gameplay_effect_applied_to_target(
        owning_ability: UPtr<UGameplayAbility>,
        target_filter: FGameplayTargetDataFilterHandle,
        source_tag_requirements: FGameplayTagRequirements,
        target_tag_requirements: FGameplayTagRequirements,
        asset_tag_requirements: FGameplayTagRequirements,
        granted_tag_requirements: FGameplayTagRequirements,
        trigger_once: bool,
        optional_external_owner: UPtr<crate::bindings::engine::AActor>,
        listen_for_periodic_effects: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayEffectApplied_Target> {
        let mut __stack = crate::core_data::StackAlloc::<600>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_filter,
                __buffer.add(8).cast::<FGameplayTargetDataFilterHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_tag_requirements,
                __buffer.add(24).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_tag_requirements,
                __buffer.add(160).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_tag_requirements,
                __buffer.add(296).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &granted_tag_requirements,
                __buffer.add(432).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_once,
                __buffer.add(568).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_owner,
                __buffer.add(576).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &listen_for_periodic_effects,
                __buffer.add(584).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEffectApplied_Target::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_applied_target_wait_gameplay_effect_applied_to_target,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(target_filter);
        std::mem::forget(source_tag_requirements);
        std::mem::forget(target_tag_requirements);
        std::mem::forget(asset_tag_requirements);
        std::mem::forget(granted_tag_requirements);
        std::mem::forget(trigger_once);
        std::mem::forget(optional_external_owner);
        std::mem::forget(listen_for_periodic_effects);
        unsafe {
            __buffer
                .add(592)
                .cast::<UPtr<UAbilityTask_WaitGameplayEffectApplied_Target>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayEffectBlockedImmunity {
    __padding_end: [u8; 472],
}
impl UAbilityTask_WaitGameplayEffectBlockedImmunity {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectBlockedImmunity")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectBlockedImmunity")
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
    pub fn wait_gameplay_effect_blocked_by_immunity(
        owning_ability: UPtr<UGameplayAbility>,
        source_tag_requirements: FGameplayTagRequirements,
        target_tag_requirements: FGameplayTagRequirements,
        optional_external_target: UPtr<crate::bindings::engine::AActor>,
        only_trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayEffectBlockedImmunity> {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_blocked_immunity_wait_gameplay_effect_blocked_by_immunity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_tag_requirements,
                __buffer.add(8).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_tag_requirements,
                __buffer.add(144).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_target,
                __buffer.add(280).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(288).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEffectBlockedImmunity::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_blocked_immunity_wait_gameplay_effect_blocked_by_immunity,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(source_tag_requirements);
        std::mem::forget(target_tag_requirements);
        std::mem::forget(optional_external_target);
        std::mem::forget(only_trigger_once);
        unsafe {
            __buffer
                .add(296)
                .cast::<UPtr<UAbilityTask_WaitGameplayEffectBlockedImmunity>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayEffectRemoved {
    __padding_end: [u8; 232],
}
impl UAbilityTask_WaitGameplayEffectRemoved {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectRemoved")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectRemoved")
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
    pub fn wait_for_gameplay_effect_removed(
        owning_ability: UPtr<UGameplayAbility>,
        handle: FActiveGameplayEffectHandle,
    ) -> UPtr<UAbilityTask_WaitGameplayEffectRemoved> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_removed_wait_for_gameplay_effect_removed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(8).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEffectRemoved::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_removed_wait_for_gameplay_effect_removed,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(handle);
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<UAbilityTask_WaitGameplayEffectRemoved>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayEffectStackChange {
    __padding_end: [u8; 224],
}
impl UAbilityTask_WaitGameplayEffectStackChange {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectStackChange")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEffectStackChange")
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
    pub fn wait_for_gameplay_effect_stack_change(
        owning_ability: UPtr<UGameplayAbility>,
        handle: FActiveGameplayEffectHandle,
    ) -> UPtr<UAbilityTask_WaitGameplayEffectStackChange> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_stack_change_wait_for_gameplay_effect_stack_change,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(8).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEffectStackChange::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_effect_stack_change_wait_for_gameplay_effect_stack_change,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(handle);
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<UAbilityTask_WaitGameplayEffectStackChange>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayEvent {
    __padding_end: [u8; 216],
}
impl UAbilityTask_WaitGameplayEvent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEvent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayEvent")
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
    pub fn wait_gameplay_event(
        owning_ability: UPtr<UGameplayAbility>,
        event_tag: crate::bindings::gameplay_tags::FGameplayTag,
        optional_external_target: UPtr<crate::bindings::engine::AActor>,
        only_trigger_once: bool,
        only_match_exact: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayEvent> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_event_wait_gameplay_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_external_target,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_match_exact,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayEvent::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_event_wait_gameplay_event,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(event_tag);
        std::mem::forget(optional_external_target);
        std::mem::forget(only_trigger_once);
        std::mem::forget(only_match_exact);
        unsafe { __buffer.add(40).cast::<UPtr<UAbilityTask_WaitGameplayEvent>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayTag {
    __padding_end: [u8; 192],
}
impl UAbilityTask_WaitGameplayTag {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTag")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTag")
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
pub struct UAbilityTask_WaitGameplayTagAdded {
    __padding_end: [u8; 216],
}
impl UAbilityTask_WaitGameplayTagAdded {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagAdded")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagAdded")
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
    pub fn wait_gameplay_tag_add(
        owning_ability: UPtr<UGameplayAbility>,
        tag: crate::bindings::gameplay_tags::FGameplayTag,
        in_optional_external_target: UPtr<crate::bindings::engine::AActor>,
        only_trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayTagAdded> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_added_wait_gameplay_tag_add,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_optional_external_target,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayTagAdded::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_added_wait_gameplay_tag_add,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(tag);
        std::mem::forget(in_optional_external_target);
        std::mem::forget(only_trigger_once);
        unsafe {
            __buffer.add(40).cast::<UPtr<UAbilityTask_WaitGameplayTagAdded>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayTagRemoved {
    __padding_end: [u8; 216],
}
impl UAbilityTask_WaitGameplayTagRemoved {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagRemoved")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagRemoved")
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
    pub fn wait_gameplay_tag_remove(
        owning_ability: UPtr<UGameplayAbility>,
        tag: crate::bindings::gameplay_tags::FGameplayTag,
        in_optional_external_target: UPtr<crate::bindings::engine::AActor>,
        only_trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayTagRemoved> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_removed_wait_gameplay_tag_remove,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_optional_external_target,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &only_trigger_once,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayTagRemoved::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_removed_wait_gameplay_tag_remove,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(tag);
        std::mem::forget(in_optional_external_target);
        std::mem::forget(only_trigger_once);
        unsafe {
            __buffer.add(40).cast::<UPtr<UAbilityTask_WaitGameplayTagRemoved>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayTagCountChanged {
    __padding_end: [u8; 208],
}
impl UAbilityTask_WaitGameplayTagCountChanged {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagCountChanged")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagCountChanged")
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
    pub fn wait_gameplay_tag_count_change(
        owning_ability: UPtr<UGameplayAbility>,
        tag: crate::bindings::gameplay_tags::FGameplayTag,
        in_optional_external_target: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UAbilityTask_WaitGameplayTagCountChanged> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_change,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_optional_external_target,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayTagCountChanged::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_count_changed_wait_gameplay_tag_count_change,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(tag);
        std::mem::forget(in_optional_external_target);
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<UAbilityTask_WaitGameplayTagCountChanged>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitGameplayTagQuery {
    __padding_end: [u8; 384],
}
impl UAbilityTask_WaitGameplayTagQuery {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagQuery")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitGameplayTagQuery")
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
    pub fn wait_gameplay_tag_query(
        owning_ability: UPtr<UGameplayAbility>,
        tag_query: crate::bindings::gameplay_tags::FGameplayTagQuery,
        in_optional_external_target: UPtr<crate::bindings::engine::AActor>,
        trigger_condition: EWaitGameplayTagQueryTriggerCondition,
        b_only_trigger_once: bool,
    ) -> UPtr<UAbilityTask_WaitGameplayTagQuery> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_query_wait_gameplay_tag_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_query,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_optional_external_target,
                __buffer.add(80).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &trigger_condition,
                __buffer.add(88).cast::<EWaitGameplayTagQueryTriggerCondition>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_trigger_once,
                __buffer.add(89).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitGameplayTagQuery::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_gameplay_tag_query_wait_gameplay_tag_query,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(tag_query);
        std::mem::forget(in_optional_external_target);
        std::mem::forget(trigger_condition);
        std::mem::forget(b_only_trigger_once);
        unsafe {
            __buffer.add(96).cast::<UPtr<UAbilityTask_WaitGameplayTagQuery>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitInputPress {
    __padding_end: [u8; 192],
}
impl UAbilityTask_WaitInputPress {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitInputPress")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitInputPress")
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
    pub fn wait_input_press(
        owning_ability: UPtr<UGameplayAbility>,
        b_test_already_pressed: bool,
    ) -> UPtr<UAbilityTask_WaitInputPress> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_input_press_wait_input_press,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_test_already_pressed,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitInputPress::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_input_press_wait_input_press,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(b_test_already_pressed);
        unsafe { __buffer.add(16).cast::<UPtr<UAbilityTask_WaitInputPress>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitInputRelease {
    __padding_end: [u8; 192],
}
impl UAbilityTask_WaitInputRelease {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitInputRelease")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitInputRelease")
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
    pub fn wait_input_release(
        owning_ability: UPtr<UGameplayAbility>,
        b_test_already_released: bool,
    ) -> UPtr<UAbilityTask_WaitInputRelease> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_input_release_wait_input_release,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_test_already_released,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitInputRelease::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_input_release_wait_input_release,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(b_test_already_released);
        unsafe { __buffer.add(16).cast::<UPtr<UAbilityTask_WaitInputRelease>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitMovementModeChange {
    __padding_end: [u8; 192],
}
impl UAbilityTask_WaitMovementModeChange {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitMovementModeChange")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitMovementModeChange")
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
    pub fn create_wait_movement_mode_change(
        owning_ability: UPtr<UGameplayAbility>,
        new_mode: crate::bindings::engine::EMovementMode,
    ) -> UPtr<UAbilityTask_WaitMovementModeChange> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_movement_mode_change_create_wait_movement_mode_change,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mode,
                __buffer.add(8).cast::<crate::bindings::engine::EMovementMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitMovementModeChange::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_movement_mode_change_create_wait_movement_mode_change,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(new_mode);
        unsafe {
            __buffer.add(16).cast::<UPtr<UAbilityTask_WaitMovementModeChange>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitOverlap {
    __padding_end: [u8; 176],
}
impl UAbilityTask_WaitOverlap {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitOverlap")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitOverlap")
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
    pub fn wait_for_overlap(
        owning_ability: UPtr<UGameplayAbility>,
    ) -> UPtr<UAbilityTask_WaitOverlap> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_overlap_wait_for_overlap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitOverlap::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_overlap_wait_for_overlap,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        unsafe { __buffer.add(8).cast::<UPtr<UAbilityTask_WaitOverlap>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitTargetData {
    __padding_end: [u8; 232],
}
impl UAbilityTask_WaitTargetData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitTargetData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitTargetData")
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
    pub fn wait_target_data_using_actor(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        confirmation_type: EGameplayTargetingConfirmation,
        target_actor: UPtr<AGameplayAbilityTargetActor>,
    ) -> UPtr<UAbilityTask_WaitTargetData> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_wait_target_data_using_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &confirmation_type,
                __buffer.add(20).cast::<EGameplayTargetingConfirmation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(24).cast::<UPtr<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitTargetData::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_wait_target_data_using_actor,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(confirmation_type);
        std::mem::forget(target_actor);
        unsafe { __buffer.add(32).cast::<UPtr<UAbilityTask_WaitTargetData>>().read() }
    }
    pub fn wait_target_data(
        owning_ability: UPtr<UGameplayAbility>,
        task_instance_name: FName,
        confirmation_type: EGameplayTargetingConfirmation,
        class: TSubclassOf<AGameplayAbilityTargetActor>,
    ) -> UPtr<UAbilityTask_WaitTargetData> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_wait_target_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &task_instance_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &confirmation_type,
                __buffer.add(20).cast::<EGameplayTargetingConfirmation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer.add(24).cast::<TSubclassOf<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitTargetData::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_wait_target_data,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(task_instance_name);
        std::mem::forget(confirmation_type);
        std::mem::forget(class);
        unsafe { __buffer.add(32).cast::<UPtr<UAbilityTask_WaitTargetData>>().read() }
    }
    pub fn finish_spawning_actor(
        &mut self,
        owning_ability: UPtr<UGameplayAbility>,
        spawned_actor: UPtr<AGameplayAbilityTargetActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_finish_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spawned_actor,
                __buffer.add(8).cast::<UPtr<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_finish_spawning_actor,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(spawned_actor);
    }
    pub fn begin_spawning_actor(
        &mut self,
        owning_ability: UPtr<UGameplayAbility>,
        class: TSubclassOf<AGameplayAbilityTargetActor>,
        spawned_actor: &mut UPtr<AGameplayAbilityTargetActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_begin_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer.add(8).cast::<TSubclassOf<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawned_actor,
                __buffer.add(16).cast::<UPtr<AGameplayAbilityTargetActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_target_data_begin_spawning_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<AGameplayAbilityTargetActor>>()
                .swap(spawned_actor);
        }
        std::mem::forget(owning_ability);
        std::mem::forget(class);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilityTask_WaitVelocityChange {
    __padding_end: [u8; 216],
}
impl UAbilityTask_WaitVelocityChange {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitVelocityChange")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilityTask_WaitVelocityChange")
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
    pub fn create_wait_velocity_change(
        owning_ability: UPtr<UGameplayAbility>,
        direction: crate::bindings::core_u_object::FVector,
        minimum_magnitude: f32,
    ) -> UPtr<UAbilityTask_WaitVelocityChange> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_velocity_change_create_wait_velocity_change,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &direction,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &minimum_magnitude,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilityTask_WaitVelocityChange::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_task_wait_velocity_change_create_wait_velocity_change,
                __buffer,
            )
        };
        std::mem::forget(owning_ability);
        std::mem::forget(direction);
        std::mem::forget(minimum_magnitude);
        unsafe {
            __buffer.add(40).cast::<UPtr<UAbilityTask_WaitVelocityChange>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAbilitySystemBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAbilitySystemBlueprintLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemBlueprintLibrary")
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
    pub fn unbind_gameplay_tag_changed_event_wrapper_for_handle(
        tag: crate::bindings::gameplay_tags::FGameplayTag,
        handle: FGameplayTagChangedEventWrapperSpecHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_unbind_gameplay_tag_changed_event_wrapper_for_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(16).cast::<FGameplayTagChangedEventWrapperSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_unbind_gameplay_tag_changed_event_wrapper_for_handle,
                __buffer,
            )
        };
        std::mem::forget(tag);
        std::mem::forget(handle);
    }
    pub fn unbind_all_gameplay_tag_changed_event_wrappers_for_handle(
        handle: FGameplayTagChangedEventWrapperSpecHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_unbind_all_gameplay_tag_changed_event_wrappers_for_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FGameplayTagChangedEventWrapperSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_unbind_all_gameplay_tag_changed_event_wrappers_for_handle,
                __buffer,
            )
        };
        std::mem::forget(handle);
    }
    pub fn target_data_has_origin(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_origin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_origin,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn target_data_has_hit_result(
        hit_result: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit_result,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_hit_result,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn target_data_has_end_point(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_end_point,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_end_point,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn target_data_has_actor(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_target_data_has_actor,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn set_stack_count_to_max(
        spec_handle: FGameplayEffectSpecHandle,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_stack_count_to_max,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_stack_count_to_max,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        unsafe { __buffer.add(16).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn set_stack_count(
        spec_handle: FGameplayEffectSpecHandle,
        stack_count: i32,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_stack_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stack_count,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_stack_count,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(stack_count);
        unsafe { __buffer.add(24).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn set_duration(
        spec_handle: FGameplayEffectSpecHandle,
        duration: f32,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_duration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_set_duration,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(duration);
        unsafe { __buffer.add(24).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn send_gameplay_event_to_actor(
        actor: UPtr<crate::bindings::engine::AActor>,
        event_tag: crate::bindings::gameplay_tags::FGameplayTag,
        payload: FGameplayEventData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_send_gameplay_event_to_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload,
                __buffer.add(24).cast::<FGameplayEventData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_send_gameplay_event_to_actor,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(event_tag);
        std::mem::forget(payload);
    }
    pub fn remove_loose_gameplay_tags(
        actor: UPtr<crate::bindings::engine::AActor>,
        gameplay_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        b_should_replicate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_remove_loose_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tags,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_replicate,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_remove_loose_gameplay_tags,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(b_should_replicate);
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn remove_gameplay_tags(
        actor: UPtr<crate::bindings::engine::AActor>,
        gameplay_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        replication_rule: EGameplayTagReplicationState,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_remove_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tags,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &replication_rule,
                __buffer.add(40).cast::<EGameplayTagReplicationState>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_remove_gameplay_tags,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(replication_rule);
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn not_equal_gameplay_attribute_gameplay_attribute(
        attribute_a: FGameplayAttribute,
        attribute_b: FGameplayAttribute,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<145>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_gameplay_attribute_gameplay_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_a,
                __buffer.add(0).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_b,
                __buffer.add(72).cast::<FGameplayAttribute>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_gameplay_attribute_gameplay_attribute,
                __buffer,
            )
        };
        std::mem::forget(attribute_a);
        std::mem::forget(attribute_b);
        unsafe { __buffer.add(144).cast::<bool>().read() }
    }
    pub fn not_equal_gameplay_ability_spec_handle(
        a: &FGameplayAbilitySpecHandle,
        b: &FGameplayAbilitySpecHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_gameplay_ability_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(4).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_gameplay_ability_spec_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn not_equal_active_gameplay_effect_handle(
        a: &FActiveGameplayEffectHandle,
        b: &FActiveGameplayEffectHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_active_gameplay_effect_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(8).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_not_equal_active_gameplay_effect_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn make_spec_handle_by_class(
        gameplay_effect: TSubclassOf<UGameplayEffect>,
        instigator: UPtr<crate::bindings::engine::AActor>,
        effect_causer: UPtr<crate::bindings::engine::AActor>,
        level: f32,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_spec_handle_by_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instigator,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_causer,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(24).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_spec_handle_by_class,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect);
        std::mem::forget(instigator);
        std::mem::forget(effect_causer);
        std::mem::forget(level);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn make_spec_handle(
        in_gameplay_effect: UPtr<UGameplayEffect>,
        in_instigator: UPtr<crate::bindings::engine::AActor>,
        in_effect_causer: UPtr<crate::bindings::engine::AActor>,
        in_level: f32,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_gameplay_effect,
                __buffer.add(0).cast::<UPtr<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_instigator,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_effect_causer,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_level, __buffer.add(24).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_spec_handle,
                __buffer,
            )
        };
        std::mem::forget(in_gameplay_effect);
        std::mem::forget(in_instigator);
        std::mem::forget(in_effect_causer);
        std::mem::forget(in_level);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn make_gameplay_cue_parameters(
        normalized_magnitude: f32,
        raw_magnitude: f32,
        effect_context: FGameplayEffectContextHandle,
        matched_tag_name: crate::bindings::gameplay_tags::FGameplayTag,
        original_tag: crate::bindings::gameplay_tags::FGameplayTag,
        aggregated_source_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
        aggregated_target_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
        location: crate::bindings::core_u_object::FVector,
        normal: crate::bindings::core_u_object::FVector,
        instigator: UPtr<crate::bindings::engine::AActor>,
        effect_causer: UPtr<crate::bindings::engine::AActor>,
        source_object: UPtr<crate::bindings::core_u_object::UObject>,
        physical_material: UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
        gameplay_effect_level: i32,
        ability_level: i32,
        target_attach_component: UPtr<crate::bindings::engine::USceneComponent>,
        b_replicate_location_when_using_minimal_rep_proxy: bool,
    ) -> FGameplayCueParameters {
        let mut __stack = crate::core_data::StackAlloc::<448>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_gameplay_cue_parameters,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &normalized_magnitude,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &raw_magnitude,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(8).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &matched_tag_name,
                __buffer.add(32).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &original_tag,
                __buffer.add(44).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &aggregated_source_tags,
                __buffer
                    .add(56)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &aggregated_target_tags,
                __buffer
                    .add(88)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(120).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &normal,
                __buffer.add(144).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instigator,
                __buffer.add(168).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_causer,
                __buffer.add(176).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_object,
                __buffer
                    .add(184)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &physical_material,
                __buffer
                    .add(192)
                    .cast::<UPtr<crate::bindings::physics_core::UPhysicalMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_level,
                __buffer.add(200).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_level,
                __buffer.add(204).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_attach_component,
                __buffer
                    .add(208)
                    .cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replicate_location_when_using_minimal_rep_proxy,
                __buffer.add(216).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_gameplay_cue_parameters,
                __buffer,
            )
        };
        std::mem::forget(normalized_magnitude);
        std::mem::forget(raw_magnitude);
        std::mem::forget(effect_context);
        std::mem::forget(matched_tag_name);
        std::mem::forget(original_tag);
        std::mem::forget(aggregated_source_tags);
        std::mem::forget(aggregated_target_tags);
        std::mem::forget(location);
        std::mem::forget(normal);
        std::mem::forget(instigator);
        std::mem::forget(effect_causer);
        std::mem::forget(source_object);
        std::mem::forget(physical_material);
        std::mem::forget(gameplay_effect_level);
        std::mem::forget(ability_level);
        std::mem::forget(target_attach_component);
        std::mem::forget(b_replicate_location_when_using_minimal_rep_proxy);
        unsafe { __buffer.add(224).cast::<FGameplayCueParameters>().read() }
    }
    pub fn make_filter_handle(
        filter: FGameplayTargetDataFilter,
        filter_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> FGameplayTargetDataFilterHandle {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_filter_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter,
                __buffer.add(0).cast::<FGameplayTargetDataFilter>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_actor,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_make_filter_handle,
                __buffer,
            )
        };
        std::mem::forget(filter);
        std::mem::forget(filter_actor);
        unsafe { __buffer.add(40).cast::<FGameplayTargetDataFilterHandle>().read() }
    }
    pub fn is_valid(attribute: FGameplayAttribute) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(0).cast::<FGameplayAttribute>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_valid,
                __buffer,
            )
        };
        std::mem::forget(attribute);
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn is_instigator_locally_controlled_player(
        parameters: FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<225>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_instigator_locally_controlled_player,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_instigator_locally_controlled_player,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe { __buffer.add(224).cast::<bool>().read() }
    }
    pub fn is_instigator_locally_controlled(parameters: FGameplayCueParameters) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<225>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_instigator_locally_controlled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_instigator_locally_controlled,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe { __buffer.add(224).cast::<bool>().read() }
    }
    pub fn is_gameplay_ability_active(gameplay_ability: UPtr<UGameplayAbility>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_gameplay_ability_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_ability,
                __buffer.add(0).cast::<UPtr<UGameplayAbility>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_gameplay_ability_active,
                __buffer,
            )
        };
        std::mem::forget(gameplay_ability);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_duration_gameplay_effect_spec_handle(
        handle: FGameplayEffectSpecHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_duration_gameplay_effect_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_duration_gameplay_effect_spec_handle,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_active_gameplay_effect_handle_valid(
        handle: FActiveGameplayEffectHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_active_gameplay_effect_handle_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_active_gameplay_effect_handle_valid,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_active_gameplay_effect_handle_active(
        handle: FActiveGameplayEffectHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_active_gameplay_effect_handle_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_is_active_gameplay_effect_handle_active,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_hit_result(parameters: FGameplayCueParameters) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<225>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_has_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_has_hit_result,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe { __buffer.add(224).cast::<bool>().read() }
    }
    pub fn has_any_abilities_with_asset_tag(
        ability_system_component: UPtr<UAbilitySystemComponent>,
        asset_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_has_any_abilities_with_asset_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system_component,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_has_any_abilities_with_asset_tag,
                __buffer,
            )
        };
        std::mem::forget(ability_system_component);
        std::mem::forget(asset_tag);
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_target_data_origin(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_origin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_origin,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_target_data_end_point_transform(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_end_point_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_end_point_transform,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_target_data_end_point(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_end_point,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_target_data_end_point,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_origin(
        parameters: FGameplayCueParameters,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_origin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_origin,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe {
            __buffer.add(224).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_modified_attribute_magnitude(
        spec_handle: FGameplayEffectSpecHandle,
        attribute: FGameplayAttribute,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_modified_attribute_magnitude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(16).cast::<FGameplayAttribute>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_modified_attribute_magnitude,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(attribute);
        unsafe { __buffer.add(88).cast::<f32>().read() }
    }
    pub fn get_instigator_transform(
        parameters: FGameplayCueParameters,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_instigator_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_instigator_transform,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe {
            __buffer.add(224).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_instigator_actor(
        parameters: FGameplayCueParameters,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_instigator_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_instigator_actor,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe {
            __buffer.add(224).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn get_hit_result_from_target_data(
        hit_result: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> crate::bindings::engine::FHitResult {
        let mut __stack = crate::core_data::StackAlloc::<312>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_hit_result_from_target_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit_result,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_hit_result_from_target_data,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe { __buffer.add(48).cast::<crate::bindings::engine::FHitResult>().read() }
    }
    pub fn get_hit_result(
        parameters: FGameplayCueParameters,
    ) -> crate::bindings::engine::FHitResult {
        let mut __stack = crate::core_data::StackAlloc::<488>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_hit_result,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe { __buffer.add(224).cast::<crate::bindings::engine::FHitResult>().read() }
    }
    pub fn get_granted_tags(
        spec_handle: FGameplayEffectSpecHandle,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_granted_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_granted_tags,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_gameplay_effect_ui_data(
        effect_class: TSubclassOf<UGameplayEffect>,
        data_type: TSubclassOf<UGameplayEffectUIData>,
    ) -> UPtr<UGameplayEffectUIData> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_ui_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(8).cast::<TSubclassOf<UGameplayEffectUIData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_ui_data,
                __buffer,
            )
        };
        std::mem::forget(effect_class);
        std::mem::forget(data_type);
        unsafe { __buffer.add(16).cast::<UPtr<UGameplayEffectUIData>>().read() }
    }
    pub fn get_gameplay_effect_granted_tags(
        effect_class: TSubclassOf<UGameplayEffect>,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_granted_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_granted_tags,
                __buffer,
            )
        };
        std::mem::forget(effect_class);
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_gameplay_effect_from_spec_handle(
        handle: FGameplayEffectSpecHandle,
    ) -> UPtr<UGameplayEffect> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_from_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_from_spec_handle,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe { __buffer.add(16).cast::<UPtr<UGameplayEffect>>().read() }
    }
    pub fn get_gameplay_effect_from_active_effect_handle(
        active_handle: &FActiveGameplayEffectHandle,
    ) -> UPtr<UGameplayEffect> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_from_active_effect_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_from_active_effect_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UGameplayEffect>>().read() }
    }
    pub fn get_gameplay_effect_asset_tags(
        effect_class: TSubclassOf<UGameplayEffect>,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_asset_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_effect_asset_tags,
                __buffer,
            )
        };
        std::mem::forget(effect_class);
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_gameplay_cue_end_location_and_normal(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        parameters: FGameplayCueParameters,
        location: &mut crate::bindings::core_u_object::FVector,
        normal: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<281>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_cue_end_location_and_normal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(232).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normal,
                __buffer.add(256).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_cue_end_location_and_normal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(232)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(location);
        }
        unsafe {
            __buffer
                .add(256)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(normal);
        }
        std::mem::forget(target_actor);
        std::mem::forget(parameters);
        unsafe { __buffer.add(280).cast::<bool>().read() }
    }
    pub fn get_gameplay_cue_direction(
        target_actor: UPtr<crate::bindings::engine::AActor>,
        parameters: FGameplayCueParameters,
        direction: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<257>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_cue_direction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                direction,
                __buffer.add(232).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_cue_direction,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(232)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(direction);
        }
        std::mem::forget(target_actor);
        std::mem::forget(parameters);
        unsafe { __buffer.add(256).cast::<bool>().read() }
    }
    pub fn get_gameplay_ability_from_spec_handle(
        ability_system: UPtr<UAbilitySystemComponent>,
        ability_spec_handle: &FGameplayAbilitySpecHandle,
        b_is_instance: &mut bool,
    ) -> UPtr<UGameplayAbility> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_ability_from_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ability_spec_handle,
                __buffer.add(8).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_instance,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_gameplay_ability_from_spec_handle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_instance);
        }
        std::mem::forget(ability_system);
        unsafe { __buffer.add(16).cast::<UPtr<UGameplayAbility>>().read() }
    }
    pub fn get_float_attribute_from_ability_system_component(
        ability_system: UPtr<UAbilitySystemComponent>,
        attribute: FGameplayAttribute,
        b_successfully_found_attribute: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_from_ability_system_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_successfully_found_attribute,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_from_ability_system_component,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<bool>().swap(b_successfully_found_attribute);
        }
        std::mem::forget(ability_system);
        std::mem::forget(attribute);
        unsafe { __buffer.add(84).cast::<f32>().read() }
    }
    pub fn get_float_attribute_base_from_ability_system_component(
        ability_system_component: UPtr<UAbilitySystemComponent>,
        attribute: FGameplayAttribute,
        b_successfully_found_attribute: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_base_from_ability_system_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system_component,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_successfully_found_attribute,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_base_from_ability_system_component,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<bool>().swap(b_successfully_found_attribute);
        }
        std::mem::forget(ability_system_component);
        std::mem::forget(attribute);
        unsafe { __buffer.add(84).cast::<f32>().read() }
    }
    pub fn get_float_attribute_base(
        actor: UPtr<crate::bindings::engine::AActor>,
        attribute: FGameplayAttribute,
        b_successfully_found_attribute: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_base,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_successfully_found_attribute,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute_base,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<bool>().swap(b_successfully_found_attribute);
        }
        std::mem::forget(actor);
        std::mem::forget(attribute);
        unsafe { __buffer.add(84).cast::<f32>().read() }
    }
    pub fn get_float_attribute(
        actor: UPtr<crate::bindings::engine::AActor>,
        attribute: FGameplayAttribute,
        b_successfully_found_attribute: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_successfully_found_attribute,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_float_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<bool>().swap(b_successfully_found_attribute);
        }
        std::mem::forget(actor);
        std::mem::forget(attribute);
        unsafe { __buffer.add(84).cast::<f32>().read() }
    }
    pub fn get_effect_context(
        spec_handle: FGameplayEffectSpecHandle,
    ) -> FGameplayEffectContextHandle {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_effect_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_effect_context,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        unsafe { __buffer.add(16).cast::<FGameplayEffectContextHandle>().read() }
    }
    pub fn get_duration_policy_from_gameplay_effect_spec_handle(
        handle: FGameplayEffectSpecHandle,
    ) -> EGameplayEffectDurationType {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_duration_policy_from_gameplay_effect_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_duration_policy_from_gameplay_effect_spec_handle,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe { __buffer.add(16).cast::<EGameplayEffectDurationType>().read() }
    }
    pub fn get_debug_string_from_gameplay_attribute(
        attribute: &FGameplayAttribute,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_debug_string_from_gameplay_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute,
                __buffer.add(0).cast::<FGameplayAttribute>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_debug_string_from_gameplay_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<FString>().read() }
    }
    pub fn get_data_count_from_target_data(
        target_data: &FGameplayAbilityTargetDataHandle,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_data_count_from_target_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_data_count_from_target_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<i32>().read() }
    }
    pub fn get_asset_tags(
        spec_handle: FGameplayEffectSpecHandle,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_asset_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_asset_tags,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_all_linked_gameplay_effect_spec_handles(
        spec_handle: FGameplayEffectSpecHandle,
    ) -> TArray<FGameplayEffectSpecHandle> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_all_linked_gameplay_effect_spec_handles,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_all_linked_gameplay_effect_spec_handles,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        unsafe { __buffer.add(16).cast::<TArray<FGameplayEffectSpecHandle>>().read() }
    }
    pub fn get_all_actors_from_target_data(
        target_data: &FGameplayAbilityTargetDataHandle,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_all_actors_from_target_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_all_actors_from_target_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_actors_from_target_data(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actors_from_target_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actors_from_target_data,
                __buffer,
            )
        };
        std::mem::forget(index);
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_actor_count(parameters: FGameplayCueParameters) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<228>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actor_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actor_count,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe { __buffer.add(224).cast::<i32>().read() }
    }
    pub fn get_actor_by_index(
        parameters: FGameplayCueParameters,
        index: i32,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actor_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(224).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_actor_by_index,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        std::mem::forget(index);
        unsafe {
            __buffer.add(232).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn get_active_gameplay_effect_total_duration(
        active_handle: FActiveGameplayEffectHandle,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_total_duration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_total_duration,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_active_gameplay_effect_start_time(
        active_handle: FActiveGameplayEffectHandle,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_start_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_start_time,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_active_gameplay_effect_stack_limit_count(
        active_handle: FActiveGameplayEffectHandle,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_stack_limit_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_stack_limit_count,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_active_gameplay_effect_stack_count(
        active_handle: FActiveGameplayEffectHandle,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_stack_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_stack_count,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_active_gameplay_effect_remaining_duration(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        active_handle: FActiveGameplayEffectHandle,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_remaining_duration,
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
                &active_handle,
                __buffer.add(8).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_remaining_duration,
                __buffer,
            )
        };
        std::mem::forget(world_context_object);
        std::mem::forget(active_handle);
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_active_gameplay_effect_expected_end_time(
        active_handle: FActiveGameplayEffectHandle,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_expected_end_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_expected_end_time,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_active_gameplay_effect_debug_string(
        active_handle: FActiveGameplayEffectHandle,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_debug_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_active_gameplay_effect_debug_string,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_ability_system_component_from_active_gameplay_effect_handle(
        handle: FActiveGameplayEffectHandle,
    ) -> UPtr<UAbilitySystemComponent> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_ability_system_component_from_active_gameplay_effect_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_ability_system_component_from_active_gameplay_effect_handle,
                __buffer,
            )
        };
        std::mem::forget(handle);
        unsafe { __buffer.add(8).cast::<UPtr<UAbilitySystemComponent>>().read() }
    }
    pub fn get_ability_system_component(
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UAbilitySystemComponent> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_ability_system_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_get_ability_system_component,
                __buffer,
            )
        };
        std::mem::forget(actor);
        unsafe { __buffer.add(8).cast::<UPtr<UAbilitySystemComponent>>().read() }
    }
    pub fn forward_gameplay_cue_to_target(
        target_cue_interface: TScriptInterface<UGameplayCueInterface>,
        event_type: EGameplayCueEvent,
        parameters: FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_forward_gameplay_cue_to_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_cue_interface,
                __buffer.add(0).cast::<TScriptInterface<UGameplayCueInterface>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_type,
                __buffer.add(16).cast::<EGameplayCueEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(24).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_forward_gameplay_cue_to_target,
                __buffer,
            )
        };
        std::mem::forget(target_cue_interface);
        std::mem::forget(event_type);
        std::mem::forget(parameters);
    }
    pub fn filter_target_data(
        target_data_handle: &FGameplayAbilityTargetDataHandle,
        actor_filter_class: FGameplayTargetDataFilterHandle,
    ) -> FGameplayAbilityTargetDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_filter_target_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data_handle,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_filter_class,
                __buffer.add(40).cast::<FGameplayTargetDataFilterHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_filter_target_data,
                __buffer,
            )
        };
        std::mem::forget(actor_filter_class);
        unsafe { __buffer.add(56).cast::<FGameplayAbilityTargetDataHandle>().read() }
    }
    pub fn evaluate_attribute_value_with_tags_and_base(
        ability_system: UPtr<UAbilitySystemComponent>,
        attribute: FGameplayAttribute,
        source_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        target_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        base_value: f32,
        b_success: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<156>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_evaluate_attribute_value_with_tags_and_base,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_tags,
                __buffer
                    .add(80)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_tags,
                __buffer
                    .add(112)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_value,
                __buffer.add(144).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_success,
                __buffer.add(148).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_evaluate_attribute_value_with_tags_and_base,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(148).cast::<bool>().swap(b_success);
        }
        std::mem::forget(ability_system);
        std::mem::forget(attribute);
        std::mem::forget(base_value);
        unsafe { __buffer.add(152).cast::<f32>().read() }
    }
    pub fn evaluate_attribute_value_with_tags(
        ability_system: UPtr<UAbilitySystemComponent>,
        attribute: FGameplayAttribute,
        source_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        target_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        b_success: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_evaluate_attribute_value_with_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_tags,
                __buffer
                    .add(80)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_tags,
                __buffer
                    .add(112)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_success,
                __buffer.add(144).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_evaluate_attribute_value_with_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(144).cast::<bool>().swap(b_success);
        }
        std::mem::forget(ability_system);
        std::mem::forget(attribute);
        unsafe { __buffer.add(148).cast::<f32>().read() }
    }
    pub fn equal_equal_gameplay_attribute_gameplay_attribute(
        attribute_a: FGameplayAttribute,
        attribute_b: FGameplayAttribute,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<145>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_gameplay_attribute_gameplay_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_a,
                __buffer.add(0).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_b,
                __buffer.add(72).cast::<FGameplayAttribute>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_gameplay_attribute_gameplay_attribute,
                __buffer,
            )
        };
        std::mem::forget(attribute_a);
        std::mem::forget(attribute_b);
        unsafe { __buffer.add(144).cast::<bool>().read() }
    }
    pub fn equal_equal_gameplay_ability_spec_handle(
        a: &FGameplayAbilitySpecHandle,
        b: &FGameplayAbilitySpecHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_gameplay_ability_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(4).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_gameplay_ability_spec_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn equal_equal_active_gameplay_effect_handle(
        a: &FActiveGameplayEffectHandle,
        b: &FActiveGameplayEffectHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_active_gameplay_effect_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                a,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b,
                __buffer.add(8).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_equal_equal_active_gameplay_effect_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn effect_context_set_origin(
        effect_context: FGameplayEffectContextHandle,
        origin: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_set_origin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &origin,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_set_origin,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        std::mem::forget(origin);
    }
    pub fn effect_context_is_valid(
        effect_context: FGameplayEffectContextHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_is_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_is_valid,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn effect_context_is_instigator_locally_controlled(
        effect_context: FGameplayEffectContextHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_is_instigator_locally_controlled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_is_instigator_locally_controlled,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn effect_context_has_hit_result(
        effect_context: FGameplayEffectContextHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_has_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_has_hit_result,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn effect_context_get_source_object(
        effect_context: FGameplayEffectContextHandle,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_source_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_source_object,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn effect_context_get_original_instigator_actor(
        effect_context: FGameplayEffectContextHandle,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_original_instigator_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_original_instigator_actor,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn effect_context_get_origin(
        effect_context: FGameplayEffectContextHandle,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_origin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_origin,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn effect_context_get_instigator_actor(
        effect_context: FGameplayEffectContextHandle,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_instigator_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_instigator_actor,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn effect_context_get_hit_result(
        effect_context: FGameplayEffectContextHandle,
    ) -> crate::bindings::engine::FHitResult {
        let mut __stack = crate::core_data::StackAlloc::<288>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_hit_result,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe { __buffer.add(24).cast::<crate::bindings::engine::FHitResult>().read() }
    }
    pub fn effect_context_get_effect_causer(
        effect_context: FGameplayEffectContextHandle,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_effect_causer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_get_effect_causer,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn effect_context_add_hit_result(
        effect_context: FGameplayEffectContextHandle,
        hit_result: crate::bindings::engine::FHitResult,
        b_reset: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<289>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_add_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(0).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hit_result,
                __buffer.add(24).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_reset, __buffer.add(288).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_effect_context_add_hit_result,
                __buffer,
            )
        };
        std::mem::forget(effect_context);
        std::mem::forget(hit_result);
        std::mem::forget(b_reset);
    }
    pub fn does_target_data_contain_actor(
        target_data: &FGameplayAbilityTargetDataHandle,
        index: i32,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_does_target_data_contain_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_data,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(40).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_does_target_data_contain_actor,
                __buffer,
            )
        };
        std::mem::forget(index);
        std::mem::forget(actor);
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn does_gameplay_cue_meet_tag_requirements(
        parameters: FGameplayCueParameters,
        source_tag_reqs: &FGameplayTagRequirements,
        target_tag_reqs: &FGameplayTagRequirements,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<497>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_does_gameplay_cue_meet_tag_requirements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_tag_reqs,
                __buffer.add(224).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_tag_reqs,
                __buffer.add(360).cast::<FGameplayTagRequirements>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_does_gameplay_cue_meet_tag_requirements,
                __buffer,
            )
        };
        std::mem::forget(parameters);
        unsafe { __buffer.add(496).cast::<bool>().read() }
    }
    pub fn conv_scalable_float_to_float(input: &FScalableFloat, level: f32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_conv_scalable_float_to_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input,
                __buffer.add(0).cast::<FScalableFloat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(56).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_conv_scalable_float_to_float,
                __buffer,
            )
        };
        std::mem::forget(level);
        unsafe { __buffer.add(60).cast::<f32>().read() }
    }
    pub fn conv_scalable_float_to_double(input: &FScalableFloat, level: f32) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_conv_scalable_float_to_double,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input,
                __buffer.add(0).cast::<FScalableFloat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(56).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_conv_scalable_float_to_double,
                __buffer,
            )
        };
        std::mem::forget(level);
        unsafe { __buffer.add(64).cast::<f64>().read() }
    }
    pub fn clone_spec_handle(
        in_new_instigator: UPtr<crate::bindings::engine::AActor>,
        in_effect_causer: UPtr<crate::bindings::engine::AActor>,
        gameplay_effect_spec_handle_clone: FGameplayEffectSpecHandle,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_clone_spec_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_instigator,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_effect_causer,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_spec_handle_clone,
                __buffer.add(16).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_clone_spec_handle,
                __buffer,
            )
        };
        std::mem::forget(in_new_instigator);
        std::mem::forget(in_effect_causer);
        std::mem::forget(gameplay_effect_spec_handle_clone);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn break_gameplay_cue_parameters(
        parameters: &FGameplayCueParameters,
        normalized_magnitude: &mut f32,
        raw_magnitude: &mut f32,
        effect_context: &mut FGameplayEffectContextHandle,
        matched_tag_name: &mut crate::bindings::gameplay_tags::FGameplayTag,
        original_tag: &mut crate::bindings::gameplay_tags::FGameplayTag,
        aggregated_source_tags: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
        aggregated_target_tags: &mut crate::bindings::gameplay_tags::FGameplayTagContainer,
        location: &mut crate::bindings::core_u_object::FVector,
        normal: &mut crate::bindings::core_u_object::FVector,
        instigator: &mut UPtr<crate::bindings::engine::AActor>,
        effect_causer: &mut UPtr<crate::bindings::engine::AActor>,
        source_object: &mut UPtr<crate::bindings::core_u_object::UObject>,
        physical_material: &mut UPtr<crate::bindings::physics_core::UPhysicalMaterial>,
        gameplay_effect_level: &mut i32,
        ability_level: &mut i32,
        target_attach_component: &mut UPtr<crate::bindings::engine::USceneComponent>,
        b_replicate_location_when_using_minimal_rep_proxy: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<441>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_break_gameplay_cue_parameters,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(0).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normalized_magnitude,
                __buffer.add(224).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                raw_magnitude,
                __buffer.add(228).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_context,
                __buffer.add(232).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                matched_tag_name,
                __buffer.add(256).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                original_tag,
                __buffer.add(268).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                aggregated_source_tags,
                __buffer
                    .add(280)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                aggregated_target_tags,
                __buffer
                    .add(312)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(344).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normal,
                __buffer.add(368).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                instigator,
                __buffer.add(392).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_causer,
                __buffer.add(400).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_object,
                __buffer
                    .add(408)
                    .cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                physical_material,
                __buffer
                    .add(416)
                    .cast::<UPtr<crate::bindings::physics_core::UPhysicalMaterial>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_effect_level,
                __buffer.add(424).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ability_level,
                __buffer.add(428).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_attach_component,
                __buffer
                    .add(432)
                    .cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_replicate_location_when_using_minimal_rep_proxy,
                __buffer.add(440).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_break_gameplay_cue_parameters,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(224).cast::<f32>().swap(normalized_magnitude);
        }
        unsafe {
            __buffer.add(228).cast::<f32>().swap(raw_magnitude);
        }
        unsafe {
            __buffer
                .add(232)
                .cast::<FGameplayEffectContextHandle>()
                .swap(effect_context);
        }
        unsafe {
            __buffer
                .add(256)
                .cast::<crate::bindings::gameplay_tags::FGameplayTag>()
                .swap(matched_tag_name);
        }
        unsafe {
            __buffer
                .add(268)
                .cast::<crate::bindings::gameplay_tags::FGameplayTag>()
                .swap(original_tag);
        }
        unsafe {
            __buffer
                .add(280)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(aggregated_source_tags);
        }
        unsafe {
            __buffer
                .add(312)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .swap(aggregated_target_tags);
        }
        unsafe {
            __buffer
                .add(344)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(location);
        }
        unsafe {
            __buffer
                .add(368)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(normal);
        }
        unsafe {
            __buffer
                .add(392)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(instigator);
        }
        unsafe {
            __buffer
                .add(400)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(effect_causer);
        }
        unsafe {
            __buffer
                .add(408)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .swap(source_object);
        }
        unsafe {
            __buffer
                .add(416)
                .cast::<UPtr<crate::bindings::physics_core::UPhysicalMaterial>>()
                .swap(physical_material);
        }
        unsafe {
            __buffer.add(424).cast::<i32>().swap(gameplay_effect_level);
        }
        unsafe {
            __buffer.add(428).cast::<i32>().swap(ability_level);
        }
        unsafe {
            __buffer
                .add(432)
                .cast::<UPtr<crate::bindings::engine::USceneComponent>>()
                .swap(target_attach_component);
        }
        unsafe {
            __buffer
                .add(440)
                .cast::<bool>()
                .swap(b_replicate_location_when_using_minimal_rep_proxy);
        }
    }
    pub fn bind_event_wrapper_to_gameplay_tag_changed(
        ability_system_component: UPtr<UAbilitySystemComponent>,
        tag: crate::bindings::gameplay_tags::FGameplayTag,
        gameplay_tag_changed_event_wrapper_delegate: FBindEventWrapperToGameplayTagChanged_GameplayTagChangedEventWrapperDelegate,
        b_execute_immediately_if_tag_applied: bool,
        tag_listening_policy: EGameplayTagEventType,
    ) -> FGameplayTagChangedEventWrapperSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_gameplay_tag_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system_component,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_tag_changed_event_wrapper_delegate,
                __buffer
                    .add(24)
                    .cast::<
                        FBindEventWrapperToGameplayTagChanged_GameplayTagChangedEventWrapperDelegate,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_execute_immediately_if_tag_applied,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_listening_policy,
                __buffer.add(57).cast::<EGameplayTagEventType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_gameplay_tag_changed,
                __buffer,
            )
        };
        std::mem::forget(ability_system_component);
        std::mem::forget(tag);
        std::mem::forget(gameplay_tag_changed_event_wrapper_delegate);
        std::mem::forget(b_execute_immediately_if_tag_applied);
        std::mem::forget(tag_listening_policy);
        unsafe {
            __buffer.add(64).cast::<FGameplayTagChangedEventWrapperSpecHandle>().read()
        }
    }
    pub fn bind_event_wrapper_to_any_of_gameplay_tags_changed(
        ability_system_component: UPtr<UAbilitySystemComponent>,
        tags: &TArray<crate::bindings::gameplay_tags::FGameplayTag>,
        gameplay_tag_changed_event_wrapper_delegate: FBindEventWrapperToAnyOfGameplayTagsChanged_GameplayTagChangedEventWrapperDelegate,
        b_execute_immediately_if_tag_applied: bool,
        tag_listening_policy: EGameplayTagEventType,
    ) -> FGameplayTagChangedEventWrapperSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tags_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system_component,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tags,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::gameplay_tags::FGameplayTag>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_tag_changed_event_wrapper_delegate,
                __buffer
                    .add(24)
                    .cast::<
                        FBindEventWrapperToAnyOfGameplayTagsChanged_GameplayTagChangedEventWrapperDelegate,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_execute_immediately_if_tag_applied,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_listening_policy,
                __buffer.add(57).cast::<EGameplayTagEventType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tags_changed,
                __buffer,
            )
        };
        std::mem::forget(ability_system_component);
        std::mem::forget(gameplay_tag_changed_event_wrapper_delegate);
        std::mem::forget(b_execute_immediately_if_tag_applied);
        std::mem::forget(tag_listening_policy);
        unsafe {
            __buffer.add(64).cast::<FGameplayTagChangedEventWrapperSpecHandle>().read()
        }
    }
    pub fn bind_event_wrapper_to_any_of_gameplay_tag_container_changed(
        ability_system_component: UPtr<UAbilitySystemComponent>,
        tag_container: crate::bindings::gameplay_tags::FGameplayTagContainer,
        gameplay_tag_changed_event_wrapper_delegate: FBindEventWrapperToAnyOfGameplayTagContainerChanged_GameplayTagChangedEventWrapperDelegate,
        b_execute_immediately_if_tag_applied: bool,
        tag_listening_policy: EGameplayTagEventType,
    ) -> FGameplayTagChangedEventWrapperSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tag_container_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_system_component,
                __buffer.add(0).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_container,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_tag_changed_event_wrapper_delegate,
                __buffer
                    .add(40)
                    .cast::<
                        FBindEventWrapperToAnyOfGameplayTagContainerChanged_GameplayTagChangedEventWrapperDelegate,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_execute_immediately_if_tag_applied,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_listening_policy,
                __buffer.add(73).cast::<EGameplayTagEventType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_bind_event_wrapper_to_any_of_gameplay_tag_container_changed,
                __buffer,
            )
        };
        std::mem::forget(ability_system_component);
        std::mem::forget(tag_container);
        std::mem::forget(gameplay_tag_changed_event_wrapper_delegate);
        std::mem::forget(b_execute_immediately_if_tag_applied);
        std::mem::forget(tag_listening_policy);
        unsafe {
            __buffer.add(80).cast::<FGameplayTagChangedEventWrapperSpecHandle>().read()
        }
    }
    pub fn assign_tag_set_by_caller_magnitude(
        spec_handle: FGameplayEffectSpecHandle,
        data_tag: crate::bindings::gameplay_tags::FGameplayTag,
        magnitude: f32,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_assign_tag_set_by_caller_magnitude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_tag,
                __buffer.add(16).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(28).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_assign_tag_set_by_caller_magnitude,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(data_tag);
        std::mem::forget(magnitude);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn assign_set_by_caller_magnitude(
        spec_handle: FGameplayEffectSpecHandle,
        data_name: FName,
        magnitude: f32,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_assign_set_by_caller_magnitude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&magnitude, __buffer.add(28).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_assign_set_by_caller_magnitude,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(data_name);
        std::mem::forget(magnitude);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn append_target_data_handle(
        target_handle: FGameplayAbilityTargetDataHandle,
        handle_to_add: &FGameplayAbilityTargetDataHandle,
    ) -> FGameplayAbilityTargetDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_append_target_data_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_handle,
                __buffer.add(0).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle_to_add,
                __buffer.add(40).cast::<FGameplayAbilityTargetDataHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_append_target_data_handle,
                __buffer,
            )
        };
        std::mem::forget(target_handle);
        unsafe { __buffer.add(80).cast::<FGameplayAbilityTargetDataHandle>().read() }
    }
    pub fn add_loose_gameplay_tags(
        actor: UPtr<crate::bindings::engine::AActor>,
        gameplay_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        b_should_replicate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_loose_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tags,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_replicate,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_loose_gameplay_tags,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(b_should_replicate);
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn add_linked_gameplay_effect_spec(
        spec_handle: FGameplayEffectSpecHandle,
        linked_gameplay_effect_spec: FGameplayEffectSpecHandle,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_linked_gameplay_effect_spec,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &linked_gameplay_effect_spec,
                __buffer.add(16).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_linked_gameplay_effect_spec,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(linked_gameplay_effect_spec);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn add_linked_gameplay_effect(
        spec_handle: FGameplayEffectSpecHandle,
        linked_gameplay_effect: TSubclassOf<UGameplayEffect>,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_linked_gameplay_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &linked_gameplay_effect,
                __buffer.add(16).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_linked_gameplay_effect,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(linked_gameplay_effect);
        unsafe { __buffer.add(24).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn add_granted_tags(
        spec_handle: FGameplayEffectSpecHandle,
        new_gameplay_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_granted_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_gameplay_tags,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_granted_tags,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(new_gameplay_tags);
        unsafe { __buffer.add(48).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn add_granted_tag(
        spec_handle: FGameplayEffectSpecHandle,
        new_gameplay_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_granted_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_gameplay_tag,
                __buffer.add(16).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_granted_tag,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(new_gameplay_tag);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn add_gameplay_tags(
        actor: UPtr<crate::bindings::engine::AActor>,
        gameplay_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        replication_rule: EGameplayTagReplicationState,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tags,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &replication_rule,
                __buffer.add(40).cast::<EGameplayTagReplicationState>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_gameplay_tags,
                __buffer,
            )
        };
        std::mem::forget(actor);
        std::mem::forget(replication_rule);
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn add_asset_tags(
        spec_handle: FGameplayEffectSpecHandle,
        new_gameplay_tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_asset_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_gameplay_tags,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_asset_tags,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(new_gameplay_tags);
        unsafe { __buffer.add(48).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn add_asset_tag(
        spec_handle: FGameplayEffectSpecHandle,
        new_gameplay_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_asset_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_gameplay_tag,
                __buffer.add(16).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_add_asset_tag,
                __buffer,
            )
        };
        std::mem::forget(spec_handle);
        std::mem::forget(new_gameplay_tag);
        unsafe { __buffer.add(32).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn ability_target_data_from_locations(
        source_location: &FGameplayAbilityTargetingLocationInfo,
        target_location: &FGameplayAbilityTargetingLocationInfo,
    ) -> FGameplayAbilityTargetDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<328>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_locations,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_location,
                __buffer.add(0).cast::<FGameplayAbilityTargetingLocationInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_location,
                __buffer.add(144).cast::<FGameplayAbilityTargetingLocationInfo>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_locations,
                __buffer,
            )
        };
        unsafe { __buffer.add(288).cast::<FGameplayAbilityTargetDataHandle>().read() }
    }
    pub fn ability_target_data_from_hit_result(
        hit_result: &crate::bindings::engine::FHitResult,
    ) -> FGameplayAbilityTargetDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit_result,
                __buffer.add(0).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_hit_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(264).cast::<FGameplayAbilityTargetDataHandle>().read() }
    }
    pub fn ability_target_data_from_actor_array(
        actor_array: &TArray<UPtr<crate::bindings::engine::AActor>>,
        one_target_per_handle: bool,
    ) -> FGameplayAbilityTargetDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_actor_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actor_array,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &one_target_per_handle,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_actor_array,
                __buffer,
            )
        };
        std::mem::forget(one_target_per_handle);
        unsafe { __buffer.add(24).cast::<FGameplayAbilityTargetDataHandle>().read() }
    }
    pub fn ability_target_data_from_actor(
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> FGameplayAbilityTargetDataHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UAbilitySystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_blueprint_library_ability_target_data_from_actor,
                __buffer,
            )
        };
        std::mem::forget(actor);
        unsafe { __buffer.add(8).cast::<FGameplayAbilityTargetDataHandle>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilitySystemCheatManagerExtension {
    __padding_end: [u8; 48],
}
impl UAbilitySystemCheatManagerExtension {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemCheatManagerExtension")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemCheatManagerExtension")
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
pub struct UAbilitySystemComponent {
    #[doc(hidden)]
    pub(crate) __padding_416: [u8; 416],
    pub affected_anim_instance_tag: FName,
    #[doc(hidden)]
    pub(crate) __padding_1216: [u8; 784],
    pub activatable_abilities: FGameplayAbilitySpecContainer,
    __padding_end: [u8; 3352],
}
impl UAbilitySystemComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemComponent")
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
    pub fn update_active_gameplay_effect_set_by_caller_magnitudes(
        &mut self,
        active_handle: FActiveGameplayEffectHandle,
        new_set_by_caller_values: &TMap<
            crate::bindings::gameplay_tags::FGameplayTag,
            f32,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitudes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_set_by_caller_values,
                __buffer
                    .add(8)
                    .cast::<TMap<crate::bindings::gameplay_tags::FGameplayTag, f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitudes,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
    }
    pub fn update_active_gameplay_effect_set_by_caller_magnitude(
        &mut self,
        active_handle: FActiveGameplayEffectHandle,
        set_by_caller_tag: crate::bindings::gameplay_tags::FGameplayTag,
        new_value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &set_by_caller_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_value, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_update_active_gameplay_effect_set_by_caller_magnitude,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        std::mem::forget(set_by_caller_tag);
        std::mem::forget(new_value);
    }
    pub fn try_activate_ability_by_class(
        &mut self,
        in_ability_to_activate: TSubclassOf<UGameplayAbility>,
        b_allow_remote_activation: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_try_activate_ability_by_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ability_to_activate,
                __buffer.add(0).cast::<TSubclassOf<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_remote_activation,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_try_activate_ability_by_class,
                __buffer,
            )
        };
        std::mem::forget(in_ability_to_activate);
        std::mem::forget(b_allow_remote_activation);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn try_activate_ability(
        &mut self,
        ability_to_activate: FGameplayAbilitySpecHandle,
        b_allow_remote_activation: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_try_activate_ability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_to_activate,
                __buffer.add(0).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_remote_activation,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_try_activate_ability,
                __buffer,
            )
        };
        std::mem::forget(ability_to_activate);
        std::mem::forget(b_allow_remote_activation);
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn try_activate_abilities_by_tag(
        &mut self,
        gameplay_tag_container: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        b_allow_remote_activation: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_try_activate_abilities_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                gameplay_tag_container,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_remote_activation,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_try_activate_abilities_by_tag,
                __buffer,
            )
        };
        std::mem::forget(b_allow_remote_activation);
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn target_confirm(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_target_confirm,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_target_confirm,
                __buffer,
            )
        };
    }
    pub fn target_cancel(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_target_cancel,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_target_cancel,
                __buffer,
            )
        };
    }
    pub fn set_user_ability_activation_inhibited(&mut self, new_inhibit: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_set_user_ability_activation_inhibited,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_inhibit,
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
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_set_user_ability_activation_inhibited,
                __buffer,
            )
        };
        std::mem::forget(new_inhibit);
    }
    pub fn set_active_gameplay_effect_level_using_query(
        &mut self,
        query: FGameplayEffectQuery,
        new_level: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<452>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_set_active_gameplay_effect_level_using_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &query,
                __buffer.add(0).cast::<FGameplayEffectQuery>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_level,
                __buffer.add(448).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_set_active_gameplay_effect_level_using_query,
                __buffer,
            )
        };
        std::mem::forget(query);
        std::mem::forget(new_level);
    }
    pub fn set_active_gameplay_effect_level(
        &mut self,
        active_handle: FActiveGameplayEffectHandle,
        new_level: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_set_active_gameplay_effect_level,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &active_handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_level, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_set_active_gameplay_effect_level,
                __buffer,
            )
        };
        std::mem::forget(active_handle);
        std::mem::forget(new_level);
    }
    pub fn remove_active_gameplay_effect_by_source_effect(
        &mut self,
        gameplay_effect: TSubclassOf<UGameplayEffect>,
        instigator_ability_system_component: UPtr<UAbilitySystemComponent>,
        stacks_to_remove: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_gameplay_effect_by_source_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instigator_ability_system_component,
                __buffer.add(8).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stacks_to_remove,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_gameplay_effect_by_source_effect,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect);
        std::mem::forget(instigator_ability_system_component);
        std::mem::forget(stacks_to_remove);
    }
    pub fn remove_active_gameplay_effect(
        &mut self,
        handle: FActiveGameplayEffectHandle,
        stacks_to_remove: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_gameplay_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stacks_to_remove,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_gameplay_effect,
                __buffer,
            )
        };
        std::mem::forget(handle);
        std::mem::forget(stacks_to_remove);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn remove_active_effects_with_tags(
        &mut self,
        tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tags,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_tags,
                __buffer,
            )
        };
        std::mem::forget(tags);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn remove_active_effects_with_source_tags(
        &mut self,
        tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_source_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tags,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_source_tags,
                __buffer,
            )
        };
        std::mem::forget(tags);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn remove_active_effects_with_granted_tags(
        &mut self,
        tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_granted_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tags,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_granted_tags,
                __buffer,
            )
        };
        std::mem::forget(tags);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn remove_active_effects_with_applied_tags(
        &mut self,
        tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_applied_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tags,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_remove_active_effects_with_applied_tags,
                __buffer,
            )
        };
        std::mem::forget(tags);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn release_input_id(&mut self, input_id: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_release_input_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&input_id, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_release_input_id,
                __buffer,
            )
        };
        std::mem::forget(input_id);
    }
    pub fn press_input_id(&mut self, input_id: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_press_input_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&input_id, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_press_input_id,
                __buffer,
            )
        };
        std::mem::forget(input_id);
    }
    pub fn make_outgoing_spec(
        &self,
        gameplay_effect_class: TSubclassOf<UGameplayEffect>,
        level: f32,
        context: FGameplayEffectContextHandle,
    ) -> FGameplayEffectSpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_make_outgoing_spec,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(16).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_make_outgoing_spec,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect_class);
        std::mem::forget(level);
        std::mem::forget(context);
        unsafe { __buffer.add(40).cast::<FGameplayEffectSpecHandle>().read() }
    }
    pub fn make_effect_context(&self) -> FGameplayEffectContextHandle {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_make_effect_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_make_effect_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FGameplayEffectContextHandle>().read() }
    }
    pub fn init_stats(
        &mut self,
        attributes: TSubclassOf<UAttributeSet>,
        data_table: UPtr<crate::bindings::engine::UDataTable>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_k2_init_stats,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attributes,
                __buffer.add(0).cast::<TSubclassOf<UAttributeSet>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_table,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UDataTable>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_k2_init_stats,
                __buffer,
            )
        };
        std::mem::forget(attributes);
        std::mem::forget(data_table);
    }
    pub fn give_ability_and_activate_once(
        &mut self,
        ability_class: TSubclassOf<UGameplayAbility>,
        level: i32,
        input_id: i32,
    ) -> FGameplayAbilitySpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_k2_give_ability_and_activate_once,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&input_id, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_k2_give_ability_and_activate_once,
                __buffer,
            )
        };
        std::mem::forget(ability_class);
        std::mem::forget(level);
        std::mem::forget(input_id);
        unsafe { __buffer.add(16).cast::<FGameplayAbilitySpecHandle>().read() }
    }
    pub fn give_ability(
        &mut self,
        ability_class: TSubclassOf<UGameplayAbility>,
        level: i32,
        input_id: i32,
    ) -> FGameplayAbilitySpecHandle {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_k2_give_ability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ability_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayAbility>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&input_id, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_k2_give_ability,
                __buffer,
            )
        };
        std::mem::forget(ability_class);
        std::mem::forget(level);
        std::mem::forget(input_id);
        unsafe { __buffer.add(16).cast::<FGameplayAbilitySpecHandle>().read() }
    }
    pub fn is_gameplay_cue_active(
        &self,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_is_gameplay_cue_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_is_gameplay_cue_active,
                __buffer,
            )
        };
        std::mem::forget(gameplay_cue_tag);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn input_confirm(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_input_confirm,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_input_confirm,
                __buffer,
            )
        };
    }
    pub fn input_cancel(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_input_cancel,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_input_cancel,
                __buffer,
            )
        };
    }
    pub fn get_user_ability_activation_inhibited(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_user_ability_activation_inhibited,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_user_ability_activation_inhibited,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_gameplay_tag_count(
        &self,
        gameplay_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_tag_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_tag_count,
                __buffer,
            )
        };
        std::mem::forget(gameplay_tag);
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_gameplay_effect_magnitude(
        &self,
        handle: FActiveGameplayEffectHandle,
        attribute: FGameplayAttribute,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<84>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_magnitude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &handle,
                __buffer.add(0).cast::<FActiveGameplayEffectHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(8).cast::<FGameplayAttribute>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_magnitude,
                __buffer,
            )
        };
        std::mem::forget(handle);
        std::mem::forget(attribute);
        unsafe { __buffer.add(80).cast::<f32>().read() }
    }
    pub fn get_gameplay_effect_count_if_loaded(
        &self,
        soft_source_gameplay_effect: TSoftObjectPtr<
            crate::bindings::core_u_object::UClass,
        >,
        optional_instigator_filter_component: UPtr<UAbilitySystemComponent>,
        b_enforce_on_going_check: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_count_if_loaded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &soft_source_gameplay_effect,
                __buffer
                    .add(0)
                    .cast::<TSoftObjectPtr<crate::bindings::core_u_object::UClass>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_instigator_filter_component,
                __buffer.add(48).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enforce_on_going_check,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_count_if_loaded,
                __buffer,
            )
        };
        std::mem::forget(soft_source_gameplay_effect);
        std::mem::forget(optional_instigator_filter_component);
        std::mem::forget(b_enforce_on_going_check);
        unsafe { __buffer.add(60).cast::<i32>().read() }
    }
    pub fn get_gameplay_effect_count(
        &self,
        source_gameplay_effect: TSubclassOf<UGameplayEffect>,
        optional_instigator_filter_component: UPtr<UAbilitySystemComponent>,
        b_enforce_on_going_check: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_gameplay_effect,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_instigator_filter_component,
                __buffer.add(8).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enforce_on_going_check,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_effect_count,
                __buffer,
            )
        };
        std::mem::forget(source_gameplay_effect);
        std::mem::forget(optional_instigator_filter_component);
        std::mem::forget(b_enforce_on_going_check);
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn get_gameplay_attribute_value(
        &self,
        attribute: FGameplayAttribute,
        b_found: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_attribute_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(0).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_found, __buffer.add(72).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_gameplay_attribute_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<bool>().swap(b_found);
        }
        std::mem::forget(attribute);
        unsafe { __buffer.add(76).cast::<f32>().read() }
    }
    pub fn get_attribute_set(
        &self,
        attribute_set_class: TSubclassOf<UAttributeSet>,
    ) -> UPtr<UAttributeSet> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_attribute_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_set_class,
                __buffer.add(0).cast::<TSubclassOf<UAttributeSet>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_attribute_set,
                __buffer,
            )
        };
        std::mem::forget(attribute_set_class);
        unsafe { __buffer.add(8).cast::<UPtr<UAttributeSet>>().read() }
    }
    pub fn get_all_attributes(
        &mut self,
        out_attributes: &mut TArray<FGameplayAttribute>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_all_attributes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_attributes,
                __buffer.add(0).cast::<TArray<FGameplayAttribute>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_all_attributes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FGameplayAttribute>>().swap(out_attributes);
        }
    }
    pub fn get_all_abilities(
        &self,
        out_ability_handles: &mut TArray<FGameplayAbilitySpecHandle>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_all_abilities,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_ability_handles,
                __buffer.add(0).cast::<TArray<FGameplayAbilitySpecHandle>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_all_abilities,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FGameplayAbilitySpecHandle>>()
                .swap(out_ability_handles);
        }
    }
    pub fn get_active_effects_with_all_tags(
        &self,
        tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> TArray<FActiveGameplayEffectHandle> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_active_effects_with_all_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tags,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_active_effects_with_all_tags,
                __buffer,
            )
        };
        std::mem::forget(tags);
        unsafe { __buffer.add(32).cast::<TArray<FActiveGameplayEffectHandle>>().read() }
    }
    pub fn get_active_effects(
        &self,
        query: &FGameplayEffectQuery,
    ) -> TArray<FActiveGameplayEffectHandle> {
        let mut __stack = crate::core_data::StackAlloc::<464>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_active_effects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                query,
                __buffer.add(0).cast::<FGameplayEffectQuery>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_get_active_effects,
                __buffer,
            )
        };
        unsafe { __buffer.add(448).cast::<TArray<FActiveGameplayEffectHandle>>().read() }
    }
    pub fn find_all_abilities_with_tags(
        &self,
        out_ability_handles: &mut TArray<FGameplayAbilitySpecHandle>,
        tags: crate::bindings::gameplay_tags::FGameplayTagContainer,
        b_exact_match: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_with_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_ability_handles,
                __buffer.add(0).cast::<TArray<FGameplayAbilitySpecHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tags,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_with_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FGameplayAbilitySpecHandle>>()
                .swap(out_ability_handles);
        }
        std::mem::forget(tags);
        std::mem::forget(b_exact_match);
    }
    pub fn find_all_abilities_with_input_id(
        &self,
        out_ability_handles: &mut TArray<FGameplayAbilitySpecHandle>,
        input_id: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_with_input_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_ability_handles,
                __buffer.add(0).cast::<TArray<FGameplayAbilitySpecHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&input_id, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_with_input_id,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FGameplayAbilitySpecHandle>>()
                .swap(out_ability_handles);
        }
        std::mem::forget(input_id);
    }
    pub fn find_all_abilities_matching_query(
        &self,
        out_ability_handles: &mut TArray<FGameplayAbilitySpecHandle>,
        query: crate::bindings::gameplay_tags::FGameplayTagQuery,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_matching_query,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_ability_handles,
                __buffer.add(0).cast::<TArray<FGameplayAbilitySpecHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &query,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagQuery>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_find_all_abilities_matching_query,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<FGameplayAbilitySpecHandle>>()
                .swap(out_ability_handles);
        }
        std::mem::forget(query);
    }
    pub fn clear_all_abilities_with_input_id(&mut self, input_id: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_clear_all_abilities_with_input_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&input_id, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_clear_all_abilities_with_input_id,
                __buffer,
            )
        };
        std::mem::forget(input_id);
    }
    pub fn clear_all_abilities(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_clear_all_abilities,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_clear_all_abilities,
                __buffer,
            )
        };
    }
    pub fn clear_ability(&mut self, handle: &FGameplayAbilitySpecHandle) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_clear_ability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                handle,
                __buffer.add(0).cast::<FGameplayAbilitySpecHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_clear_ability,
                __buffer,
            )
        };
    }
    pub fn apply_gameplay_effect_to_target(
        &mut self,
        gameplay_effect_class: TSubclassOf<UGameplayEffect>,
        target: UPtr<UAbilitySystemComponent>,
        level: f32,
        context: FGameplayEffectContextHandle,
    ) -> FActiveGameplayEffectHandle {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_to_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(8).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(24).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_to_target,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect_class);
        std::mem::forget(target);
        std::mem::forget(level);
        std::mem::forget(context);
        unsafe { __buffer.add(48).cast::<FActiveGameplayEffectHandle>().read() }
    }
    pub fn apply_gameplay_effect_to_self(
        &mut self,
        gameplay_effect_class: TSubclassOf<UGameplayEffect>,
        level: f32,
        effect_context: FGameplayEffectContextHandle,
    ) -> FActiveGameplayEffectHandle {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_to_self,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect_class,
                __buffer.add(0).cast::<TSubclassOf<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&level, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &effect_context,
                __buffer.add(16).cast::<FGameplayEffectContextHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_to_self,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect_class);
        std::mem::forget(level);
        std::mem::forget(effect_context);
        unsafe { __buffer.add(40).cast::<FActiveGameplayEffectHandle>().read() }
    }
    pub fn apply_gameplay_effect_spec_to_target(
        &mut self,
        spec_handle: &FGameplayEffectSpecHandle,
        target: UPtr<UAbilitySystemComponent>,
    ) -> FActiveGameplayEffectHandle {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_spec_to_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(16).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_spec_to_target,
                __buffer,
            )
        };
        std::mem::forget(target);
        unsafe { __buffer.add(24).cast::<FActiveGameplayEffectHandle>().read() }
    }
    pub fn apply_gameplay_effect_spec_to_self(
        &mut self,
        spec_handle: &FGameplayEffectSpecHandle,
    ) -> FActiveGameplayEffectHandle {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_spec_to_self,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                spec_handle,
                __buffer.add(0).cast::<FGameplayEffectSpecHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_ability_system_component_bp_apply_gameplay_effect_spec_to_self,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FActiveGameplayEffectHandle>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAbilitySystemDebugHUDExtension {
    __padding_end: [u8; 48],
}
impl UAbilitySystemDebugHUDExtension {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension")
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
pub struct UAbilitySystemDebugHUDExtension_Tags {
    __padding_end: [u8; 136],
}
impl UAbilitySystemDebugHUDExtension_Tags {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension_Tags")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension_Tags")
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
pub struct UAbilitySystemDebugHUDExtension_Attributes {
    __padding_end: [u8; 136],
}
impl UAbilitySystemDebugHUDExtension_Attributes {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension_Attributes")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension_Attributes")
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
pub struct UAbilitySystemDebugHUDExtension_BlockedAbilityTags {
    __padding_end: [u8; 136],
}
impl UAbilitySystemDebugHUDExtension_BlockedAbilityTags {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension_BlockedAbilityTags")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemDebugHUDExtension_BlockedAbilityTags")
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
pub struct UAbilitySystemGlobals {
    __padding_end: [u8; 992],
}
impl UAbilitySystemGlobals {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemGlobals")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemGlobals")
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
pub struct IAbilitySystemInterface {}
#[repr(C, align(8))]
pub struct UAbilitySystemInterface {
    __padding_end: [u8; 48],
}
impl UAbilitySystemInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemInterface")
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
pub struct IAbilitySystemReplicationProxyInterface {}
#[repr(C, align(8))]
pub struct UAbilitySystemReplicationProxyInterface {
    __padding_end: [u8; 48],
}
impl UAbilitySystemReplicationProxyInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemReplicationProxyInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemReplicationProxyInterface")
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
pub struct UAttributeSet {
    __padding_end: [u8; 56],
}
impl UAttributeSet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeSet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttributeSet")
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
pub struct UAbilitySystemTestAttributeSet {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub max_health: f32,
    pub health: f32,
    pub mana: FGameplayAttributeData,
    pub max_mana: f32,
    pub damage: f32,
    pub spell_damage: f32,
    pub physical_damage: f32,
    pub crit_chance: f32,
    pub crit_multiplier: f32,
    pub armor_damage_reduction: f32,
    pub dodge_chance: f32,
    pub life_steal: f32,
    pub strength: f32,
    pub stacking_attribute1: f32,
    pub stacking_attribute2: f32,
    pub no_stack_attribute: f32,
}
impl UAbilitySystemTestAttributeSet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemTestAttributeSet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitySystemTestAttributeSet")
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
pub struct AAbilitySystemTestPawn {
    #[doc(hidden)]
    pub(crate) __padding_1328: [u8; 1328],
    pub ability_system_component: UPtr<UAbilitySystemComponent>,
}
impl AAbilitySystemTestPawn {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAbilitySystemTestPawn")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAbilitySystemTestPawn")
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
pub struct UAnimNotify_GameplayCue {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub gameplay_cue: FGameplayCueTag,
}
impl UAnimNotify_GameplayCue {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_GameplayCue")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_GameplayCue")
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
pub struct UAnimNotify_GameplayCueState {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub gameplay_cue: FGameplayCueTag,
    __padding_end: [u8; 28],
}
impl UAnimNotify_GameplayCueState {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_GameplayCueState")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNotify_GameplayCueState")
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
pub struct UGameplayAbilitiesDeveloperSettings {
    __padding_end: [u8; 592],
}
impl UGameplayAbilitiesDeveloperSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilitiesDeveloperSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilitiesDeveloperSettings")
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
pub struct UGameplayAbilitiesEditorDeveloperSettings {
    __padding_end: [u8; 120],
}
impl UGameplayAbilitiesEditorDeveloperSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilitiesEditorDeveloperSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilitiesEditorDeveloperSettings")
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
pub struct UGameplayAbilityBlueprint {
    __padding_end: [u8; 1432],
}
impl UGameplayAbilityBlueprint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilityBlueprint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayAbilityBlueprint")
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
pub struct UGameplayCueFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UGameplayCueFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueFunctionLibrary")
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
    pub fn remove_gameplay_cue_on_actor(
        target: UPtr<crate::bindings::engine::AActor>,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
        parameters: &FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_remove_gameplay_cue_on_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(24).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UGameplayCueFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_remove_gameplay_cue_on_actor,
                __buffer,
            )
        };
        std::mem::forget(target);
        std::mem::forget(gameplay_cue_tag);
    }
    pub fn make_gameplay_cue_parameters_from_hit_result(
        hit_result: &crate::bindings::engine::FHitResult,
    ) -> FGameplayCueParameters {
        let mut __stack = crate::core_data::StackAlloc::<488>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_make_gameplay_cue_parameters_from_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit_result,
                __buffer.add(0).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UGameplayCueFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_make_gameplay_cue_parameters_from_hit_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(264).cast::<FGameplayCueParameters>().read() }
    }
    pub fn execute_gameplay_cue_on_actor(
        target: UPtr<crate::bindings::engine::AActor>,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
        parameters: &FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_execute_gameplay_cue_on_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(24).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UGameplayCueFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_execute_gameplay_cue_on_actor,
                __buffer,
            )
        };
        std::mem::forget(target);
        std::mem::forget(gameplay_cue_tag);
    }
    pub fn add_gameplay_cue_on_actor(
        target: UPtr<crate::bindings::engine::AActor>,
        gameplay_cue_tag: crate::bindings::gameplay_tags::FGameplayTag,
        parameters: &FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_add_gameplay_cue_on_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_cue_tag,
                __buffer.add(8).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(24).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UGameplayCueFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_function_library_add_gameplay_cue_on_actor,
                __buffer,
            )
        };
        std::mem::forget(target);
        std::mem::forget(gameplay_cue_tag);
    }
}
pub struct IGameplayCueInterface {}
#[repr(C, align(8))]
pub struct UGameplayCueInterface {
    __padding_end: [u8; 48],
}
impl UGameplayCueInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueInterface")
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
    pub fn forward_gameplay_cue_to_parent(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_interface_forward_gameplay_cue_to_parent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_interface_forward_gameplay_cue_to_parent,
                __buffer,
            )
        };
    }
    pub fn blueprint_custom_handler(
        &mut self,
        event_type: EGameplayCueEvent,
        parameters: &FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_interface_blueprint_custom_handler,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_type,
                __buffer.add(0).cast::<EGameplayCueEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_interface_blueprint_custom_handler,
                __buffer,
            )
        };
        std::mem::forget(event_type);
    }
}
#[repr(C, align(8))]
pub struct UGameplayCueManager {
    __padding_end: [u8; 928],
}
impl UGameplayCueManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueManager")
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
pub struct AGameplayCueNotify_Actor {
    __padding_end: [u8; 1256],
}
impl AGameplayCueNotify_Actor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCueNotify_Actor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCueNotify_Actor")
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
    pub fn on_become_relevant_while_active(
        &mut self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_while_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_while_active,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn on_cease_relevant_on_remove(
        &mut self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_on_remove,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_on_remove,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn on_execute(
        &mut self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_on_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_on_execute,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn on_burst_on_active(
        &mut self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<233>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_on_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_on_active,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        unsafe { __buffer.add(232).cast::<bool>().read() }
    }
    pub fn handle_gameplay_cue(
        &mut self,
        my_target: UPtr<crate::bindings::engine::AActor>,
        event_type: EGameplayCueEvent,
        parameters: &FGameplayCueParameters,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_k2_handle_gameplay_cue,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &my_target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_type,
                __buffer.add(8).cast::<EGameplayCueEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(16).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_k2_handle_gameplay_cue,
                __buffer,
            )
        };
        std::mem::forget(my_target);
        std::mem::forget(event_type);
    }
    pub fn end_gameplay_cue(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_k2_end_gameplay_cue,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_actor_k2_end_gameplay_cue,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGameplayCueNotify_Burst {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub default_spawn_condition: FGameplayCueNotify_SpawnCondition,
    pub default_placement_info: FGameplayCueNotify_PlacementInfo,
    pub burst_effects: FGameplayCueNotify_BurstEffects,
}
impl UGameplayCueNotify_Burst {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_Burst")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_Burst")
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
    pub fn on_burst(
        &self,
        target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
        spawn_results: &FGameplayCueNotify_SpawnResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_burst_on_burst,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_results,
                __buffer.add(232).cast::<FGameplayCueNotify_SpawnResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_cue_notify_burst_on_burst,
                __buffer,
            )
        };
        std::mem::forget(target);
    }
}
#[repr(C, align(8))]
pub struct AGameplayCueNotify_BurstLatent {
    #[doc(hidden)]
    pub(crate) __padding_1256: [u8; 1256],
    pub default_spawn_condition: FGameplayCueNotify_SpawnCondition,
    pub default_placement_info: FGameplayCueNotify_PlacementInfo,
    pub burst_effects: FGameplayCueNotify_BurstEffects,
    pub burst_spawn_results: FGameplayCueNotify_SpawnResult,
}
impl AGameplayCueNotify_BurstLatent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCueNotify_BurstLatent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCueNotify_BurstLatent")
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
    pub fn on_burst(
        &mut self,
        target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
        spawn_results: &FGameplayCueNotify_SpawnResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_burst_latent_on_burst,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_results,
                __buffer.add(232).cast::<FGameplayCueNotify_SpawnResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_burst_latent_on_burst,
                __buffer,
            )
        };
        std::mem::forget(target);
    }
}
#[repr(C, align(8))]
pub struct UGameplayCueNotify_HitImpact {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub sound: UPtr<crate::bindings::engine::USoundBase>,
    pub particle_system: UPtr<crate::bindings::engine::UParticleSystem>,
}
impl UGameplayCueNotify_HitImpact {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_HitImpact")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueNotify_HitImpact")
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
pub struct AGameplayCueNotify_Looping {
    #[doc(hidden)]
    pub(crate) __padding_1256: [u8; 1256],
    pub default_spawn_condition: FGameplayCueNotify_SpawnCondition,
    pub default_placement_info: FGameplayCueNotify_PlacementInfo,
    pub application_effects: FGameplayCueNotify_BurstEffects,
    pub application_spawn_results: FGameplayCueNotify_SpawnResult,
    pub looping_effects: FGameplayCueNotify_LoopingEffects,
    pub looping_spawn_results: FGameplayCueNotify_SpawnResult,
    pub recurring_effects: FGameplayCueNotify_BurstEffects,
    pub recurring_spawn_results: FGameplayCueNotify_SpawnResult,
    pub removal_effects: FGameplayCueNotify_BurstEffects,
    pub removal_spawn_results: FGameplayCueNotify_SpawnResult,
    __padding_end: [u8; 8],
}
impl AGameplayCueNotify_Looping {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCueNotify_Looping")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCueNotify_Looping")
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
    pub fn on_removal(
        &mut self,
        target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
        spawn_results: &FGameplayCueNotify_SpawnResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_removal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_results,
                __buffer.add(232).cast::<FGameplayCueNotify_SpawnResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_removal,
                __buffer,
            )
        };
        std::mem::forget(target);
    }
    pub fn on_recurring(
        &mut self,
        target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
        spawn_results: &FGameplayCueNotify_SpawnResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_recurring,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_results,
                __buffer.add(232).cast::<FGameplayCueNotify_SpawnResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_recurring,
                __buffer,
            )
        };
        std::mem::forget(target);
    }
    pub fn on_looping_start(
        &mut self,
        target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
        spawn_results: &FGameplayCueNotify_SpawnResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_looping_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_results,
                __buffer.add(232).cast::<FGameplayCueNotify_SpawnResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_looping_start,
                __buffer,
            )
        };
        std::mem::forget(target);
    }
    pub fn on_application(
        &mut self,
        target: UPtr<crate::bindings::engine::AActor>,
        parameters: &FGameplayCueParameters,
        spawn_results: &FGameplayCueNotify_SpawnResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_application,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parameters,
                __buffer.add(8).cast::<FGameplayCueParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_results,
                __buffer.add(232).cast::<FGameplayCueNotify_SpawnResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .a_gameplay_cue_notify_looping_on_application,
                __buffer,
            )
        };
        std::mem::forget(target);
    }
}
#[repr(C, align(8))]
pub struct UGameplayCueSet {
    __padding_end: [u8; 152],
}
impl UGameplayCueSet {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueSet")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueSet")
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
pub struct UGameplayCueTranslator {
    __padding_end: [u8; 48],
}
impl UGameplayCueTranslator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueTranslator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueTranslator")
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
pub struct UGameplayCueTranslator_Test {
    __padding_end: [u8; 48],
}
impl UGameplayCueTranslator_Test {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueTranslator_Test")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCueTranslator_Test")
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
pub struct UGameplayEffectCalculation {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub relevant_attributes_to_capture: TArray<
        FGameplayEffectAttributeCaptureDefinition,
    >,
}
impl UGameplayEffectCalculation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectCalculation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectCalculation")
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
pub struct UAbilitiesGameplayEffectComponent {
    __padding_end: [u8; 80],
}
impl UAbilitiesGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitiesGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAbilitiesGameplayEffectComponent")
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
pub struct UAdditionalEffectsGameplayEffectComponent {
    __padding_end: [u8; 136],
}
impl UAdditionalEffectsGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAdditionalEffectsGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAdditionalEffectsGameplayEffectComponent")
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
pub struct UAssetTagsGameplayEffectComponent {
    __padding_end: [u8; 160],
}
impl UAssetTagsGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetTagsGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetTagsGameplayEffectComponent")
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
pub struct UBlockAbilityTagsGameplayEffectComponent {
    __padding_end: [u8; 160],
}
impl UBlockAbilityTagsGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlockAbilityTagsGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlockAbilityTagsGameplayEffectComponent")
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
pub struct UCancelAbilityTagsGameplayEffectComponent {
    __padding_end: [u8; 264],
}
impl UCancelAbilityTagsGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCancelAbilityTagsGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCancelAbilityTagsGameplayEffectComponent")
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
pub struct UChanceToApplyGameplayEffectComponent {
    __padding_end: [u8; 120],
}
impl UChanceToApplyGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChanceToApplyGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChanceToApplyGameplayEffectComponent")
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
pub struct UCustomCanApplyGameplayEffectComponent {
    __padding_end: [u8; 80],
}
impl UCustomCanApplyGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomCanApplyGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomCanApplyGameplayEffectComponent")
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
pub struct UImmunityGameplayEffectComponent {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub immunity_queries: TArray<FGameplayEffectQuery>,
}
impl UImmunityGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImmunityGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImmunityGameplayEffectComponent")
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
pub struct URemoveOtherGameplayEffectComponent {
    __padding_end: [u8; 80],
}
impl URemoveOtherGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOtherGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URemoveOtherGameplayEffectComponent")
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
pub struct UTargetTagRequirementsGameplayEffectComponent {
    __padding_end: [u8; 472],
}
impl UTargetTagRequirementsGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetTagRequirementsGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetTagRequirementsGameplayEffectComponent")
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
pub struct UTargetTagsGameplayEffectComponent {
    __padding_end: [u8; 160],
}
impl UTargetTagsGameplayEffectComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetTagsGameplayEffectComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetTagsGameplayEffectComponent")
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
pub struct UGameplayEffectCustomApplicationRequirement {
    __padding_end: [u8; 48],
}
impl UGameplayEffectCustomApplicationRequirement {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectCustomApplicationRequirement")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectCustomApplicationRequirement")
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
    pub fn can_apply_gameplay_effect(
        &self,
        gameplay_effect: UPtr<UGameplayEffect>,
        spec: &FGameplayEffectSpec,
        asc: UPtr<UAbilitySystemComponent>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<681>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_effect_custom_application_requirement_can_apply_gameplay_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gameplay_effect,
                __buffer.add(0).cast::<UPtr<UGameplayEffect>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spec,
                __buffer.add(8).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asc,
                __buffer.add(672).cast::<UPtr<UAbilitySystemComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_effect_custom_application_requirement_can_apply_gameplay_effect,
                __buffer,
            )
        };
        std::mem::forget(gameplay_effect);
        std::mem::forget(asc);
        unsafe { __buffer.add(680).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGameplayEffectExecutionCalculation {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub b_requires_passed_in_tags: bool,
    __padding_end: [u8; 55],
}
impl UGameplayEffectExecutionCalculation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectExecutionCalculation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayEffectExecutionCalculation")
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
    pub fn execute(
        &self,
        execution_params: &FGameplayEffectCustomExecutionParameters,
        out_execution_output: &mut FGameplayEffectCustomExecutionOutput,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<264>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_effect_execution_calculation_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                execution_params,
                __buffer.add(0).cast::<FGameplayEffectCustomExecutionParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_execution_output,
                __buffer.add(240).cast::<FGameplayEffectCustomExecutionOutput>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_effect_execution_calculation_execute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(240)
                .cast::<FGameplayEffectCustomExecutionOutput>()
                .swap(out_execution_output);
        }
    }
}
#[repr(C, align(8))]
pub struct UGameplayModMagnitudeCalculation {
    __padding_end: [u8; 72],
}
impl UGameplayModMagnitudeCalculation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayModMagnitudeCalculation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayModMagnitudeCalculation")
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
    pub fn get_captured_attribute_magnitude(
        &self,
        effect_spec: &FGameplayEffectSpec,
        attribute: FGameplayAttribute,
        source_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
        target_tags: &crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<804>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_k2_get_captured_attribute_magnitude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute,
                __buffer.add(664).cast::<FGameplayAttribute>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_tags,
                __buffer
                    .add(736)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_tags,
                __buffer
                    .add(768)
                    .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_k2_get_captured_attribute_magnitude,
                __buffer,
            )
        };
        std::mem::forget(attribute);
        unsafe { __buffer.add(800).cast::<f32>().read() }
    }
    pub fn get_target_spec_tags(
        &self,
        effect_spec: &FGameplayEffectSpec,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_spec_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_spec_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(664)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_target_aggregated_tags(
        &self,
        effect_spec: &FGameplayEffectSpec,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_aggregated_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_aggregated_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(664)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_target_actor_tags(
        &self,
        effect_spec: &FGameplayEffectSpec,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_actor_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_target_actor_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(664)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_source_spec_tags(
        &self,
        effect_spec: &FGameplayEffectSpec,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_spec_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_spec_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(664)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_source_aggregated_tags(
        &self,
        effect_spec: &FGameplayEffectSpec,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_aggregated_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_aggregated_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(664)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_source_actor_tags(
        &self,
        effect_spec: &FGameplayEffectSpec,
    ) -> crate::bindings::gameplay_tags::FGameplayTagContainer {
        let mut __stack = crate::core_data::StackAlloc::<696>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_actor_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_source_actor_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(664)
                .cast::<crate::bindings::gameplay_tags::FGameplayTagContainer>()
                .read()
        }
    }
    pub fn get_set_by_caller_magnitude_by_tag(
        &self,
        effect_spec: &FGameplayEffectSpec,
        tag: &crate::bindings::gameplay_tags::FGameplayTag,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<680>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tag,
                __buffer.add(664).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(676).cast::<f32>().read() }
    }
    pub fn get_set_by_caller_magnitude_by_name(
        &self,
        effect_spec: &FGameplayEffectSpec,
        magnitude_name: &FName,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<680>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                magnitude_name,
                __buffer.add(664).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_get_set_by_caller_magnitude_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(676).cast::<f32>().read() }
    }
    pub fn calculate_base_magnitude(&self, spec: &FGameplayEffectSpec) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<668>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_calculate_base_magnitude,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                spec,
                __buffer.add(0).cast::<FGameplayEffectSpec>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_gameplay_mod_magnitude_calculation_calculate_base_magnitude,
                __buffer,
            )
        };
        unsafe { __buffer.add(664).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGameplayTagReponseTable {
    __padding_end: [u8; 608],
}
impl UGameplayTagReponseTable {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagReponseTable")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagReponseTable")
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
pub struct UMovieSceneGameplayCueTriggerSection {
    __padding_end: [u8; 656],
}
impl UMovieSceneGameplayCueTriggerSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGameplayCueTriggerSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGameplayCueTriggerSection")
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
pub struct UMovieSceneGameplayCueSection {
    __padding_end: [u8; 544],
}
impl UMovieSceneGameplayCueSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGameplayCueSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGameplayCueSection")
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
pub struct UMovieSceneGameplayCueTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneGameplayCueTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGameplayCueTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneGameplayCueTrack")
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
    pub fn set_sequencer_track_handler(
        in_gameplay_cue_track_handler: FSetSequencerTrackHandler_InGameplayCueTrackHandler,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_movie_scene_gameplay_cue_track_set_sequencer_track_handler,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_gameplay_cue_track_handler,
                __buffer
                    .add(0)
                    .cast::<FSetSequencerTrackHandler_InGameplayCueTrackHandler>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_abilities::UMovieSceneGameplayCueTrack::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_abilities::__FUNCTION_PTRS
                    .u_movie_scene_gameplay_cue_track_set_sequencer_track_handler,
                __buffer,
            )
        };
        std::mem::forget(in_gameplay_cue_track_handler);
    }
}
pub struct ITickableAttributeSetInterface {}
#[repr(C, align(8))]
pub struct UTickableAttributeSetInterface {
    __padding_end: [u8; 48],
}
impl UTickableAttributeSetInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableAttributeSetInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTickableAttributeSetInterface")
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
pub struct FGameplayEffectQuery_CustomMatchDelegate_BP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FGetActiveEffects_CustomMatchDelegate_BP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetActiveGameplayEffectLevelUsingQuery_CustomMatchDelegate_BP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindEventWrapperToAnyOfGameplayTagContainerChanged_GameplayTagChangedEventWrapperDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindEventWrapperToAnyOfGameplayTagsChanged_GameplayTagChangedEventWrapperDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FBindEventWrapperToGameplayTagChanged_GameplayTagChangedEventWrapperDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSetSequencerTrackHandler_InGameplayCueTrackHandler {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FGameplayEffect_CustomMatchDelegate_BP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FAbilityAsync_WaitAttributeChanged_Changed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityAsync_WaitGameplayEffectApplied_OnApplied {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityAsync_WaitGameplayEvent_EventReceived {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityAsync_WaitGameplayTagAdded_Added {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityAsync_WaitGameplayTagRemoved_Removed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityAsync_WaitGameplayTagCountChanged_TagCountChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityAsync_WaitGameplayTagQuery_Triggered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_ApplyRootMotionConstantForce_OnFinish {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_ApplyRootMotionJumpForce_OnFinish {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_ApplyRootMotionJumpForce_OnLanded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_ApplyRootMotionMoveToActorForce_OnFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_ApplyRootMotionMoveToForce_OnTimedOut {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_ApplyRootMotionMoveToForce_OnTimedOutAndDestinationReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_ApplyRootMotionRadialForce_OnFinish {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_MoveToLocation_OnTargetLocationReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_NetworkSyncPoint_OnSync {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayAnimAndWait_OnCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayAnimAndWait_OnBlendOut {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayAnimAndWait_OnBlendIn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayAnimAndWait_OnInterrupted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayAnimAndWait_OnCancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayMontageAndWait_OnCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayMontageAndWait_OnBlendedIn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayMontageAndWait_OnBlendOut {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayMontageAndWait_OnInterrupted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_PlayMontageAndWait_OnCancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_Repeat_OnPerformAction {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_Repeat_OnFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_SpawnActor_Success {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_SpawnActor_DidNotSpawn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_StartAbilityState_OnStateEnded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_StartAbilityState_OnStateInterrupted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_VisualizeTargeting_TimeElapsed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitAbilityActivate_OnActivate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitAbilityCommit_OnCommit {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitAttributeChange_OnChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitAttributeChangeRatioThreshold_OnChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitAttributeChangeThreshold_OnChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitCancel_OnCancel {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitConfirm_OnConfirm {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitConfirmCancel_OnConfirm {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitConfirmCancel_OnCancel {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitDelay_OnFinish {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEffectApplied_Self_OnApplied {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEffectApplied_Target_OnApplied {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEffectBlockedImmunity_Blocked {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEffectRemoved_OnRemoved {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEffectRemoved_InvalidHandle {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEffectStackChange_OnChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEffectStackChange_InvalidHandle {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayEvent_EventReceived {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayTagAdded_Added {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayTagRemoved_Removed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayTagCountChanged_TagCountChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitGameplayTagQuery_Triggered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitInputPress_OnPress {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitInputRelease_OnRelease {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitMovementModeChange_OnChange {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitOverlap_OnOverlap {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitTargetData_ValidData {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitTargetData_Cancelled {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAbilityTask_WaitVelocityChange_OnVelocityChage {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImmunityGameplayEffectComponent_CustomMatchDelegate_BP {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRemoveOtherGameplayEffectComponent_CustomMatchDelegate_BP {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EGameplayEffectAttributeCaptureSource(pub u8);
impl EGameplayEffectAttributeCaptureSource {
    pub const SOURCE: EGameplayEffectAttributeCaptureSource = EGameplayEffectAttributeCaptureSource(
        0,
    );
    pub const TARGET: EGameplayEffectAttributeCaptureSource = EGameplayEffectAttributeCaptureSource(
        1,
    );
}
#[repr(transparent)]
pub struct EGameplayEffectGrantedAbilityRemovePolicy(pub u8);
impl EGameplayEffectGrantedAbilityRemovePolicy {
    pub const CANCEL_ABILITY_IMMEDIATELY: EGameplayEffectGrantedAbilityRemovePolicy = EGameplayEffectGrantedAbilityRemovePolicy(
        0,
    );
    pub const REMOVE_ABILITY_ON_END: EGameplayEffectGrantedAbilityRemovePolicy = EGameplayEffectGrantedAbilityRemovePolicy(
        1,
    );
    pub const DO_NOTHING: EGameplayEffectGrantedAbilityRemovePolicy = EGameplayEffectGrantedAbilityRemovePolicy(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayAbilityActivationMode(pub u8);
impl EGameplayAbilityActivationMode {
    pub const AUTHORITY: EGameplayAbilityActivationMode = EGameplayAbilityActivationMode(
        0,
    );
    pub const NON_AUTHORITY: EGameplayAbilityActivationMode = EGameplayAbilityActivationMode(
        1,
    );
    pub const PREDICTING: EGameplayAbilityActivationMode = EGameplayAbilityActivationMode(
        2,
    );
    pub const CONFIRMED: EGameplayAbilityActivationMode = EGameplayAbilityActivationMode(
        3,
    );
    pub const REJECTED: EGameplayAbilityActivationMode = EGameplayAbilityActivationMode(
        4,
    );
}
#[repr(transparent)]
pub struct EGameplayCuePayloadType(pub u8);
impl EGameplayCuePayloadType {
    pub const CUE_PARAMETERS: EGameplayCuePayloadType = EGameplayCuePayloadType(0);
    pub const FROM_SPEC: EGameplayCuePayloadType = EGameplayCuePayloadType(1);
}
#[repr(transparent)]
pub struct EGameplayAbilityTriggerSource(pub u8);
impl EGameplayAbilityTriggerSource {
    pub const GAMEPLAY_EVENT: EGameplayAbilityTriggerSource = EGameplayAbilityTriggerSource(
        0,
    );
    pub const OWNED_TAG_ADDED: EGameplayAbilityTriggerSource = EGameplayAbilityTriggerSource(
        1,
    );
    pub const OWNED_TAG_PRESENT: EGameplayAbilityTriggerSource = EGameplayAbilityTriggerSource(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayAbilityInputBinds(pub u8);
impl EGameplayAbilityInputBinds {
    pub const ABILITY1: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(0);
    pub const ABILITY2: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(1);
    pub const ABILITY3: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(2);
    pub const ABILITY4: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(3);
    pub const ABILITY5: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(4);
    pub const ABILITY6: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(5);
    pub const ABILITY7: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(6);
    pub const ABILITY8: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(7);
    pub const ABILITY9: EGameplayAbilityInputBinds = EGameplayAbilityInputBinds(8);
}
#[repr(transparent)]
pub struct ETargetDataFilterSelf(pub u8);
impl ETargetDataFilterSelf {
    pub const TDFS_ANY: ETargetDataFilterSelf = ETargetDataFilterSelf(0);
    pub const TDFS_NO_SELF: ETargetDataFilterSelf = ETargetDataFilterSelf(1);
    pub const TDFS_NO_OTHERS: ETargetDataFilterSelf = ETargetDataFilterSelf(2);
}
#[repr(transparent)]
pub struct EGameplayAbilityTargetingLocationType(pub u8);
impl EGameplayAbilityTargetingLocationType {
    pub const LITERAL_TRANSFORM: EGameplayAbilityTargetingLocationType = EGameplayAbilityTargetingLocationType(
        0,
    );
    pub const ACTOR_TRANSFORM: EGameplayAbilityTargetingLocationType = EGameplayAbilityTargetingLocationType(
        1,
    );
    pub const SOCKET_TRANSFORM: EGameplayAbilityTargetingLocationType = EGameplayAbilityTargetingLocationType(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayCueNotify_LocallyControlledSource(pub u8);
impl EGameplayCueNotify_LocallyControlledSource {
    pub const INSTIGATOR_ACTOR: EGameplayCueNotify_LocallyControlledSource = EGameplayCueNotify_LocallyControlledSource(
        0,
    );
    pub const TARGET_ACTOR: EGameplayCueNotify_LocallyControlledSource = EGameplayCueNotify_LocallyControlledSource(
        1,
    );
}
#[repr(transparent)]
pub struct EGameplayCueNotify_LocallyControlledPolicy(pub u8);
impl EGameplayCueNotify_LocallyControlledPolicy {
    pub const ALWAYS: EGameplayCueNotify_LocallyControlledPolicy = EGameplayCueNotify_LocallyControlledPolicy(
        0,
    );
    pub const LOCAL_ONLY: EGameplayCueNotify_LocallyControlledPolicy = EGameplayCueNotify_LocallyControlledPolicy(
        1,
    );
    pub const NOT_LOCAL: EGameplayCueNotify_LocallyControlledPolicy = EGameplayCueNotify_LocallyControlledPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayCueNotify_AttachPolicy(pub u8);
impl EGameplayCueNotify_AttachPolicy {
    pub const DO_NOT_ATTACH: EGameplayCueNotify_AttachPolicy = EGameplayCueNotify_AttachPolicy(
        0,
    );
    pub const ATTACH_TO_TARGET: EGameplayCueNotify_AttachPolicy = EGameplayCueNotify_AttachPolicy(
        1,
    );
}
#[repr(transparent)]
pub struct EGameplayCueNotify_EffectPlaySpace(pub u8);
impl EGameplayCueNotify_EffectPlaySpace {
    pub const WORLD_SPACE: EGameplayCueNotify_EffectPlaySpace = EGameplayCueNotify_EffectPlaySpace(
        0,
    );
    pub const CAMERA_SPACE: EGameplayCueNotify_EffectPlaySpace = EGameplayCueNotify_EffectPlaySpace(
        1,
    );
}
#[repr(transparent)]
pub struct EAttributeBasedFloatCalculationType(pub u8);
impl EAttributeBasedFloatCalculationType {
    pub const ATTRIBUTE_MAGNITUDE: EAttributeBasedFloatCalculationType = EAttributeBasedFloatCalculationType(
        0,
    );
    pub const ATTRIBUTE_BASE_VALUE: EAttributeBasedFloatCalculationType = EAttributeBasedFloatCalculationType(
        1,
    );
    pub const ATTRIBUTE_BONUS_MAGNITUDE: EAttributeBasedFloatCalculationType = EAttributeBasedFloatCalculationType(
        2,
    );
    pub const ATTRIBUTE_MAGNITUDE_EVALUATED_UP_TO_CHANNEL: EAttributeBasedFloatCalculationType = EAttributeBasedFloatCalculationType(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayModEvaluationChannel(pub u8);
impl EGameplayModEvaluationChannel {
    pub const CHANNEL0: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(0);
    pub const CHANNEL1: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(1);
    pub const CHANNEL2: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(2);
    pub const CHANNEL3: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(3);
    pub const CHANNEL4: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(4);
    pub const CHANNEL5: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(5);
    pub const CHANNEL6: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(6);
    pub const CHANNEL7: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(7);
    pub const CHANNEL8: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(8);
    pub const CHANNEL9: EGameplayModEvaluationChannel = EGameplayModEvaluationChannel(9);
}
#[repr(transparent)]
pub struct EGameplayEffectMagnitudeCalculation(pub u8);
impl EGameplayEffectMagnitudeCalculation {
    pub const SCALABLE_FLOAT: EGameplayEffectMagnitudeCalculation = EGameplayEffectMagnitudeCalculation(
        0,
    );
    pub const ATTRIBUTE_BASED: EGameplayEffectMagnitudeCalculation = EGameplayEffectMagnitudeCalculation(
        1,
    );
    pub const CUSTOM_CALCULATION_CLASS: EGameplayEffectMagnitudeCalculation = EGameplayEffectMagnitudeCalculation(
        2,
    );
    pub const SET_BY_CALLER: EGameplayEffectMagnitudeCalculation = EGameplayEffectMagnitudeCalculation(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayEffectScopedModifierAggregatorType(pub u8);
impl EGameplayEffectScopedModifierAggregatorType {
    pub const CAPTURED_ATTRIBUTE_BACKED: EGameplayEffectScopedModifierAggregatorType = EGameplayEffectScopedModifierAggregatorType(
        0,
    );
    pub const TRANSIENT: EGameplayEffectScopedModifierAggregatorType = EGameplayEffectScopedModifierAggregatorType(
        1,
    );
}
#[repr(transparent)]
pub struct EGameplayModOp(pub u8);
impl EGameplayModOp {
    pub const ADD_BASE: EGameplayModOp = EGameplayModOp(0);
    pub const MULTIPLY_ADDITIVE: EGameplayModOp = EGameplayModOp(1);
    pub const DIVIDE_ADDITIVE: EGameplayModOp = EGameplayModOp(2);
    pub const MULTIPLY_COMPOUND: EGameplayModOp = EGameplayModOp(4);
    pub const ADD_FINAL: EGameplayModOp = EGameplayModOp(5);
    pub const MAX: EGameplayModOp = EGameplayModOp(6);
    pub const ADDITIVE: EGameplayModOp = EGameplayModOp(0);
    pub const MULTIPLICITIVE: EGameplayModOp = EGameplayModOp(1);
    pub const DIVISION: EGameplayModOp = EGameplayModOp(2);
    pub const OVERRIDE: EGameplayModOp = EGameplayModOp(3);
}
#[repr(transparent)]
pub struct EGameplayAbilityInstancingPolicy(pub u8);
impl EGameplayAbilityInstancingPolicy {
    pub const NON_INSTANCED: EGameplayAbilityInstancingPolicy = EGameplayAbilityInstancingPolicy(
        0,
    );
    pub const INSTANCED_PER_ACTOR: EGameplayAbilityInstancingPolicy = EGameplayAbilityInstancingPolicy(
        1,
    );
    pub const INSTANCED_PER_EXECUTION: EGameplayAbilityInstancingPolicy = EGameplayAbilityInstancingPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayAbilityNetExecutionPolicy(pub u8);
impl EGameplayAbilityNetExecutionPolicy {
    pub const LOCAL_PREDICTED: EGameplayAbilityNetExecutionPolicy = EGameplayAbilityNetExecutionPolicy(
        0,
    );
    pub const LOCAL_ONLY: EGameplayAbilityNetExecutionPolicy = EGameplayAbilityNetExecutionPolicy(
        1,
    );
    pub const SERVER_INITIATED: EGameplayAbilityNetExecutionPolicy = EGameplayAbilityNetExecutionPolicy(
        2,
    );
    pub const SERVER_ONLY: EGameplayAbilityNetExecutionPolicy = EGameplayAbilityNetExecutionPolicy(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayAbilityNetSecurityPolicy(pub u8);
impl EGameplayAbilityNetSecurityPolicy {
    pub const CLIENT_OR_SERVER: EGameplayAbilityNetSecurityPolicy = EGameplayAbilityNetSecurityPolicy(
        0,
    );
    pub const SERVER_ONLY_EXECUTION: EGameplayAbilityNetSecurityPolicy = EGameplayAbilityNetSecurityPolicy(
        1,
    );
    pub const SERVER_ONLY_TERMINATION: EGameplayAbilityNetSecurityPolicy = EGameplayAbilityNetSecurityPolicy(
        2,
    );
    pub const SERVER_ONLY: EGameplayAbilityNetSecurityPolicy = EGameplayAbilityNetSecurityPolicy(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayAbilityReplicationPolicy(pub u8);
impl EGameplayAbilityReplicationPolicy {
    pub const REPLICATE_NO: EGameplayAbilityReplicationPolicy = EGameplayAbilityReplicationPolicy(
        0,
    );
    pub const REPLICATE_YES: EGameplayAbilityReplicationPolicy = EGameplayAbilityReplicationPolicy(
        1,
    );
}
#[repr(transparent)]
pub struct EAbilityGenericReplicatedEvent(pub u8);
impl EAbilityGenericReplicatedEvent {
    pub const GENERIC_CONFIRM: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        0,
    );
    pub const GENERIC_CANCEL: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        1,
    );
    pub const INPUT_PRESSED: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        2,
    );
    pub const INPUT_RELEASED: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        3,
    );
    pub const GENERIC_SIGNAL_FROM_CLIENT: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        4,
    );
    pub const GENERIC_SIGNAL_FROM_SERVER: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        5,
    );
    pub const GAME_CUSTOM1: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        6,
    );
    pub const GAME_CUSTOM2: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        7,
    );
    pub const GAME_CUSTOM3: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        8,
    );
    pub const GAME_CUSTOM4: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        9,
    );
    pub const GAME_CUSTOM5: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        10,
    );
    pub const GAME_CUSTOM6: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(
        11,
    );
    pub const MAX: EAbilityGenericReplicatedEvent = EAbilityGenericReplicatedEvent(12);
}
#[repr(transparent)]
pub struct EGameplayCueEvent(pub u8);
impl EGameplayCueEvent {
    pub const ON_ACTIVE: EGameplayCueEvent = EGameplayCueEvent(0);
    pub const WHILE_ACTIVE: EGameplayCueEvent = EGameplayCueEvent(1);
    pub const EXECUTED: EGameplayCueEvent = EGameplayCueEvent(2);
    pub const REMOVED: EGameplayCueEvent = EGameplayCueEvent(3);
}
#[repr(transparent)]
pub struct EWaitGameplayTagQueryTriggerCondition(pub u8);
impl EWaitGameplayTagQueryTriggerCondition {
    pub const WHEN_TRUE: EWaitGameplayTagQueryTriggerCondition = EWaitGameplayTagQueryTriggerCondition(
        0,
    );
    pub const WHEN_FALSE: EWaitGameplayTagQueryTriggerCondition = EWaitGameplayTagQueryTriggerCondition(
        1,
    );
}
#[repr(transparent)]
pub struct ERootMotionMoveToActorTargetOffsetType(pub u8);
impl ERootMotionMoveToActorTargetOffsetType {
    pub const ALIGN_FROM_TARGET_TO_SOURCE: ERootMotionMoveToActorTargetOffsetType = ERootMotionMoveToActorTargetOffsetType(
        0,
    );
    pub const ALIGN_TO_TARGET_FORWARD: ERootMotionMoveToActorTargetOffsetType = ERootMotionMoveToActorTargetOffsetType(
        1,
    );
    pub const ALIGN_TO_WORLD_SPACE: ERootMotionMoveToActorTargetOffsetType = ERootMotionMoveToActorTargetOffsetType(
        2,
    );
}
#[repr(transparent)]
pub struct EAbilityTaskNetSyncType(pub u8);
impl EAbilityTaskNetSyncType {
    pub const BOTH_WAIT: EAbilityTaskNetSyncType = EAbilityTaskNetSyncType(0);
    pub const ONLY_SERVER_WAIT: EAbilityTaskNetSyncType = EAbilityTaskNetSyncType(1);
    pub const ONLY_CLIENT_WAIT: EAbilityTaskNetSyncType = EAbilityTaskNetSyncType(2);
}
#[repr(transparent)]
pub struct EWaitAttributeChangeComparison(pub u8);
impl EWaitAttributeChangeComparison {
    pub const NONE: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(0);
    pub const GREATER_THAN: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(
        1,
    );
    pub const LESS_THAN: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(
        2,
    );
    pub const GREATER_THAN_OR_EQUAL_TO: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(
        3,
    );
    pub const LESS_THAN_OR_EQUAL_TO: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(
        4,
    );
    pub const NOT_EQUAL_TO: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(
        5,
    );
    pub const EXACTLY_EQUAL_TO: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(
        6,
    );
    pub const MAX: EWaitAttributeChangeComparison = EWaitAttributeChangeComparison(7);
}
#[repr(transparent)]
pub struct EGameplayTargetingConfirmation(pub u8);
impl EGameplayTargetingConfirmation {
    pub const INSTANT: EGameplayTargetingConfirmation = EGameplayTargetingConfirmation(
        0,
    );
    pub const USER_CONFIRMED: EGameplayTargetingConfirmation = EGameplayTargetingConfirmation(
        1,
    );
    pub const CUSTOM: EGameplayTargetingConfirmation = EGameplayTargetingConfirmation(2);
    pub const CUSTOM_MULTI: EGameplayTargetingConfirmation = EGameplayTargetingConfirmation(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayTagReplicationState(pub u8);
impl EGameplayTagReplicationState {
    pub const NONE: EGameplayTagReplicationState = EGameplayTagReplicationState(0);
    pub const TAG_ONLY: EGameplayTagReplicationState = EGameplayTagReplicationState(1);
    pub const COUNT_TO_OWNER: EGameplayTagReplicationState = EGameplayTagReplicationState(
        2,
    );
    pub const TAG_AND_COUNT_TO_ALL: EGameplayTagReplicationState = EGameplayTagReplicationState(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayTagEventType(pub u8);
impl EGameplayTagEventType {
    pub const NEW_OR_REMOVED: EGameplayTagEventType = EGameplayTagEventType(0);
    pub const ANY_COUNT_CHANGE: EGameplayTagEventType = EGameplayTagEventType(1);
}
#[repr(transparent)]
pub struct EGameplayEffectDurationType(pub u8);
impl EGameplayEffectDurationType {
    pub const INSTANT: EGameplayEffectDurationType = EGameplayEffectDurationType(0);
    pub const INFINITE: EGameplayEffectDurationType = EGameplayEffectDurationType(1);
    pub const HAS_DURATION: EGameplayEffectDurationType = EGameplayEffectDurationType(2);
}
#[repr(transparent)]
pub struct EGameplayEffectPeriodInhibitionRemovedPolicy(pub u8);
impl EGameplayEffectPeriodInhibitionRemovedPolicy {
    pub const NEVER_RESET: EGameplayEffectPeriodInhibitionRemovedPolicy = EGameplayEffectPeriodInhibitionRemovedPolicy(
        0,
    );
    pub const RESET_PERIOD: EGameplayEffectPeriodInhibitionRemovedPolicy = EGameplayEffectPeriodInhibitionRemovedPolicy(
        1,
    );
    pub const EXECUTE_AND_RESET_PERIOD: EGameplayEffectPeriodInhibitionRemovedPolicy = EGameplayEffectPeriodInhibitionRemovedPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayEffectStackingType(pub u8);
impl EGameplayEffectStackingType {
    pub const NONE: EGameplayEffectStackingType = EGameplayEffectStackingType(0);
    pub const AGGREGATE_BY_SOURCE: EGameplayEffectStackingType = EGameplayEffectStackingType(
        1,
    );
    pub const AGGREGATE_BY_TARGET: EGameplayEffectStackingType = EGameplayEffectStackingType(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayEffectStackingDurationPolicy(pub u8);
impl EGameplayEffectStackingDurationPolicy {
    pub const REFRESH_ON_SUCCESSFUL_APPLICATION: EGameplayEffectStackingDurationPolicy = EGameplayEffectStackingDurationPolicy(
        0,
    );
    pub const NEVER_REFRESH: EGameplayEffectStackingDurationPolicy = EGameplayEffectStackingDurationPolicy(
        1,
    );
    pub const EXTEND_DURATION: EGameplayEffectStackingDurationPolicy = EGameplayEffectStackingDurationPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct EGameplayEffectStackingPeriodPolicy(pub u8);
impl EGameplayEffectStackingPeriodPolicy {
    pub const RESET_ON_SUCCESSFUL_APPLICATION: EGameplayEffectStackingPeriodPolicy = EGameplayEffectStackingPeriodPolicy(
        0,
    );
    pub const NEVER_RESET: EGameplayEffectStackingPeriodPolicy = EGameplayEffectStackingPeriodPolicy(
        1,
    );
}
#[repr(transparent)]
pub struct EGameplayEffectStackingExpirationPolicy(pub u8);
impl EGameplayEffectStackingExpirationPolicy {
    pub const CLEAR_ENTIRE_STACK: EGameplayEffectStackingExpirationPolicy = EGameplayEffectStackingExpirationPolicy(
        0,
    );
    pub const REMOVE_SINGLE_STACK_AND_REFRESH_DURATION: EGameplayEffectStackingExpirationPolicy = EGameplayEffectStackingExpirationPolicy(
        1,
    );
    pub const REFRESH_DURATION: EGameplayEffectStackingExpirationPolicy = EGameplayEffectStackingExpirationPolicy(
        2,
    );
}
#[repr(transparent)]
pub struct ECancelAbilityTagsGameplayEffectComponentMode(pub u8);
impl ECancelAbilityTagsGameplayEffectComponentMode {
    pub const ON_APPLICATION: ECancelAbilityTagsGameplayEffectComponentMode = ECancelAbilityTagsGameplayEffectComponentMode(
        0,
    );
    pub const ON_EXECUTION: ECancelAbilityTagsGameplayEffectComponentMode = ECancelAbilityTagsGameplayEffectComponentMode(
        1,
    );
}
