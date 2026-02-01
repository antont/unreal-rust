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
    pub uai_async_task_blueprint_proxy_on_move_completed: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_use_blackboard: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_unclaim_task_resource: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_set_path_following_component: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_set_move_block_detection: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_run_behavior_tree: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_on_using_black_board: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_on_gameplay_task_resources_claimed: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_move_to_location: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_move_to_actor: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_k2_set_focus: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_k2_set_focal_point: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_k2_clear_focus: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_has_partial_path: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_get_path_following_component: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_get_move_status: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_get_immediate_move_destination: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_get_focus_actor: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_get_focal_point_on_actor: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_get_focal_point: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_get_ai_perception_component: *mut crate::ffi::UFunctionOpague,
    pub aai_controller_claim_task_resource: *mut crate::ffi::UFunctionOpague,
    pub uai_system_ai_logging_verbose: *mut crate::ffi::UFunctionOpague,
    pub uai_system_ai_ignore_players: *mut crate::ffi::UFunctionOpague,
    pub u_brain_component_stop_logic: *mut crate::ffi::UFunctionOpague,
    pub u_brain_component_start_logic: *mut crate::ffi::UFunctionOpague,
    pub u_brain_component_restart_logic: *mut crate::ffi::UFunctionOpague,
    pub u_brain_component_is_running: *mut crate::ffi::UFunctionOpague,
    pub u_brain_component_is_paused: *mut crate::ffi::UFunctionOpague,
    pub u_behavior_tree_component_set_dynamic_subtree: *mut crate::ffi::UFunctionOpague,
    pub u_behavior_tree_component_get_tag_cooldown_end_time: *mut crate::ffi::UFunctionOpague,
    pub u_behavior_tree_component_add_cooldown_tag_duration: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_asset_provider_get_blackboard_asset: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_vector: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_string: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_object: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_name: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_int: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_float: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_enum: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_class: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_set_value_as_bool: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_is_vector_value_set: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_vector: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_string: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_object: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_name: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_int: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_float: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_enum: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_class: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_value_as_bool: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_rotation_from_entry: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_get_location_from_entry: *mut crate::ffi::UFunctionOpague,
    pub u_blackboard_component_clear_value: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_stop_using_external_event: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_start_using_external_event: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_vector: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_string: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_rotator: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_object: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_name: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_int: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_float: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_enum: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_class: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_set_blackboard_value_as_bool: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_owners_blackboard: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_owner_component: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_vector: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_string: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_rotator: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_object: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_name: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_int: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_float: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_enum: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_class: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_bool: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_get_blackboard_value_as_actor: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_clear_blackboard_value_as_vector: *mut crate::ffi::UFunctionOpague,
    pub ubt_function_library_clear_blackboard_value: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_tick_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_tick: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_observer_deactivated_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_observer_deactivated: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_observer_activated_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_observer_activated: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_execution_start_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_execution_start: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_execution_finish_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_receive_execution_finish: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_perform_condition_check_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_perform_condition_check: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_is_decorator_observer_active: *mut crate::ffi::UFunctionOpague,
    pub ubt_decorator_blueprint_base_is_decorator_execution_active: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_tick_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_tick: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_search_start_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_search_start: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_deactivation_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_deactivation: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_activation_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_receive_activation: *mut crate::ffi::UFunctionOpague,
    pub ubt_service_blueprint_base_is_service_active: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_set_finish_on_message_with_id: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_set_finish_on_message: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_receive_tick_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_receive_tick: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_receive_execute_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_receive_execute: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_receive_abort_ai: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_receive_abort: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_is_task_executing: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_is_task_aborting: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_finish_execute: *mut crate::ffi::UFunctionOpague,
    pub ubt_task_blueprint_base_finish_abort: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_vector: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_struct: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_string: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_object: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_name: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_int32: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_float: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_enum: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_class: *mut crate::ffi::UFunctionOpague,
    pub u_value_or_bb_key_blueprint_utility_get_bool: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_unlock_ai_resources_with_animation: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_spawn_ai_from_class: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_simple_move_to_location: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_simple_move_to_actor: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_send_ai_message: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_lock_ai_resources_with_animation: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_is_valid_ai_rotation: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_is_valid_ai_location: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_is_valid_ai_direction: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_get_next_nav_link_index: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_get_current_path_points: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_get_current_path_index: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_get_current_path: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_get_blackboard: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_get_ai_controller: *mut crate::ffi::UFunctionOpague,
    pub uai_blueprint_helper_library_create_move_to_proxy_object: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_context_blueprint_base_provide_single_location: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_context_blueprint_base_provide_single_actor: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_context_blueprint_base_provide_locations_set: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_context_blueprint_base_provide_actors_set: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_instance_blueprint_wrapper_set_named_param: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_instance_blueprint_wrapper_get_results_as_locations: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_instance_blueprint_wrapper_get_results_as_actors: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_instance_blueprint_wrapper_get_query_results_as_locations: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_instance_blueprint_wrapper_get_query_results_as_actors: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_instance_blueprint_wrapper_get_item_score: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_instance_blueprint_wrapper_eqs_query_done_signature_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_manager_run_eqs_query: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_generator_blueprint_base_get_querier: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_generator_blueprint_base_do_item_generation_from_actors: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_generator_blueprint_base_do_item_generation: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_generator_blueprint_base_add_generated_vector: *mut crate::ffi::UFunctionOpague,
    pub u_env_query_generator_blueprint_base_add_generated_actor: *mut crate::ffi::UFunctionOpague,
    pub u_path_following_component_on_nav_data_registered: *mut crate::ffi::UFunctionOpague,
    pub u_path_following_component_on_actor_bump: *mut crate::ffi::UFunctionOpague,
    pub u_path_following_component_get_path_destination: *mut crate::ffi::UFunctionOpague,
    pub u_path_following_component_get_path_action_type: *mut crate::ffi::UFunctionOpague,
    pub u_crowd_following_component_suspend_crowd_steering: *mut crate::ffi::UFunctionOpague,
    pub u_generated_nav_links_proxy_receive_smart_link_reached: *mut crate::ffi::UFunctionOpague,
    pub a_nav_link_proxy_set_smart_link_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_nav_link_proxy_resume_path_following: *mut crate::ffi::UFunctionOpague,
    pub a_nav_link_proxy_receive_smart_link_reached: *mut crate::ffi::UFunctionOpague,
    pub a_nav_link_proxy_is_smart_link_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_nav_link_proxy_has_moving_agents: *mut crate::ffi::UFunctionOpague,
    pub a_nav_link_proxy_copy_end_points_from_simple_link_to_smart_link: *mut crate::ffi::UFunctionOpague,
    pub u_nav_local_grid_manager_set_local_navigation_grid_density: *mut crate::ffi::UFunctionOpague,
    pub u_nav_local_grid_manager_remove_local_navigation_grid: *mut crate::ffi::UFunctionOpague,
    pub u_nav_local_grid_manager_find_local_navigation_grid_path: *mut crate::ffi::UFunctionOpague,
    pub u_nav_local_grid_manager_add_local_navigation_grid_for_points: *mut crate::ffi::UFunctionOpague,
    pub u_nav_local_grid_manager_add_local_navigation_grid_for_point: *mut crate::ffi::UFunctionOpague,
    pub u_nav_local_grid_manager_add_local_navigation_grid_for_capsule: *mut crate::ffi::UFunctionOpague,
    pub u_nav_local_grid_manager_add_local_navigation_grid_for_box: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_set_sense_enabled: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_request_stimuli_listener_update: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_on_owner_end_play: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_is_sense_enabled: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_get_perceived_hostile_actors_by_sense: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_get_perceived_hostile_actors: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_get_known_perceived_actors: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_get_currently_perceived_actors: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_get_actors_perception: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_component_forget_all: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_stimuli_source_component_unregister_from_sense: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_stimuli_source_component_unregister_from_perception_system: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_stimuli_source_component_register_with_perception_system: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_stimuli_source_component_register_for_sense: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_system_report_perception_event: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_system_report_event: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_system_register_perception_stimuli_source: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_system_on_perception_stimuli_source_end_play: *mut crate::ffi::UFunctionOpague,
    pub uai_perception_system_get_sense_class_for_stimulus: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_blueprint_on_update: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_blueprint_on_listener_updated: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_blueprint_on_listener_unregistered: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_blueprint_on_listener_registered: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_blueprint_k2_on_new_pawn: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_blueprint_get_all_listener_components: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_blueprint_get_all_listener_actors: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_damage_report_damage_event: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_hearing_report_noise_event: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_prediction_request_pawn_prediction_event: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_prediction_request_controller_prediction_event: *mut crate::ffi::UFunctionOpague,
    pub uai_sense_touch_report_touch_event: *mut crate::ffi::UFunctionOpague,
    pub u_pawn_sensing_component_set_sensing_updates_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_pawn_sensing_component_set_sensing_interval: *mut crate::ffi::UFunctionOpague,
    pub u_pawn_sensing_component_set_peripheral_vision_angle: *mut crate::ffi::UFunctionOpague,
    pub u_pawn_sensing_component_see_pawn_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_pawn_sensing_component_hear_noise_delegate_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_pawn_sensing_component_get_peripheral_vision_cosine: *mut crate::ffi::UFunctionOpague,
    pub u_pawn_sensing_component_get_peripheral_vision_angle: *mut crate::ffi::UFunctionOpague,
    pub uai_task_move_to_ai_move_to: *mut crate::ffi::UFunctionOpague,
    pub uai_task_run_eqs_run_eqs: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            uai_async_task_blueprint_proxy_on_move_completed: std::ptr::null_mut(),
            aai_controller_use_blackboard: std::ptr::null_mut(),
            aai_controller_unclaim_task_resource: std::ptr::null_mut(),
            aai_controller_set_path_following_component: std::ptr::null_mut(),
            aai_controller_set_move_block_detection: std::ptr::null_mut(),
            aai_controller_run_behavior_tree: std::ptr::null_mut(),
            aai_controller_on_using_black_board: std::ptr::null_mut(),
            aai_controller_on_gameplay_task_resources_claimed: std::ptr::null_mut(),
            aai_controller_move_to_location: std::ptr::null_mut(),
            aai_controller_move_to_actor: std::ptr::null_mut(),
            aai_controller_k2_set_focus: std::ptr::null_mut(),
            aai_controller_k2_set_focal_point: std::ptr::null_mut(),
            aai_controller_k2_clear_focus: std::ptr::null_mut(),
            aai_controller_has_partial_path: std::ptr::null_mut(),
            aai_controller_get_path_following_component: std::ptr::null_mut(),
            aai_controller_get_move_status: std::ptr::null_mut(),
            aai_controller_get_immediate_move_destination: std::ptr::null_mut(),
            aai_controller_get_focus_actor: std::ptr::null_mut(),
            aai_controller_get_focal_point_on_actor: std::ptr::null_mut(),
            aai_controller_get_focal_point: std::ptr::null_mut(),
            aai_controller_get_ai_perception_component: std::ptr::null_mut(),
            aai_controller_claim_task_resource: std::ptr::null_mut(),
            uai_system_ai_logging_verbose: std::ptr::null_mut(),
            uai_system_ai_ignore_players: std::ptr::null_mut(),
            u_brain_component_stop_logic: std::ptr::null_mut(),
            u_brain_component_start_logic: std::ptr::null_mut(),
            u_brain_component_restart_logic: std::ptr::null_mut(),
            u_brain_component_is_running: std::ptr::null_mut(),
            u_brain_component_is_paused: std::ptr::null_mut(),
            u_behavior_tree_component_set_dynamic_subtree: std::ptr::null_mut(),
            u_behavior_tree_component_get_tag_cooldown_end_time: std::ptr::null_mut(),
            u_behavior_tree_component_add_cooldown_tag_duration: std::ptr::null_mut(),
            u_blackboard_asset_provider_get_blackboard_asset: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_vector: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_string: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_rotator: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_object: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_name: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_int: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_float: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_enum: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_class: std::ptr::null_mut(),
            u_blackboard_component_set_value_as_bool: std::ptr::null_mut(),
            u_blackboard_component_is_vector_value_set: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_vector: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_string: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_rotator: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_object: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_name: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_int: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_float: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_enum: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_class: std::ptr::null_mut(),
            u_blackboard_component_get_value_as_bool: std::ptr::null_mut(),
            u_blackboard_component_get_rotation_from_entry: std::ptr::null_mut(),
            u_blackboard_component_get_location_from_entry: std::ptr::null_mut(),
            u_blackboard_component_clear_value: std::ptr::null_mut(),
            ubt_function_library_stop_using_external_event: std::ptr::null_mut(),
            ubt_function_library_start_using_external_event: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_vector: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_string: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_rotator: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_object: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_name: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_int: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_float: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_enum: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_class: std::ptr::null_mut(),
            ubt_function_library_set_blackboard_value_as_bool: std::ptr::null_mut(),
            ubt_function_library_get_owners_blackboard: std::ptr::null_mut(),
            ubt_function_library_get_owner_component: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_vector: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_string: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_rotator: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_object: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_name: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_int: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_float: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_enum: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_class: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_bool: std::ptr::null_mut(),
            ubt_function_library_get_blackboard_value_as_actor: std::ptr::null_mut(),
            ubt_function_library_clear_blackboard_value_as_vector: std::ptr::null_mut(),
            ubt_function_library_clear_blackboard_value: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_tick_ai: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_tick: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_observer_deactivated_ai: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_observer_deactivated: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_observer_activated_ai: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_observer_activated: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_execution_start_ai: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_execution_start: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_execution_finish_ai: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_receive_execution_finish: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_perform_condition_check_ai: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_perform_condition_check: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_is_decorator_observer_active: std::ptr::null_mut(),
            ubt_decorator_blueprint_base_is_decorator_execution_active: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_tick_ai: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_tick: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_search_start_ai: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_search_start: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_deactivation_ai: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_deactivation: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_activation_ai: std::ptr::null_mut(),
            ubt_service_blueprint_base_receive_activation: std::ptr::null_mut(),
            ubt_service_blueprint_base_is_service_active: std::ptr::null_mut(),
            ubt_task_blueprint_base_set_finish_on_message_with_id: std::ptr::null_mut(),
            ubt_task_blueprint_base_set_finish_on_message: std::ptr::null_mut(),
            ubt_task_blueprint_base_receive_tick_ai: std::ptr::null_mut(),
            ubt_task_blueprint_base_receive_tick: std::ptr::null_mut(),
            ubt_task_blueprint_base_receive_execute_ai: std::ptr::null_mut(),
            ubt_task_blueprint_base_receive_execute: std::ptr::null_mut(),
            ubt_task_blueprint_base_receive_abort_ai: std::ptr::null_mut(),
            ubt_task_blueprint_base_receive_abort: std::ptr::null_mut(),
            ubt_task_blueprint_base_is_task_executing: std::ptr::null_mut(),
            ubt_task_blueprint_base_is_task_aborting: std::ptr::null_mut(),
            ubt_task_blueprint_base_finish_execute: std::ptr::null_mut(),
            ubt_task_blueprint_base_finish_abort: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_vector: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_struct: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_string: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_rotator: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_object: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_name: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_int32: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_float: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_enum: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_class: std::ptr::null_mut(),
            u_value_or_bb_key_blueprint_utility_get_bool: std::ptr::null_mut(),
            uai_blueprint_helper_library_unlock_ai_resources_with_animation: std::ptr::null_mut(),
            uai_blueprint_helper_library_spawn_ai_from_class: std::ptr::null_mut(),
            uai_blueprint_helper_library_simple_move_to_location: std::ptr::null_mut(),
            uai_blueprint_helper_library_simple_move_to_actor: std::ptr::null_mut(),
            uai_blueprint_helper_library_send_ai_message: std::ptr::null_mut(),
            uai_blueprint_helper_library_lock_ai_resources_with_animation: std::ptr::null_mut(),
            uai_blueprint_helper_library_is_valid_ai_rotation: std::ptr::null_mut(),
            uai_blueprint_helper_library_is_valid_ai_location: std::ptr::null_mut(),
            uai_blueprint_helper_library_is_valid_ai_direction: std::ptr::null_mut(),
            uai_blueprint_helper_library_get_next_nav_link_index: std::ptr::null_mut(),
            uai_blueprint_helper_library_get_current_path_points: std::ptr::null_mut(),
            uai_blueprint_helper_library_get_current_path_index: std::ptr::null_mut(),
            uai_blueprint_helper_library_get_current_path: std::ptr::null_mut(),
            uai_blueprint_helper_library_get_blackboard: std::ptr::null_mut(),
            uai_blueprint_helper_library_get_ai_controller: std::ptr::null_mut(),
            uai_blueprint_helper_library_create_move_to_proxy_object: std::ptr::null_mut(),
            u_env_query_context_blueprint_base_provide_single_location: std::ptr::null_mut(),
            u_env_query_context_blueprint_base_provide_single_actor: std::ptr::null_mut(),
            u_env_query_context_blueprint_base_provide_locations_set: std::ptr::null_mut(),
            u_env_query_context_blueprint_base_provide_actors_set: std::ptr::null_mut(),
            u_env_query_instance_blueprint_wrapper_set_named_param: std::ptr::null_mut(),
            u_env_query_instance_blueprint_wrapper_get_results_as_locations: std::ptr::null_mut(),
            u_env_query_instance_blueprint_wrapper_get_results_as_actors: std::ptr::null_mut(),
            u_env_query_instance_blueprint_wrapper_get_query_results_as_locations: std::ptr::null_mut(),
            u_env_query_instance_blueprint_wrapper_get_query_results_as_actors: std::ptr::null_mut(),
            u_env_query_instance_blueprint_wrapper_get_item_score: std::ptr::null_mut(),
            u_env_query_instance_blueprint_wrapper_eqs_query_done_signature_delegate_signature: std::ptr::null_mut(),
            u_env_query_manager_run_eqs_query: std::ptr::null_mut(),
            u_env_query_generator_blueprint_base_get_querier: std::ptr::null_mut(),
            u_env_query_generator_blueprint_base_do_item_generation_from_actors: std::ptr::null_mut(),
            u_env_query_generator_blueprint_base_do_item_generation: std::ptr::null_mut(),
            u_env_query_generator_blueprint_base_add_generated_vector: std::ptr::null_mut(),
            u_env_query_generator_blueprint_base_add_generated_actor: std::ptr::null_mut(),
            u_path_following_component_on_nav_data_registered: std::ptr::null_mut(),
            u_path_following_component_on_actor_bump: std::ptr::null_mut(),
            u_path_following_component_get_path_destination: std::ptr::null_mut(),
            u_path_following_component_get_path_action_type: std::ptr::null_mut(),
            u_crowd_following_component_suspend_crowd_steering: std::ptr::null_mut(),
            u_generated_nav_links_proxy_receive_smart_link_reached: std::ptr::null_mut(),
            a_nav_link_proxy_set_smart_link_enabled: std::ptr::null_mut(),
            a_nav_link_proxy_resume_path_following: std::ptr::null_mut(),
            a_nav_link_proxy_receive_smart_link_reached: std::ptr::null_mut(),
            a_nav_link_proxy_is_smart_link_enabled: std::ptr::null_mut(),
            a_nav_link_proxy_has_moving_agents: std::ptr::null_mut(),
            a_nav_link_proxy_copy_end_points_from_simple_link_to_smart_link: std::ptr::null_mut(),
            u_nav_local_grid_manager_set_local_navigation_grid_density: std::ptr::null_mut(),
            u_nav_local_grid_manager_remove_local_navigation_grid: std::ptr::null_mut(),
            u_nav_local_grid_manager_find_local_navigation_grid_path: std::ptr::null_mut(),
            u_nav_local_grid_manager_add_local_navigation_grid_for_points: std::ptr::null_mut(),
            u_nav_local_grid_manager_add_local_navigation_grid_for_point: std::ptr::null_mut(),
            u_nav_local_grid_manager_add_local_navigation_grid_for_capsule: std::ptr::null_mut(),
            u_nav_local_grid_manager_add_local_navigation_grid_for_box: std::ptr::null_mut(),
            uai_perception_component_set_sense_enabled: std::ptr::null_mut(),
            uai_perception_component_request_stimuli_listener_update: std::ptr::null_mut(),
            uai_perception_component_on_owner_end_play: std::ptr::null_mut(),
            uai_perception_component_is_sense_enabled: std::ptr::null_mut(),
            uai_perception_component_get_perceived_hostile_actors_by_sense: std::ptr::null_mut(),
            uai_perception_component_get_perceived_hostile_actors: std::ptr::null_mut(),
            uai_perception_component_get_known_perceived_actors: std::ptr::null_mut(),
            uai_perception_component_get_currently_perceived_actors: std::ptr::null_mut(),
            uai_perception_component_get_actors_perception: std::ptr::null_mut(),
            uai_perception_component_forget_all: std::ptr::null_mut(),
            uai_perception_stimuli_source_component_unregister_from_sense: std::ptr::null_mut(),
            uai_perception_stimuli_source_component_unregister_from_perception_system: std::ptr::null_mut(),
            uai_perception_stimuli_source_component_register_with_perception_system: std::ptr::null_mut(),
            uai_perception_stimuli_source_component_register_for_sense: std::ptr::null_mut(),
            uai_perception_system_report_perception_event: std::ptr::null_mut(),
            uai_perception_system_report_event: std::ptr::null_mut(),
            uai_perception_system_register_perception_stimuli_source: std::ptr::null_mut(),
            uai_perception_system_on_perception_stimuli_source_end_play: std::ptr::null_mut(),
            uai_perception_system_get_sense_class_for_stimulus: std::ptr::null_mut(),
            uai_sense_blueprint_on_update: std::ptr::null_mut(),
            uai_sense_blueprint_on_listener_updated: std::ptr::null_mut(),
            uai_sense_blueprint_on_listener_unregistered: std::ptr::null_mut(),
            uai_sense_blueprint_on_listener_registered: std::ptr::null_mut(),
            uai_sense_blueprint_k2_on_new_pawn: std::ptr::null_mut(),
            uai_sense_blueprint_get_all_listener_components: std::ptr::null_mut(),
            uai_sense_blueprint_get_all_listener_actors: std::ptr::null_mut(),
            uai_sense_damage_report_damage_event: std::ptr::null_mut(),
            uai_sense_hearing_report_noise_event: std::ptr::null_mut(),
            uai_sense_prediction_request_pawn_prediction_event: std::ptr::null_mut(),
            uai_sense_prediction_request_controller_prediction_event: std::ptr::null_mut(),
            uai_sense_touch_report_touch_event: std::ptr::null_mut(),
            u_pawn_sensing_component_set_sensing_updates_enabled: std::ptr::null_mut(),
            u_pawn_sensing_component_set_sensing_interval: std::ptr::null_mut(),
            u_pawn_sensing_component_set_peripheral_vision_angle: std::ptr::null_mut(),
            u_pawn_sensing_component_see_pawn_delegate_delegate_signature: std::ptr::null_mut(),
            u_pawn_sensing_component_hear_noise_delegate_delegate_signature: std::ptr::null_mut(),
            u_pawn_sensing_component_get_peripheral_vision_cosine: std::ptr::null_mut(),
            u_pawn_sensing_component_get_peripheral_vision_angle: std::ptr::null_mut(),
            uai_task_move_to_ai_move_to: std::ptr::null_mut(),
            uai_task_run_eqs_run_eqs: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAIAsyncTaskBlueprintProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMoveCompleted"),
                &raw mut __FUNCTION_PTRS.uai_async_task_blueprint_proxy_on_move_completed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AAIController::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UseBlackboard"),
                &raw mut __FUNCTION_PTRS.aai_controller_use_blackboard,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnclaimTaskResource"),
                &raw mut __FUNCTION_PTRS.aai_controller_unclaim_task_resource,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPathFollowingComponent"),
                &raw mut __FUNCTION_PTRS.aai_controller_set_path_following_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMoveBlockDetection"),
                &raw mut __FUNCTION_PTRS.aai_controller_set_move_block_detection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RunBehaviorTree"),
                &raw mut __FUNCTION_PTRS.aai_controller_run_behavior_tree,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnUsingBlackBoard"),
                &raw mut __FUNCTION_PTRS.aai_controller_on_using_black_board,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnGameplayTaskResourcesClaimed"),
                &raw mut __FUNCTION_PTRS
                    .aai_controller_on_gameplay_task_resources_claimed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveToLocation"),
                &raw mut __FUNCTION_PTRS.aai_controller_move_to_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveToActor"),
                &raw mut __FUNCTION_PTRS.aai_controller_move_to_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_SetFocus"),
                &raw mut __FUNCTION_PTRS.aai_controller_k2_set_focus,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_SetFocalPoint"),
                &raw mut __FUNCTION_PTRS.aai_controller_k2_set_focal_point,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ClearFocus"),
                &raw mut __FUNCTION_PTRS.aai_controller_k2_clear_focus,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasPartialPath"),
                &raw mut __FUNCTION_PTRS.aai_controller_has_partial_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathFollowingComponent"),
                &raw mut __FUNCTION_PTRS.aai_controller_get_path_following_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMoveStatus"),
                &raw mut __FUNCTION_PTRS.aai_controller_get_move_status,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetImmediateMoveDestination"),
                &raw mut __FUNCTION_PTRS.aai_controller_get_immediate_move_destination,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFocusActor"),
                &raw mut __FUNCTION_PTRS.aai_controller_get_focus_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFocalPointOnActor"),
                &raw mut __FUNCTION_PTRS.aai_controller_get_focal_point_on_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFocalPoint"),
                &raw mut __FUNCTION_PTRS.aai_controller_get_focal_point,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAIPerceptionComponent"),
                &raw mut __FUNCTION_PTRS.aai_controller_get_ai_perception_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClaimTaskResource"),
                &raw mut __FUNCTION_PTRS.aai_controller_claim_task_resource,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAISystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AILoggingVerbose"),
                &raw mut __FUNCTION_PTRS.uai_system_ai_logging_verbose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AIIgnorePlayers"),
                &raw mut __FUNCTION_PTRS.uai_system_ai_ignore_players,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBrainComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopLogic"),
                &raw mut __FUNCTION_PTRS.u_brain_component_stop_logic,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartLogic"),
                &raw mut __FUNCTION_PTRS.u_brain_component_start_logic,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RestartLogic"),
                &raw mut __FUNCTION_PTRS.u_brain_component_restart_logic,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRunning"),
                &raw mut __FUNCTION_PTRS.u_brain_component_is_running,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPaused"),
                &raw mut __FUNCTION_PTRS.u_brain_component_is_paused,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBehaviorTreeComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDynamicSubtree"),
                &raw mut __FUNCTION_PTRS.u_behavior_tree_component_set_dynamic_subtree,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTagCooldownEndTime"),
                &raw mut __FUNCTION_PTRS
                    .u_behavior_tree_component_get_tag_cooldown_end_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddCooldownTagDuration"),
                &raw mut __FUNCTION_PTRS
                    .u_behavior_tree_component_add_cooldown_tag_duration,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlackboardAssetProvider::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardAsset"),
                &raw mut __FUNCTION_PTRS.u_blackboard_asset_provider_get_blackboard_asset,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlackboardComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsVector"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsString"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsRotator"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsObject"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsName"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsInt"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsFloat"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsEnum"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsClass"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetValueAsBool"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_set_value_as_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsVectorValueSet"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_is_vector_value_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsVector"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsString"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsRotator"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsObject"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsName"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsInt"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsFloat"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsEnum"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsClass"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetValueAsBool"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_value_as_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRotationFromEntry"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_rotation_from_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocationFromEntry"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_get_location_from_entry,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearValue"),
                &raw mut __FUNCTION_PTRS.u_blackboard_component_clear_value,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBTFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopUsingExternalEvent"),
                &raw mut __FUNCTION_PTRS.ubt_function_library_stop_using_external_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartUsingExternalEvent"),
                &raw mut __FUNCTION_PTRS.ubt_function_library_start_using_external_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsVector"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsString"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsRotator"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsObject"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsName"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsInt"),
                &raw mut __FUNCTION_PTRS.ubt_function_library_set_blackboard_value_as_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsFloat"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsEnum"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsClass"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBlackboardValueAsBool"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOwnersBlackboard"),
                &raw mut __FUNCTION_PTRS.ubt_function_library_get_owners_blackboard,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOwnerComponent"),
                &raw mut __FUNCTION_PTRS.ubt_function_library_get_owner_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsVector"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsString"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsRotator"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsObject"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsName"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsInt"),
                &raw mut __FUNCTION_PTRS.ubt_function_library_get_blackboard_value_as_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsFloat"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsEnum"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsClass"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsBool"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboardValueAsActor"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearBlackboardValueAsVector"),
                &raw mut __FUNCTION_PTRS
                    .ubt_function_library_clear_blackboard_value_as_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearBlackboardValue"),
                &raw mut __FUNCTION_PTRS.ubt_function_library_clear_blackboard_value,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBTDecorator_BlueprintBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveTickAI"),
                &raw mut __FUNCTION_PTRS.ubt_decorator_blueprint_base_receive_tick_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveTick"),
                &raw mut __FUNCTION_PTRS.ubt_decorator_blueprint_base_receive_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveObserverDeactivatedAI"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_deactivated_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveObserverDeactivated"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_deactivated,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveObserverActivatedAI"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_activated_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveObserverActivated"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_activated,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveExecutionStartAI"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_start_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveExecutionStart"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveExecutionFinishAI"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_finish_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveExecutionFinish"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_finish,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PerformConditionCheckAI"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_perform_condition_check_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PerformConditionCheck"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_perform_condition_check,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDecoratorObserverActive"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_is_decorator_observer_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDecoratorExecutionActive"),
                &raw mut __FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_is_decorator_execution_active,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBTService_BlueprintBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveTickAI"),
                &raw mut __FUNCTION_PTRS.ubt_service_blueprint_base_receive_tick_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveTick"),
                &raw mut __FUNCTION_PTRS.ubt_service_blueprint_base_receive_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveSearchStartAI"),
                &raw mut __FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_search_start_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveSearchStart"),
                &raw mut __FUNCTION_PTRS.ubt_service_blueprint_base_receive_search_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveDeactivationAI"),
                &raw mut __FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_deactivation_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveDeactivation"),
                &raw mut __FUNCTION_PTRS.ubt_service_blueprint_base_receive_deactivation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveActivationAI"),
                &raw mut __FUNCTION_PTRS.ubt_service_blueprint_base_receive_activation_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveActivation"),
                &raw mut __FUNCTION_PTRS.ubt_service_blueprint_base_receive_activation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsServiceActive"),
                &raw mut __FUNCTION_PTRS.ubt_service_blueprint_base_is_service_active,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBTTask_BlueprintBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFinishOnMessageWithId"),
                &raw mut __FUNCTION_PTRS
                    .ubt_task_blueprint_base_set_finish_on_message_with_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFinishOnMessage"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_set_finish_on_message,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveTickAI"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_receive_tick_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveTick"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_receive_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveExecuteAI"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_receive_execute_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveExecute"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_receive_execute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveAbortAI"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_receive_abort_ai,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveAbort"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_receive_abort,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTaskExecuting"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_is_task_executing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTaskAborting"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_is_task_aborting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FinishExecute"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_finish_execute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FinishAbort"),
                &raw mut __FUNCTION_PTRS.ubt_task_blueprint_base_finish_abort,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UValueOrBBKeyBlueprintUtility::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVector"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStruct"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetString"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRotator"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObject"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetName"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInt32"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_int32,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloat"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnum"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetClass"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBool"),
                &raw mut __FUNCTION_PTRS.u_value_or_bb_key_blueprint_utility_get_bool,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAIBlueprintHelperLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnlockAIResourcesWithAnimation"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_unlock_ai_resources_with_animation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpawnAIFromClass"),
                &raw mut __FUNCTION_PTRS.uai_blueprint_helper_library_spawn_ai_from_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SimpleMoveToLocation"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_simple_move_to_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SimpleMoveToActor"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_simple_move_to_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SendAIMessage"),
                &raw mut __FUNCTION_PTRS.uai_blueprint_helper_library_send_ai_message,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LockAIResourcesWithAnimation"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_lock_ai_resources_with_animation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidAIRotation"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidAILocation"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidAIDirection"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_direction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNextNavLinkIndex"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_next_nav_link_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentPathPoints"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path_points,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentPathIndex"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentPath"),
                &raw mut __FUNCTION_PTRS.uai_blueprint_helper_library_get_current_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboard"),
                &raw mut __FUNCTION_PTRS.uai_blueprint_helper_library_get_blackboard,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAIController"),
                &raw mut __FUNCTION_PTRS.uai_blueprint_helper_library_get_ai_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateMoveToProxyObject"),
                &raw mut __FUNCTION_PTRS
                    .uai_blueprint_helper_library_create_move_to_proxy_object,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEnvQueryContext_BlueprintBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProvideSingleLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_single_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProvideSingleActor"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_single_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProvideLocationsSet"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_locations_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProvideActorsSet"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_actors_set,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEnvQueryInstanceBlueprintWrapper::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNamedParam"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_set_named_param,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetResultsAsLocations"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_results_as_locations,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetResultsAsActors"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_results_as_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetQueryResultsAsLocations"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_query_results_as_locations,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetQueryResultsAsActors"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_query_results_as_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetItemScore"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_item_score,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EQSQueryDoneSignature__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_eqs_query_done_signature_delegate_signature,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEnvQueryManager::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RunEQSQuery"),
                &raw mut __FUNCTION_PTRS.u_env_query_manager_run_eqs_query,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UEnvQueryGenerator_BlueprintBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetQuerier"),
                &raw mut __FUNCTION_PTRS.u_env_query_generator_blueprint_base_get_querier,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoItemGenerationFromActors"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_do_item_generation_from_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoItemGeneration"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_do_item_generation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGeneratedVector"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_add_generated_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGeneratedActor"),
                &raw mut __FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_add_generated_actor,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPathFollowingComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnNavDataRegistered"),
                &raw mut __FUNCTION_PTRS
                    .u_path_following_component_on_nav_data_registered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnActorBump"),
                &raw mut __FUNCTION_PTRS.u_path_following_component_on_actor_bump,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathDestination"),
                &raw mut __FUNCTION_PTRS.u_path_following_component_get_path_destination,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathActionType"),
                &raw mut __FUNCTION_PTRS.u_path_following_component_get_path_action_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCrowdFollowingComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SuspendCrowdSteering"),
                &raw mut __FUNCTION_PTRS
                    .u_crowd_following_component_suspend_crowd_steering,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGeneratedNavLinksProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveSmartLinkReached"),
                &raw mut __FUNCTION_PTRS
                    .u_generated_nav_links_proxy_receive_smart_link_reached,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ANavLinkProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSmartLinkEnabled"),
                &raw mut __FUNCTION_PTRS.a_nav_link_proxy_set_smart_link_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResumePathFollowing"),
                &raw mut __FUNCTION_PTRS.a_nav_link_proxy_resume_path_following,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReceiveSmartLinkReached"),
                &raw mut __FUNCTION_PTRS.a_nav_link_proxy_receive_smart_link_reached,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSmartLinkEnabled"),
                &raw mut __FUNCTION_PTRS.a_nav_link_proxy_is_smart_link_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasMovingAgents"),
                &raw mut __FUNCTION_PTRS.a_nav_link_proxy_has_moving_agents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CopyEndPointsFromSimpleLinkToSmartLink"),
                &raw mut __FUNCTION_PTRS
                    .a_nav_link_proxy_copy_end_points_from_simple_link_to_smart_link,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNavLocalGridManager::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalNavigationGridDensity"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_local_grid_manager_set_local_navigation_grid_density,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveLocalNavigationGrid"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_local_grid_manager_remove_local_navigation_grid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindLocalNavigationGridPath"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_local_grid_manager_find_local_navigation_grid_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLocalNavigationGridForPoints"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_points,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLocalNavigationGridForPoint"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_point,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLocalNavigationGridForCapsule"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_capsule,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLocalNavigationGridForBox"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_box,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAIPerceptionComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSenseEnabled"),
                &raw mut __FUNCTION_PTRS.uai_perception_component_set_sense_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestStimuliListenerUpdate"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_component_request_stimuli_listener_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnOwnerEndPlay"),
                &raw mut __FUNCTION_PTRS.uai_perception_component_on_owner_end_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSenseEnabled"),
                &raw mut __FUNCTION_PTRS.uai_perception_component_is_sense_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPerceivedHostileActorsBySense"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_component_get_perceived_hostile_actors_by_sense,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPerceivedHostileActors"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_component_get_perceived_hostile_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetKnownPerceivedActors"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_component_get_known_perceived_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentlyPerceivedActors"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_component_get_currently_perceived_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorsPerception"),
                &raw mut __FUNCTION_PTRS.uai_perception_component_get_actors_perception,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ForgetAll"),
                &raw mut __FUNCTION_PTRS.uai_perception_component_forget_all,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAIPerceptionStimuliSourceComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterFromSense"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_unregister_from_sense,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterFromPerceptionSystem"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_unregister_from_perception_system,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterWithPerceptionSystem"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_register_with_perception_system,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterForSense"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_register_for_sense,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAIPerceptionSystem::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReportPerceptionEvent"),
                &raw mut __FUNCTION_PTRS.uai_perception_system_report_perception_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReportEvent"),
                &raw mut __FUNCTION_PTRS.uai_perception_system_report_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterPerceptionStimuliSource"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_system_register_perception_stimuli_source,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPerceptionStimuliSourceEndPlay"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_system_on_perception_stimuli_source_end_play,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSenseClassForStimulus"),
                &raw mut __FUNCTION_PTRS
                    .uai_perception_system_get_sense_class_for_stimulus,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAISense_Blueprint::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnUpdate"),
                &raw mut __FUNCTION_PTRS.uai_sense_blueprint_on_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnListenerUpdated"),
                &raw mut __FUNCTION_PTRS.uai_sense_blueprint_on_listener_updated,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnListenerUnregistered"),
                &raw mut __FUNCTION_PTRS.uai_sense_blueprint_on_listener_unregistered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnListenerRegistered"),
                &raw mut __FUNCTION_PTRS.uai_sense_blueprint_on_listener_registered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnNewPawn"),
                &raw mut __FUNCTION_PTRS.uai_sense_blueprint_k2_on_new_pawn,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllListenerComponents"),
                &raw mut __FUNCTION_PTRS.uai_sense_blueprint_get_all_listener_components,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllListenerActors"),
                &raw mut __FUNCTION_PTRS.uai_sense_blueprint_get_all_listener_actors,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAISense_Damage::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReportDamageEvent"),
                &raw mut __FUNCTION_PTRS.uai_sense_damage_report_damage_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAISense_Hearing::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReportNoiseEvent"),
                &raw mut __FUNCTION_PTRS.uai_sense_hearing_report_noise_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAISense_Prediction::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestPawnPredictionEvent"),
                &raw mut __FUNCTION_PTRS
                    .uai_sense_prediction_request_pawn_prediction_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestControllerPredictionEvent"),
                &raw mut __FUNCTION_PTRS
                    .uai_sense_prediction_request_controller_prediction_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAISense_Touch::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReportTouchEvent"),
                &raw mut __FUNCTION_PTRS.uai_sense_touch_report_touch_event,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPawnSensingComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSensingUpdatesEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_pawn_sensing_component_set_sensing_updates_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSensingInterval"),
                &raw mut __FUNCTION_PTRS.u_pawn_sensing_component_set_sensing_interval,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPeripheralVisionAngle"),
                &raw mut __FUNCTION_PTRS
                    .u_pawn_sensing_component_set_peripheral_vision_angle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SeePawnDelegate__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_pawn_sensing_component_see_pawn_delegate_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HearNoiseDelegate__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_pawn_sensing_component_hear_noise_delegate_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPeripheralVisionCosine"),
                &raw mut __FUNCTION_PTRS
                    .u_pawn_sensing_component_get_peripheral_vision_cosine,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPeripheralVisionAngle"),
                &raw mut __FUNCTION_PTRS
                    .u_pawn_sensing_component_get_peripheral_vision_angle,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAITask_MoveTo::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AIMoveTo"),
                &raw mut __FUNCTION_PTRS.uai_task_move_to_ai_move_to,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAITask_RunEQS::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RunEQS"),
                &raw mut __FUNCTION_PTRS.uai_task_run_eqs_run_eqs,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FActorPerceptionUpdateInfo {
    pub target_id: i32,
    pub target: TWeakObjectPtr<crate::bindings::engine::AActor>,
    pub stimulus: FAIStimulus,
}
impl FActorPerceptionUpdateInfo {}
#[repr(C, align(8))]
pub struct FAIStimulus {
    pub age: f32,
    pub expiration_age: f32,
    pub strength: f32,
    pub stimulus_location: crate::bindings::core_u_object::FVector,
    pub receiver_location: crate::bindings::core_u_object::FVector,
    pub tag: FName,
    #[doc(hidden)]
    pub(crate) __padding_92: [u8; 16],
    pub flags_92: u8,
}
impl FAIStimulus {}
#[repr(C, align(4))]
pub struct FAIRequestID {
    pub(crate) __padding_end: [u8; 4],
}
impl FAIRequestID {}
#[repr(C, align(1))]
pub struct FGenericTeamId {
    pub team_id: u8,
}
impl FGenericTeamId {}
#[repr(C, align(8))]
pub struct FBlackboardKeySelector {
    pub allowed_types: TArray<UPtr<UBlackboardKeyType>>,
    pub selected_key_name: FName,
    pub selected_key_type: TSubclassOf<UBlackboardKeyType>,
    pub selected_key_id: i32,
    pub flags_44: u8,
}
impl FBlackboardKeySelector {}
#[repr(C, align(8))]
pub struct FValueOrBlackboardKeyBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub key: FName,
}
impl FValueOrBlackboardKeyBase {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Bool {
    pub(crate) __padding_end: [u8; 32],
}
impl FValueOrBBKey_Bool {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Class {
    pub(crate) __padding_end: [u8; 40],
}
impl FValueOrBBKey_Class {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Enum {
    pub(crate) __padding_end: [u8; 56],
}
impl FValueOrBBKey_Enum {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Float {
    pub(crate) __padding_end: [u8; 32],
}
impl FValueOrBBKey_Float {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Int32 {
    pub(crate) __padding_end: [u8; 32],
}
impl FValueOrBBKey_Int32 {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Name {
    pub(crate) __padding_end: [u8; 40],
}
impl FValueOrBBKey_Name {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_String {
    pub(crate) __padding_end: [u8; 40],
}
impl FValueOrBBKey_String {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Object {
    pub(crate) __padding_end: [u8; 40],
}
impl FValueOrBBKey_Object {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Rotator {
    pub(crate) __padding_end: [u8; 48],
}
impl FValueOrBBKey_Rotator {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Vector {
    pub(crate) __padding_end: [u8; 48],
}
impl FValueOrBBKey_Vector {}
#[repr(C, align(8))]
pub struct FValueOrBBKey_Struct {
    pub(crate) __padding_end: [u8; 48],
}
impl FValueOrBBKey_Struct {}
#[repr(C, align(8))]
pub struct FEnvQueryResult {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    #[doc(hidden)]
    pub(crate) __padding_44: [u8; 20],
    pub option_index: i32,
    pub query_id: i32,
    pub(crate) __padding_end: [u8; 12],
}
impl FEnvQueryResult {}
#[repr(C, align(8))]
pub struct FEnvQueryInstance {
    pub(crate) __padding_end: [u8; 448],
}
impl FEnvQueryInstance {}
#[repr(C, align(4))]
pub struct FEnvNamedValue {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub value: f32,
}
impl FEnvNamedValue {}
#[repr(C, align(8))]
pub struct FAIDynamicParam {
    pub param_name: FName,
    pub param_type: EAIParamType,
    pub flags_13: u8,
    pub value: f32,
    pub bb_key: FBlackboardKeySelector,
}
impl FAIDynamicParam {}
#[repr(C, align(8))]
pub struct FActorPerceptionBlueprintInfo {
    pub target: UPtr<crate::bindings::engine::AActor>,
    pub last_sensed_stimuli: TArray<FAIStimulus>,
    pub flags_24: u8,
}
impl FActorPerceptionBlueprintInfo {}
#[repr(C, align(4))]
pub struct FAISenseAffiliationFilter {
    pub flags_0: u8,
}
impl FAISenseAffiliationFilter {}
#[repr(C, align(8))]
pub struct FAIDamageEvent {
    pub amount: f32,
    pub location: crate::bindings::core_u_object::FVector,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub damaged_actor: UPtr<crate::bindings::engine::AActor>,
    pub instigator: UPtr<crate::bindings::engine::AActor>,
    pub tag: FName,
}
impl FAIDamageEvent {}
#[repr(C, align(8))]
pub struct FAINoiseEvent {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub noise_location: crate::bindings::core_u_object::FVector,
    pub loudness: f32,
    pub max_range: f32,
    pub instigator: UPtr<crate::bindings::engine::AActor>,
    pub tag: FName,
}
impl FAINoiseEvent {}
#[repr(C, align(8))]
pub struct UAIAsyncTaskBlueprintProxy {
    __padding_end: [u8; 128],
}
impl UAIAsyncTaskBlueprintProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIAsyncTaskBlueprintProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIAsyncTaskBlueprintProxy")
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
pub struct IAIResourceInterface {}
#[repr(C, align(8))]
pub struct UAIResourceInterface {
    __padding_end: [u8; 48],
}
impl UAIResourceInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResourceInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResourceInterface")
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
pub struct UAISenseBlueprintListener {
    __padding_end: [u8; 368],
}
impl UAISenseBlueprintListener {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseBlueprintListener")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseBlueprintListener")
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
pub struct UAISenseConfig {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub debug_color: crate::bindings::core_u_object::FColor,
    pub max_age: f32,
    pub flags_56: u8,
    __padding_end: [u8; 23],
}
impl UAISenseConfig {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig")
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
pub struct UAISenseConfig_Blueprint {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Blueprint>,
}
impl UAISenseConfig_Blueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Blueprint")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Blueprint")
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
pub struct UAISenseConfig_Hearing {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Hearing>,
    pub hearing_range: f32,
    pub lo_s_hearing_range: f32,
    #[doc(hidden)]
    pub(crate) __padding_100: [u8; 4],
    pub detection_by_affiliation: FAISenseAffiliationFilter,
}
impl UAISenseConfig_Hearing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Hearing")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Hearing")
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
pub struct UAISenseConfig_Prediction {
    __padding_end: [u8; 80],
}
impl UAISenseConfig_Prediction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Prediction")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Prediction")
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
pub struct UAISenseConfig_Sight {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Sight>,
    pub sight_radius: f32,
    pub lose_sight_radius: f32,
    pub peripheral_vision_angle_degrees: f32,
    pub detection_by_affiliation: FAISenseAffiliationFilter,
    pub auto_success_range_from_last_seen_location: f32,
    pub point_of_view_backward_offset: f32,
    pub near_clipping_radius: f32,
}
impl UAISenseConfig_Sight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Sight")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Sight")
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
pub struct UAISenseConfig_Team {
    __padding_end: [u8; 80],
}
impl UAISenseConfig_Team {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Team")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Team")
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
pub struct UAISenseConfig_Touch {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub detection_by_affiliation: FAISenseAffiliationFilter,
}
impl UAISenseConfig_Touch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Touch")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Touch")
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
pub struct UAISenseEvent {
    __padding_end: [u8; 48],
}
impl UAISenseEvent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent")
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
pub struct UAISenseEvent_Damage {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub event: FAIDamageEvent,
}
impl UAISenseEvent_Damage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent_Damage")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent_Damage")
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
pub struct UAISenseEvent_Hearing {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub event: FAINoiseEvent,
}
impl UAISenseEvent_Hearing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent_Hearing")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseEvent_Hearing")
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
pub struct ICrowdAgentInterface {}
#[repr(C, align(8))]
pub struct UCrowdAgentInterface {
    __padding_end: [u8; 48],
}
impl UCrowdAgentInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdAgentInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdAgentInterface")
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
pub struct IEQSQueryResultSourceInterface {}
#[repr(C, align(8))]
pub struct UEQSQueryResultSourceInterface {
    __padding_end: [u8; 48],
}
impl UEQSQueryResultSourceInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEQSQueryResultSourceInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEQSQueryResultSourceInterface")
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
pub struct IGenericTeamAgentInterface {}
#[repr(C, align(8))]
pub struct UGenericTeamAgentInterface {
    __padding_end: [u8; 48],
}
impl UGenericTeamAgentInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenericTeamAgentInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenericTeamAgentInterface")
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
pub struct AAIController {
    #[doc(hidden)]
    pub(crate) __padding_1376: [u8; 1376],
    pub flags_1376: u8,
    #[doc(hidden)]
    pub(crate) __padding_1392: [u8; 8],
    pub brain_component: UPtr<UBrainComponent>,
    #[doc(hidden)]
    pub(crate) __padding_1408: [u8; 8],
    pub blackboard: UPtr<UBlackboardComponent>,
    #[doc(hidden)]
    pub(crate) __padding_1424: [u8; 8],
    pub default_navigation_filter_class: TSubclassOf<
        crate::bindings::navigation_system::UNavigationQueryFilter,
    >,
    __padding_end: [u8; 32],
}
impl AAIController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAIController")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAIController")
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
    pub fn use_blackboard(
        &mut self,
        blackboard_asset: UPtr<UBlackboardData>,
        blackboard_component: &mut UPtr<UBlackboardComponent>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_use_blackboard,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blackboard_asset,
                __buffer.add(0).cast::<UPtr<UBlackboardData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                blackboard_component,
                __buffer.add(8).cast::<UPtr<UBlackboardComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_use_blackboard,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<UBlackboardComponent>>()
                .swap(blackboard_component);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn unclaim_task_resource(
        &mut self,
        resource_class: TSubclassOf<
            crate::bindings::gameplay_tasks::UGameplayTaskResource,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_unclaim_task_resource,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &resource_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<
                            crate::bindings::gameplay_tasks::UGameplayTaskResource,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_unclaim_task_resource,
                __buffer,
            )
        };
    }
    pub fn set_path_following_component(
        &mut self,
        new_pf_component: UPtr<UPathFollowingComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_set_path_following_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_pf_component,
                __buffer.add(0).cast::<UPtr<UPathFollowingComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_set_path_following_component,
                __buffer,
            )
        };
    }
    pub fn set_move_block_detection(&mut self, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_set_move_block_detection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_set_move_block_detection,
                __buffer,
            )
        };
    }
    pub fn run_behavior_tree(&mut self, bt_asset: UPtr<UBehaviorTree>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_run_behavior_tree,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bt_asset,
                __buffer.add(0).cast::<UPtr<UBehaviorTree>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_run_behavior_tree,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn on_using_black_board(
        &mut self,
        blackboard_comp: UPtr<UBlackboardComponent>,
        blackboard_asset: UPtr<UBlackboardData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_on_using_black_board,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blackboard_comp,
                __buffer.add(0).cast::<UPtr<UBlackboardComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blackboard_asset,
                __buffer.add(8).cast::<UPtr<UBlackboardData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_on_using_black_board,
                __buffer,
            )
        };
    }
    pub fn move_to_location(
        &mut self,
        dest: &crate::bindings::core_u_object::FVector,
        acceptance_radius: f32,
        b_stop_on_overlap: bool,
        b_use_pathfinding: bool,
        b_project_destination_to_navigation: bool,
        b_can_strafe: bool,
        filter_class: TSubclassOf<
            crate::bindings::navigation_system::UNavigationQueryFilter,
        >,
        b_allow_partial_path: bool,
    ) -> EPathFollowingRequestResult {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_move_to_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                dest,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &acceptance_radius,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_stop_on_overlap,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_pathfinding,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_project_destination_to_navigation,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_can_strafe,
                __buffer.add(31).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer
                    .add(32)
                    .cast::<
                        TSubclassOf<
                            crate::bindings::navigation_system::UNavigationQueryFilter,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_partial_path,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_move_to_location,
                __buffer,
            )
        };
        unsafe { __buffer.add(41).cast::<EPathFollowingRequestResult>().read() }
    }
    pub fn move_to_actor(
        &mut self,
        goal: UPtr<crate::bindings::engine::AActor>,
        acceptance_radius: f32,
        b_stop_on_overlap: bool,
        b_use_pathfinding: bool,
        b_can_strafe: bool,
        filter_class: TSubclassOf<
            crate::bindings::navigation_system::UNavigationQueryFilter,
        >,
        b_allow_partial_path: bool,
    ) -> EPathFollowingRequestResult {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS.aai_controller_move_to_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &acceptance_radius,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_stop_on_overlap,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_pathfinding,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_can_strafe,
                __buffer.add(14).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filter_class,
                __buffer
                    .add(16)
                    .cast::<
                        TSubclassOf<
                            crate::bindings::navigation_system::UNavigationQueryFilter,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_allow_partial_path,
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
                crate::bindings::ai_module::__FUNCTION_PTRS.aai_controller_move_to_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<EPathFollowingRequestResult>().read() }
    }
    pub fn set_focus(&mut self, new_focus: UPtr<crate::bindings::engine::AActor>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS.aai_controller_k2_set_focus,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_focus,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS.aai_controller_k2_set_focus,
                __buffer,
            )
        };
    }
    pub fn set_focal_point(&mut self, fp: crate::bindings::core_u_object::FVector) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_k2_set_focal_point,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fp,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_k2_set_focal_point,
                __buffer,
            )
        };
    }
    pub fn clear_focus(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_k2_clear_focus,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_k2_clear_focus,
                __buffer,
            )
        };
    }
    pub fn has_partial_path(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_has_partial_path,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_has_partial_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_path_following_component(&self) -> UPtr<UPathFollowingComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_path_following_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_path_following_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UPathFollowingComponent>>().read() }
    }
    pub fn get_move_status(&self) -> EPathFollowingStatus {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_move_status,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_move_status,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EPathFollowingStatus>().read() }
    }
    pub fn get_immediate_move_destination(
        &self,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_immediate_move_destination,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_immediate_move_destination,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_focus_actor(&self) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_focus_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_focus_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
    pub fn get_focal_point_on_actor(
        &self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_focal_point_on_actor,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_focal_point_on_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_focal_point(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_focal_point,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_focal_point,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_ai_perception_component(&mut self) -> UPtr<UAIPerceptionComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_ai_perception_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_get_ai_perception_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UAIPerceptionComponent>>().read() }
    }
    pub fn claim_task_resource(
        &mut self,
        resource_class: TSubclassOf<
            crate::bindings::gameplay_tasks::UGameplayTaskResource,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_claim_task_resource,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &resource_class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<
                            crate::bindings::gameplay_tasks::UGameplayTaskResource,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .aai_controller_claim_task_resource,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAIResource_Movement {
    __padding_end: [u8; 64],
}
impl UAIResource_Movement {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResource_Movement")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResource_Movement")
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
pub struct UAIResource_Logic {
    __padding_end: [u8; 64],
}
impl UAIResource_Logic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResource_Logic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIResource_Logic")
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
pub struct UAISubsystem {
    __padding_end: [u8; 64],
}
impl UAISubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISubsystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISubsystem")
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
pub struct UAISystem {
    __padding_end: [u8; 472],
}
impl UAISystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UAISystem").copied()
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
pub struct UBehaviorTree {
    __padding_end: [u8; 136],
}
impl UBehaviorTree {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTree")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTree")
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
pub struct UBrainComponent {
    __padding_end: [u8; 328],
}
impl UBrainComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrainComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrainComponent")
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
    pub fn stop_logic(&mut self, reason: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS.u_brain_component_stop_logic,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&reason, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS.u_brain_component_stop_logic,
                __buffer,
            )
        };
    }
    pub fn start_logic(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_brain_component_start_logic,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_brain_component_start_logic,
                __buffer,
            )
        };
    }
    pub fn restart_logic(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_brain_component_restart_logic,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_brain_component_restart_logic,
                __buffer,
            )
        };
    }
    pub fn is_running(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS.u_brain_component_is_running,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS.u_brain_component_is_running,
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
                crate::bindings::ai_module::__FUNCTION_PTRS.u_brain_component_is_paused,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS.u_brain_component_is_paused,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBehaviorTreeComponent {
    #[doc(hidden)]
    pub(crate) __padding_848: [u8; 848],
    pub default_behavior_tree_asset: UPtr<UBehaviorTree>,
    __padding_end: [u8; 32],
}
impl UBehaviorTreeComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeComponent")
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
    pub fn set_dynamic_subtree(
        &mut self,
        inject_tag: crate::bindings::gameplay_tags::FGameplayTag,
        behavior_asset: UPtr<UBehaviorTree>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_behavior_tree_component_set_dynamic_subtree,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &inject_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_asset,
                __buffer.add(16).cast::<UPtr<UBehaviorTree>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_behavior_tree_component_set_dynamic_subtree,
                __buffer,
            )
        };
    }
    pub fn get_tag_cooldown_end_time(
        &self,
        cooldown_tag: crate::bindings::gameplay_tags::FGameplayTag,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_behavior_tree_component_get_tag_cooldown_end_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cooldown_tag,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_behavior_tree_component_get_tag_cooldown_end_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f64>().read() }
    }
    pub fn add_cooldown_tag_duration(
        &mut self,
        cooldown_tag: crate::bindings::gameplay_tags::FGameplayTag,
        cooldown_duration: f32,
        b_add_to_existing_duration: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_behavior_tree_component_add_cooldown_tag_duration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cooldown_tag,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cooldown_duration,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_to_existing_duration,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_behavior_tree_component_add_cooldown_tag_duration,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBehaviorTreeManager {
    __padding_end: [u8; 88],
}
impl UBehaviorTreeManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeManager")
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
pub struct UBehaviorTreeTypes {
    __padding_end: [u8; 48],
}
impl UBehaviorTreeTypes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeTypes")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBehaviorTreeTypes")
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
pub struct IBlackboardAssetProvider {}
#[repr(C, align(8))]
pub struct UBlackboardAssetProvider {
    __padding_end: [u8; 48],
}
impl UBlackboardAssetProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardAssetProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardAssetProvider")
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
    pub fn get_blackboard_asset(&self) -> UPtr<UBlackboardData> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_asset_provider_get_blackboard_asset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_asset_provider_get_blackboard_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UBlackboardData>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBlackboardComponent {
    __padding_end: [u8; 504],
}
impl UBlackboardComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardComponent")
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
    pub fn set_value_as_vector(
        &mut self,
        key_name: &FName,
        vector_value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vector_value,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_vector,
                __buffer,
            )
        };
    }
    pub fn set_value_as_string(&mut self, key_name: &FName, string_value: FString) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &string_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_string,
                __buffer,
            )
        };
    }
    pub fn set_value_as_rotator(
        &mut self,
        key_name: &FName,
        vector_value: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vector_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_rotator,
                __buffer,
            )
        };
    }
    pub fn set_value_as_object(
        &mut self,
        key_name: &FName,
        object_value: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_value,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_object,
                __buffer,
            )
        };
    }
    pub fn set_value_as_name(&mut self, key_name: &FName, name_value: FName) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &name_value,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_name,
                __buffer,
            )
        };
    }
    pub fn set_value_as_int(&mut self, key_name: &FName, int_value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&int_value, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_int,
                __buffer,
            )
        };
    }
    pub fn set_value_as_float(&mut self, key_name: &FName, float_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &float_value,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_float,
                __buffer,
            )
        };
    }
    pub fn set_value_as_enum(&mut self, key_name: &FName, enum_value: u8) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&enum_value, __buffer.add(12).cast::<u8>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_enum,
                __buffer,
            )
        };
    }
    pub fn set_value_as_class(
        &mut self,
        key_name: &FName,
        class_value: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_value,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_class,
                __buffer,
            )
        };
    }
    pub fn set_value_as_bool(&mut self, key_name: &FName, bool_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bool_value,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_set_value_as_bool,
                __buffer,
            )
        };
    }
    pub fn is_vector_value_set(&self, key_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_is_vector_value_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_is_vector_value_set,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_value_as_vector(
        &self,
        key_name: &FName,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_value_as_string(&self, key_name: &FName) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_value_as_rotator(
        &self,
        key_name: &FName,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_rotator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_value_as_object(
        &self,
        key_name: &FName,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_value_as_name(&self, key_name: &FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_value_as_int(&self, key_name: &FName) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_value_as_float(&self, key_name: &FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn get_value_as_enum(&self, key_name: &FName) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_enum,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<u8>().read() }
    }
    pub fn get_value_as_class(
        &self,
        key_name: &FName,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_value_as_bool(&self, key_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_value_as_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_rotation_from_entry(
        &self,
        key_name: &FName,
        result_rotation: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_rotation_from_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result_rotation,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_rotation_from_entry,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(result_rotation);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_location_from_entry(
        &self,
        key_name: &FName,
        result_location: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_location_from_entry,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                result_location,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_get_location_from_entry,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(result_location);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn clear_value(&mut self, key_name: &FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_clear_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(key_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_blackboard_component_clear_value,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBlackboardData {
    __padding_end: [u8; 104],
}
impl UBlackboardData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardData")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardData")
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
pub struct UBlackboardKeyType {
    __padding_end: [u8; 56],
}
impl UBlackboardKeyType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType")
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
pub struct UBlackboardKeyType_Bool {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Bool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Bool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Bool")
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
pub struct UBlackboardKeyType_Class {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Class {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Class")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Class")
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
pub struct UBlackboardKeyType_Enum {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Enum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Enum")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Enum")
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
pub struct UBlackboardKeyType_Float {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Float {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Float")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Float")
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
pub struct UBlackboardKeyType_Int {
    __padding_end: [u8; 64],
}
impl UBlackboardKeyType_Int {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Int")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Int")
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
pub struct UBlackboardKeyType_Name {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Name {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Name")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Name")
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
pub struct UBlackboardKeyType_NativeEnum {
    __padding_end: [u8; 80],
}
impl UBlackboardKeyType_NativeEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_NativeEnum")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_NativeEnum")
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
pub struct UBlackboardKeyType_Object {
    __padding_end: [u8; 72],
}
impl UBlackboardKeyType_Object {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Object")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Object")
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
pub struct UBlackboardKeyType_Rotator {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Rotator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Rotator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Rotator")
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
pub struct UBlackboardKeyType_String {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_String {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_String")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_String")
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
pub struct UBlackboardKeyType_Struct {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Struct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Struct")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Struct")
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
pub struct UBlackboardKeyType_Vector {
    __padding_end: [u8; 88],
}
impl UBlackboardKeyType_Vector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Vector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlackboardKeyType_Vector")
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
pub struct UBTNode {
    __padding_end: [u8; 104],
}
impl UBTNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UBTNode").unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UBTNode").copied()
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
pub struct UBTAuxiliaryNode {
    __padding_end: [u8; 112],
}
impl UBTAuxiliaryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTAuxiliaryNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTAuxiliaryNode")
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
pub struct UBTCompositeNode {
    __padding_end: [u8; 144],
}
impl UBTCompositeNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTCompositeNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTCompositeNode")
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
pub struct UBTDecorator {
    __padding_end: [u8; 120],
}
impl UBTDecorator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator")
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
pub struct UBTFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBTFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTFunctionLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTFunctionLibrary")
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
    pub fn stop_using_external_event(node_owner: UPtr<UBTNode>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_stop_using_external_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_stop_using_external_event,
                __buffer,
            )
        };
    }
    pub fn start_using_external_event(
        node_owner: UPtr<UBTNode>,
        owning_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_start_using_external_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owning_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_start_using_external_event,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_vector(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_vector,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_string(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(56).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_string,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_rotator(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_rotator,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_object(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(56).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_object,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_name(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<68>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(56).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_name,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_int(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(56).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_int,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_float(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(56).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_float,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_enum(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: u8,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(56).cast::<u8>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_enum,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_class(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer
                    .add(56)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_class,
                __buffer,
            )
        };
    }
    pub fn set_blackboard_value_as_bool(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
        value: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(56).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_set_blackboard_value_as_bool,
                __buffer,
            )
        };
    }
    pub fn get_owners_blackboard(
        node_owner: UPtr<UBTNode>,
    ) -> UPtr<UBlackboardComponent> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_owners_blackboard,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_owners_blackboard,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UBlackboardComponent>>().read() }
    }
    pub fn get_owner_component(
        node_owner: UPtr<UBTNode>,
    ) -> UPtr<UBehaviorTreeComponent> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_owner_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_owner_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UBehaviorTreeComponent>>().read() }
    }
    pub fn get_blackboard_value_as_vector(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_blackboard_value_as_string(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FString>().read() }
    }
    pub fn get_blackboard_value_as_rotator(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_rotator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_blackboard_value_as_object(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_blackboard_value_as_name(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<68>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FName>().read() }
    }
    pub fn get_blackboard_value_as_int(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<i32>().read() }
    }
    pub fn get_blackboard_value_as_float(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<f32>().read() }
    }
    pub fn get_blackboard_value_as_enum(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_enum,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<u8>().read() }
    }
    pub fn get_blackboard_value_as_class(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_blackboard_value_as_bool(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn get_blackboard_value_as_actor(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_get_blackboard_value_as_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn clear_blackboard_value_as_vector(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_clear_blackboard_value_as_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_clear_blackboard_value_as_vector,
                __buffer,
            )
        };
    }
    pub fn clear_blackboard_value(
        node_owner: UPtr<UBTNode>,
        key: &FBlackboardKeySelector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_clear_blackboard_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_owner,
                __buffer.add(0).cast::<UPtr<UBTNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                key,
                __buffer.add(8).cast::<FBlackboardKeySelector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UBTFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_function_library_clear_blackboard_value,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBTService {
    __padding_end: [u8; 128],
}
impl UBTService {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService")
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
pub struct UBTTaskNode {
    __padding_end: [u8; 128],
}
impl UBTTaskNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTaskNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTaskNode")
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
pub struct UBTComposite_Selector {
    __padding_end: [u8; 144],
}
impl UBTComposite_Selector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_Selector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_Selector")
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
pub struct UBTComposite_Sequence {
    __padding_end: [u8; 144],
}
impl UBTComposite_Sequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_Sequence")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_Sequence")
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
pub struct UBTComposite_SimpleParallel {
    __padding_end: [u8; 152],
}
impl UBTComposite_SimpleParallel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_SimpleParallel")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTComposite_SimpleParallel")
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
pub struct UBTDecorator_BlackboardBase {
    __padding_end: [u8; 168],
}
impl UBTDecorator_BlackboardBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_BlackboardBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_BlackboardBase")
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
pub struct UBTDecorator_Blackboard {
    __padding_end: [u8; 216],
}
impl UBTDecorator_Blackboard {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Blackboard")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Blackboard")
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
pub struct UBTDecorator_BlueprintBase {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTDecorator_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_BlueprintBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_BlueprintBase")
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
    pub fn receive_tick_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
        delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_tick_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_tick_ai,
                __buffer,
            )
        };
    }
    pub fn receive_tick(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
        delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_tick,
                __buffer,
            )
        };
    }
    pub fn receive_observer_deactivated_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_deactivated_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_deactivated_ai,
                __buffer,
            )
        };
    }
    pub fn receive_observer_deactivated(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_deactivated,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_deactivated,
                __buffer,
            )
        };
    }
    pub fn receive_observer_activated_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_activated_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_activated_ai,
                __buffer,
            )
        };
    }
    pub fn receive_observer_activated(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_activated,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_observer_activated,
                __buffer,
            )
        };
    }
    pub fn receive_execution_start_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_start_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_start_ai,
                __buffer,
            )
        };
    }
    pub fn receive_execution_start(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_start,
                __buffer,
            )
        };
    }
    pub fn receive_execution_finish_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
        node_result: EBTNodeResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_finish_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_result,
                __buffer.add(16).cast::<EBTNodeResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_finish_ai,
                __buffer,
            )
        };
    }
    pub fn receive_execution_finish(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
        node_result: EBTNodeResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_finish,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_result,
                __buffer.add(8).cast::<EBTNodeResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_receive_execution_finish,
                __buffer,
            )
        };
    }
    pub fn perform_condition_check_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_perform_condition_check_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_perform_condition_check_ai,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn perform_condition_check(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_perform_condition_check,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_perform_condition_check,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_decorator_observer_active(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_is_decorator_observer_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_is_decorator_observer_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_decorator_execution_active(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_is_decorator_execution_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_decorator_blueprint_base_is_decorator_execution_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBTDecorator_CheckGameplayTagsOnActor {
    __padding_end: [u8; 224],
}
impl UBTDecorator_CheckGameplayTagsOnActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_CheckGameplayTagsOnActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_CheckGameplayTagsOnActor")
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
pub struct UBTDecorator_CompareBBEntries {
    __padding_end: [u8; 224],
}
impl UBTDecorator_CompareBBEntries {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_CompareBBEntries")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_CompareBBEntries")
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
pub struct UBTDecorator_ConditionalLoop {
    __padding_end: [u8; 216],
}
impl UBTDecorator_ConditionalLoop {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ConditionalLoop")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ConditionalLoop")
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
pub struct UBTDecorator_ConeCheck {
    __padding_end: [u8; 296],
}
impl UBTDecorator_ConeCheck {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ConeCheck")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ConeCheck")
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
pub struct UBTDecorator_Cooldown {
    __padding_end: [u8; 152],
}
impl UBTDecorator_Cooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Cooldown")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Cooldown")
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
pub struct UBTDecorator_DoesPathExist {
    __padding_end: [u8; 320],
}
impl UBTDecorator_DoesPathExist {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_DoesPathExist")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_DoesPathExist")
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
pub struct UBTDecorator_ForceSuccess {
    __padding_end: [u8; 120],
}
impl UBTDecorator_ForceSuccess {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ForceSuccess")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ForceSuccess")
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
pub struct UBTDecorator_IsAtLocation {
    __padding_end: [u8; 312],
}
impl UBTDecorator_IsAtLocation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_IsAtLocation")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_IsAtLocation")
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
pub struct UBTDecorator_IsBBEntryOfClass {
    __padding_end: [u8; 208],
}
impl UBTDecorator_IsBBEntryOfClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_IsBBEntryOfClass")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_IsBBEntryOfClass")
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
pub struct UBTDecorator_KeepInCone {
    __padding_end: [u8; 256],
}
impl UBTDecorator_KeepInCone {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_KeepInCone")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_KeepInCone")
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
pub struct UBTDecorator_Loop {
    __padding_end: [u8; 192],
}
impl UBTDecorator_Loop {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Loop")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_Loop")
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
pub struct UBTDecorator_LoopUntil {
    __padding_end: [u8; 176],
}
impl UBTDecorator_LoopUntil {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_LoopUntil")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_LoopUntil")
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
pub struct UBTDecorator_ReachedMoveGoal {
    __padding_end: [u8; 120],
}
impl UBTDecorator_ReachedMoveGoal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ReachedMoveGoal")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_ReachedMoveGoal")
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
pub struct UBTDecorator_SetTagCooldown {
    __padding_end: [u8; 200],
}
impl UBTDecorator_SetTagCooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_SetTagCooldown")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_SetTagCooldown")
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
pub struct UBTDecorator_TagCooldown {
    __padding_end: [u8; 232],
}
impl UBTDecorator_TagCooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_TagCooldown")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_TagCooldown")
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
pub struct UBTDecorator_TimeLimit {
    __padding_end: [u8; 152],
}
impl UBTDecorator_TimeLimit {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_TimeLimit")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTDecorator_TimeLimit")
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
pub struct UBTService_BlackboardBase {
    __padding_end: [u8; 176],
}
impl UBTService_BlackboardBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_BlackboardBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_BlackboardBase")
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
pub struct UBTService_BlueprintBase {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTService_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_BlueprintBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_BlueprintBase")
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
    pub fn receive_tick_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
        delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_tick_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_tick_ai,
                __buffer,
            )
        };
    }
    pub fn receive_tick(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
        delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_tick,
                __buffer,
            )
        };
    }
    pub fn receive_search_start_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_search_start_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_search_start_ai,
                __buffer,
            )
        };
    }
    pub fn receive_search_start(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_search_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_search_start,
                __buffer,
            )
        };
    }
    pub fn receive_deactivation_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_deactivation_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_deactivation_ai,
                __buffer,
            )
        };
    }
    pub fn receive_deactivation(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_deactivation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_deactivation,
                __buffer,
            )
        };
    }
    pub fn receive_activation_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_activation_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_activation_ai,
                __buffer,
            )
        };
    }
    pub fn receive_activation(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_activation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_receive_activation,
                __buffer,
            )
        };
    }
    pub fn is_service_active(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_is_service_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_service_blueprint_base_is_service_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBTService_DefaultFocus {
    __padding_end: [u8; 184],
}
impl UBTService_DefaultFocus {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_DefaultFocus")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_DefaultFocus")
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
pub struct UBTService_RunEQS {
    __padding_end: [u8; 288],
}
impl UBTService_RunEQS {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_RunEQS")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTService_RunEQS")
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
pub struct UBTTask_BlackboardBase {
    __padding_end: [u8; 176],
}
impl UBTTask_BlackboardBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_BlackboardBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_BlackboardBase")
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
pub struct UBTTask_BlueprintBase {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub custom_description: FString,
    __padding_end: [u8; 8],
}
impl UBTTask_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_BlueprintBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_BlueprintBase")
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
    pub fn set_finish_on_message_with_id(
        &mut self,
        message_name: FName,
        request_id: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_set_finish_on_message_with_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &request_id,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_set_finish_on_message_with_id,
                __buffer,
            )
        };
    }
    pub fn set_finish_on_message(&mut self, message_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_set_finish_on_message,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message_name,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_set_finish_on_message,
                __buffer,
            )
        };
    }
    pub fn receive_tick_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
        delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_tick_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_tick_ai,
                __buffer,
            )
        };
    }
    pub fn receive_tick(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
        delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_tick,
                __buffer,
            )
        };
    }
    pub fn receive_execute_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_execute_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_execute_ai,
                __buffer,
            )
        };
    }
    pub fn receive_execute(
        &mut self,
        owner_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_execute,
                __buffer,
            )
        };
    }
    pub fn receive_abort_ai(
        &mut self,
        owner_controller: UPtr<AAIController>,
        controlled_pawn: UPtr<crate::bindings::engine::APawn>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_abort_ai,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_abort_ai,
                __buffer,
            )
        };
    }
    pub fn receive_abort(&mut self, owner_actor: UPtr<crate::bindings::engine::AActor>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_abort,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_receive_abort,
                __buffer,
            )
        };
    }
    pub fn is_task_executing(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_is_task_executing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_is_task_executing,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_task_aborting(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_is_task_aborting,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_is_task_aborting,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn finish_execute(&mut self, b_success: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_finish_execute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_success, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_finish_execute,
                __buffer,
            )
        };
    }
    pub fn finish_abort(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_finish_abort,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .ubt_task_blueprint_base_finish_abort,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBTTask_FinishWithResult {
    __padding_end: [u8; 184],
}
impl UBTTask_FinishWithResult {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_FinishWithResult")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_FinishWithResult")
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
pub struct UBTTask_GameplayTaskBase {
    __padding_end: [u8; 160],
}
impl UBTTask_GameplayTaskBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_GameplayTaskBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_GameplayTaskBase")
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
pub struct UBTTask_MakeNoise {
    __padding_end: [u8; 160],
}
impl UBTTask_MakeNoise {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MakeNoise")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MakeNoise")
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
pub struct UBTTask_MoveTo {
    __padding_end: [u8; 544],
}
impl UBTTask_MoveTo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MoveTo")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MoveTo")
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
pub struct UBTTask_MoveDirectlyToward {
    __padding_end: [u8; 544],
}
impl UBTTask_MoveDirectlyToward {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MoveDirectlyToward")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_MoveDirectlyToward")
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
pub struct UBTTask_PlayAnimation {
    __padding_end: [u8; 280],
}
impl UBTTask_PlayAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_PlayAnimation")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_PlayAnimation")
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
pub struct UBTTask_PlaySound {
    __padding_end: [u8; 168],
}
impl UBTTask_PlaySound {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_PlaySound")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_PlaySound")
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
pub struct UBTTask_RotateToFaceBBEntry {
    __padding_end: [u8; 208],
}
impl UBTTask_RotateToFaceBBEntry {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RotateToFaceBBEntry")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RotateToFaceBBEntry")
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
pub struct UBTTask_RunBehavior {
    __padding_end: [u8; 136],
}
impl UBTTask_RunBehavior {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunBehavior")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunBehavior")
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
pub struct UBTTask_RunBehaviorDynamic {
    __padding_end: [u8; 160],
}
impl UBTTask_RunBehaviorDynamic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunBehaviorDynamic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunBehaviorDynamic")
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
pub struct UBTTask_RunEQSQuery {
    __padding_end: [u8; 296],
}
impl UBTTask_RunEQSQuery {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunEQSQuery")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_RunEQSQuery")
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
pub struct UBTTask_SetKeyValueBool {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueBool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueBool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueBool")
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
pub struct UBTTask_SetKeyValueClass {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueClass")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueClass")
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
pub struct UBTTask_SetKeyValueEnum {
    __padding_end: [u8; 240],
}
impl UBTTask_SetKeyValueEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueEnum")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueEnum")
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
pub struct UBTTask_SetKeyValueInt32 {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueInt32 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueInt32")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueInt32")
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
pub struct UBTTask_SetKeyValueFloat {
    __padding_end: [u8; 208],
}
impl UBTTask_SetKeyValueFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueFloat")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueFloat")
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
pub struct UBTTask_SetKeyValueName {
    __padding_end: [u8; 216],
}
impl UBTTask_SetKeyValueName {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueName")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueName")
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
pub struct UBTTask_SetKeyValueString {
    __padding_end: [u8; 216],
}
impl UBTTask_SetKeyValueString {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueString")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueString")
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
pub struct UBTTask_SetKeyValueObject {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueObject")
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
pub struct UBTTask_SetKeyValueRotator {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueRotator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueRotator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueRotator")
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
pub struct UBTTask_SetKeyValueStruct {
    __padding_end: [u8; 232],
}
impl UBTTask_SetKeyValueStruct {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueStruct")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueStruct")
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
pub struct UBTTask_SetKeyValueVector {
    __padding_end: [u8; 224],
}
impl UBTTask_SetKeyValueVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueVector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetKeyValueVector")
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
pub struct UBTTask_SetTagCooldown {
    __padding_end: [u8; 208],
}
impl UBTTask_SetTagCooldown {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetTagCooldown")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_SetTagCooldown")
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
pub struct UBTTask_Wait {
    __padding_end: [u8; 192],
}
impl UBTTask_Wait {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_Wait")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_Wait")
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
pub struct UBTTask_WaitBlackboardTime {
    __padding_end: [u8; 240],
}
impl UBTTask_WaitBlackboardTime {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_WaitBlackboardTime")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBTTask_WaitBlackboardTime")
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
pub struct UValueOrBBKeyBlueprintUtility {
    __padding_end: [u8; 48],
}
impl UValueOrBBKeyBlueprintUtility {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UValueOrBBKeyBlueprintUtility")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UValueOrBBKeyBlueprintUtility")
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
    pub fn get_vector(
        value: &FValueOrBBKey_Vector,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Vector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(48).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_struct(
        value: &FValueOrBBKey_Struct,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> crate::bindings::core_u_object::FInstancedStruct {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Struct>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(48).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_struct,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::core_u_object::FInstancedStruct>()
                .read()
        }
    }
    pub fn get_string(
        value: &FValueOrBBKey_String,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_String>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(40).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FString>().read() }
    }
    pub fn get_rotator(
        value: &FValueOrBBKey_Rotator,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Rotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(48).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_rotator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_object(
        value: &FValueOrBBKey_Object,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Object>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(40).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_name(
        value: &FValueOrBBKey_Name,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Name>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(40).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FName>().read() }
    }
    pub fn get_int32(
        value: &FValueOrBBKey_Int32,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_int32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Int32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(32).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<i32>().read() }
    }
    pub fn get_float(
        value: &FValueOrBBKey_Float,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Float>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(32).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<f32>().read() }
    }
    pub fn get_enum(
        value: &FValueOrBBKey_Enum,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Enum>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(56).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_enum,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<u8>().read() }
    }
    pub fn get_class(
        value: &FValueOrBBKey_Class,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Class>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(40).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_bool(
        value: &FValueOrBBKey_Bool,
        behavior_tree_comp: UPtr<UBehaviorTreeComponent>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<FValueOrBBKey_Bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree_comp,
                __buffer.add(32).cast::<UPtr<UBehaviorTreeComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UValueOrBBKeyBlueprintUtility::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_value_or_bb_key_blueprint_utility_get_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAIBlueprintHelperLibrary {
    __padding_end: [u8; 48],
}
impl UAIBlueprintHelperLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIBlueprintHelperLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIBlueprintHelperLibrary")
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
    pub fn unlock_ai_resources_with_animation(
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
        b_unlock_movement: bool,
        unlock_ai_logic: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_unlock_ai_resources_with_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_unlock_movement,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &unlock_ai_logic,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_unlock_ai_resources_with_animation,
                __buffer,
            )
        };
    }
    pub fn spawn_ai_from_class(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        pawn_class: TSubclassOf<crate::bindings::engine::APawn>,
        behavior_tree: UPtr<UBehaviorTree>,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        b_no_collision_fail: bool,
        owner: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<crate::bindings::engine::APawn> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_spawn_ai_from_class,
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
                &pawn_class,
                __buffer.add(8).cast::<TSubclassOf<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &behavior_tree,
                __buffer.add(16).cast::<UPtr<UBehaviorTree>>(),
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
                &rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_no_collision_fail,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &owner,
                __buffer.add(80).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_spawn_ai_from_class,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<UPtr<crate::bindings::engine::APawn>>().read() }
    }
    pub fn simple_move_to_location(
        controller: UPtr<crate::bindings::engine::AController>,
        goal: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_simple_move_to_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                goal,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_simple_move_to_location,
                __buffer,
            )
        };
    }
    pub fn simple_move_to_actor(
        controller: UPtr<crate::bindings::engine::AController>,
        goal: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_simple_move_to_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_simple_move_to_actor,
                __buffer,
            )
        };
    }
    pub fn send_ai_message(
        target: UPtr<crate::bindings::engine::APawn>,
        message: FName,
        message_source: UPtr<crate::bindings::core_u_object::UObject>,
        b_success: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_send_ai_message,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&message, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &message_source,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_success,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_send_ai_message,
                __buffer,
            )
        };
    }
    pub fn lock_ai_resources_with_animation(
        anim_instance: UPtr<crate::bindings::engine::UAnimInstance>,
        b_lock_movement: bool,
        lock_ai_logic: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_lock_ai_resources_with_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_instance,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimInstance>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_lock_movement,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &lock_ai_logic,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_lock_ai_resources_with_animation,
                __buffer,
            )
        };
    }
    pub fn is_valid_ai_rotation(
        rotation: crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_rotation,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_valid_ai_location(
        location: crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_location,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_valid_ai_direction(
        direction_vector: crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_direction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &direction_vector,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_is_valid_ai_direction,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_next_nav_link_index(
        controller: UPtr<crate::bindings::engine::AController>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_next_nav_link_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_next_nav_link_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_current_path_points(
        controller: UPtr<crate::bindings::engine::AController>,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path_points,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path_points,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_current_path_index(
        controller: UPtr<crate::bindings::engine::AController>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_current_path(
        controller: UPtr<crate::bindings::engine::AController>,
    ) -> UPtr<crate::bindings::navigation_system::UNavigationPath> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AController>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_current_path,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::navigation_system::UNavigationPath>>()
                .read()
        }
    }
    pub fn get_blackboard(
        target: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<UBlackboardComponent> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_blackboard,
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
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_blackboard,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UBlackboardComponent>>().read() }
    }
    pub fn get_ai_controller(
        controlled_actor: UPtr<crate::bindings::engine::AActor>,
    ) -> UPtr<AAIController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_ai_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controlled_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_get_ai_controller,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<AAIController>>().read() }
    }
    pub fn create_move_to_proxy_object(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        pawn: UPtr<crate::bindings::engine::APawn>,
        destination: crate::bindings::core_u_object::FVector,
        target_actor: UPtr<crate::bindings::engine::AActor>,
        acceptance_radius: f32,
        b_stop_on_overlap: bool,
    ) -> UPtr<UAIAsyncTaskBlueprintProxy> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_create_move_to_proxy_object,
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
                &pawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_actor,
                __buffer.add(40).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &acceptance_radius,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_stop_on_overlap,
                __buffer.add(52).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIBlueprintHelperLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_blueprint_helper_library_create_move_to_proxy_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<UAIAsyncTaskBlueprintProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAIDataProvider {
    __padding_end: [u8; 48],
}
impl UAIDataProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider")
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
pub struct UAIDataProvider_QueryParams {
    __padding_end: [u8; 72],
}
impl UAIDataProvider_QueryParams {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider_QueryParams")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider_QueryParams")
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
pub struct UAIDataProvider_Random {
    __padding_end: [u8; 88],
}
impl UAIDataProvider_Random {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider_Random")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIDataProvider_Random")
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
pub struct ADetourCrowdAIController {
    __padding_end: [u8; 1464],
}
impl ADetourCrowdAIController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADetourCrowdAIController")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ADetourCrowdAIController")
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
pub struct UEnvQueryContext {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext")
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
pub struct UEnvQueryContext_BlueprintBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryContext_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_BlueprintBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_BlueprintBase")
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
    pub fn provide_single_location(
        &self,
        querier_object: UPtr<crate::bindings::core_u_object::UObject>,
        querier_actor: UPtr<crate::bindings::engine::AActor>,
        resulting_location: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_single_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                resulting_location,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_single_location,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(resulting_location);
        }
    }
    pub fn provide_single_actor(
        &self,
        querier_object: UPtr<crate::bindings::core_u_object::UObject>,
        querier_actor: UPtr<crate::bindings::engine::AActor>,
        resulting_actor: &mut UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_single_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                resulting_actor,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_single_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::AActor>>()
                .swap(resulting_actor);
        }
    }
    pub fn provide_locations_set(
        &self,
        querier_object: UPtr<crate::bindings::core_u_object::UObject>,
        querier_actor: UPtr<crate::bindings::engine::AActor>,
        resulting_location_set: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_locations_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                resulting_location_set,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_locations_set,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(resulting_location_set);
        }
    }
    pub fn provide_actors_set(
        &self,
        querier_object: UPtr<crate::bindings::core_u_object::UObject>,
        querier_actor: UPtr<crate::bindings::engine::AActor>,
        resulting_actors_set: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_actors_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                resulting_actors_set,
                __buffer.add(16).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_context_blueprint_base_provide_actors_set,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(resulting_actors_set);
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryContext_Item {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext_Item {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_Item")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_Item")
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
pub struct UEnvQueryContext_NavigationData {
    __padding_end: [u8; 112],
}
impl UEnvQueryContext_NavigationData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_NavigationData")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_NavigationData")
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
pub struct UEnvQueryContext_Querier {
    __padding_end: [u8; 48],
}
impl UEnvQueryContext_Querier {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_Querier")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryContext_Querier")
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
pub struct UEnvQuery {
    __padding_end: [u8; 96],
}
impl UEnvQuery {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQuery")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UEnvQuery").copied()
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
pub struct UEnvQueryDebugHelpers {
    __padding_end: [u8; 48],
}
impl UEnvQueryDebugHelpers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryDebugHelpers")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryDebugHelpers")
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
pub struct UEnvQueryNode {
    __padding_end: [u8; 56],
}
impl UEnvQueryNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryNode")
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
pub struct UEnvQueryGenerator {
    __padding_end: [u8; 88],
}
impl UEnvQueryGenerator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator")
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
pub struct UEnvQueryInstanceBlueprintWrapper {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub query_id: i32,
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 32],
    pub item_type: TSubclassOf<UEnvQueryItemType>,
    pub option_index: i32,
    __padding_end: [u8; 36],
}
impl UEnvQueryInstanceBlueprintWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryInstanceBlueprintWrapper")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryInstanceBlueprintWrapper")
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
    pub fn set_named_param(&mut self, param_name: FName, value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_set_named_param,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_set_named_param,
                __buffer,
            )
        };
    }
    pub fn get_results_as_locations(
        &self,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_results_as_locations,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_results_as_locations,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_results_as_actors(
        &self,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_results_as_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_results_as_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_query_results_as_locations(
        &self,
        result_locations: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_query_results_as_locations,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result_locations,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_query_results_as_locations,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(result_locations);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_query_results_as_actors(
        &self,
        result_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_query_results_as_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                result_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_query_results_as_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(result_actors);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_item_score(&self, item_index: i32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_item_score,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&item_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_instance_blueprint_wrapper_get_item_score,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryManager {
    __padding_end: [u8; 528],
}
impl UEnvQueryManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryManager")
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
    pub fn run_eqs_query(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        query_template: UPtr<UEnvQuery>,
        querier: UPtr<crate::bindings::core_u_object::UObject>,
        run_mode: EEnvQueryRunMode,
        wrapper_class: TSubclassOf<UEnvQueryInstanceBlueprintWrapper>,
    ) -> UPtr<UEnvQueryInstanceBlueprintWrapper> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_manager_run_eqs_query,
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
                &query_template,
                __buffer.add(8).cast::<UPtr<UEnvQuery>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &querier,
                __buffer.add(16).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &run_mode,
                __buffer.add(24).cast::<EEnvQueryRunMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &wrapper_class,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<UEnvQueryInstanceBlueprintWrapper>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UEnvQueryManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_manager_run_eqs_query,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<UPtr<UEnvQueryInstanceBlueprintWrapper>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryOption {
    __padding_end: [u8; 72],
}
impl UEnvQueryOption {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryOption")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryOption")
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
pub struct UEnvQueryTest {
    __padding_end: [u8; 608],
}
impl UEnvQueryTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest")
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
pub struct UEnvQueryTypes {
    __padding_end: [u8; 48],
}
impl UEnvQueryTypes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTypes")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTypes")
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
pub struct UEQSRenderingComponent {
    __padding_end: [u8; 1792],
}
impl UEQSRenderingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEQSRenderingComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEQSRenderingComponent")
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
pub struct AEQSTestingPawn {
    __padding_end: [u8; 2288],
}
impl AEQSTestingPawn {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEQSTestingPawn")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AEQSTestingPawn")
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
pub struct UEnvQueryGenerator_ActorsOfClass {
    __padding_end: [u8; 232],
}
impl UEnvQueryGenerator_ActorsOfClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_ActorsOfClass")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_ActorsOfClass")
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
pub struct UEnvQueryGenerator_BlueprintBase {
    __padding_end: [u8; 136],
}
impl UEnvQueryGenerator_BlueprintBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_BlueprintBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_BlueprintBase")
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
    pub fn get_querier(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_get_querier,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_get_querier,
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
    pub fn do_item_generation_from_actors(
        &self,
        context_actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_do_item_generation_from_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_do_item_generation_from_actors,
                __buffer,
            )
        };
    }
    pub fn do_item_generation(
        &self,
        context_locations: &TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_do_item_generation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                context_locations,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_do_item_generation,
                __buffer,
            )
        };
    }
    pub fn add_generated_vector(
        &self,
        generated_vector: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_add_generated_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &generated_vector,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_add_generated_vector,
                __buffer,
            )
        };
    }
    pub fn add_generated_actor(
        &self,
        generated_actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_add_generated_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &generated_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_env_query_generator_blueprint_base_add_generated_actor,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEnvQueryGenerator_Composite {
    __padding_end: [u8; 120],
}
impl UEnvQueryGenerator_Composite {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Composite")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Composite")
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
pub struct UEnvQueryGenerator_ProjectedPoints {
    __padding_end: [u8; 160],
}
impl UEnvQueryGenerator_ProjectedPoints {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_ProjectedPoints")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_ProjectedPoints")
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
pub struct UEnvQueryGenerator_Cone {
    __padding_end: [u8; 432],
}
impl UEnvQueryGenerator_Cone {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Cone")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Cone")
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
pub struct UEnvQueryGenerator_CurrentLocation {
    __padding_end: [u8; 168],
}
impl UEnvQueryGenerator_CurrentLocation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_CurrentLocation")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_CurrentLocation")
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
pub struct UEnvQueryGenerator_Donut {
    __padding_end: [u8; 536],
}
impl UEnvQueryGenerator_Donut {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Donut")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_Donut")
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
pub struct UEnvQueryGenerator_OnCircle {
    __padding_end: [u8; 672],
}
impl UEnvQueryGenerator_OnCircle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_OnCircle")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_OnCircle")
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
pub struct UEnvQueryGenerator_SimpleGrid {
    __padding_end: [u8; 296],
}
impl UEnvQueryGenerator_SimpleGrid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_SimpleGrid")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_SimpleGrid")
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
pub struct UEnvQueryGenerator_PathingGrid {
    __padding_end: [u8; 432],
}
impl UEnvQueryGenerator_PathingGrid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_PathingGrid")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_PathingGrid")
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
pub struct UEnvQueryGenerator_PerceivedActors {
    __padding_end: [u8; 184],
}
impl UEnvQueryGenerator_PerceivedActors {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_PerceivedActors")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryGenerator_PerceivedActors")
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
pub struct UEnvQueryItemType {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType")
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
pub struct UEnvQueryItemType_VectorBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_VectorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_VectorBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_VectorBase")
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
pub struct UEnvQueryItemType_ActorBase {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_ActorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_ActorBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_ActorBase")
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
pub struct UEnvQueryItemType_Actor {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Actor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Actor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Actor")
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
pub struct UEnvQueryItemType_Direction {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Direction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Direction")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Direction")
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
pub struct UEnvQueryItemType_Point {
    __padding_end: [u8; 56],
}
impl UEnvQueryItemType_Point {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Point")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryItemType_Point")
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
pub struct UEnvQueryTest_Distance {
    __padding_end: [u8; 624],
}
impl UEnvQueryTest_Distance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Distance")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Distance")
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
pub struct UEnvQueryTest_Dot {
    __padding_end: [u8; 680],
}
impl UEnvQueryTest_Dot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Dot")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Dot")
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
pub struct UEnvQueryTest_GameplayTags {
    __padding_end: [u8; 720],
}
impl UEnvQueryTest_GameplayTags {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_GameplayTags")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_GameplayTags")
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
pub struct UEnvQueryTest_Overlap {
    __padding_end: [u8; 656],
}
impl UEnvQueryTest_Overlap {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Overlap")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Overlap")
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
pub struct UEnvQueryTest_Pathfinding {
    __padding_end: [u8; 760],
}
impl UEnvQueryTest_Pathfinding {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Pathfinding")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Pathfinding")
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
pub struct UEnvQueryTest_PathfindingBatch {
    __padding_end: [u8; 824],
}
impl UEnvQueryTest_PathfindingBatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_PathfindingBatch")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_PathfindingBatch")
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
pub struct UEnvQueryTest_Project {
    __padding_end: [u8; 672],
}
impl UEnvQueryTest_Project {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Project")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Project")
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
pub struct UEnvQueryTest_Random {
    __padding_end: [u8; 608],
}
impl UEnvQueryTest_Random {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Random")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Random")
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
pub struct UEnvQueryTest_Trace {
    __padding_end: [u8; 872],
}
impl UEnvQueryTest_Trace {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Trace")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Trace")
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
pub struct UEnvQueryTest_Volume {
    __padding_end: [u8; 632],
}
impl UEnvQueryTest_Volume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Volume")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvQueryTest_Volume")
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
pub struct AGridPathAIController {
    __padding_end: [u8; 1464],
}
impl AGridPathAIController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGridPathAIController")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGridPathAIController")
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
pub struct UAIHotSpotManager {
    __padding_end: [u8; 48],
}
impl UAIHotSpotManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIHotSpotManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIHotSpotManager")
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
pub struct UPathFollowingComponent {
    __padding_end: [u8; 888],
}
impl UPathFollowingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathFollowingComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathFollowingComponent")
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
    pub fn get_path_destination(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_path_following_component_get_path_destination,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_path_following_component_get_path_destination,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_path_action_type(&self) -> EPathFollowingAction {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_path_following_component_get_path_action_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_path_following_component_get_path_action_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EPathFollowingAction>().read() }
    }
}
#[repr(C, align(8))]
pub struct UCrowdFollowingComponent {
    __padding_end: [u8; 984],
}
impl UCrowdFollowingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdFollowingComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdFollowingComponent")
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
    pub fn suspend_crowd_steering(&mut self, b_suspend: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_crowd_following_component_suspend_crowd_steering,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_suspend, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_crowd_following_component_suspend_crowd_steering,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UCrowdManager {
    __padding_end: [u8; 256],
}
impl UCrowdManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrowdManager")
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
pub struct UGeneratedNavLinksProxy {
    __padding_end: [u8; 96],
}
impl UGeneratedNavLinksProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeneratedNavLinksProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGeneratedNavLinksProxy")
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
    pub fn receive_smart_link_reached(
        &mut self,
        agent: UPtr<crate::bindings::engine::AActor>,
        destination: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_generated_nav_links_proxy_receive_smart_link_reached,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &agent,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_generated_nav_links_proxy_receive_smart_link_reached,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGridPathFollowingComponent {
    __padding_end: [u8; 936],
}
impl UGridPathFollowingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGridPathFollowingComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGridPathFollowingComponent")
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
pub struct UNavFilter_AIControllerDefault {
    __padding_end: [u8; 80],
}
impl UNavFilter_AIControllerDefault {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavFilter_AIControllerDefault")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavFilter_AIControllerDefault")
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
pub struct ANavLinkProxy {
    #[doc(hidden)]
    pub(crate) __padding_1152: [u8; 1152],
    pub point_links: TArray<crate::bindings::engine::FNavigationLink>,
    __padding_end: [u8; 88],
}
impl ANavLinkProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavLinkProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANavLinkProxy")
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
    pub fn set_smart_link_enabled(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_set_smart_link_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_set_smart_link_enabled,
                __buffer,
            )
        };
    }
    pub fn resume_path_following(
        &mut self,
        agent: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_resume_path_following,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &agent,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_resume_path_following,
                __buffer,
            )
        };
    }
    pub fn receive_smart_link_reached(
        &mut self,
        agent: UPtr<crate::bindings::engine::AActor>,
        destination: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_receive_smart_link_reached,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &agent,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                destination,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_receive_smart_link_reached,
                __buffer,
            )
        };
    }
    pub fn is_smart_link_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_is_smart_link_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_is_smart_link_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_moving_agents(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_has_moving_agents,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .a_nav_link_proxy_has_moving_agents,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNavLocalGridManager {
    __padding_end: [u8; 96],
}
impl UNavLocalGridManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLocalGridManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavLocalGridManager")
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
    pub fn set_local_navigation_grid_density(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        cell_size: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_set_local_navigation_grid_density,
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
            std::ptr::copy_nonoverlapping(&cell_size, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UNavLocalGridManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_set_local_navigation_grid_density,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn remove_local_navigation_grid(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        grid_id: i32,
        b_rebuild_grids: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_remove_local_navigation_grid,
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
            std::ptr::copy_nonoverlapping(&grid_id, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_grids,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UNavLocalGridManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_remove_local_navigation_grid,
                __buffer,
            )
        };
    }
    pub fn find_local_navigation_grid_path(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        start: &crate::bindings::core_u_object::FVector,
        end: &crate::bindings::core_u_object::FVector,
        path_points: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_find_local_navigation_grid_path,
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
                start,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                end,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                path_points,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UNavLocalGridManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_find_local_navigation_grid_path,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(path_points);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn add_local_navigation_grid_for_points(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        locations: &TArray<crate::bindings::core_u_object::FVector>,
        radius2_d: i32,
        height: f32,
        b_rebuild_grids: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_points,
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
                locations,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius2_d, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&height, __buffer.add(28).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_grids,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UNavLocalGridManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_points,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<i32>().read() }
    }
    pub fn add_local_navigation_grid_for_point(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        location: &crate::bindings::core_u_object::FVector,
        radius2_d: i32,
        height: f32,
        b_rebuild_grids: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_point,
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
                location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius2_d, __buffer.add(32).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&height, __buffer.add(36).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_grids,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UNavLocalGridManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_point,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<i32>().read() }
    }
    pub fn add_local_navigation_grid_for_capsule(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        location: &crate::bindings::core_u_object::FVector,
        capsule_radius: f32,
        capsule_half_height: f32,
        radius2_d: i32,
        height: f32,
        b_rebuild_grids: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_capsule,
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
                location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capsule_radius,
                __buffer.add(32).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capsule_half_height,
                __buffer.add(36).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius2_d, __buffer.add(40).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&height, __buffer.add(44).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_grids,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UNavLocalGridManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_capsule,
                __buffer,
            )
        };
        unsafe { __buffer.add(52).cast::<i32>().read() }
    }
    pub fn add_local_navigation_grid_for_box(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        location: &crate::bindings::core_u_object::FVector,
        extent: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        radius2_d: i32,
        height: f32,
        b_rebuild_grids: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_box,
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
                location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &extent,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&radius2_d, __buffer.add(80).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&height, __buffer.add(84).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rebuild_grids,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UNavLocalGridManager::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_nav_local_grid_manager_add_local_navigation_grid_for_box,
                __buffer,
            )
        };
        unsafe { __buffer.add(92).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPathFollowingManager {
    __padding_end: [u8; 48],
}
impl UPathFollowingManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathFollowingManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathFollowingManager")
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
pub struct UAIPerceptionComponent {
    __padding_end: [u8; 512],
}
impl UAIPerceptionComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionComponent")
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
    pub fn set_sense_enabled(
        &mut self,
        sense_class: TSubclassOf<UAISense>,
        b_enable: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_set_sense_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sense_class,
                __buffer.add(0).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enable, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_set_sense_enabled,
                __buffer,
            )
        };
    }
    pub fn request_stimuli_listener_update(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_request_stimuli_listener_update,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_request_stimuli_listener_update,
                __buffer,
            )
        };
    }
    pub fn is_sense_enabled(&self, sense_class: TSubclassOf<UAISense>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_is_sense_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sense_class,
                __buffer.add(0).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_is_sense_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_perceived_hostile_actors_by_sense(
        &self,
        sense_to_use: TSubclassOf<UAISense>,
        out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_perceived_hostile_actors_by_sense,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sense_to_use,
                __buffer.add(0).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actors,
                __buffer.add(8).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_perceived_hostile_actors_by_sense,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(out_actors);
        }
    }
    pub fn get_perceived_hostile_actors(
        &self,
        out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_perceived_hostile_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_perceived_hostile_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(out_actors);
        }
    }
    pub fn get_known_perceived_actors(
        &self,
        sense_to_use: TSubclassOf<UAISense>,
        out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_known_perceived_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sense_to_use,
                __buffer.add(0).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actors,
                __buffer.add(8).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_known_perceived_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(out_actors);
        }
    }
    pub fn get_currently_perceived_actors(
        &self,
        sense_to_use: TSubclassOf<UAISense>,
        out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_currently_perceived_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sense_to_use,
                __buffer.add(0).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_actors,
                __buffer.add(8).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_currently_perceived_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(out_actors);
        }
    }
    pub fn get_actors_perception(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        info: &mut FActorPerceptionBlueprintInfo,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_actors_perception,
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
                info,
                __buffer.add(8).cast::<FActorPerceptionBlueprintInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_get_actors_perception,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FActorPerceptionBlueprintInfo>().swap(info);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn forget_all(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_forget_all,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_component_forget_all,
                __buffer,
            )
        };
    }
}
pub struct IAIPerceptionListenerInterface {}
#[repr(C, align(8))]
pub struct UAIPerceptionListenerInterface {
    __padding_end: [u8; 48],
}
impl UAIPerceptionListenerInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionListenerInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionListenerInterface")
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
pub struct UAIPerceptionStimuliSourceComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub flags_240: u8,
    pub register_as_source_for_senses: TArray<TSubclassOf<UAISense>>,
}
impl UAIPerceptionStimuliSourceComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionStimuliSourceComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionStimuliSourceComponent")
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
    pub fn unregister_from_sense(&mut self, sense_class: TSubclassOf<UAISense>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_unregister_from_sense,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sense_class,
                __buffer.add(0).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_unregister_from_sense,
                __buffer,
            )
        };
    }
    pub fn unregister_from_perception_system(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_unregister_from_perception_system,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_unregister_from_perception_system,
                __buffer,
            )
        };
    }
    pub fn register_with_perception_system(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_register_with_perception_system,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_register_with_perception_system,
                __buffer,
            )
        };
    }
    pub fn register_for_sense(&mut self, sense_class: TSubclassOf<UAISense>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_register_for_sense,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sense_class,
                __buffer.add(0).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_stimuli_source_component_register_for_sense,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAIPerceptionSystem {
    __padding_end: [u8; 336],
}
impl UAIPerceptionSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionSystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIPerceptionSystem")
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
    pub fn report_perception_event(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        perception_event: UPtr<UAISenseEvent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_report_perception_event,
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
                &perception_event,
                __buffer.add(8).cast::<UPtr<UAISenseEvent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIPerceptionSystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_report_perception_event,
                __buffer,
            )
        };
    }
    pub fn report_event(&mut self, perception_event: UPtr<UAISenseEvent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_report_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &perception_event,
                __buffer.add(0).cast::<UPtr<UAISenseEvent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_report_event,
                __buffer,
            )
        };
    }
    pub fn register_perception_stimuli_source(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        sense: TSubclassOf<UAISense>,
        target: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_register_perception_stimuli_source,
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
                &sense,
                __buffer.add(8).cast::<TSubclassOf<UAISense>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIPerceptionSystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_register_perception_stimuli_source,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_sense_class_for_stimulus(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        stimulus: &FAIStimulus,
    ) -> TSubclassOf<UAISense> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_get_sense_class_for_stimulus,
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
                stimulus,
                __buffer.add(8).cast::<FAIStimulus>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAIPerceptionSystem::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_perception_system_get_sense_class_for_stimulus,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<TSubclassOf<UAISense>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAISense {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub notify_type: EAISenseNotifyType,
    #[doc(hidden)]
    pub(crate) __padding_52: [u8; 3],
    pub flags_52: u8,
    __padding_end: [u8; 107],
}
impl UAISense {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UAISense").unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UAISense").copied()
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
pub struct UAISenseConfig_Damage {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub implementation: TSubclassOf<UAISense_Damage>,
}
impl UAISenseConfig_Damage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Damage")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISenseConfig_Damage")
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
pub struct UAISense_Blueprint {
    #[doc(hidden)]
    pub(crate) __padding_160: [u8; 160],
    pub listener_data_type: TSubclassOf<
        crate::bindings::core_u_object::UUserDefinedStruct,
    >,
    pub listener_container: TArray<UPtr<UAIPerceptionComponent>>,
    __padding_end: [u8; 16],
}
impl UAISense_Blueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Blueprint")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Blueprint")
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
    pub fn on_update(&mut self, events_to_process: &TArray<UPtr<UAISenseEvent>>) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_update,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                events_to_process,
                __buffer.add(0).cast::<TArray<UPtr<UAISenseEvent>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_update,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn on_listener_updated(
        &mut self,
        actor_listener: UPtr<crate::bindings::engine::AActor>,
        perception_component: UPtr<UAIPerceptionComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_listener_updated,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_listener,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &perception_component,
                __buffer.add(8).cast::<UPtr<UAIPerceptionComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_listener_updated,
                __buffer,
            )
        };
    }
    pub fn on_listener_unregistered(
        &mut self,
        actor_listener: UPtr<crate::bindings::engine::AActor>,
        perception_component: UPtr<UAIPerceptionComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_listener_unregistered,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_listener,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &perception_component,
                __buffer.add(8).cast::<UPtr<UAIPerceptionComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_listener_unregistered,
                __buffer,
            )
        };
    }
    pub fn on_listener_registered(
        &mut self,
        actor_listener: UPtr<crate::bindings::engine::AActor>,
        perception_component: UPtr<UAIPerceptionComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_listener_registered,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_listener,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &perception_component,
                __buffer.add(8).cast::<UPtr<UAIPerceptionComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_on_listener_registered,
                __buffer,
            )
        };
    }
    pub fn on_new_pawn(&mut self, new_pawn: UPtr<crate::bindings::engine::APawn>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_k2_on_new_pawn,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_pawn,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_k2_on_new_pawn,
                __buffer,
            )
        };
    }
    pub fn get_all_listener_components(
        &self,
        listener_components: &mut TArray<UPtr<UAIPerceptionComponent>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_get_all_listener_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                listener_components,
                __buffer.add(0).cast::<TArray<UPtr<UAIPerceptionComponent>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_get_all_listener_components,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<UAIPerceptionComponent>>>()
                .swap(listener_components);
        }
    }
    pub fn get_all_listener_actors(
        &self,
        listener_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_get_all_listener_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                listener_actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_blueprint_get_all_listener_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(listener_actors);
        }
    }
}
#[repr(C, align(8))]
pub struct UAISense_Damage {
    __padding_end: [u8; 176],
}
impl UAISense_Damage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Damage")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Damage")
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
    pub fn report_damage_event(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        damaged_actor: UPtr<crate::bindings::engine::AActor>,
        instigator: UPtr<crate::bindings::engine::AActor>,
        damage_amount: f32,
        event_location: crate::bindings::core_u_object::FVector,
        hit_location: crate::bindings::core_u_object::FVector,
        tag: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_damage_report_damage_event,
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
                &damaged_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instigator,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &damage_amount,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &event_location,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &hit_location,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(80).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UAISense_Damage::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_damage_report_damage_event,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAISense_Hearing {
    __padding_end: [u8; 264],
}
impl UAISense_Hearing {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Hearing")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Hearing")
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
    pub fn report_noise_event(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        noise_location: crate::bindings::core_u_object::FVector,
        loudness: f32,
        instigator: UPtr<crate::bindings::engine::AActor>,
        max_range: f32,
        tag: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_hearing_report_noise_event,
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
                &noise_location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&loudness, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instigator,
                __buffer.add(40).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&max_range, __buffer.add(48).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(52).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::ai_module::UAISense_Hearing::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_hearing_report_noise_event,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAISense_Prediction {
    __padding_end: [u8; 176],
}
impl UAISense_Prediction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Prediction")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Prediction")
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
    pub fn request_pawn_prediction_event(
        requestor: UPtr<crate::bindings::engine::APawn>,
        predicted_actor: UPtr<crate::bindings::engine::AActor>,
        prediction_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_prediction_request_pawn_prediction_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requestor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::APawn>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &predicted_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &prediction_time,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAISense_Prediction::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_prediction_request_pawn_prediction_event,
                __buffer,
            )
        };
    }
    pub fn request_controller_prediction_event(
        requestor: UPtr<AAIController>,
        predicted_actor: UPtr<crate::bindings::engine::AActor>,
        prediction_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_prediction_request_controller_prediction_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requestor,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &predicted_actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &prediction_time,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAISense_Prediction::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_prediction_request_controller_prediction_event,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAISense_Sight {
    __padding_end: [u8; 488],
}
impl UAISense_Sight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Sight")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Sight")
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
pub struct UAISense_Team {
    __padding_end: [u8; 176],
}
impl UAISense_Team {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Team")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Team")
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
pub struct UAISense_Touch {
    __padding_end: [u8; 256],
}
impl UAISense_Touch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Touch")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISense_Touch")
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
    pub fn report_touch_event(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        touch_receiver: UPtr<crate::bindings::engine::AActor>,
        other_actor: UPtr<crate::bindings::engine::AActor>,
        location: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_touch_report_touch_event,
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
                &touch_receiver,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &other_actor,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>(),
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
        let __object_ptr = crate::bindings::ai_module::UAISense_Touch::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .uai_sense_touch_report_touch_event,
                __buffer,
            )
        };
    }
}
pub struct IAISightTargetInterface {}
#[repr(C, align(8))]
pub struct UAISightTargetInterface {
    __padding_end: [u8; 48],
}
impl UAISightTargetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISightTargetInterface")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAISightTargetInterface")
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
pub struct UPawnSensingComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub hearing_threshold: f32,
    pub los_hearing_threshold: f32,
    pub sight_radius: f32,
    pub sensing_interval: f32,
    pub hearing_max_sound_age: f32,
    pub flags_260: u8,
    #[doc(hidden)]
    pub(crate) __padding_320: [u8; 56],
    pub peripheral_vision_angle: f32,
}
impl UPawnSensingComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPawnSensingComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPawnSensingComponent")
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
    pub fn set_sensing_updates_enabled(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_set_sensing_updates_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_set_sensing_updates_enabled,
                __buffer,
            )
        };
    }
    pub fn set_sensing_interval(&mut self, new_sensing_interval: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_set_sensing_interval,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_sensing_interval,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_set_sensing_interval,
                __buffer,
            )
        };
    }
    pub fn set_peripheral_vision_angle(&mut self, new_peripheral_vision_angle: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_set_peripheral_vision_angle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_peripheral_vision_angle,
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
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_set_peripheral_vision_angle,
                __buffer,
            )
        };
    }
    pub fn get_peripheral_vision_cosine(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_get_peripheral_vision_cosine,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_get_peripheral_vision_cosine,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_peripheral_vision_angle(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_get_peripheral_vision_angle,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS
                    .u_pawn_sensing_component_get_peripheral_vision_angle,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAITask {
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 128],
    pub owner_controller: UPtr<AAIController>,
}
impl UAITask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UAITask").unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UAITask").copied()
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
pub struct UAITask_LockLogic {
    __padding_end: [u8; 136],
}
impl UAITask_LockLogic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_LockLogic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_LockLogic")
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
pub struct UAITask_MoveTo {
    __padding_end: [u8; 360],
}
impl UAITask_MoveTo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_MoveTo")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_MoveTo")
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
    pub fn ai_move_to(
        controller: UPtr<AAIController>,
        goal_location: crate::bindings::core_u_object::FVector,
        goal_actor: UPtr<crate::bindings::engine::AActor>,
        acceptance_radius: f32,
        stop_on_overlap: EAIOptionFlag,
        accept_partial_path: EAIOptionFlag,
        b_use_pathfinding: bool,
        b_lock_ai_logic: bool,
        b_use_continuous_goal_tracking: bool,
        project_goal_on_navigation: EAIOptionFlag,
        require_navigable_end_location: EAIOptionFlag,
    ) -> UPtr<UAITask_MoveTo> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS.uai_task_move_to_ai_move_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &goal_actor,
                __buffer.add(32).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &acceptance_radius,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stop_on_overlap,
                __buffer.add(44).cast::<EAIOptionFlag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &accept_partial_path,
                __buffer.add(45).cast::<EAIOptionFlag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_pathfinding,
                __buffer.add(46).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_lock_ai_logic,
                __buffer.add(47).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_continuous_goal_tracking,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &project_goal_on_navigation,
                __buffer.add(49).cast::<EAIOptionFlag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &require_navigable_end_location,
                __buffer.add(50).cast::<EAIOptionFlag>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAITask_MoveTo::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS.uai_task_move_to_ai_move_to,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<UAITask_MoveTo>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAITask_RunEQS {
    __padding_end: [u8; 280],
}
impl UAITask_RunEQS {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_RunEQS")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAITask_RunEQS")
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
    pub fn run_eqs(
        controller: UPtr<AAIController>,
        query_template: UPtr<UEnvQuery>,
    ) -> UPtr<UAITask_RunEQS> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::ai_module::__FUNCTION_PTRS.uai_task_run_eqs_run_eqs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &controller,
                __buffer.add(0).cast::<UPtr<AAIController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &query_template,
                __buffer.add(8).cast::<UPtr<UEnvQuery>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::ai_module::UAITask_RunEQS::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::ai_module::__FUNCTION_PTRS.uai_task_run_eqs_run_eqs,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UAITask_RunEQS>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UVisualLoggerExtension {
    __padding_end: [u8; 48],
}
impl UVisualLoggerExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVisualLoggerExtension")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVisualLoggerExtension")
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
pub struct FAIAsyncTaskBlueprintProxy_OnSuccess {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIAsyncTaskBlueprintProxy_OnFail {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIController_ReceiveMoveCompleted {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEnvQueryInstanceBlueprintWrapper_OnQueryFinishedEvent {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FGeneratedNavLinksProxy_OnSmartLinkReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FNavLinkProxy_OnSmartLinkReached {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnPerceptionUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionForgotten {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAIPerceptionComponent_OnTargetPerceptionInfoUpdated {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPawnSensingComponent_OnSeePawn {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FPawnSensingComponent_OnHearNoise {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAITask_MoveTo_OnRequestFailed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAITask_MoveTo_OnMoveFinished {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EBTDecoratorLogic(pub u8);
impl EBTDecoratorLogic {
    pub const INVALID: EBTDecoratorLogic = EBTDecoratorLogic(0);
    pub const TEST: EBTDecoratorLogic = EBTDecoratorLogic(1);
    pub const AND: EBTDecoratorLogic = EBTDecoratorLogic(2);
    pub const OR: EBTDecoratorLogic = EBTDecoratorLogic(3);
    pub const NOT: EBTDecoratorLogic = EBTDecoratorLogic(4);
}
#[repr(transparent)]
pub struct EAIParamType(pub u8);
impl EAIParamType {
    pub const FLOAT: EAIParamType = EAIParamType(0);
    pub const INT: EAIParamType = EAIParamType(1);
    pub const BOOL: EAIParamType = EAIParamType(2);
    pub const MAX: EAIParamType = EAIParamType(3);
}
#[repr(transparent)]
pub struct EEnvDirection(pub u8);
impl EEnvDirection {
    pub const TWO_POINTS: EEnvDirection = EEnvDirection(0);
    pub const ROTATION: EEnvDirection = EEnvDirection(1);
}
#[repr(transparent)]
pub struct EEnvTraceShape(pub u8);
impl EEnvTraceShape {
    pub const LINE: EEnvTraceShape = EEnvTraceShape(0);
    pub const BOX: EEnvTraceShape = EEnvTraceShape(1);
    pub const SPHERE: EEnvTraceShape = EEnvTraceShape(2);
    pub const CAPSULE: EEnvTraceShape = EEnvTraceShape(3);
}
#[repr(transparent)]
pub struct EEnvQueryTrace(pub u8);
impl EEnvQueryTrace {
    pub const NONE: EEnvQueryTrace = EEnvQueryTrace(0);
    pub const NAVIGATION: EEnvQueryTrace = EEnvQueryTrace(1);
    pub const GEOMETRY_BY_CHANNEL: EEnvQueryTrace = EEnvQueryTrace(2);
    pub const GEOMETRY_BY_PROFILE: EEnvQueryTrace = EEnvQueryTrace(3);
    pub const NAVIGATION_OVER_LEDGES: EEnvQueryTrace = EEnvQueryTrace(4);
}
#[repr(transparent)]
pub struct EEnvOverlapShape(pub u8);
impl EEnvOverlapShape {
    pub const BOX: EEnvOverlapShape = EEnvOverlapShape(0);
    pub const SPHERE: EEnvOverlapShape = EEnvOverlapShape(1);
    pub const CAPSULE: EEnvOverlapShape = EEnvOverlapShape(2);
}
#[repr(transparent)]
pub struct EEnvQueryRunMode(pub u8);
impl EEnvQueryRunMode {
    pub const SINGLE_RESULT: EEnvQueryRunMode = EEnvQueryRunMode(0);
    pub const RANDOM_BEST5_PCT: EEnvQueryRunMode = EEnvQueryRunMode(1);
    pub const RANDOM_BEST25_PCT: EEnvQueryRunMode = EEnvQueryRunMode(2);
    pub const ALL_MATCHING: EEnvQueryRunMode = EEnvQueryRunMode(3);
}
#[repr(transparent)]
pub struct EGenericAICheck(pub u8);
impl EGenericAICheck {
    pub const LESS: EGenericAICheck = EGenericAICheck(0);
    pub const LESS_OR_EQUAL: EGenericAICheck = EGenericAICheck(1);
    pub const EQUAL: EGenericAICheck = EGenericAICheck(2);
    pub const NOT_EQUAL: EGenericAICheck = EGenericAICheck(3);
    pub const GREATER_OR_EQUAL: EGenericAICheck = EGenericAICheck(4);
    pub const GREATER: EGenericAICheck = EGenericAICheck(5);
    pub const IS_TRUE: EGenericAICheck = EGenericAICheck(6);
    pub const MAX: EGenericAICheck = EGenericAICheck(7);
}
#[repr(transparent)]
pub struct EPathFollowingResult(pub u8);
impl EPathFollowingResult {
    pub const SUCCESS: EPathFollowingResult = EPathFollowingResult(0);
    pub const BLOCKED: EPathFollowingResult = EPathFollowingResult(1);
    pub const OFF_PATH: EPathFollowingResult = EPathFollowingResult(2);
    pub const ABORTED: EPathFollowingResult = EPathFollowingResult(3);
    pub const SKIPPED_DEPRECATED: EPathFollowingResult = EPathFollowingResult(4);
    pub const INVALID: EPathFollowingResult = EPathFollowingResult(5);
}
#[repr(transparent)]
pub struct EPathFollowingStatus(pub u8);
impl EPathFollowingStatus {
    pub const IDLE: EPathFollowingStatus = EPathFollowingStatus(0);
    pub const WAITING: EPathFollowingStatus = EPathFollowingStatus(1);
    pub const PAUSED: EPathFollowingStatus = EPathFollowingStatus(2);
    pub const MOVING: EPathFollowingStatus = EPathFollowingStatus(3);
}
#[repr(transparent)]
pub struct EPathFollowingRequestResult(pub u8);
impl EPathFollowingRequestResult {
    pub const FAILED: EPathFollowingRequestResult = EPathFollowingRequestResult(0);
    pub const ALREADY_AT_GOAL: EPathFollowingRequestResult = EPathFollowingRequestResult(
        1,
    );
    pub const REQUEST_SUCCESSFUL: EPathFollowingRequestResult = EPathFollowingRequestResult(
        2,
    );
}
#[repr(transparent)]
pub struct EBTNodeResult(pub u8);
impl EBTNodeResult {
    pub const SUCCEEDED: EBTNodeResult = EBTNodeResult(0);
    pub const FAILED: EBTNodeResult = EBTNodeResult(1);
    pub const ABORTED: EBTNodeResult = EBTNodeResult(2);
    pub const IN_PROGRESS: EBTNodeResult = EBTNodeResult(3);
}
#[repr(transparent)]
pub struct EPathFollowingAction(pub u8);
impl EPathFollowingAction {
    pub const ERROR: EPathFollowingAction = EPathFollowingAction(0);
    pub const NO_MOVE: EPathFollowingAction = EPathFollowingAction(1);
    pub const DIRECT_MOVE: EPathFollowingAction = EPathFollowingAction(2);
    pub const PARTIAL_PATH: EPathFollowingAction = EPathFollowingAction(3);
    pub const PATH_TO_GOAL: EPathFollowingAction = EPathFollowingAction(4);
}
#[repr(transparent)]
pub struct EAIOptionFlag(pub u8);
impl EAIOptionFlag {
    pub const DEFAULT: EAIOptionFlag = EAIOptionFlag(0);
    pub const ENABLE: EAIOptionFlag = EAIOptionFlag(1);
    pub const DISABLE: EAIOptionFlag = EAIOptionFlag(2);
    pub const MAX: EAIOptionFlag = EAIOptionFlag(3);
}
#[repr(transparent)]
pub struct EBTFlowAbortMode(pub u8);
impl EBTFlowAbortMode {
    pub const NONE: EBTFlowAbortMode = EBTFlowAbortMode(0);
    pub const LOWER_PRIORITY: EBTFlowAbortMode = EBTFlowAbortMode(1);
    pub const SELF: EBTFlowAbortMode = EBTFlowAbortMode(2);
    pub const BOTH: EBTFlowAbortMode = EBTFlowAbortMode(3);
}
#[repr(transparent)]
pub struct EBTParallelMode(pub u8);
impl EBTParallelMode {
    pub const ABORT_BACKGROUND: EBTParallelMode = EBTParallelMode(0);
    pub const WAIT_FOR_BACKGROUND: EBTParallelMode = EBTParallelMode(1);
}
#[repr(transparent)]
pub struct EBTBlackboardRestart(pub u8);
impl EBTBlackboardRestart {
    pub const VALUE_CHANGE: EBTBlackboardRestart = EBTBlackboardRestart(0);
    pub const RESULT_CHANGE: EBTBlackboardRestart = EBTBlackboardRestart(1);
}
#[repr(transparent)]
pub struct EBasicKeyOperation(pub u8);
impl EBasicKeyOperation {
    pub const SET: EBasicKeyOperation = EBasicKeyOperation(0);
    pub const NOT_SET: EBasicKeyOperation = EBasicKeyOperation(1);
}
#[repr(transparent)]
pub struct EArithmeticKeyOperation(pub u8);
impl EArithmeticKeyOperation {
    pub const EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(0);
    pub const NOT_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(1);
    pub const LESS: EArithmeticKeyOperation = EArithmeticKeyOperation(2);
    pub const LESS_OR_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(3);
    pub const GREATER: EArithmeticKeyOperation = EArithmeticKeyOperation(4);
    pub const GREATER_OR_EQUAL: EArithmeticKeyOperation = EArithmeticKeyOperation(5);
}
#[repr(transparent)]
pub struct ETextKeyOperation(pub u8);
impl ETextKeyOperation {
    pub const EQUAL: ETextKeyOperation = ETextKeyOperation(0);
    pub const NOT_EQUAL: ETextKeyOperation = ETextKeyOperation(1);
    pub const CONTAIN: ETextKeyOperation = ETextKeyOperation(2);
    pub const NOT_CONTAIN: ETextKeyOperation = ETextKeyOperation(3);
}
#[repr(transparent)]
pub struct EBlackBoardEntryComparison(pub u8);
impl EBlackBoardEntryComparison {
    pub const EQUAL: EBlackBoardEntryComparison = EBlackBoardEntryComparison(0);
    pub const NOT_EQUAL: EBlackBoardEntryComparison = EBlackBoardEntryComparison(1);
}
#[repr(transparent)]
pub struct FAIDistanceType(pub u8);
impl FAIDistanceType {
    pub const DISTANCE3_D: FAIDistanceType = FAIDistanceType(0);
    pub const DISTANCE2_D: FAIDistanceType = FAIDistanceType(1);
    pub const DISTANCE_Z: FAIDistanceType = FAIDistanceType(2);
    pub const MAX: FAIDistanceType = FAIDistanceType(3);
}
#[repr(transparent)]
pub struct EEnvQueryResultNormalizationOption(pub u8);
impl EEnvQueryResultNormalizationOption {
    pub const DEFAULT: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        0,
    );
    pub const NORMALIZED: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        1,
    );
    pub const UNALTERED: EEnvQueryResultNormalizationOption = EEnvQueryResultNormalizationOption(
        2,
    );
}
#[repr(transparent)]
pub struct EEnvQueryStatus(pub u8);
impl EEnvQueryStatus {
    pub const PROCESSING: EEnvQueryStatus = EEnvQueryStatus(0);
    pub const SUCCESS: EEnvQueryStatus = EEnvQueryStatus(1);
    pub const FAILED: EEnvQueryStatus = EEnvQueryStatus(2);
    pub const ABORTED: EEnvQueryStatus = EEnvQueryStatus(3);
    pub const OWNER_LOST: EEnvQueryStatus = EEnvQueryStatus(4);
    pub const MISSING_PARAM: EEnvQueryStatus = EEnvQueryStatus(5);
}
#[repr(transparent)]
pub struct EEnvTestPurpose(pub u8);
impl EEnvTestPurpose {
    pub const FILTER: EEnvTestPurpose = EEnvTestPurpose(0);
    pub const SCORE: EEnvTestPurpose = EEnvTestPurpose(1);
    pub const FILTER_AND_SCORE: EEnvTestPurpose = EEnvTestPurpose(2);
}
#[repr(transparent)]
pub struct EEnvTestFilterOperator(pub u8);
impl EEnvTestFilterOperator {
    pub const ALL_PASS: EEnvTestFilterOperator = EEnvTestFilterOperator(0);
    pub const ANY_PASS: EEnvTestFilterOperator = EEnvTestFilterOperator(1);
}
#[repr(transparent)]
pub struct EEnvTestScoreOperator(pub u8);
impl EEnvTestScoreOperator {
    pub const AVERAGE_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(0);
    pub const MIN_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(1);
    pub const MAX_SCORE: EEnvTestScoreOperator = EEnvTestScoreOperator(2);
    pub const MULTIPLY: EEnvTestScoreOperator = EEnvTestScoreOperator(3);
}
#[repr(transparent)]
pub struct EEnvTestFilterType(pub u8);
impl EEnvTestFilterType {
    pub const MINIMUM: EEnvTestFilterType = EEnvTestFilterType(0);
    pub const MAXIMUM: EEnvTestFilterType = EEnvTestFilterType(1);
    pub const RANGE: EEnvTestFilterType = EEnvTestFilterType(2);
    pub const MATCH: EEnvTestFilterType = EEnvTestFilterType(3);
}
#[repr(transparent)]
pub struct EEnvTestScoreEquation(pub u8);
impl EEnvTestScoreEquation {
    pub const LINEAR: EEnvTestScoreEquation = EEnvTestScoreEquation(0);
    pub const SQUARE: EEnvTestScoreEquation = EEnvTestScoreEquation(1);
    pub const INVERSE_LINEAR: EEnvTestScoreEquation = EEnvTestScoreEquation(2);
    pub const SQUARE_ROOT: EEnvTestScoreEquation = EEnvTestScoreEquation(3);
    pub const CONSTANT: EEnvTestScoreEquation = EEnvTestScoreEquation(4);
}
#[repr(transparent)]
pub struct EEnvQueryTestClamping(pub u8);
impl EEnvQueryTestClamping {
    pub const NONE: EEnvQueryTestClamping = EEnvQueryTestClamping(0);
    pub const SPECIFIED_VALUE: EEnvQueryTestClamping = EEnvQueryTestClamping(1);
    pub const FILTER_THRESHOLD: EEnvQueryTestClamping = EEnvQueryTestClamping(2);
}
#[repr(transparent)]
pub struct EEQSNormalizationType(pub u8);
impl EEQSNormalizationType {
    pub const ABSOLUTE: EEQSNormalizationType = EEQSNormalizationType(0);
    pub const RELATIVE_TO_SCORES: EEQSNormalizationType = EEQSNormalizationType(1);
}
#[repr(transparent)]
pub struct EEnvQueryHightlightMode(pub u8);
impl EEnvQueryHightlightMode {
    pub const ALL: EEnvQueryHightlightMode = EEnvQueryHightlightMode(0);
    pub const BEST5_PCT: EEnvQueryHightlightMode = EEnvQueryHightlightMode(1);
    pub const BEST25_PCT: EEnvQueryHightlightMode = EEnvQueryHightlightMode(2);
}
#[repr(transparent)]
pub struct EPointOnCircleSpacingMethod(pub u8);
impl EPointOnCircleSpacingMethod {
    pub const BY_SPACE_BETWEEN: EPointOnCircleSpacingMethod = EPointOnCircleSpacingMethod(
        0,
    );
    pub const BY_NUMBER_OF_POINTS: EPointOnCircleSpacingMethod = EPointOnCircleSpacingMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EEnvTestDistance(pub u8);
impl EEnvTestDistance {
    pub const DISTANCE3_D: EEnvTestDistance = EEnvTestDistance(0);
    pub const DISTANCE2_D: EEnvTestDistance = EEnvTestDistance(1);
    pub const DISTANCE_Z: EEnvTestDistance = EEnvTestDistance(2);
    pub const DISTANCE_ABSOLUTE_Z: EEnvTestDistance = EEnvTestDistance(3);
}
#[repr(transparent)]
pub struct EEnvTestDot(pub u8);
impl EEnvTestDot {
    pub const DOT3_D: EEnvTestDot = EEnvTestDot(0);
    pub const DOT2_D: EEnvTestDot = EEnvTestDot(1);
}
#[repr(transparent)]
pub struct EEnvTestPathfinding(pub u8);
impl EEnvTestPathfinding {
    pub const PATH_EXIST: EEnvTestPathfinding = EEnvTestPathfinding(0);
    pub const PATH_COST: EEnvTestPathfinding = EEnvTestPathfinding(1);
    pub const PATH_LENGTH: EEnvTestPathfinding = EEnvTestPathfinding(2);
}
#[repr(transparent)]
pub struct EAISenseNotifyType(pub u8);
impl EAISenseNotifyType {
    pub const ON_EVERY_PERCEPTION: EAISenseNotifyType = EAISenseNotifyType(0);
    pub const ON_PERCEPTION_CHANGE: EAISenseNotifyType = EAISenseNotifyType(1);
}
