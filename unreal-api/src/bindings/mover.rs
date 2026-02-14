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
    pub u_mover_component_unbind_process_generated_movement: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_try_get_floor_check_hit_result: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_set_use_deferred_group_movement: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_set_up_direction_override: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_set_updated_component: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_set_primary_visual_component: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_set_planar_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_set_gravity_override: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_set_base_visual_component_transform: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_remove_movement_mode: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_remove_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_remove_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_queue_next_mode: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_queue_layered_move_activation: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_physics_volume_changed: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_on_begin_overlap: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_unregister_move: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_schedule_instant_movement_effect: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_register_moves: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_register_move: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_queue_movement_modifier: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_queue_layered_move_activation_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_queue_layered_move: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_queue_instant_movement_effect: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_find_movement_modifier: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_k2_find_active_layered_move: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_is_using_deferred_group_movement: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_is_modifier_active_or_queued: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_has_valid_cached_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_has_valid_cached_input_cmd: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_has_gameplay_tag_in_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_has_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_handle_impact: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_up_direction: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_updated_component: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_target_orientation: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_sync_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_starting_movement_mode_names: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_sim_blackboard: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_primary_visual_component: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_predicted_trajectory: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_planar_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_movement_mode_name: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_movement_mode: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_movement_intent: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_movement_base_bone_name: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_movement_base: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_last_time_step: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_last_input_cmd: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_gravity_acceleration: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_future_trajectory: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_get_base_visual_component_transform: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_find_shared_settings_mutable_bp: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_find_shared_settings_bp: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_find_movement_mode_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_find_movement_mode: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_cancel_modifier_from_handle: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_cancel_features_with_tag: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_bind_process_generated_movement: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_add_movement_mode_from_object: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_add_movement_mode_from_class: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_add_gameplay_tags: *mut crate::ffi::UFunctionOpague,
    pub u_mover_component_add_gameplay_tag: *mut crate::ffi::UFunctionOpague,
    pub u_mover_network_physics_liaison_component_base_handle_owning_pawn_controller_changed_server: *mut crate::ffi::UFunctionOpague,
    pub u_mover_network_physics_liaison_component_base_handle_component_physics_state_changed: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_set_playback_direction: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_set_path_origin_transform: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_set_is_moving: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_set_default_playback_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_is_moving: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_is_joint_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_is_in_reverse: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_get_path_origin_transform: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_mover_component_get_default_playback_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_mover_pathed_physics_liaison_component_handle_movement_mode_changed: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_set_use_async_produce_input: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_set_use_async_movement_simulation_tick: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_set_enable_produce_input: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_on_controller_changed: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_get_use_async_produce_input: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_get_use_async_movement_simulation_tick: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_get_enable_produce_input: *mut crate::ffi::UFunctionOpague,
    pub u_mover_standalone_liaison_component_add_tick_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_mover_debug_component_set_history_tracking: *mut crate::ffi::UFunctionOpague,
    pub u_mover_debug_component_on_movement_sim_tick: *mut crate::ffi::UFunctionOpague,
    pub u_mover_debug_component_on_movement_sim_rollback: *mut crate::ffi::UFunctionOpague,
    pub u_mover_debug_component_on_history_tracking_rollback: *mut crate::ffi::UFunctionOpague,
    pub u_mover_debug_component_get_past_trajectory: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_un_crouch: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_set_handle_stance_changes: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_set_handle_jump: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_on_mover_pre_simulation_tick: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_on_mover_post_finalize: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_jump: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_is_swimming: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_is_slope_sliding: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_is_on_ground: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_is_flying: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_is_falling: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_is_crouching: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_is_airborne: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_get_handle_stance_changes: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_get_handle_jump: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_crouch: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_can_crouch: *mut crate::ffi::UFunctionOpague,
    pub u_character_mover_component_can_actor_jump: *mut crate::ffi::UFunctionOpague,
    pub u_layered_move_logic_on_start: *mut crate::ffi::UFunctionOpague,
    pub u_layered_move_logic_on_end: *mut crate::ffi::UFunctionOpague,
    pub u_layered_move_logic_k2_set_active_move_data: *mut crate::ffi::UFunctionOpague,
    pub u_layered_move_logic_k2_get_active_move_data: *mut crate::ffi::UFunctionOpague,
    pub u_layered_move_logic_is_finished: *mut crate::ffi::UFunctionOpague,
    pub u_layered_move_logic_generate_move: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_simulation_tick: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_k2_on_unregistered: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_k2_on_registered: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_k2_on_deactivated: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_k2_on_activated: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_k2_get_mover_component: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_generate_move: *mut crate::ffi::UFunctionOpague,
    pub u_async_falling_mode_process_landed: *mut crate::ffi::UFunctionOpague,
    pub u_async_nav_walking_mode_set_turn_generator_class: *mut crate::ffi::UFunctionOpague,
    pub u_async_nav_walking_mode_get_turn_generator: *mut crate::ffi::UFunctionOpague,
    pub u_async_walking_mode_set_turn_generator_class: *mut crate::ffi::UFunctionOpague,
    pub u_async_walking_mode_get_turn_generator: *mut crate::ffi::UFunctionOpague,
    pub u_falling_mode_process_landed: *mut crate::ffi::UFunctionOpague,
    pub u_nav_walking_mode_set_turn_generator_class: *mut crate::ffi::UFunctionOpague,
    pub u_nav_walking_mode_get_turn_generator: *mut crate::ffi::UFunctionOpague,
    pub u_walking_mode_set_turn_generator_class: *mut crate::ffi::UFunctionOpague,
    pub u_walking_mode_get_turn_generator: *mut crate::ffi::UFunctionOpague,
    pub u_simple_walking_mode_generate_walk_move: *mut crate::ffi::UFunctionOpague,
    pub u_nav_mover_component_is_swimming: *mut crate::ffi::UFunctionOpague,
    pub u_nav_mover_component_get_velocity_for_nav_movement: *mut crate::ffi::UFunctionOpague,
    pub u_nav_mover_component_get_max_speed_for_nav_movement: *mut crate::ffi::UFunctionOpague,
    pub u_nav_mover_component_consume_nav_movement_data: *mut crate::ffi::UFunctionOpague,
    pub u_air_movement_utils_try_move_to_fall_along_surface: *mut crate::ffi::UFunctionOpague,
    pub u_air_movement_utils_test_falling_move_along_hit_surface: *mut crate::ffi::UFunctionOpague,
    pub u_air_movement_utils_is_valid_landing_spot: *mut crate::ffi::UFunctionOpague,
    pub u_air_movement_utils_compute_controlled_free_move: *mut crate::ffi::UFunctionOpague,
    pub u_async_movement_utils_test_sliding_move_along_hit_surface: *mut crate::ffi::UFunctionOpague,
    pub u_async_movement_utils_test_depenetrating_move: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_world_rotator_to_based: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_world_location_to_based: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_world_direction_to_based: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_rotator_to_world: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_rotator_to_local: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_location_to_world: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_location_to_local: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_direction_to_world: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_direction_to_local: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_based_rotator_to_world: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_based_location_to_world: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_transform_based_direction_to_world: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_is_base_simulating_physics: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_is_a_dynamic_base: *mut crate::ffi::UFunctionOpague,
    pub u_based_movement_utils_get_movement_base_transform: *mut crate::ffi::UFunctionOpague,
    pub u_planar_constraint_utils_set_plane_constraint_origin: *mut crate::ffi::UFunctionOpague,
    pub u_planar_constraint_utils_set_planar_constraint_normal: *mut crate::ffi::UFunctionOpague,
    pub u_planar_constraint_utils_set_planar_constraint_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_planar_constraint_utils_constrain_normal_to_plane: *mut crate::ffi::UFunctionOpague,
    pub u_planar_constraint_utils_constrain_location_to_plane: *mut crate::ffi::UFunctionOpague,
    pub u_planar_constraint_utils_constrain_direction_to_plane: *mut crate::ffi::UFunctionOpague,
    pub u_floor_query_utils_is_hit_surface_walkable: *mut crate::ffi::UFunctionOpague,
    pub u_ground_movement_utils_try_walk_to_slide_along_surface: *mut crate::ffi::UFunctionOpague,
    pub u_ground_movement_utils_test_move_to_keep_min_height_above_floor: *mut crate::ffi::UFunctionOpague,
    pub u_ground_movement_utils_test_move_to_adjust_to_floor: *mut crate::ffi::UFunctionOpague,
    pub u_ground_movement_utils_test_grounded_move_along_hit_surface: *mut crate::ffi::UFunctionOpague,
    pub u_ground_movement_utils_compute_deflected_move_onto_ramp: *mut crate::ffi::UFunctionOpague,
    pub u_ground_movement_utils_compute_controlled_ground_move: *mut crate::ffi::UFunctionOpague,
    pub u_ground_movement_utils_can_step_up_on_hit_surface: *mut crate::ffi::UFunctionOpague,
    pub u_turn_generator_interface_get_turn: *mut crate::ffi::UFunctionOpague,
    pub u_movement_record_utils_k2_set_delta_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movement_record_utils_k2_get_total_move_delta: *mut crate::ffi::UFunctionOpague,
    pub u_movement_record_utils_k2_get_relevant_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_movement_record_utils_k2_get_relevant_move_delta: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_try_safe_move_updated_component_no_movement_record: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_try_safe_move_updated_component: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_try_safe_move_and_slide_updated_component_no_movement_record: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_try_safe_move_and_slide_updated_component: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_try_move_to_slide_along_surface_no_movement_record: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_try_move_to_slide_along_surface: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_test_encroachment_and_adjust: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_test_encroachment: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_project_to_gravity_floor: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_is_exceeding_max_speed: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_is_angular_velocity_zero: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_get_gravity_vertical_component: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_find_teleport_spot: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_deduce_up_direction_from_gravity: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_constrain_to_plane: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_velocity_from_positions: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_velocity_from_gravity: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_slide_delta: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_direction_intent: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_combined_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_angular_velocity_degrees: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_compute_angular_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_can_escape_gravity: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_apply_gravity_to_orientation_intent: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_apply_angular_velocity_to_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_apply_angular_velocity_to_quat: *mut crate::ffi::UFunctionOpague,
    pub u_movement_utils_apply_angular_velocity: *mut crate::ffi::UFunctionOpague,
    pub u_play_mover_montage_callback_proxy_on_mover_montage_ended: *mut crate::ffi::UFunctionOpague,
    pub u_play_mover_montage_callback_proxy_create_proxy_object_for_play_mover_montage: *mut crate::ffi::UFunctionOpague,
    pub u_water_movement_utils_update_water_spline_data: *mut crate::ffi::UFunctionOpague,
    pub u_water_movement_utils_compute_controlled_water_move: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_transition_trigger: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_transition_k2_on_unregistered: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_transition_k2_on_registered: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_transition_k2_get_mover_component: *mut crate::ffi::UFunctionOpague,
    pub u_base_movement_mode_transition_evaluate: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_set_velocity_input: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_set_directional_input: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_get_velocity_from_sync_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_get_orientation_from_sync_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_get_move_direction_intent_from_sync_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_get_move_direction_intent_from_inputs: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_get_location_from_sync_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_model_blueprint_library_get_angular_velocity_degrees_from_sync_state: *mut crate::ffi::UFunctionOpague,
    pub u_mover_trajectory_predictor_setup: *mut crate::ffi::UFunctionOpague,
    pub u_mover_simulation_get_rollback_blackboard_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_mover_simulation_get_rollback_blackboard: *mut crate::ffi::UFunctionOpague,
    pub u_mover_simulation_get_blackboard_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_mover_simulation_get_blackboard: *mut crate::ffi::UFunctionOpague,
    pub u_mover_simulation_attempt_teleport: *mut crate::ffi::UFunctionOpague,
    pub u_mover_input_producer_interface_produce_input: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_collection_library_k2_get_data_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_collection_library_k2_add_data_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_mover_data_collection_library_clear_data_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_movement_mode_set_use_joint_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_movement_mode_set_path_duration_begin_play_only: *mut crate::ffi::UFunctionOpague,
    pub u_pathed_physics_movement_mode_bp_find_pattern: *mut crate::ffi::UFunctionOpague,
    pub u_look_at_rotation_pattern_set_relative_look_at_location: *mut crate::ffi::UFunctionOpague,
    pub u_look_at_rotation_pattern_set_look_at_location: *mut crate::ffi::UFunctionOpague,
    pub u_physics_character_mover_component_on_mover_pre_movement: *mut crate::ffi::UFunctionOpague,
    pub u_physics_character_mover_component_on_mover_post_simulation_tick: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_mover_component_unbind_process_generated_movement: std::ptr::null_mut(),
            u_mover_component_try_get_floor_check_hit_result: std::ptr::null_mut(),
            u_mover_component_set_use_deferred_group_movement: std::ptr::null_mut(),
            u_mover_component_set_up_direction_override: std::ptr::null_mut(),
            u_mover_component_set_updated_component: std::ptr::null_mut(),
            u_mover_component_set_primary_visual_component: std::ptr::null_mut(),
            u_mover_component_set_planar_constraint: std::ptr::null_mut(),
            u_mover_component_set_gravity_override: std::ptr::null_mut(),
            u_mover_component_set_base_visual_component_transform: std::ptr::null_mut(),
            u_mover_component_remove_movement_mode: std::ptr::null_mut(),
            u_mover_component_remove_gameplay_tags: std::ptr::null_mut(),
            u_mover_component_remove_gameplay_tag: std::ptr::null_mut(),
            u_mover_component_queue_next_mode: std::ptr::null_mut(),
            u_mover_component_queue_layered_move_activation: std::ptr::null_mut(),
            u_mover_component_physics_volume_changed: std::ptr::null_mut(),
            u_mover_component_on_begin_overlap: std::ptr::null_mut(),
            u_mover_component_k2_unregister_move: std::ptr::null_mut(),
            u_mover_component_k2_schedule_instant_movement_effect: std::ptr::null_mut(),
            u_mover_component_k2_register_moves: std::ptr::null_mut(),
            u_mover_component_k2_register_move: std::ptr::null_mut(),
            u_mover_component_k2_queue_movement_modifier: std::ptr::null_mut(),
            u_mover_component_k2_queue_layered_move_activation_with_context: std::ptr::null_mut(),
            u_mover_component_k2_queue_layered_move: std::ptr::null_mut(),
            u_mover_component_k2_queue_instant_movement_effect: std::ptr::null_mut(),
            u_mover_component_k2_find_movement_modifier: std::ptr::null_mut(),
            u_mover_component_k2_find_active_layered_move: std::ptr::null_mut(),
            u_mover_component_is_using_deferred_group_movement: std::ptr::null_mut(),
            u_mover_component_is_modifier_active_or_queued: std::ptr::null_mut(),
            u_mover_component_has_valid_cached_state: std::ptr::null_mut(),
            u_mover_component_has_valid_cached_input_cmd: std::ptr::null_mut(),
            u_mover_component_has_gameplay_tag_in_state: std::ptr::null_mut(),
            u_mover_component_has_gameplay_tag: std::ptr::null_mut(),
            u_mover_component_handle_impact: std::ptr::null_mut(),
            u_mover_component_get_velocity: std::ptr::null_mut(),
            u_mover_component_get_up_direction: std::ptr::null_mut(),
            u_mover_component_get_updated_component: std::ptr::null_mut(),
            u_mover_component_get_target_orientation: std::ptr::null_mut(),
            u_mover_component_get_sync_state: std::ptr::null_mut(),
            u_mover_component_get_starting_movement_mode_names: std::ptr::null_mut(),
            u_mover_component_get_sim_blackboard: std::ptr::null_mut(),
            u_mover_component_get_primary_visual_component: std::ptr::null_mut(),
            u_mover_component_get_predicted_trajectory: std::ptr::null_mut(),
            u_mover_component_get_planar_constraint: std::ptr::null_mut(),
            u_mover_component_get_movement_mode_name: std::ptr::null_mut(),
            u_mover_component_get_movement_mode: std::ptr::null_mut(),
            u_mover_component_get_movement_intent: std::ptr::null_mut(),
            u_mover_component_get_movement_base_bone_name: std::ptr::null_mut(),
            u_mover_component_get_movement_base: std::ptr::null_mut(),
            u_mover_component_get_last_time_step: std::ptr::null_mut(),
            u_mover_component_get_last_input_cmd: std::ptr::null_mut(),
            u_mover_component_get_gravity_acceleration: std::ptr::null_mut(),
            u_mover_component_get_future_trajectory: std::ptr::null_mut(),
            u_mover_component_get_base_visual_component_transform: std::ptr::null_mut(),
            u_mover_component_find_shared_settings_mutable_bp: std::ptr::null_mut(),
            u_mover_component_find_shared_settings_bp: std::ptr::null_mut(),
            u_mover_component_find_movement_mode_by_name: std::ptr::null_mut(),
            u_mover_component_find_movement_mode: std::ptr::null_mut(),
            u_mover_component_cancel_modifier_from_handle: std::ptr::null_mut(),
            u_mover_component_cancel_features_with_tag: std::ptr::null_mut(),
            u_mover_component_bind_process_generated_movement: std::ptr::null_mut(),
            u_mover_component_add_movement_mode_from_object: std::ptr::null_mut(),
            u_mover_component_add_movement_mode_from_class: std::ptr::null_mut(),
            u_mover_component_add_gameplay_tags: std::ptr::null_mut(),
            u_mover_component_add_gameplay_tag: std::ptr::null_mut(),
            u_mover_network_physics_liaison_component_base_handle_owning_pawn_controller_changed_server: std::ptr::null_mut(),
            u_mover_network_physics_liaison_component_base_handle_component_physics_state_changed: std::ptr::null_mut(),
            u_pathed_physics_mover_component_set_playback_direction: std::ptr::null_mut(),
            u_pathed_physics_mover_component_set_path_origin_transform: std::ptr::null_mut(),
            u_pathed_physics_mover_component_set_is_moving: std::ptr::null_mut(),
            u_pathed_physics_mover_component_set_default_playback_behavior: std::ptr::null_mut(),
            u_pathed_physics_mover_component_is_moving: std::ptr::null_mut(),
            u_pathed_physics_mover_component_is_joint_enabled: std::ptr::null_mut(),
            u_pathed_physics_mover_component_is_in_reverse: std::ptr::null_mut(),
            u_pathed_physics_mover_component_get_path_origin_transform: std::ptr::null_mut(),
            u_pathed_physics_mover_component_get_default_playback_behavior: std::ptr::null_mut(),
            u_mover_pathed_physics_liaison_component_handle_movement_mode_changed: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_set_use_async_produce_input: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_set_use_async_movement_simulation_tick: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_set_enable_produce_input: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_on_controller_changed: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_get_use_async_produce_input: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_get_use_async_movement_simulation_tick: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_get_enable_produce_input: std::ptr::null_mut(),
            u_mover_standalone_liaison_component_add_tick_dependency: std::ptr::null_mut(),
            u_mover_debug_component_set_history_tracking: std::ptr::null_mut(),
            u_mover_debug_component_on_movement_sim_tick: std::ptr::null_mut(),
            u_mover_debug_component_on_movement_sim_rollback: std::ptr::null_mut(),
            u_mover_debug_component_on_history_tracking_rollback: std::ptr::null_mut(),
            u_mover_debug_component_get_past_trajectory: std::ptr::null_mut(),
            u_character_mover_component_un_crouch: std::ptr::null_mut(),
            u_character_mover_component_set_handle_stance_changes: std::ptr::null_mut(),
            u_character_mover_component_set_handle_jump: std::ptr::null_mut(),
            u_character_mover_component_on_mover_pre_simulation_tick: std::ptr::null_mut(),
            u_character_mover_component_on_mover_post_finalize: std::ptr::null_mut(),
            u_character_mover_component_jump: std::ptr::null_mut(),
            u_character_mover_component_is_swimming: std::ptr::null_mut(),
            u_character_mover_component_is_slope_sliding: std::ptr::null_mut(),
            u_character_mover_component_is_on_ground: std::ptr::null_mut(),
            u_character_mover_component_is_flying: std::ptr::null_mut(),
            u_character_mover_component_is_falling: std::ptr::null_mut(),
            u_character_mover_component_is_crouching: std::ptr::null_mut(),
            u_character_mover_component_is_airborne: std::ptr::null_mut(),
            u_character_mover_component_get_handle_stance_changes: std::ptr::null_mut(),
            u_character_mover_component_get_handle_jump: std::ptr::null_mut(),
            u_character_mover_component_crouch: std::ptr::null_mut(),
            u_character_mover_component_can_crouch: std::ptr::null_mut(),
            u_character_mover_component_can_actor_jump: std::ptr::null_mut(),
            u_layered_move_logic_on_start: std::ptr::null_mut(),
            u_layered_move_logic_on_end: std::ptr::null_mut(),
            u_layered_move_logic_k2_set_active_move_data: std::ptr::null_mut(),
            u_layered_move_logic_k2_get_active_move_data: std::ptr::null_mut(),
            u_layered_move_logic_is_finished: std::ptr::null_mut(),
            u_layered_move_logic_generate_move: std::ptr::null_mut(),
            u_base_movement_mode_simulation_tick: std::ptr::null_mut(),
            u_base_movement_mode_k2_on_unregistered: std::ptr::null_mut(),
            u_base_movement_mode_k2_on_registered: std::ptr::null_mut(),
            u_base_movement_mode_k2_on_deactivated: std::ptr::null_mut(),
            u_base_movement_mode_k2_on_activated: std::ptr::null_mut(),
            u_base_movement_mode_k2_get_mover_component: std::ptr::null_mut(),
            u_base_movement_mode_generate_move: std::ptr::null_mut(),
            u_async_falling_mode_process_landed: std::ptr::null_mut(),
            u_async_nav_walking_mode_set_turn_generator_class: std::ptr::null_mut(),
            u_async_nav_walking_mode_get_turn_generator: std::ptr::null_mut(),
            u_async_walking_mode_set_turn_generator_class: std::ptr::null_mut(),
            u_async_walking_mode_get_turn_generator: std::ptr::null_mut(),
            u_falling_mode_process_landed: std::ptr::null_mut(),
            u_nav_walking_mode_set_turn_generator_class: std::ptr::null_mut(),
            u_nav_walking_mode_get_turn_generator: std::ptr::null_mut(),
            u_walking_mode_set_turn_generator_class: std::ptr::null_mut(),
            u_walking_mode_get_turn_generator: std::ptr::null_mut(),
            u_simple_walking_mode_generate_walk_move: std::ptr::null_mut(),
            u_nav_mover_component_is_swimming: std::ptr::null_mut(),
            u_nav_mover_component_get_velocity_for_nav_movement: std::ptr::null_mut(),
            u_nav_mover_component_get_max_speed_for_nav_movement: std::ptr::null_mut(),
            u_nav_mover_component_consume_nav_movement_data: std::ptr::null_mut(),
            u_air_movement_utils_try_move_to_fall_along_surface: std::ptr::null_mut(),
            u_air_movement_utils_test_falling_move_along_hit_surface: std::ptr::null_mut(),
            u_air_movement_utils_is_valid_landing_spot: std::ptr::null_mut(),
            u_air_movement_utils_compute_controlled_free_move: std::ptr::null_mut(),
            u_async_movement_utils_test_sliding_move_along_hit_surface: std::ptr::null_mut(),
            u_async_movement_utils_test_depenetrating_move: std::ptr::null_mut(),
            u_based_movement_utils_transform_world_rotator_to_based: std::ptr::null_mut(),
            u_based_movement_utils_transform_world_location_to_based: std::ptr::null_mut(),
            u_based_movement_utils_transform_world_direction_to_based: std::ptr::null_mut(),
            u_based_movement_utils_transform_rotator_to_world: std::ptr::null_mut(),
            u_based_movement_utils_transform_rotator_to_local: std::ptr::null_mut(),
            u_based_movement_utils_transform_location_to_world: std::ptr::null_mut(),
            u_based_movement_utils_transform_location_to_local: std::ptr::null_mut(),
            u_based_movement_utils_transform_direction_to_world: std::ptr::null_mut(),
            u_based_movement_utils_transform_direction_to_local: std::ptr::null_mut(),
            u_based_movement_utils_transform_based_rotator_to_world: std::ptr::null_mut(),
            u_based_movement_utils_transform_based_location_to_world: std::ptr::null_mut(),
            u_based_movement_utils_transform_based_direction_to_world: std::ptr::null_mut(),
            u_based_movement_utils_is_base_simulating_physics: std::ptr::null_mut(),
            u_based_movement_utils_is_a_dynamic_base: std::ptr::null_mut(),
            u_based_movement_utils_get_movement_base_transform: std::ptr::null_mut(),
            u_planar_constraint_utils_set_plane_constraint_origin: std::ptr::null_mut(),
            u_planar_constraint_utils_set_planar_constraint_normal: std::ptr::null_mut(),
            u_planar_constraint_utils_set_planar_constraint_enabled: std::ptr::null_mut(),
            u_planar_constraint_utils_constrain_normal_to_plane: std::ptr::null_mut(),
            u_planar_constraint_utils_constrain_location_to_plane: std::ptr::null_mut(),
            u_planar_constraint_utils_constrain_direction_to_plane: std::ptr::null_mut(),
            u_floor_query_utils_is_hit_surface_walkable: std::ptr::null_mut(),
            u_ground_movement_utils_try_walk_to_slide_along_surface: std::ptr::null_mut(),
            u_ground_movement_utils_test_move_to_keep_min_height_above_floor: std::ptr::null_mut(),
            u_ground_movement_utils_test_move_to_adjust_to_floor: std::ptr::null_mut(),
            u_ground_movement_utils_test_grounded_move_along_hit_surface: std::ptr::null_mut(),
            u_ground_movement_utils_compute_deflected_move_onto_ramp: std::ptr::null_mut(),
            u_ground_movement_utils_compute_controlled_ground_move: std::ptr::null_mut(),
            u_ground_movement_utils_can_step_up_on_hit_surface: std::ptr::null_mut(),
            u_turn_generator_interface_get_turn: std::ptr::null_mut(),
            u_movement_record_utils_k2_set_delta_seconds: std::ptr::null_mut(),
            u_movement_record_utils_k2_get_total_move_delta: std::ptr::null_mut(),
            u_movement_record_utils_k2_get_relevant_velocity: std::ptr::null_mut(),
            u_movement_record_utils_k2_get_relevant_move_delta: std::ptr::null_mut(),
            u_movement_utils_try_safe_move_updated_component_no_movement_record: std::ptr::null_mut(),
            u_movement_utils_try_safe_move_updated_component: std::ptr::null_mut(),
            u_movement_utils_try_safe_move_and_slide_updated_component_no_movement_record: std::ptr::null_mut(),
            u_movement_utils_try_safe_move_and_slide_updated_component: std::ptr::null_mut(),
            u_movement_utils_try_move_to_slide_along_surface_no_movement_record: std::ptr::null_mut(),
            u_movement_utils_try_move_to_slide_along_surface: std::ptr::null_mut(),
            u_movement_utils_test_encroachment_and_adjust: std::ptr::null_mut(),
            u_movement_utils_test_encroachment: std::ptr::null_mut(),
            u_movement_utils_project_to_gravity_floor: std::ptr::null_mut(),
            u_movement_utils_is_exceeding_max_speed: std::ptr::null_mut(),
            u_movement_utils_is_angular_velocity_zero: std::ptr::null_mut(),
            u_movement_utils_get_gravity_vertical_component: std::ptr::null_mut(),
            u_movement_utils_find_teleport_spot: std::ptr::null_mut(),
            u_movement_utils_deduce_up_direction_from_gravity: std::ptr::null_mut(),
            u_movement_utils_constrain_to_plane: std::ptr::null_mut(),
            u_movement_utils_compute_velocity_from_positions: std::ptr::null_mut(),
            u_movement_utils_compute_velocity_from_gravity: std::ptr::null_mut(),
            u_movement_utils_compute_velocity: std::ptr::null_mut(),
            u_movement_utils_compute_slide_delta: std::ptr::null_mut(),
            u_movement_utils_compute_direction_intent: std::ptr::null_mut(),
            u_movement_utils_compute_combined_velocity: std::ptr::null_mut(),
            u_movement_utils_compute_angular_velocity_degrees: std::ptr::null_mut(),
            u_movement_utils_compute_angular_velocity: std::ptr::null_mut(),
            u_movement_utils_can_escape_gravity: std::ptr::null_mut(),
            u_movement_utils_apply_gravity_to_orientation_intent: std::ptr::null_mut(),
            u_movement_utils_apply_angular_velocity_to_rotator: std::ptr::null_mut(),
            u_movement_utils_apply_angular_velocity_to_quat: std::ptr::null_mut(),
            u_movement_utils_apply_angular_velocity: std::ptr::null_mut(),
            u_play_mover_montage_callback_proxy_on_mover_montage_ended: std::ptr::null_mut(),
            u_play_mover_montage_callback_proxy_create_proxy_object_for_play_mover_montage: std::ptr::null_mut(),
            u_water_movement_utils_update_water_spline_data: std::ptr::null_mut(),
            u_water_movement_utils_compute_controlled_water_move: std::ptr::null_mut(),
            u_base_movement_mode_transition_trigger: std::ptr::null_mut(),
            u_base_movement_mode_transition_k2_on_unregistered: std::ptr::null_mut(),
            u_base_movement_mode_transition_k2_on_registered: std::ptr::null_mut(),
            u_base_movement_mode_transition_k2_get_mover_component: std::ptr::null_mut(),
            u_base_movement_mode_transition_evaluate: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_set_velocity_input: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_set_directional_input: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_get_velocity_from_sync_state: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_get_orientation_from_sync_state: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_get_move_direction_intent_from_sync_state: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_get_move_direction_intent_from_inputs: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_get_location_from_sync_state: std::ptr::null_mut(),
            u_mover_data_model_blueprint_library_get_angular_velocity_degrees_from_sync_state: std::ptr::null_mut(),
            u_mover_trajectory_predictor_setup: std::ptr::null_mut(),
            u_mover_simulation_get_rollback_blackboard_mutable: std::ptr::null_mut(),
            u_mover_simulation_get_rollback_blackboard: std::ptr::null_mut(),
            u_mover_simulation_get_blackboard_mutable: std::ptr::null_mut(),
            u_mover_simulation_get_blackboard: std::ptr::null_mut(),
            u_mover_simulation_attempt_teleport: std::ptr::null_mut(),
            u_mover_input_producer_interface_produce_input: std::ptr::null_mut(),
            u_mover_data_collection_library_k2_get_data_from_collection: std::ptr::null_mut(),
            u_mover_data_collection_library_k2_add_data_to_collection: std::ptr::null_mut(),
            u_mover_data_collection_library_clear_data_from_collection: std::ptr::null_mut(),
            u_pathed_physics_movement_mode_set_use_joint_constraint: std::ptr::null_mut(),
            u_pathed_physics_movement_mode_set_path_duration_begin_play_only: std::ptr::null_mut(),
            u_pathed_physics_movement_mode_bp_find_pattern: std::ptr::null_mut(),
            u_look_at_rotation_pattern_set_relative_look_at_location: std::ptr::null_mut(),
            u_look_at_rotation_pattern_set_look_at_location: std::ptr::null_mut(),
            u_physics_character_mover_component_on_mover_pre_movement: std::ptr::null_mut(),
            u_physics_character_mover_component_on_mover_post_simulation_tick: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnbindProcessGeneratedMovement"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_unbind_process_generated_movement,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryGetFloorCheckHitResult"),
                &raw mut __FUNCTION_PTRS.u_mover_component_try_get_floor_check_hit_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUseDeferredGroupMovement"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_set_use_deferred_group_movement,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUpDirectionOverride"),
                &raw mut __FUNCTION_PTRS.u_mover_component_set_up_direction_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUpdatedComponent"),
                &raw mut __FUNCTION_PTRS.u_mover_component_set_updated_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPrimaryVisualComponent"),
                &raw mut __FUNCTION_PTRS.u_mover_component_set_primary_visual_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlanarConstraint"),
                &raw mut __FUNCTION_PTRS.u_mover_component_set_planar_constraint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGravityOverride"),
                &raw mut __FUNCTION_PTRS.u_mover_component_set_gravity_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBaseVisualComponentTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_set_base_visual_component_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveMovementMode"),
                &raw mut __FUNCTION_PTRS.u_mover_component_remove_movement_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveGameplayTags"),
                &raw mut __FUNCTION_PTRS.u_mover_component_remove_gameplay_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveGameplayTag"),
                &raw mut __FUNCTION_PTRS.u_mover_component_remove_gameplay_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("QueueNextMode"),
                &raw mut __FUNCTION_PTRS.u_mover_component_queue_next_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("QueueLayeredMoveActivation"),
                &raw mut __FUNCTION_PTRS.u_mover_component_queue_layered_move_activation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PhysicsVolumeChanged"),
                &raw mut __FUNCTION_PTRS.u_mover_component_physics_volume_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnBeginOverlap"),
                &raw mut __FUNCTION_PTRS.u_mover_component_on_begin_overlap,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_UnregisterMove"),
                &raw mut __FUNCTION_PTRS.u_mover_component_k2_unregister_move,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_ScheduleInstantMovementEffect"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_k2_schedule_instant_movement_effect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_RegisterMoves"),
                &raw mut __FUNCTION_PTRS.u_mover_component_k2_register_moves,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_RegisterMove"),
                &raw mut __FUNCTION_PTRS.u_mover_component_k2_register_move,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_QueueMovementModifier"),
                &raw mut __FUNCTION_PTRS.u_mover_component_k2_queue_movement_modifier,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_QueueLayeredMoveActivationWithContext"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_k2_queue_layered_move_activation_with_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_QueueLayeredMove"),
                &raw mut __FUNCTION_PTRS.u_mover_component_k2_queue_layered_move,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_QueueInstantMovementEffect"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_k2_queue_instant_movement_effect,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_FindMovementModifier"),
                &raw mut __FUNCTION_PTRS.u_mover_component_k2_find_movement_modifier,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_FindActiveLayeredMove"),
                &raw mut __FUNCTION_PTRS.u_mover_component_k2_find_active_layered_move,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsUsingDeferredGroupMovement"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_is_using_deferred_group_movement,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsModifierActiveOrQueued"),
                &raw mut __FUNCTION_PTRS.u_mover_component_is_modifier_active_or_queued,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasValidCachedState"),
                &raw mut __FUNCTION_PTRS.u_mover_component_has_valid_cached_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasValidCachedInputCmd"),
                &raw mut __FUNCTION_PTRS.u_mover_component_has_valid_cached_input_cmd,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasGameplayTagInState"),
                &raw mut __FUNCTION_PTRS.u_mover_component_has_gameplay_tag_in_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasGameplayTag"),
                &raw mut __FUNCTION_PTRS.u_mover_component_has_gameplay_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleImpact"),
                &raw mut __FUNCTION_PTRS.u_mover_component_handle_impact,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVelocity"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUpDirection"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_up_direction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUpdatedComponent"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_updated_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetOrientation"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_target_orientation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSyncState"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_sync_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStartingMovementModeNames"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_get_starting_movement_mode_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSimBlackboard"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_sim_blackboard,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPrimaryVisualComponent"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_primary_visual_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPredictedTrajectory"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_predicted_trajectory,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlanarConstraint"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_planar_constraint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMovementModeName"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_movement_mode_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMovementMode"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_movement_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMovementIntent"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_movement_intent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMovementBaseBoneName"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_movement_base_bone_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMovementBase"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_movement_base,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastTimeStep"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_last_time_step,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLastInputCmd"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_last_input_cmd,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGravityAcceleration"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_gravity_acceleration,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFutureTrajectory"),
                &raw mut __FUNCTION_PTRS.u_mover_component_get_future_trajectory,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBaseVisualComponentTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_get_base_visual_component_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSharedSettings_Mutable_BP"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_find_shared_settings_mutable_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSharedSettings_BP"),
                &raw mut __FUNCTION_PTRS.u_mover_component_find_shared_settings_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindMovementModeByName"),
                &raw mut __FUNCTION_PTRS.u_mover_component_find_movement_mode_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindMovementMode"),
                &raw mut __FUNCTION_PTRS.u_mover_component_find_movement_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelModifierFromHandle"),
                &raw mut __FUNCTION_PTRS.u_mover_component_cancel_modifier_from_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelFeaturesWithTag"),
                &raw mut __FUNCTION_PTRS.u_mover_component_cancel_features_with_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BindProcessGeneratedMovement"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_component_bind_process_generated_movement,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMovementModeFromObject"),
                &raw mut __FUNCTION_PTRS.u_mover_component_add_movement_mode_from_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMovementModeFromClass"),
                &raw mut __FUNCTION_PTRS.u_mover_component_add_movement_mode_from_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGameplayTags"),
                &raw mut __FUNCTION_PTRS.u_mover_component_add_gameplay_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGameplayTag"),
                &raw mut __FUNCTION_PTRS.u_mover_component_add_gameplay_tag,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverNetworkPhysicsLiaisonComponentBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleOwningPawnControllerChanged_Server"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_network_physics_liaison_component_base_handle_owning_pawn_controller_changed_server,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleComponentPhysicsStateChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_network_physics_liaison_component_base_handle_component_physics_state_changed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPathedPhysicsMoverComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlaybackDirection"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_playback_direction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPathOriginTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_path_origin_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetIsMoving"),
                &raw mut __FUNCTION_PTRS.u_pathed_physics_mover_component_set_is_moving,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDefaultPlaybackBehavior"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_default_playback_behavior,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsMoving"),
                &raw mut __FUNCTION_PTRS.u_pathed_physics_mover_component_is_moving,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsJointEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_mover_component_is_joint_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInReverse"),
                &raw mut __FUNCTION_PTRS.u_pathed_physics_mover_component_is_in_reverse,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPathOriginTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_mover_component_get_path_origin_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultPlaybackBehavior"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_mover_component_get_default_playback_behavior,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverPathedPhysicsLiaisonComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HandleMovementModeChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_pathed_physics_liaison_component_handle_movement_mode_changed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverStandaloneLiaisonComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUseAsyncProduceInput"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_use_async_produce_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUseAsyncMovementSimulationTick"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_use_async_movement_simulation_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnableProduceInput"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_enable_produce_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnControllerChanged"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_on_controller_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUseAsyncProduceInput"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_use_async_produce_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUseAsyncMovementSimulationTick"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_use_async_movement_simulation_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnableProduceInput"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_enable_produce_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTickDependency"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_add_tick_dependency,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverDebugComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHistoryTracking"),
                &raw mut __FUNCTION_PTRS.u_mover_debug_component_set_history_tracking,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMovementSimTick"),
                &raw mut __FUNCTION_PTRS.u_mover_debug_component_on_movement_sim_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMovementSimRollback"),
                &raw mut __FUNCTION_PTRS.u_mover_debug_component_on_movement_sim_rollback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnHistoryTrackingRollback"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_debug_component_on_history_tracking_rollback,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPastTrajectory"),
                &raw mut __FUNCTION_PTRS.u_mover_debug_component_get_past_trajectory,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCharacterMoverComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnCrouch"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_un_crouch,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHandleStanceChanges"),
                &raw mut __FUNCTION_PTRS
                    .u_character_mover_component_set_handle_stance_changes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHandleJump"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_set_handle_jump,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMoverPreSimulationTick"),
                &raw mut __FUNCTION_PTRS
                    .u_character_mover_component_on_mover_pre_simulation_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMoverPostFinalize"),
                &raw mut __FUNCTION_PTRS
                    .u_character_mover_component_on_mover_post_finalize,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Jump"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_jump,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSwimming"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_is_swimming,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSlopeSliding"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_is_slope_sliding,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsOnGround"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_is_on_ground,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsFlying"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_is_flying,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsFalling"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_is_falling,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCrouching"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_is_crouching,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAirborne"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_is_airborne,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHandleStanceChanges"),
                &raw mut __FUNCTION_PTRS
                    .u_character_mover_component_get_handle_stance_changes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHandleJump"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_get_handle_jump,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Crouch"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_crouch,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanCrouch"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_can_crouch,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanActorJump"),
                &raw mut __FUNCTION_PTRS.u_character_mover_component_can_actor_jump,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULayeredMoveLogic::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnStart"),
                &raw mut __FUNCTION_PTRS.u_layered_move_logic_on_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnEnd"),
                &raw mut __FUNCTION_PTRS.u_layered_move_logic_on_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_SetActiveMoveData"),
                &raw mut __FUNCTION_PTRS.u_layered_move_logic_k2_set_active_move_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetActiveMoveData"),
                &raw mut __FUNCTION_PTRS.u_layered_move_logic_k2_get_active_move_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsFinished"),
                &raw mut __FUNCTION_PTRS.u_layered_move_logic_is_finished,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateMove"),
                &raw mut __FUNCTION_PTRS.u_layered_move_logic_generate_move,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBaseMovementMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SimulationTick"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_simulation_tick,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnUnregistered"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_k2_on_unregistered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnRegistered"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_k2_on_registered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnDeactivated"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_k2_on_deactivated,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnActivated"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_k2_on_activated,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetMoverComponent"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_k2_get_mover_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateMove"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_generate_move,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncFallingMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProcessLanded"),
                &raw mut __FUNCTION_PTRS.u_async_falling_mode_process_landed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncNavWalkingMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTurnGeneratorClass"),
                &raw mut __FUNCTION_PTRS
                    .u_async_nav_walking_mode_set_turn_generator_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTurnGenerator"),
                &raw mut __FUNCTION_PTRS.u_async_nav_walking_mode_get_turn_generator,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncWalkingMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTurnGeneratorClass"),
                &raw mut __FUNCTION_PTRS.u_async_walking_mode_set_turn_generator_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTurnGenerator"),
                &raw mut __FUNCTION_PTRS.u_async_walking_mode_get_turn_generator,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UFallingMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProcessLanded"),
                &raw mut __FUNCTION_PTRS.u_falling_mode_process_landed,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNavWalkingMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTurnGeneratorClass"),
                &raw mut __FUNCTION_PTRS.u_nav_walking_mode_set_turn_generator_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTurnGenerator"),
                &raw mut __FUNCTION_PTRS.u_nav_walking_mode_get_turn_generator,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWalkingMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTurnGeneratorClass"),
                &raw mut __FUNCTION_PTRS.u_walking_mode_set_turn_generator_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTurnGenerator"),
                &raw mut __FUNCTION_PTRS.u_walking_mode_get_turn_generator,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = USimpleWalkingMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GenerateWalkMove"),
                &raw mut __FUNCTION_PTRS.u_simple_walking_mode_generate_walk_move,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UNavMoverComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSwimming"),
                &raw mut __FUNCTION_PTRS.u_nav_mover_component_is_swimming,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVelocityForNavMovement"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_mover_component_get_velocity_for_nav_movement,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMaxSpeedForNavMovement"),
                &raw mut __FUNCTION_PTRS
                    .u_nav_mover_component_get_max_speed_for_nav_movement,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConsumeNavMovementData"),
                &raw mut __FUNCTION_PTRS.u_nav_mover_component_consume_nav_movement_data,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAirMovementUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryMoveToFallAlongSurface"),
                &raw mut __FUNCTION_PTRS
                    .u_air_movement_utils_try_move_to_fall_along_surface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestFallingMoveAlongHitSurface"),
                &raw mut __FUNCTION_PTRS
                    .u_air_movement_utils_test_falling_move_along_hit_surface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidLandingSpot"),
                &raw mut __FUNCTION_PTRS.u_air_movement_utils_is_valid_landing_spot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeControlledFreeMove"),
                &raw mut __FUNCTION_PTRS
                    .u_air_movement_utils_compute_controlled_free_move,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAsyncMovementUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestSlidingMoveAlongHitSurface"),
                &raw mut __FUNCTION_PTRS
                    .u_async_movement_utils_test_sliding_move_along_hit_surface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestDepenetratingMove"),
                &raw mut __FUNCTION_PTRS.u_async_movement_utils_test_depenetrating_move,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBasedMovementUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformWorldRotatorToBased"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_rotator_to_based,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformWorldLocationToBased"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_location_to_based,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformWorldDirectionToBased"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_direction_to_based,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformRotatorToWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_rotator_to_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformRotatorToLocal"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_rotator_to_local,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformLocationToWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_location_to_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformLocationToLocal"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_location_to_local,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformDirectionToWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_direction_to_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformDirectionToLocal"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_direction_to_local,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformBasedRotatorToWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_rotator_to_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformBasedLocationToWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_location_to_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TransformBasedDirectionToWorld"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_direction_to_world,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsBaseSimulatingPhysics"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_is_base_simulating_physics,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsADynamicBase"),
                &raw mut __FUNCTION_PTRS.u_based_movement_utils_is_a_dynamic_base,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMovementBaseTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_based_movement_utils_get_movement_base_transform,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPlanarConstraintUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlaneConstraintOrigin"),
                &raw mut __FUNCTION_PTRS
                    .u_planar_constraint_utils_set_plane_constraint_origin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlanarConstraintNormal"),
                &raw mut __FUNCTION_PTRS
                    .u_planar_constraint_utils_set_planar_constraint_normal,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlanarConstraintEnabled"),
                &raw mut __FUNCTION_PTRS
                    .u_planar_constraint_utils_set_planar_constraint_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConstrainNormalToPlane"),
                &raw mut __FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_normal_to_plane,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConstrainLocationToPlane"),
                &raw mut __FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_location_to_plane,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConstrainDirectionToPlane"),
                &raw mut __FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_direction_to_plane,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UFloorQueryUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsHitSurfaceWalkable"),
                &raw mut __FUNCTION_PTRS.u_floor_query_utils_is_hit_surface_walkable,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGroundMovementUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryWalkToSlideAlongSurface"),
                &raw mut __FUNCTION_PTRS
                    .u_ground_movement_utils_try_walk_to_slide_along_surface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestMoveToKeepMinHeightAboveFloor"),
                &raw mut __FUNCTION_PTRS
                    .u_ground_movement_utils_test_move_to_keep_min_height_above_floor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestMoveToAdjustToFloor"),
                &raw mut __FUNCTION_PTRS
                    .u_ground_movement_utils_test_move_to_adjust_to_floor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestGroundedMoveAlongHitSurface"),
                &raw mut __FUNCTION_PTRS
                    .u_ground_movement_utils_test_grounded_move_along_hit_surface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeDeflectedMoveOntoRamp"),
                &raw mut __FUNCTION_PTRS
                    .u_ground_movement_utils_compute_deflected_move_onto_ramp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeControlledGroundMove"),
                &raw mut __FUNCTION_PTRS
                    .u_ground_movement_utils_compute_controlled_ground_move,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanStepUpOnHitSurface"),
                &raw mut __FUNCTION_PTRS
                    .u_ground_movement_utils_can_step_up_on_hit_surface,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UTurnGeneratorInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTurn"),
                &raw mut __FUNCTION_PTRS.u_turn_generator_interface_get_turn,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovementRecordUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_SetDeltaSeconds"),
                &raw mut __FUNCTION_PTRS.u_movement_record_utils_k2_set_delta_seconds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetTotalMoveDelta"),
                &raw mut __FUNCTION_PTRS.u_movement_record_utils_k2_get_total_move_delta,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetRelevantVelocity"),
                &raw mut __FUNCTION_PTRS.u_movement_record_utils_k2_get_relevant_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetRelevantMoveDelta"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_record_utils_k2_get_relevant_move_delta,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMovementUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TrySafeMoveUpdatedComponentNoMovementRecord"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_updated_component_no_movement_record,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TrySafeMoveUpdatedComponent"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_try_safe_move_updated_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "TrySafeMoveAndSlideUpdatedComponentNoMovementRecord",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_and_slide_updated_component_no_movement_record,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TrySafeMoveAndSlideUpdatedComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_and_slide_updated_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryMoveToSlideAlongSurfaceNoMovementRecord"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_try_move_to_slide_along_surface_no_movement_record,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TryMoveToSlideAlongSurface"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_try_move_to_slide_along_surface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestEncroachmentAndAdjust"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_test_encroachment_and_adjust,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TestEncroachment"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_test_encroachment,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProjectToGravityFloor"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_project_to_gravity_floor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsExceedingMaxSpeed"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_is_exceeding_max_speed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAngularVelocityZero"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_is_angular_velocity_zero,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGravityVerticalComponent"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_get_gravity_vertical_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindTeleportSpot"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_find_teleport_spot,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeduceUpDirectionFromGravity"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_deduce_up_direction_from_gravity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConstrainToPlane"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_constrain_to_plane,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeVelocityFromPositions"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_compute_velocity_from_positions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeVelocityFromGravity"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_compute_velocity_from_gravity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeVelocity"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_compute_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeSlideDelta"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_compute_slide_delta,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeDirectionIntent"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_compute_direction_intent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeCombinedVelocity"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_compute_combined_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeAngularVelocityDegrees"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_compute_angular_velocity_degrees,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeAngularVelocity"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_compute_angular_velocity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanEscapeGravity"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_can_escape_gravity,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyGravityToOrientationIntent"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_apply_gravity_to_orientation_intent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyAngularVelocityToRotator"),
                &raw mut __FUNCTION_PTRS
                    .u_movement_utils_apply_angular_velocity_to_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyAngularVelocityToQuat"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_apply_angular_velocity_to_quat,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ApplyAngularVelocity"),
                &raw mut __FUNCTION_PTRS.u_movement_utils_apply_angular_velocity,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPlayMoverMontageCallbackProxy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMoverMontageEnded"),
                &raw mut __FUNCTION_PTRS
                    .u_play_mover_montage_callback_proxy_on_mover_montage_ended,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateProxyObjectForPlayMoverMontage"),
                &raw mut __FUNCTION_PTRS
                    .u_play_mover_montage_callback_proxy_create_proxy_object_for_play_mover_montage,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UWaterMovementUtils::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpdateWaterSplineData"),
                &raw mut __FUNCTION_PTRS.u_water_movement_utils_update_water_spline_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeControlledWaterMove"),
                &raw mut __FUNCTION_PTRS
                    .u_water_movement_utils_compute_controlled_water_move,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBaseMovementModeTransition::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Trigger"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_transition_trigger,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnUnregistered"),
                &raw mut __FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_on_unregistered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_OnRegistered"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_transition_k2_on_registered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetMoverComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_get_mover_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Evaluate"),
                &raw mut __FUNCTION_PTRS.u_base_movement_mode_transition_evaluate,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverDataModelBlueprintLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVelocityInput"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_set_velocity_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDirectionalInput"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_set_directional_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVelocityFromSyncState"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_velocity_from_sync_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOrientationFromSyncState"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_orientation_from_sync_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMoveDirectionIntentFromSyncState"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_move_direction_intent_from_sync_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMoveDirectionIntentFromInputs"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_move_direction_intent_from_inputs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocationFromSyncState"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_location_from_sync_state,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAngularVelocityDegreesFromSyncState"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_angular_velocity_degrees_from_sync_state,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverTrajectoryPredictor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Setup"),
                &raw mut __FUNCTION_PTRS.u_mover_trajectory_predictor_setup,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverSimulation::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRollbackBlackboard_Mutable"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_simulation_get_rollback_blackboard_mutable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRollbackBlackboard"),
                &raw mut __FUNCTION_PTRS.u_mover_simulation_get_rollback_blackboard,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboard_Mutable"),
                &raw mut __FUNCTION_PTRS.u_mover_simulation_get_blackboard_mutable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBlackboard"),
                &raw mut __FUNCTION_PTRS.u_mover_simulation_get_blackboard,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AttemptTeleport"),
                &raw mut __FUNCTION_PTRS.u_mover_simulation_attempt_teleport,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverInputProducerInterface::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProduceInput"),
                &raw mut __FUNCTION_PTRS.u_mover_input_producer_interface_produce_input,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UMoverDataCollectionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_GetDataFromCollection"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_collection_library_k2_get_data_from_collection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("K2_AddDataToCollection"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_collection_library_k2_add_data_to_collection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearDataFromCollection"),
                &raw mut __FUNCTION_PTRS
                    .u_mover_data_collection_library_clear_data_from_collection,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPathedPhysicsMovementMode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUseJointConstraint"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_set_use_joint_constraint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPathDuration_BeginPlayOnly"),
                &raw mut __FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_set_path_duration_begin_play_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BP_FindPattern"),
                &raw mut __FUNCTION_PTRS.u_pathed_physics_movement_mode_bp_find_pattern,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = ULookAtRotationPattern::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRelativeLookAtLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_look_at_rotation_pattern_set_relative_look_at_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLookAtLocation"),
                &raw mut __FUNCTION_PTRS.u_look_at_rotation_pattern_set_look_at_location,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UPhysicsCharacterMoverComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMoverPreMovement"),
                &raw mut __FUNCTION_PTRS
                    .u_physics_character_mover_component_on_mover_pre_movement,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnMoverPostSimulationTick"),
                &raw mut __FUNCTION_PTRS
                    .u_physics_character_mover_component_on_mover_post_simulation_tick,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMoverAuxStateContext {
    pub aux_state_collection: FMoverDataCollection,
}
impl FMoverAuxStateContext {}
#[repr(C, align(8))]
pub struct FMoverDataCollection {
    pub(crate) __padding_end: [u8; 16],
}
impl FMoverDataCollection {}
#[repr(C, align(8))]
pub struct FMoverSyncState {
    pub movement_mode: FName,
    pub layered_moves: FLayeredMoveGroup,
    pub layered_move_instances: FLayeredMoveInstanceGroup,
    pub movement_modifiers: FMovementModifierGroup,
    pub sync_state_collection: FMoverDataCollection,
}
impl FMoverSyncState {}
#[repr(C, align(8))]
pub struct FMovementModifierGroup {
    pub(crate) __padding_end: [u8; 32],
}
impl FMovementModifierGroup {}
#[repr(C, align(8))]
pub struct FLayeredMoveInstanceGroup {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub residual_clamping: f32,
    pub b_apply_residual_velocity: bool,
    pub residual_velocity: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 16],
}
impl FLayeredMoveInstanceGroup {}
#[repr(C, align(8))]
pub struct FLayeredMoveGroup {
    pub residual_velocity: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 48],
    pub residual_clamping: f32,
    pub b_apply_residual_velocity: bool,
}
impl FLayeredMoveGroup {}
#[repr(C, align(8))]
pub struct FMoverTimeStep {
    pub server_frame: i32,
    pub base_sim_time_ms: f64,
    pub step_ms: f32,
}
impl FMoverTimeStep {}
#[repr(C, align(8))]
pub struct FMoverInputCmdContext {
    pub input_collection: FMoverDataCollection,
}
impl FMoverInputCmdContext {}
#[repr(C, align(8))]
pub struct FProposedMove {
    pub preferred_mode: FName,
    pub direction_intent: crate::bindings::core_u_object::FVector,
    pub linear_velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity_degrees: crate::bindings::core_u_object::FVector,
    pub flags_88: u8,
    pub mix_mode: EMoveMixMode,
}
impl FProposedMove {}
#[repr(C, align(8))]
pub struct FMoverTickStartData {
    pub input_cmd: FMoverInputCmdContext,
    pub sync_state: FMoverSyncState,
    pub aux_state: FMoverAuxStateContext,
}
impl FMoverTickStartData {}
#[repr(C, align(8))]
pub struct FMoverDataStructBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FMoverDataStructBase {}
#[repr(C, align(8))]
pub struct FMoverDictionaryData {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bool_values: TMap<FName, bool>,
    pub int_values: TMap<FName, i32>,
    pub float_values: TMap<FName, f64>,
    pub vector_values: TMap<FName, crate::bindings::core_u_object::FVector>,
    pub rotator_values: TMap<FName, crate::bindings::core_u_object::FRotator>,
    pub name_values: TMap<FName, FName>,
}
impl FMoverDictionaryData {}
#[repr(C, align(8))]
pub struct FFloorResultData {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub floor_result: FFloorCheckResult,
}
impl FFloorResultData {}
#[repr(C, align(8))]
pub struct FFloorCheckResult {
    pub flags_0: u8,
    pub line_dist: f32,
    pub floor_dist: f32,
    pub hit_result: crate::bindings::engine::FHitResult,
}
impl FFloorCheckResult {}
#[repr(C, align(8))]
pub struct FInstantMovementEffect {
    pub(crate) __padding_end: [u8; 8],
}
impl FInstantMovementEffect {}
#[repr(C, align(8))]
pub struct FTeleportEffect {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target_location: crate::bindings::core_u_object::FVector,
    pub b_use_actor_rotation: bool,
    pub target_rotation: crate::bindings::core_u_object::FRotator,
}
impl FTeleportEffect {}
#[repr(C, align(8))]
pub struct FAsyncTeleportEffect {
    pub(crate) __padding_end: [u8; 64],
}
impl FAsyncTeleportEffect {}
#[repr(C, align(8))]
pub struct FJumpImpulseEffect {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub upwards_speed: f32,
}
impl FJumpImpulseEffect {}
#[repr(C, align(8))]
pub struct FApplyVelocityEffect {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub velocity_to_apply: crate::bindings::core_u_object::FVector,
    pub b_additive_velocity: bool,
    pub force_movement_mode: FName,
}
impl FApplyVelocityEffect {}
#[repr(C, align(8))]
pub struct FLayeredMoveBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub mix_mode: EMoveMixMode,
    pub priority: u8,
    pub duration_ms: f32,
    pub start_sim_time_ms: f64,
    pub finish_velocity_settings: FLayeredMoveFinishVelocitySettings,
}
impl FLayeredMoveBase {}
#[repr(C, align(8))]
pub struct FLayeredMoveFinishVelocitySettings {
    pub set_velocity: crate::bindings::core_u_object::FVector,
    pub clamp_velocity: f32,
    pub finish_velocity_mode: ELayeredMoveFinishVelocityMode,
}
impl FLayeredMoveFinishVelocitySettings {}
#[repr(C, align(8))]
pub struct FLayeredMove_MontageStateProvider {
    pub(crate) __padding_end: [u8; 56],
}
impl FLayeredMove_MontageStateProvider {}
#[repr(C, align(8))]
pub struct FLayeredMove_AnimRootMotion {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub montage_state: FMoverAnimMontageState,
}
impl FLayeredMove_AnimRootMotion {}
#[repr(C, align(8))]
pub struct FMoverAnimMontageState {
    pub montage: UPtr<crate::bindings::engine::UAnimMontage>,
    pub starting_montage_position: f32,
    pub play_rate: f32,
    pub current_position: f32,
}
impl FMoverAnimMontageState {}
#[repr(C, align(8))]
pub struct FLayeredMove_LinearVelocity {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub velocity: crate::bindings::core_u_object::FVector,
    pub magnitude_over_time: UPtr<crate::bindings::engine::UCurveFloat>,
    pub settings_flags: u8,
}
impl FLayeredMove_LinearVelocity {}
#[repr(C, align(8))]
pub struct FLayeredMove_JumpImpulseOverDuration {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub upwards_speed: f32,
}
impl FLayeredMove_JumpImpulseOverDuration {}
#[repr(C, align(8))]
pub struct FLayeredMove_JumpTo {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub jump_distance: f32,
    pub jump_height: f32,
    pub b_use_actor_rotation: bool,
    pub jump_rotation: crate::bindings::core_u_object::FRotator,
    pub path_offset_curve: UPtr<crate::bindings::engine::UCurveVector>,
    pub time_mapping_curve: UPtr<crate::bindings::engine::UCurveFloat>,
}
impl FLayeredMove_JumpTo {}
#[repr(C, align(8))]
pub struct FLayeredMove_MoveTo {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub start_location: crate::bindings::core_u_object::FVector,
    pub target_location: crate::bindings::core_u_object::FVector,
    pub b_restrict_speed_to_expected: bool,
    pub path_offset_curve: UPtr<crate::bindings::engine::UCurveVector>,
    pub time_mapping_curve: UPtr<crate::bindings::engine::UCurveFloat>,
}
impl FLayeredMove_MoveTo {}
#[repr(C, align(8))]
pub struct FLayeredMove_MoveToDynamic {
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 128],
    pub location_actor: UPtr<crate::bindings::engine::AActor>,
}
impl FLayeredMove_MoveToDynamic {}
#[repr(C, align(8))]
pub struct FLayeredMove_RadialImpulse {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub location: crate::bindings::core_u_object::FVector,
    pub location_actor: UPtr<crate::bindings::engine::AActor>,
    pub radius: f32,
    pub magnitude: f32,
    pub b_is_push: bool,
    pub b_no_vertical_velocity: bool,
    pub distance_falloff: UPtr<crate::bindings::engine::UCurveFloat>,
    pub magnitude_over_time: UPtr<crate::bindings::engine::UCurveFloat>,
    pub b_use_fixed_world_direction: bool,
    pub fixed_world_direction: crate::bindings::core_u_object::FRotator,
}
impl FLayeredMove_RadialImpulse {}
#[repr(C, align(8))]
pub struct FLayeredMoveActivationParams {
    pub duration_ms: f64,
}
impl FLayeredMoveActivationParams {}
#[repr(C, align(8))]
pub struct FLaunchMoveActivationParams {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub launch_velocity: crate::bindings::core_u_object::FVector,
    pub force_movement_mode: FName,
}
impl FLaunchMoveActivationParams {}
#[repr(C, align(8))]
pub struct FLayeredMoveInstancedData {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub duration_ms: f64,
    pub start_sim_time_ms: f64,
}
impl FLayeredMoveInstancedData {}
#[repr(C, align(8))]
pub struct FLaunchMoveData {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub launch_velocity: crate::bindings::core_u_object::FVector,
    pub force_movement_mode: FName,
}
impl FLaunchMoveData {}
#[repr(C, align(8))]
pub struct FLayeredMove_Launch {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub launch_velocity: crate::bindings::core_u_object::FVector,
    pub force_movement_mode: FName,
}
impl FLayeredMove_Launch {}
#[repr(C, align(8))]
pub struct FLayeredMove_MultiJump {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub maximum_in_air_jumps: i32,
    pub upwards_speed: f32,
    pub jumps_in_air_remaining: i32,
    pub time_of_last_jump_ms: f64,
}
impl FLayeredMove_MultiJump {}
#[repr(C, align(16))]
pub struct FLayeredMove_RootMotionAttribute {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub b_constrain_world_rot_to_movement_plane: bool,
    pub(crate) __padding_end: [u8; 135],
}
impl FLayeredMove_RootMotionAttribute {}
#[repr(C, align(8))]
pub struct FSimpleSpringState {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub current_accel: crate::bindings::core_u_object::FVector,
}
impl FSimpleSpringState {}
#[repr(C, align(16))]
pub struct FSmoothWalkingState {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub spring_velocity: crate::bindings::core_u_object::FVector,
    pub spring_acceleration: crate::bindings::core_u_object::FVector,
    pub intermediate_velocity: crate::bindings::core_u_object::FVector,
    pub intermediate_facing: crate::bindings::core_u_object::FQuat,
    pub intermediate_angular_velocity: crate::bindings::core_u_object::FVector,
}
impl FSmoothWalkingState {}
#[repr(C, align(4))]
pub struct FSwimmingControlSettings {
    pub cancel_crouch_immersion_depth: f32,
    pub dbno_swim_immersion_depth: f32,
    pub max_acceleration: f32,
    pub max_acceleration_sprinting: f32,
    pub braking_deceleration: f32,
    pub max_normal_speed: f32,
    pub max_sprint_speed: f32,
    pub min_sprint_jump_speed: f32,
    pub sprint_jump_air_accel_angle_limit: f32,
    pub sprint_retrigger_delay: f32,
    pub sprint_delay_after_firing: f32,
    pub max_targeting_speed: f32,
    pub backwards_speed_multiplier: f32,
    pub backwards_speed_cos_angle: f32,
    pub angled_speed_multiplier: f32,
    pub angled_speed_cos_angle: f32,
    pub friction: f32,
    pub friction_sprinting: f32,
    pub friction_direction_change_dot: f32,
    pub friction_direction_change_multiplier: f32,
    pub max_speed_up: f32,
    pub max_speed_down: f32,
    pub max_horizontal_entry_speed: f32,
    pub water_force_multiplier: f32,
    pub water_force_second_multiplier: f32,
    pub max_water_force: f32,
    pub water_velocity_depth_for_max: f32,
    pub water_velocity_min_multiplier: f32,
    pub water_sim_max_time_step: f32,
    pub water_sim_sub_step_time: f32,
    pub bobbing_max_force: f32,
    pub bobbing_ideal_depth_tolerance: f32,
    pub bobbing_friction_down: f32,
    pub bobbing_exp_drag_down: f32,
    pub bobbing_friction_down_submerged: f32,
    pub bobbing_exp_drag_down_submerged: f32,
    pub bobbing_friction_up: f32,
    pub bobbing_exp_drag_up: f32,
    pub bobbing_friction_multiplier: f32,
    pub bobbing_exp_drag_multiplier: f32,
    pub boost_drag_multiplier: f32,
    pub jump_multiplier: f32,
}
impl FSwimmingControlSettings {}
#[repr(C, align(8))]
pub struct FMovementModifierBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub duration_ms: f32,
    pub start_sim_time_ms: f64,
    pub(crate) __padding_end: [u8; 8],
}
impl FMovementModifierBase {}
#[repr(C, align(2))]
pub struct FMovementModifierHandle {
    pub(crate) __padding_end: [u8; 2],
}
impl FMovementModifierHandle {}
#[repr(C, align(8))]
pub struct FStanceModifier {
    pub(crate) __padding_end: [u8; 40],
}
impl FStanceModifier {}
#[repr(C, align(8))]
pub struct FMoverInputContainerDataStruct {
    pub(crate) __padding_end: [u8; 24],
}
impl FMoverInputContainerDataStruct {}
#[repr(C, align(16))]
pub struct FFreeMoveParams {
    pub move_input_type: EMoveInputType,
    pub move_input: crate::bindings::core_u_object::FVector,
    pub orientation_intent: crate::bindings::core_u_object::FRotator,
    pub prior_velocity: crate::bindings::core_u_object::FVector,
    pub prior_orientation: crate::bindings::core_u_object::FRotator,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub turning_boost: f32,
    pub turning_rate: f32,
    pub delta_seconds: f32,
    pub friction: f32,
    pub world_to_gravity_quat: crate::bindings::core_u_object::FQuat,
    pub b_use_acceleration_for_velocity_move: bool,
}
impl FFreeMoveParams {}
#[repr(C, align(16))]
pub struct FRelativeBaseInfo {
    pub movement_base: TWeakObjectPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub bone_name: FName,
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub contact_local_position: crate::bindings::core_u_object::FVector,
}
impl FRelativeBaseInfo {}
#[repr(C, align(8))]
pub struct FPlanarConstraint {
    pub b_constrain_to_plane: bool,
    pub plane_constraint_normal: crate::bindings::core_u_object::FVector,
    pub plane_constraint_origin: crate::bindings::core_u_object::FVector,
}
impl FPlanarConstraint {}
#[repr(C, align(16))]
pub struct FGroundMoveParams {
    pub move_input_type: EMoveInputType,
    pub move_input: crate::bindings::core_u_object::FVector,
    pub orientation_intent: crate::bindings::core_u_object::FRotator,
    pub prior_velocity: crate::bindings::core_u_object::FVector,
    pub prior_orientation: crate::bindings::core_u_object::FRotator,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub friction: f32,
    pub turning_rate: f32,
    pub turning_boost: f32,
    pub ground_normal: crate::bindings::core_u_object::FVector,
    pub delta_seconds: f32,
    pub up_direction: crate::bindings::core_u_object::FVector,
    pub world_to_gravity_quat: crate::bindings::core_u_object::FQuat,
    pub b_use_acceleration_for_velocity_move: bool,
}
impl FGroundMoveParams {}
#[repr(C, align(8))]
pub struct FMovementSubstep {
    pub(crate) __padding_end: [u8; 48],
}
impl FMovementSubstep {}
#[repr(C, align(8))]
pub struct FMovementRecord {
    pub(crate) __padding_end: [u8; 72],
}
impl FMovementRecord {}
#[repr(C, align(16))]
pub struct FTrajectorySampleInfo {
    pub transform: crate::bindings::core_u_object::FTransform,
    pub linear_velocity: crate::bindings::core_u_object::FVector,
    pub instantaneous_acceleration: crate::bindings::core_u_object::FVector,
    pub angular_velocity: crate::bindings::core_u_object::FRotator,
    pub sim_time_ms: f64,
}
impl FTrajectorySampleInfo {}
#[repr(C, align(8))]
pub struct FComputeVelocityParams {
    pub delta_seconds: f32,
    pub initial_velocity: crate::bindings::core_u_object::FVector,
    pub move_direction_intent: crate::bindings::core_u_object::FVector,
    pub max_speed: f32,
    pub turning_boost: f32,
    pub friction: f32,
    pub deceleration: f32,
    pub acceleration: f32,
    pub move_input: crate::bindings::core_u_object::FVector,
    pub move_input_type: EMoveInputType,
    pub b_use_acceleration_for_velocity_move: bool,
}
impl FComputeVelocityParams {}
#[repr(C, align(8))]
pub struct FComputeCombinedVelocityParams {
    pub delta_seconds: f32,
    pub initial_velocity: crate::bindings::core_u_object::FVector,
    pub move_direction_intent: crate::bindings::core_u_object::FVector,
    pub max_speed: f32,
    pub turning_boost: f32,
    pub friction: f32,
    pub deceleration: f32,
    pub acceleration: f32,
    pub external_acceleration: crate::bindings::core_u_object::FVector,
    pub overall_max_speed: f32,
}
impl FComputeCombinedVelocityParams {}
#[repr(C, align(4))]
pub struct FMovingComponentSet {
    pub updated_component: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub updated_primitive: TWeakObjectPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub mover_component: TWeakObjectPtr<UMoverComponent>,
}
impl FMovingComponentSet {}
#[repr(C, align(16))]
pub struct FWaterMoveParams {
    pub move_input_type: EMoveInputType,
    pub move_input: crate::bindings::core_u_object::FVector,
    pub orientation_intent: crate::bindings::core_u_object::FRotator,
    pub prior_velocity: crate::bindings::core_u_object::FVector,
    pub prior_orientation: crate::bindings::core_u_object::FRotator,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub friction: f32,
    pub turning_rate: f32,
    pub turning_boost: f32,
    pub delta_seconds: f32,
    pub move_acceleration: crate::bindings::core_u_object::FVector,
    pub move_speed: f32,
    pub world_to_gravity_quat: crate::bindings::core_u_object::FQuat,
}
impl FWaterMoveParams {}
#[repr(C, align(8))]
pub struct FWaterFlowSplineData {
    pub(crate) __padding_end: [u8; 232],
}
impl FWaterFlowSplineData {}
#[repr(C, align(8))]
pub struct FWaterCheckResult {
    pub flags_0: u8,
    pub hit_result: crate::bindings::engine::FHitResult,
    pub water_spline_data: FWaterFlowSplineData,
}
impl FWaterCheckResult {}
#[repr(C, align(8))]
pub struct FUpdateWaterSplineDataParams {
    pub target_immersion_depth: f32,
    pub water_velocity_depth_for_max: f32,
    pub water_velocity_min_multiplier: f32,
    pub player_velocity: crate::bindings::core_u_object::FVector,
    pub player_location: crate::bindings::core_u_object::FVector,
    pub capsule_half_height: f32,
}
impl FUpdateWaterSplineDataParams {}
#[repr(C, align(4))]
pub struct FTransitionEvalResult {
    pub next_mode: FName,
}
impl FTransitionEvalResult {}
#[repr(C, align(8))]
pub struct FCharacterDefaultInputs {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub move_input_type: EMoveInputType,
    pub move_input: crate::bindings::core_u_object::FVector,
    pub orientation_intent: crate::bindings::core_u_object::FVector,
    pub control_rotation: crate::bindings::core_u_object::FRotator,
    pub suggested_movement_mode: FName,
    pub b_using_movement_base: bool,
    pub movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub movement_base_bone_name: FName,
    pub b_is_jump_just_pressed: bool,
    pub b_is_jump_pressed: bool,
}
impl FCharacterDefaultInputs {}
#[repr(C, align(16))]
pub struct FMoverDefaultSyncState {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub location: crate::bindings::core_u_object::FVector,
    pub orientation: crate::bindings::core_u_object::FRotator,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub angular_velocity_degrees: crate::bindings::core_u_object::FVector,
    pub move_direction_intent: crate::bindings::core_u_object::FVector,
    pub movement_base: TWeakObjectPtr<crate::bindings::engine::UPrimitiveComponent>,
    pub movement_base_bone_name: FName,
    pub movement_base_pos: crate::bindings::core_u_object::FVector,
    pub movement_base_quat: crate::bindings::core_u_object::FQuat,
}
impl FMoverDefaultSyncState {}
#[repr(C, align(4))]
pub struct FMovementModeTickEndState {
    pub remaining_ms: f32,
    pub next_mode_name: FName,
    pub b_ended_with_no_changes: bool,
}
impl FMovementModeTickEndState {}
#[repr(C, align(8))]
pub struct FMoverTickEndData {
    pub sync_state: FMoverSyncState,
    pub aux_state: FMoverAuxStateContext,
    pub movement_end_state: FMovementModeTickEndState,
    pub(crate) __padding_end: [u8; 76],
}
impl FMoverTickEndData {}
#[repr(C, align(8))]
pub struct FSimulationTickParams {
    pub moving_comps: FMovingComponentSet,
    pub sim_blackboard: UPtr<UMoverBlackboard>,
    pub start_state: FMoverTickStartData,
    pub time_step: FMoverTimeStep,
    pub proposed_move: FProposedMove,
}
impl FSimulationTickParams {}
#[repr(C, align(8))]
pub struct FMoverPredictTrajectoryParams {
    pub num_prediction_samples: i32,
    pub seconds_per_sample: f32,
    pub b_use_visual_component_root: bool,
    pub b_disable_gravity: bool,
    pub optional_start_sync_state: TOptional<FMoverSyncState>,
    pub optional_start_aux_state: TOptional<FMoverAuxStateContext>,
    pub optional_input_cmds: TArray<FMoverInputCmdContext>,
}
impl FMoverPredictTrajectoryParams {}
#[repr(C, align(8))]
pub struct FMoverOnImpactParams {
    pub movement_mode_name: FName,
    pub hit_result: crate::bindings::engine::FHitResult,
    pub attempted_move_delta: crate::bindings::core_u_object::FVector,
}
impl FMoverOnImpactParams {}
#[repr(C, align(8))]
pub struct FMoverDataPersistence {
    pub required_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub b_copy_from_prior_frame: bool,
}
impl FMoverDataPersistence {}
#[repr(C, align(8))]
pub struct FApplyVelocityPhysicsEffect {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub velocity_to_apply: crate::bindings::core_u_object::FVector,
    pub b_additive_velocity: bool,
    pub force_movement_mode: FName,
}
impl FApplyVelocityPhysicsEffect {}
#[repr(C, align(8))]
pub struct FPhysicsStanceModifier {
    pub(crate) __padding_end: [u8; 40],
}
impl FPhysicsStanceModifier {}
#[repr(C, align(16))]
pub struct FPathedPhysicsMovementInputs {
    pub(crate) __padding_end: [u8; 128],
}
impl FPathedPhysicsMovementInputs {}
#[repr(C, align(16))]
pub struct FPathedPhysicsMovementState {
    pub(crate) __padding_end: [u8; 128],
}
impl FPathedPhysicsMovementState {}
#[repr(C, align(8))]
pub struct FPointMovementPathPoint {
    pub basis: EPointMovementLocationBasis,
    pub location: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 32],
}
impl FPointMovementPathPoint {}
#[repr(C, align(8))]
pub struct FMovementSettingsInputs {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub max_speed: f32,
    pub acceleration: f32,
}
impl FMovementSettingsInputs {}
#[repr(C, align(8))]
pub struct FMoverAIInputs {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub rvo_velocity_delta: crate::bindings::core_u_object::FVector,
}
impl FMoverAIInputs {}
#[repr(C, align(8))]
pub struct FMoverLaunchInputs {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub launch_velocity: crate::bindings::core_u_object::FVector,
    pub mode: EMoverLaunchVelocityMode,
}
impl FMoverLaunchInputs {}
#[repr(C, align(8))]
pub struct FMoverUserDefinedDataStruct {
    pub(crate) __padding_end: [u8; 24],
}
impl FMoverUserDefinedDataStruct {}
#[repr(C, align(8))]
pub struct UCommonLegacyMovementSettings {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub b_should_remain_vertical: bool,
    pub ground_movement_mode_name: FName,
    pub air_movement_mode_name: FName,
    pub swimming_movement_mode_name: FName,
    pub max_walk_slope_cosine: f32,
    pub floor_sweep_distance: f32,
    pub b_use_flat_base_for_floor_checks: bool,
    pub max_step_height: f32,
    pub max_speed: f32,
    pub b_use_acceleration_for_velocity_move: bool,
    pub ground_friction: f32,
    pub flags_124: u8,
    pub braking_friction: f32,
    pub braking_friction_factor: f32,
    pub deceleration: f32,
    pub acceleration: f32,
    pub turning_rate: f32,
    pub turning_boost: f32,
    pub b_ignore_base_rotation: bool,
    pub jump_upwards_speed: f32,
    pub swimming_start_immersion_depth: f32,
    pub swimming_ideal_immersion_depth: f32,
    pub swimming_stop_immersion_depth: f32,
}
impl UCommonLegacyMovementSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCommonLegacyMovementSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCommonLegacyMovementSettings")
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
pub struct IMoverBackendLiaisonInterface {}
#[repr(C, align(8))]
pub struct UMoverBackendLiaisonInterface {
    __padding_end: [u8; 48],
}
impl UMoverBackendLiaisonInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverBackendLiaisonInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverBackendLiaisonInterface")
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
pub struct UStanceSettings {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub crouching_max_acceleration: f32,
    pub crouching_max_speed: f32,
    pub crouch_half_height: f32,
    pub crouched_eye_height: f32,
}
impl UStanceSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStanceSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStanceSettings")
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
pub struct UMoverComponent {
    #[doc(hidden)]
    pub(crate) __padding_528: [u8; 528],
    pub movement_modes: TMap<FName, UPtr<UBaseMovementMode>>,
    pub starting_movement_mode: FName,
    pub transitions: TArray<UPtr<UBaseMovementModeTransition>>,
    pub persistent_sync_state_data_types: TArray<FMoverDataPersistence>,
    pub input_producer: UPtr<crate::bindings::core_u_object::UObject>,
    pub b_gather_input_from_all_input_producer_components: bool,
    #[doc(hidden)]
    pub(crate) __padding_688: [u8; 16],
    pub movement_mixer: UPtr<UMovementMixer>,
    #[doc(hidden)]
    pub(crate) __padding_1736: [u8; 1040],
    pub smoothing_mode: EMoverSmoothingMode,
    pub flags_1737: u8,
    #[doc(hidden)]
    pub(crate) __padding_1776: [u8; 32],
    pub world_to_gravity_transform: crate::bindings::core_u_object::FQuat,
    pub gravity_to_world_transform: crate::bindings::core_u_object::FQuat,
    __padding_end: [u8; 32],
}
impl UMoverComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverComponent")
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
    pub fn unbind_process_generated_movement(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_unbind_process_generated_movement,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_unbind_process_generated_movement,
                __buffer,
            )
        };
    }
    pub fn try_get_floor_check_hit_result(
        &self,
        out_hit_result: &mut crate::bindings::engine::FHitResult,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<265>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_try_get_floor_check_hit_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hit_result,
                __buffer.add(0).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_try_get_floor_check_hit_result,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::FHitResult>()
                .swap(out_hit_result);
        }
        unsafe { __buffer.add(264).cast::<bool>().read() }
    }
    pub fn set_use_deferred_group_movement(&mut self, b_enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_use_deferred_group_movement,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_use_deferred_group_movement,
                __buffer,
            )
        };
    }
    pub fn set_up_direction_override(
        &mut self,
        b_override_up_direction: bool,
        up_direction: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_up_direction_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override_up_direction,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &up_direction,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_up_direction_override,
                __buffer,
            )
        };
    }
    pub fn set_updated_component(
        &mut self,
        new_updated_component: UPtr<crate::bindings::engine::USceneComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_updated_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_updated_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_updated_component,
                __buffer,
            )
        };
    }
    pub fn set_primary_visual_component(
        &mut self,
        scene_component: UPtr<crate::bindings::engine::USceneComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_primary_visual_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scene_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_primary_visual_component,
                __buffer,
            )
        };
    }
    pub fn set_planar_constraint(&mut self, in_constraint: &FPlanarConstraint) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_planar_constraint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_constraint,
                __buffer.add(0).cast::<FPlanarConstraint>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_planar_constraint,
                __buffer,
            )
        };
    }
    pub fn set_gravity_override(
        &mut self,
        b_override_gravity: bool,
        gravity_acceleration: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_gravity_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override_gravity,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &gravity_acceleration,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_gravity_override,
                __buffer,
            )
        };
    }
    pub fn set_base_visual_component_transform(
        &mut self,
        component_transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_base_visual_component_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                component_transform,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_set_base_visual_component_transform,
                __buffer,
            )
        };
    }
    pub fn remove_movement_mode(&mut self, mode_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_remove_movement_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mode_name,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_remove_movement_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn remove_gameplay_tags(
        &mut self,
        tags_to_remove: &crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_remove_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tags_to_remove,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_remove_gameplay_tags,
                __buffer,
            )
        };
    }
    pub fn remove_gameplay_tag(
        &mut self,
        tag_to_remove: crate::bindings::gameplay_tags::FGameplayTag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_remove_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_to_remove,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_remove_gameplay_tag,
                __buffer,
            )
        };
    }
    pub fn queue_next_mode(&mut self, desired_mode_name: FName, b_should_reenter: bool) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_queue_next_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &desired_mode_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_reenter,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_queue_next_mode,
                __buffer,
            )
        };
    }
    pub fn queue_layered_move_activation(
        &mut self,
        move_logic_class: TSubclassOf<ULayeredMoveLogic>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_queue_layered_move_activation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_logic_class,
                __buffer.add(0).cast::<TSubclassOf<ULayeredMoveLogic>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_queue_layered_move_activation,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn k2_unregister_move(&mut self, move_class: TSubclassOf<ULayeredMoveLogic>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_unregister_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_class,
                __buffer.add(0).cast::<TSubclassOf<ULayeredMoveLogic>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_unregister_move,
                __buffer,
            )
        };
    }
    pub fn k2_schedule_instant_movement_effect(&mut self, effect_as_raw_data: &i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_schedule_instant_movement_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_as_raw_data,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_schedule_instant_movement_effect,
                __buffer,
            )
        };
    }
    pub fn k2_register_moves(
        &mut self,
        move_classes: TArray<TSubclassOf<ULayeredMoveLogic>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_register_moves,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_classes,
                __buffer.add(0).cast::<TArray<TSubclassOf<ULayeredMoveLogic>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_register_moves,
                __buffer,
            )
        };
    }
    pub fn k2_register_move(&mut self, move_class: TSubclassOf<ULayeredMoveLogic>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_register_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_class,
                __buffer.add(0).cast::<TSubclassOf<ULayeredMoveLogic>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_register_move,
                __buffer,
            )
        };
    }
    pub fn k2_queue_movement_modifier(
        &mut self,
        move_as_raw_data: &i32,
    ) -> FMovementModifierHandle {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_movement_modifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_as_raw_data,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_movement_modifier,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FMovementModifierHandle>().read() }
    }
    pub fn k2_queue_layered_move_activation_with_context(
        &mut self,
        move_logic_class: TSubclassOf<ULayeredMoveLogic>,
        move_as_raw_data: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_layered_move_activation_with_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_logic_class,
                __buffer.add(0).cast::<TSubclassOf<ULayeredMoveLogic>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_as_raw_data,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_layered_move_activation_with_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn k2_queue_layered_move(&mut self, move_as_raw_data: &i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_layered_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_as_raw_data,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_layered_move,
                __buffer,
            )
        };
    }
    pub fn k2_queue_instant_movement_effect(&mut self, effect_as_raw_data: &i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_instant_movement_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                effect_as_raw_data,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_queue_instant_movement_effect,
                __buffer,
            )
        };
    }
    pub fn k2_find_movement_modifier(
        &self,
        modifier_handle: FMovementModifierHandle,
        b_found_modifier: &mut bool,
        target_as_raw_bytes: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_find_movement_modifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modifier_handle,
                __buffer.add(0).cast::<FMovementModifierHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_found_modifier,
                __buffer.add(2).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_as_raw_bytes,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_find_movement_modifier,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(2).cast::<bool>().swap(b_found_modifier);
        }
        unsafe {
            __buffer.add(4).cast::<i32>().swap(target_as_raw_bytes);
        }
    }
    pub fn k2_find_active_layered_move(
        &self,
        did_succeed: &mut bool,
        target_as_raw_bytes: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_find_active_layered_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                did_succeed,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_as_raw_bytes,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_k2_find_active_layered_move,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(did_succeed);
        }
        unsafe {
            __buffer.add(4).cast::<i32>().swap(target_as_raw_bytes);
        }
    }
    pub fn is_using_deferred_group_movement(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_is_using_deferred_group_movement,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_is_using_deferred_group_movement,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_modifier_active_or_queued(
        &self,
        modifier_handle: &FMovementModifierHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_is_modifier_active_or_queued,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                modifier_handle,
                __buffer.add(0).cast::<FMovementModifierHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_is_modifier_active_or_queued,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn has_valid_cached_state(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_valid_cached_state,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_valid_cached_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_valid_cached_input_cmd(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_valid_cached_input_cmd,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_valid_cached_input_cmd,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_gameplay_tag_in_state(
        &self,
        sync_state: &FMoverSyncState,
        tag_to_find: crate::bindings::gameplay_tags::FGameplayTag,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<238>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_gameplay_tag_in_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sync_state,
                __buffer.add(0).cast::<FMoverSyncState>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_to_find,
                __buffer.add(224).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
                __buffer.add(236).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_gameplay_tag_in_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(237).cast::<bool>().read() }
    }
    pub fn has_gameplay_tag(
        &self,
        tag_to_find: crate::bindings::gameplay_tags::FGameplayTag,
        b_exact_match: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_to_find,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exact_match,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_has_gameplay_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn handle_impact(&mut self, impact_params: &mut FMoverOnImpactParams) {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_mover_component_handle_impact,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                impact_params,
                __buffer.add(0).cast::<FMoverOnImpactParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_mover_component_handle_impact,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FMoverOnImpactParams>().swap(impact_params);
        }
    }
    pub fn get_velocity(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_mover_component_get_velocity,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_mover_component_get_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_up_direction(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_up_direction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_up_direction,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_updated_component(
        &self,
    ) -> UPtr<crate::bindings::engine::USceneComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_updated_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_updated_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::USceneComponent>>()
                .read()
        }
    }
    pub fn get_target_orientation(&self) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_target_orientation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_target_orientation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_sync_state(&self) -> FMoverSyncState {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_mover_component_get_sync_state,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_mover_component_get_sync_state,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FMoverSyncState>().read() }
    }
    pub fn get_sim_blackboard(&self) -> UPtr<UMoverBlackboard> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_sim_blackboard,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_sim_blackboard,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMoverBlackboard>>().read() }
    }
    pub fn get_primary_visual_component(
        &self,
    ) -> UPtr<crate::bindings::engine::USceneComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_primary_visual_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_primary_visual_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::USceneComponent>>()
                .read()
        }
    }
    pub fn get_predicted_trajectory(
        &mut self,
        prediction_params: FMoverPredictTrajectoryParams,
    ) -> TArray<FTrajectorySampleInfo> {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_predicted_trajectory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &prediction_params,
                __buffer.add(0).cast::<FMoverPredictTrajectoryParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_predicted_trajectory,
                __buffer,
            )
        };
        unsafe { __buffer.add(288).cast::<TArray<FTrajectorySampleInfo>>().read() }
    }
    pub fn get_planar_constraint(&self) -> FPlanarConstraint {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_planar_constraint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_planar_constraint,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FPlanarConstraint>().read() }
    }
    pub fn get_movement_mode_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_mode_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_mode_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_movement_mode(&self) -> UPtr<UBaseMovementMode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UBaseMovementMode>>().read() }
    }
    pub fn get_movement_intent(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_intent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_intent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_movement_base_bone_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_base_bone_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_base_bone_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_movement_base(
        &self,
    ) -> UPtr<crate::bindings::engine::UPrimitiveComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_base,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_movement_base,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>()
                .read()
        }
    }
    pub fn get_last_time_step(&self) -> FMoverTimeStep {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_last_time_step,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_last_time_step,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FMoverTimeStep>().read() }
    }
    pub fn get_last_input_cmd(&self) -> FMoverInputCmdContext {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_last_input_cmd,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_last_input_cmd,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FMoverInputCmdContext>().read() }
    }
    pub fn get_gravity_acceleration(&self) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_gravity_acceleration,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_gravity_acceleration,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_future_trajectory(
        &mut self,
        future_seconds: f32,
        samples_per_second: f32,
    ) -> TArray<FTrajectorySampleInfo> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_future_trajectory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &future_seconds,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &samples_per_second,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_future_trajectory,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<FTrajectorySampleInfo>>().read() }
    }
    pub fn get_base_visual_component_transform(
        &self,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_base_visual_component_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_get_base_visual_component_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn find_shared_settings_mutable_bp(
        &self,
        shared_setting: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_shared_settings_mutable_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shared_setting,
                __buffer
                    .add(0)
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_shared_settings_mutable_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn find_shared_settings_bp(
        &self,
        shared_setting: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_shared_settings_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shared_setting,
                __buffer
                    .add(0)
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_shared_settings_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn find_movement_mode_by_name(
        &self,
        movement_mode_name: FName,
    ) -> UPtr<UBaseMovementMode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_movement_mode_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode_name,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_movement_mode_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UBaseMovementMode>>().read() }
    }
    pub fn find_movement_mode(
        &self,
        movement_mode: TSubclassOf<UBaseMovementMode>,
    ) -> UPtr<UBaseMovementMode> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_movement_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode,
                __buffer.add(0).cast::<TSubclassOf<UBaseMovementMode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_find_movement_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UBaseMovementMode>>().read() }
    }
    pub fn cancel_modifier_from_handle(
        &mut self,
        modifier_handle: FMovementModifierHandle,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_cancel_modifier_from_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modifier_handle,
                __buffer.add(0).cast::<FMovementModifierHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_cancel_modifier_from_handle,
                __buffer,
            )
        };
    }
    pub fn cancel_features_with_tag(
        &mut self,
        tag_to_cancel: crate::bindings::gameplay_tags::FGameplayTag,
        b_require_exact_match: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_cancel_features_with_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_to_cancel,
                __buffer.add(0).cast::<crate::bindings::gameplay_tags::FGameplayTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_require_exact_match,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_cancel_features_with_tag,
                __buffer,
            )
        };
    }
    pub fn bind_process_generated_movement(
        &mut self,
        process_generated_movement_event: FBindProcessGeneratedMovement_ProcessGeneratedMovementEvent,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_bind_process_generated_movement,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &process_generated_movement_event,
                __buffer
                    .add(0)
                    .cast::<
                        FBindProcessGeneratedMovement_ProcessGeneratedMovementEvent,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_bind_process_generated_movement,
                __buffer,
            )
        };
    }
    pub fn add_movement_mode_from_object(
        &mut self,
        mode_name: FName,
        movement_mode: UPtr<UBaseMovementMode>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_movement_mode_from_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mode_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode,
                __buffer.add(16).cast::<UPtr<UBaseMovementMode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_movement_mode_from_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_movement_mode_from_class(
        &mut self,
        mode_name: FName,
        movement_mode: TSubclassOf<UBaseMovementMode>,
    ) -> UPtr<UBaseMovementMode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_movement_mode_from_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mode_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_mode,
                __buffer.add(16).cast::<TSubclassOf<UBaseMovementMode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_movement_mode_from_class,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UBaseMovementMode>>().read() }
    }
    pub fn add_gameplay_tags(
        &mut self,
        tags_to_add: &crate::bindings::gameplay_tags::FGameplayTagContainer,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_gameplay_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                tags_to_add,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_gameplay_tags,
                __buffer,
            )
        };
    }
    pub fn add_gameplay_tag(
        &mut self,
        tag_to_add: crate::bindings::gameplay_tags::FGameplayTag,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_gameplay_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_to_add,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_component_add_gameplay_tag,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMoverNetworkPhysicsLiaisonComponentBase {
    __padding_end: [u8; 1008],
}
impl UMoverNetworkPhysicsLiaisonComponentBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverNetworkPhysicsLiaisonComponentBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverNetworkPhysicsLiaisonComponentBase")
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
pub struct UMoverNetworkPhysicsLiaisonComponent {
    __padding_end: [u8; 1024],
}
impl UMoverNetworkPhysicsLiaisonComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverNetworkPhysicsLiaisonComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverNetworkPhysicsLiaisonComponent")
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
pub struct UMoverNetworkPredictionLiaisonComponent {
    __padding_end: [u8; 704],
}
impl UMoverNetworkPredictionLiaisonComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverNetworkPredictionLiaisonComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverNetworkPredictionLiaisonComponent")
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
pub struct UPathedPhysicsMoverComponent {
    #[doc(hidden)]
    pub(crate) __padding_1865: [u8; 1865],
    pub default_playback_behavior: EPathedPhysicsPlaybackBehavior,
    pub movement_start_delay: f32,
    __padding_end: [u8; 32],
}
impl UPathedPhysicsMoverComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedPhysicsMoverComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedPhysicsMoverComponent")
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
    pub fn set_playback_direction(&mut self, b_play_forward: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_playback_direction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_play_forward,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_playback_direction,
                __buffer,
            )
        };
    }
    pub fn set_path_origin_transform(
        &mut self,
        new_path_origin: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_path_origin_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_path_origin,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_path_origin_transform,
                __buffer,
            )
        };
    }
    pub fn set_is_moving(&mut self, b_should_move: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_is_moving,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_move,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_is_moving,
                __buffer,
            )
        };
    }
    pub fn set_default_playback_behavior(
        &mut self,
        playback_behavior: EPathedPhysicsPlaybackBehavior,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_default_playback_behavior,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &playback_behavior,
                __buffer.add(0).cast::<EPathedPhysicsPlaybackBehavior>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_set_default_playback_behavior,
                __buffer,
            )
        };
    }
    pub fn is_moving(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_is_moving,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_is_moving,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_joint_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_is_joint_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_is_joint_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_in_reverse(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_is_in_reverse,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_is_in_reverse,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_path_origin_transform(
        &self,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_get_path_origin_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_get_path_origin_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_default_playback_behavior(&self) -> EPathedPhysicsPlaybackBehavior {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_get_default_playback_behavior,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_mover_component_get_default_playback_behavior,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EPathedPhysicsPlaybackBehavior>().read() }
    }
}
#[repr(C, align(16))]
pub struct UMoverPathedPhysicsLiaisonComponent {
    __padding_end: [u8; 2112],
}
impl UMoverPathedPhysicsLiaisonComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverPathedPhysicsLiaisonComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverPathedPhysicsLiaisonComponent")
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
pub struct UMoverStandaloneLiaisonComponent {
    __padding_end: [u8; 1296],
}
impl UMoverStandaloneLiaisonComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverStandaloneLiaisonComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverStandaloneLiaisonComponent")
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
    pub fn set_use_async_produce_input(&mut self, b_use_async_input_production: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_use_async_produce_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_async_input_production,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_use_async_produce_input,
                __buffer,
            )
        };
    }
    pub fn set_use_async_movement_simulation_tick(
        &mut self,
        b_use_async_movement_sim: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_use_async_movement_simulation_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_async_movement_sim,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_use_async_movement_simulation_tick,
                __buffer,
            )
        };
    }
    pub fn set_enable_produce_input(&mut self, b_enable_input_production: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_enable_produce_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_input_production,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_set_enable_produce_input,
                __buffer,
            )
        };
    }
    pub fn get_use_async_produce_input(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_use_async_produce_input,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_use_async_produce_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_use_async_movement_simulation_tick(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_use_async_movement_simulation_tick,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_use_async_movement_simulation_tick,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_enable_produce_input(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_enable_produce_input,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_get_enable_produce_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_tick_dependency(
        &mut self,
        other_component: UPtr<crate::bindings::engine::UActorComponent>,
        tick_order: EMoverTickDependencyOrder,
        tick_phase: EMoverTickPhase,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_add_tick_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &other_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UActorComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tick_order,
                __buffer.add(8).cast::<EMoverTickDependencyOrder>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tick_phase,
                __buffer.add(9).cast::<EMoverTickPhase>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_standalone_liaison_component_add_tick_dependency,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMoverDebugComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub lookahead_seconds: f32,
    pub lookahead_samples_per_second: i32,
    pub b_show_trajectory: bool,
    pub b_show_trail: bool,
    pub b_show_corrections: bool,
    pub history_tracking_seconds: f32,
    pub history_samples_per_second: i32,
    __padding_end: [u8; 100],
}
impl UMoverDebugComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDebugComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDebugComponent")
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
    pub fn set_history_tracking(
        &mut self,
        seconds_to_track: f32,
        samples_per_second: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_debug_component_set_history_tracking,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &seconds_to_track,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &samples_per_second,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_debug_component_set_history_tracking,
                __buffer,
            )
        };
    }
    pub fn get_past_trajectory(&self) -> TArray<FTrajectorySampleInfo> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_debug_component_get_past_trajectory,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_debug_component_get_past_trajectory,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FTrajectorySampleInfo>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UCharacterMoverComponent {
    #[doc(hidden)]
    pub(crate) __padding_1890: [u8; 1890],
    pub flags_1890: u8,
    __padding_end: [u8; 29],
}
impl UCharacterMoverComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCharacterMoverComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCharacterMoverComponent")
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
    pub fn un_crouch(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_un_crouch,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_un_crouch,
                __buffer,
            )
        };
    }
    pub fn set_handle_stance_changes(&mut self, b_in_handle_stance_changes: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_set_handle_stance_changes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_handle_stance_changes,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_set_handle_stance_changes,
                __buffer,
            )
        };
    }
    pub fn set_handle_jump(&mut self, b_in_handle_jump: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_set_handle_jump,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_handle_jump,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_set_handle_jump,
                __buffer,
            )
        };
    }
    pub fn jump(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_character_mover_component_jump,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_character_mover_component_jump,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_swimming(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_swimming,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_swimming,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_slope_sliding(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_slope_sliding,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_slope_sliding,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_on_ground(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_on_ground,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_on_ground,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_flying(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_flying,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_flying,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_falling(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_falling,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_falling,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_crouching(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_crouching,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_crouching,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_airborne(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_airborne,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_is_airborne,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_handle_stance_changes(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_get_handle_stance_changes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_get_handle_stance_changes,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_handle_jump(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_get_handle_jump,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_get_handle_jump,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn crouch(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_crouch,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_crouch,
                __buffer,
            )
        };
    }
    pub fn can_crouch(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_can_crouch,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_can_crouch,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_actor_jump(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_can_actor_jump,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_character_mover_component_can_actor_jump,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULayeredMoveLogic {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub default_duration_ms: f64,
    pub mix_mode: EMoveMixMode,
    pub priority: u8,
    pub finish_velocity_settings: FLayeredMoveFinishVelocitySettings,
    pub instanced_data_struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    __padding_end: [u8; 16],
}
impl ULayeredMoveLogic {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayeredMoveLogic")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayeredMoveLogic")
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
    pub fn on_start(
        &mut self,
        time_step: &FMoverTimeStep,
        sim_blackboard: UPtr<UMoverBlackboard>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_layered_move_logic_on_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_step,
                __buffer.add(0).cast::<FMoverTimeStep>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_blackboard,
                __buffer.add(24).cast::<UPtr<UMoverBlackboard>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_layered_move_logic_on_start,
                __buffer,
            )
        };
    }
    pub fn on_end(
        &mut self,
        time_step: &FMoverTimeStep,
        sim_blackboard: UPtr<UMoverBlackboard>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_layered_move_logic_on_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_step,
                __buffer.add(0).cast::<FMoverTimeStep>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_blackboard,
                __buffer.add(24).cast::<UPtr<UMoverBlackboard>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_layered_move_logic_on_end,
                __buffer,
            )
        };
    }
    pub fn k2_set_active_move_data(
        move_logic: UPtr<ULayeredMoveLogic>,
        out_move_data: &FLayeredMoveInstancedData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_layered_move_logic_k2_set_active_move_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_logic,
                __buffer.add(0).cast::<UPtr<ULayeredMoveLogic>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_move_data,
                __buffer.add(8).cast::<FLayeredMoveInstancedData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::ULayeredMoveLogic::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_layered_move_logic_k2_set_active_move_data,
                __buffer,
            )
        };
    }
    pub fn k2_get_active_move_data(
        move_logic: UPtr<ULayeredMoveLogic>,
        out_move_data: &mut FLayeredMoveInstancedData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_layered_move_logic_k2_get_active_move_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_logic,
                __buffer.add(0).cast::<UPtr<ULayeredMoveLogic>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_move_data,
                __buffer.add(8).cast::<FLayeredMoveInstancedData>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::ULayeredMoveLogic::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_layered_move_logic_k2_get_active_move_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FLayeredMoveInstancedData>().swap(out_move_data);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn is_finished(
        &mut self,
        time_step: &FMoverTimeStep,
        sim_blackboard: UPtr<UMoverBlackboard>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_layered_move_logic_is_finished,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_step,
                __buffer.add(0).cast::<FMoverTimeStep>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_blackboard,
                __buffer.add(24).cast::<UPtr<UMoverBlackboard>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_layered_move_logic_is_finished,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn generate_move(
        &mut self,
        time_step: &FMoverTimeStep,
        sim_blackboard: UPtr<UMoverBlackboard>,
        start_state: &FMoverTickStartData,
        out_proposed_move: &mut FProposedMove,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<385>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_layered_move_logic_generate_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_step,
                __buffer.add(0).cast::<FMoverTimeStep>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_blackboard,
                __buffer.add(24).cast::<UPtr<UMoverBlackboard>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                start_state,
                __buffer.add(32).cast::<FMoverTickStartData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_proposed_move,
                __buffer.add(288).cast::<FProposedMove>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_layered_move_logic_generate_move,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(288).cast::<FProposedMove>().swap(out_proposed_move);
        }
        unsafe { __buffer.add(384).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULaunchMoveLogic {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub launch_velocity: crate::bindings::core_u_object::FVector,
    pub force_movement_mode: FName,
}
impl ULaunchMoveLogic {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULaunchMoveLogic")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULaunchMoveLogic")
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
pub struct UBaseMovementMode {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub transitions: TArray<UPtr<UBaseMovementModeTransition>>,
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 32],
    pub b_supports_async: bool,
}
impl UBaseMovementMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMovementMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMovementMode")
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
    pub fn simulation_tick(
        &mut self,
        params: &FSimulationTickParams,
        output_state: &mut FMoverTickEndData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<744>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_simulation_tick,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(0).cast::<FSimulationTickParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_state,
                __buffer.add(408).cast::<FMoverTickEndData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_simulation_tick,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(408).cast::<FMoverTickEndData>().swap(output_state);
        }
    }
    pub fn on_unregistered(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_unregistered,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_unregistered,
                __buffer,
            )
        };
    }
    pub fn on_registered(&mut self, mode_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_registered,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mode_name,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_registered,
                __buffer,
            )
        };
    }
    pub fn on_deactivated(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_deactivated,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_deactivated,
                __buffer,
            )
        };
    }
    pub fn on_activated(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_activated,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_on_activated,
                __buffer,
            )
        };
    }
    pub fn get_mover_component(&self) -> UPtr<UMoverComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_get_mover_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_k2_get_mover_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMoverComponent>>().read() }
    }
    pub fn generate_move(
        &self,
        start_state: &FMoverTickStartData,
        time_step: &FMoverTimeStep,
        out_proposed_move: &mut FProposedMove,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<376>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_generate_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                start_state,
                __buffer.add(0).cast::<FMoverTickStartData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_step,
                __buffer.add(256).cast::<FMoverTimeStep>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_proposed_move,
                __buffer.add(280).cast::<FProposedMove>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_generate_move,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(280).cast::<FProposedMove>().swap(out_proposed_move);
        }
    }
}
#[repr(C, align(8))]
pub struct UAsyncFallingMode {
    #[doc(hidden)]
    pub(crate) __padding_144: [u8; 144],
    pub b_cancel_vertical_speed_on_landing: bool,
    pub air_control_percentage: f32,
    pub falling_deceleration: f32,
    pub over_terminal_speed_falling_deceleration: f32,
    pub terminal_movement_plane_speed: f32,
    pub b_should_clamp_terminal_vertical_speed: bool,
    pub vertical_falling_deceleration: f32,
    pub terminal_vertical_speed: f32,
    __padding_end: [u8; 8],
}
impl UAsyncFallingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncFallingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncFallingMode")
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
    pub fn process_landed(
        &self,
        floor_result: &FFloorCheckResult,
        velocity: &mut crate::bindings::core_u_object::FVector,
        base_info: &mut FRelativeBaseInfo,
        tick_end_data: &mut FMoverTickEndData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<752>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_falling_mode_process_landed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                floor_result,
                __buffer.add(0).cast::<FFloorCheckResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                velocity,
                __buffer.add(280).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                base_info,
                __buffer.add(304).cast::<FRelativeBaseInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tick_end_data,
                __buffer.add(416).cast::<FMoverTickEndData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_falling_mode_process_landed,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(280)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(velocity);
        }
        unsafe {
            __buffer.add(304).cast::<FRelativeBaseInfo>().swap(base_info);
        }
        unsafe {
            __buffer.add(416).cast::<FMoverTickEndData>().swap(tick_end_data);
        }
    }
}
#[repr(C, align(8))]
pub struct UAsyncFlyingMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub b_respect_distance_over_walkable_surfaces: bool,
    __padding_end: [u8; 15],
}
impl UAsyncFlyingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncFlyingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncFlyingMode")
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
pub struct UAsyncNavWalkingMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub b_sweep_while_nav_walking: bool,
    pub b_project_nav_mesh_walking: bool,
    pub nav_mesh_projection_height_scale_up: f32,
    pub nav_mesh_projection_height_scale_down: f32,
    pub nav_mesh_projection_interval: f32,
    pub nav_mesh_projection_interp_speed: f32,
    pub behavior_off_nav_mesh: EOffNavMeshBehavior,
    pub b_slide_along_nav_mesh_edge: bool,
    #[doc(hidden)]
    pub(crate) __padding_208: [u8; 66],
    pub flags_208: u8,
    __padding_end: [u8; 23],
}
impl UAsyncNavWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncNavWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncNavWalkingMode")
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
    pub fn set_turn_generator_class(
        &mut self,
        turn_generator_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_nav_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turn_generator_class,
                __buffer
                    .add(0)
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_nav_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
    }
    pub fn get_turn_generator(
        &mut self,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_nav_walking_mode_get_turn_generator,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_nav_walking_mode_get_turn_generator,
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
}
#[repr(C, align(8))]
pub struct UAsyncWalkingMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub floor_check_policy: EStaticFloorCheckPolicy,
    __padding_end: [u8; 23],
}
impl UAsyncWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncWalkingMode")
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
    pub fn set_turn_generator_class(
        &mut self,
        turn_generator_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turn_generator_class,
                __buffer
                    .add(0)
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
    }
    pub fn get_turn_generator(
        &mut self,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_walking_mode_get_turn_generator,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_walking_mode_get_turn_generator,
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
}
#[repr(C, align(8))]
pub struct UFallingMode {
    #[doc(hidden)]
    pub(crate) __padding_144: [u8; 144],
    pub b_cancel_vertical_speed_on_landing: bool,
    pub air_control_percentage: f32,
    pub falling_deceleration: f32,
    pub falling_lateral_friction: f32,
    pub over_terminal_speed_falling_deceleration: f32,
    pub terminal_movement_plane_speed: f32,
    pub b_should_clamp_terminal_vertical_speed: bool,
    pub vertical_falling_deceleration: f32,
    pub terminal_vertical_speed: f32,
    __padding_end: [u8; 12],
}
impl UFallingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFallingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFallingMode")
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
    pub fn process_landed(
        &self,
        floor_result: &FFloorCheckResult,
        velocity: &mut crate::bindings::core_u_object::FVector,
        base_info: &mut FRelativeBaseInfo,
        tick_end_data: &mut FMoverTickEndData,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<752>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS.u_falling_mode_process_landed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                floor_result,
                __buffer.add(0).cast::<FFloorCheckResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                velocity,
                __buffer.add(280).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                base_info,
                __buffer.add(304).cast::<FRelativeBaseInfo>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                tick_end_data,
                __buffer.add(416).cast::<FMoverTickEndData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS.u_falling_mode_process_landed,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(280)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(velocity);
        }
        unsafe {
            __buffer.add(304).cast::<FRelativeBaseInfo>().swap(base_info);
        }
        unsafe {
            __buffer.add(416).cast::<FMoverTickEndData>().swap(tick_end_data);
        }
    }
}
#[repr(C, align(8))]
pub struct UFlyingMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub b_respect_distance_over_walkable_surfaces: bool,
    __padding_end: [u8; 15],
}
impl UFlyingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFlyingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFlyingMode")
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
pub struct UNavWalkingMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub b_sweep_while_nav_walking: bool,
    pub b_project_nav_mesh_walking: bool,
    pub nav_mesh_projection_height_scale_up: f32,
    pub nav_mesh_projection_height_scale_down: f32,
    pub nav_mesh_projection_interval: f32,
    pub nav_mesh_projection_interp_speed: f32,
    pub behavior_off_nav_mesh: EOffNavMeshBehavior,
    pub b_slide_along_nav_mesh_edge: bool,
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 40],
    pub nav_mover_component: UPtr<UNavMoverComponent>,
    #[doc(hidden)]
    pub(crate) __padding_208: [u8; 16],
    pub flags_208: u8,
    __padding_end: [u8; 31],
}
impl UNavWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavWalkingMode")
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
    pub fn set_turn_generator_class(
        &mut self,
        turn_generator_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turn_generator_class,
                __buffer
                    .add(0)
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
    }
    pub fn get_turn_generator(
        &mut self,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_walking_mode_get_turn_generator,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_walking_mode_get_turn_generator,
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
}
#[repr(C, align(8))]
pub struct UWalkingMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub floor_check_policy: EStaticFloorCheckPolicy,
    __padding_end: [u8; 23],
}
impl UWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWalkingMode")
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
    pub fn set_turn_generator_class(
        &mut self,
        turn_generator_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turn_generator_class,
                __buffer
                    .add(0)
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_walking_mode_set_turn_generator_class,
                __buffer,
            )
        };
    }
    pub fn get_turn_generator(
        &mut self,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_walking_mode_get_turn_generator,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_walking_mode_get_turn_generator,
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
}
#[repr(C, align(8))]
pub struct USimpleWalkingMode {
    #[doc(hidden)]
    pub(crate) __padding_144: [u8; 144],
    pub max_speed_override: f32,
}
impl USimpleWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleWalkingMode")
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
    pub fn generate_walk_move(
        &mut self,
        start_state: &mut FMoverTickStartData,
        delta_seconds: f32,
        desired_velocity: &crate::bindings::core_u_object::FVector,
        desired_facing: &crate::bindings::core_u_object::FQuat,
        current_facing: &crate::bindings::core_u_object::FQuat,
        in_out_angular_velocity_degrees: &mut crate::bindings::core_u_object::FVector,
        in_out_velocity: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<400>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_simple_walking_mode_generate_walk_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                start_state,
                __buffer.add(0).cast::<FMoverTickStartData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(256).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                desired_velocity,
                __buffer.add(264).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                desired_facing,
                __buffer.add(288).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                current_facing,
                __buffer.add(320).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_angular_velocity_degrees,
                __buffer.add(352).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_velocity,
                __buffer.add(376).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_simple_walking_mode_generate_walk_move,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FMoverTickStartData>().swap(start_state);
        }
        unsafe {
            __buffer
                .add(352)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(in_out_angular_velocity_degrees);
        }
        unsafe {
            __buffer
                .add(376)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(in_out_velocity);
        }
    }
}
#[repr(C, align(8))]
pub struct USimpleSpringWalkingMode {
    #[doc(hidden)]
    pub(crate) __padding_152: [u8; 152],
    pub velocity_smoothing_time: f32,
    pub facing_smoothing_time: f32,
    pub velocity_deadzone_threshold: f32,
}
impl USimpleSpringWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleSpringWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleSpringWalkingMode")
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
pub struct USmoothWalkingMode {
    #[doc(hidden)]
    pub(crate) __padding_152: [u8; 152],
    pub acceleration: f32,
    pub deceleration: f32,
    pub directional_acceleration_factor: f32,
    pub turning_strength: f32,
    pub acceleration_smoothing_time: f32,
    pub deceleration_smoothing_time: f32,
    pub acceleration_smoothing_compensation: f32,
    pub deceleration_smoothing_compensation: f32,
    pub velocity_deadzone_threshold: f32,
    pub acceleration_deadzone_threshold: f32,
    pub outside_influence_smoothing_time: f32,
    pub facing_smoothing_time: f32,
    pub b_smooth_facing_with_double_spring: bool,
    pub facing_deadzone_threshold: f32,
    pub angular_velocity_deadzone_threshold: f32,
}
impl USmoothWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothWalkingMode")
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
pub struct USwimmingMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub surface_swimming_water_control_settings: FSwimmingControlSettings,
    __padding_end: [u8; 16],
}
impl USwimmingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USwimmingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USwimmingMode")
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
pub struct UNavMoverComponent {
    #[doc(hidden)]
    pub(crate) __padding_316: [u8; 316],
    pub nav_movement_properties: crate::bindings::engine::FNavMovementProperties,
    __padding_end: [u8; 92],
}
impl UNavMoverComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavMoverComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNavMoverComponent")
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
    pub fn is_swimming(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_is_swimming,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_is_swimming,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_velocity_for_nav_movement(
        &self,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_get_velocity_for_nav_movement,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_get_velocity_for_nav_movement,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_max_speed_for_nav_movement(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_get_max_speed_for_nav_movement,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_get_max_speed_for_nav_movement,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn consume_nav_movement_data(
        &mut self,
        out_move_input_intent: &mut crate::bindings::core_u_object::FVector,
        out_move_input_velocity: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_consume_nav_movement_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_move_input_intent,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_move_input_velocity,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_nav_mover_component_consume_nav_movement_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_move_input_intent);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_move_input_velocity);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMotionWarpingMoverAdapter {
    __padding_end: [u8; 80],
}
impl UMotionWarpingMoverAdapter {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingMoverAdapter")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMotionWarpingMoverAdapter")
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
pub struct UAirMovementUtils {
    __padding_end: [u8; 48],
}
impl UAirMovementUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAirMovementUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAirMovementUtils")
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
    pub fn try_move_to_fall_along_surface(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        pct_of_delta_to_move: f32,
        rotation: crate::bindings::core_u_object::FQuat,
        normal: &crate::bindings::core_u_object::FVector,
        hit: &mut crate::bindings::engine::FHitResult,
        b_handle_impact: bool,
        floor_sweep_distance: f32,
        max_walk_slope_cosine: f32,
        b_use_flat_base_for_floor_checks: bool,
        out_floor_result: &mut FFloorCheckResult,
        move_record: &mut FMovementRecord,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<756>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_try_move_to_fall_along_surface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pct_of_delta_to_move,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normal,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(120).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_handle_impact,
                __buffer.add(384).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &floor_sweep_distance,
                __buffer.add(388).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(392).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_flat_base_for_floor_checks,
                __buffer.add(396).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_floor_result,
                __buffer.add(400).cast::<FFloorCheckResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_record,
                __buffer.add(680).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UAirMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_try_move_to_fall_along_surface,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(120).cast::<crate::bindings::engine::FHitResult>().swap(hit);
        }
        unsafe {
            __buffer.add(400).cast::<FFloorCheckResult>().swap(out_floor_result);
        }
        unsafe {
            __buffer.add(680).cast::<FMovementRecord>().swap(move_record);
        }
        unsafe { __buffer.add(752).cast::<f32>().read() }
    }
    pub fn test_falling_move_along_hit_surface(
        moving_comps: &FMovingComponentSet,
        original_move_delta: &crate::bindings::core_u_object::FVector,
        location_at_hit: &crate::bindings::core_u_object::FVector,
        target_rotation: &crate::bindings::core_u_object::FQuat,
        b_handle_impact: bool,
        floor_sweep_distance: f32,
        max_walk_slope_cosine: f32,
        b_use_flat_base_for_floor_checks: bool,
        in_out_hit: &mut crate::bindings::engine::FHitResult,
        out_floor_result: &mut FFloorCheckResult,
        in_out_move_record: &mut FMovementRecord,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<748>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_test_falling_move_along_hit_surface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                original_move_delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location_at_hit,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_rotation,
                __buffer.add(80).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_handle_impact,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &floor_sweep_distance,
                __buffer.add(116).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(120).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_flat_base_for_floor_checks,
                __buffer.add(124).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_hit,
                __buffer.add(128).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_floor_result,
                __buffer.add(392).cast::<FFloorCheckResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_move_record,
                __buffer.add(672).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UAirMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_test_falling_move_along_hit_surface,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(128)
                .cast::<crate::bindings::engine::FHitResult>()
                .swap(in_out_hit);
        }
        unsafe {
            __buffer.add(392).cast::<FFloorCheckResult>().swap(out_floor_result);
        }
        unsafe {
            __buffer.add(672).cast::<FMovementRecord>().swap(in_out_move_record);
        }
        unsafe { __buffer.add(744).cast::<f32>().read() }
    }
    pub fn is_valid_landing_spot(
        moving_comps: &FMovingComponentSet,
        location: &crate::bindings::core_u_object::FVector,
        hit: &crate::bindings::engine::FHitResult,
        floor_sweep_distance: f32,
        max_walk_slope_cosine: f32,
        b_use_flat_base_for_floor_checks: bool,
        out_floor_result: &mut FFloorCheckResult,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<609>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_is_valid_landing_spot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(48).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &floor_sweep_distance,
                __buffer.add(312).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(316).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_flat_base_for_floor_checks,
                __buffer.add(320).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_floor_result,
                __buffer.add(328).cast::<FFloorCheckResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UAirMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_is_valid_landing_spot,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(328).cast::<FFloorCheckResult>().swap(out_floor_result);
        }
        unsafe { __buffer.add(608).cast::<bool>().read() }
    }
    pub fn compute_controlled_free_move(in_params: &FFreeMoveParams) -> FProposedMove {
        let mut __stack = crate::core_data::StackAlloc::<288>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_compute_controlled_free_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_params,
                __buffer.add(0).cast::<FFreeMoveParams>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UAirMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_air_movement_utils_compute_controlled_free_move,
                __buffer,
            )
        };
        unsafe { __buffer.add(192).cast::<FProposedMove>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAsyncMovementUtils {
    __padding_end: [u8; 48],
}
impl UAsyncMovementUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncMovementUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncMovementUtils")
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
    pub fn test_sliding_move_along_hit_surface(
        moving_comps: &FMovingComponentSet,
        original_move_delta: &crate::bindings::core_u_object::FVector,
        location_at_hit: &crate::bindings::core_u_object::FVector,
        target_rotation: &crate::bindings::core_u_object::FQuat,
        in_out_hit: &mut crate::bindings::engine::FHitResult,
        in_out_move_record: &mut FMovementRecord,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<452>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_movement_utils_test_sliding_move_along_hit_surface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                original_move_delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location_at_hit,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_rotation,
                __buffer.add(80).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_hit,
                __buffer.add(112).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_move_record,
                __buffer.add(376).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UAsyncMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_movement_utils_test_sliding_move_along_hit_surface,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(112)
                .cast::<crate::bindings::engine::FHitResult>()
                .swap(in_out_hit);
        }
        unsafe {
            __buffer.add(376).cast::<FMovementRecord>().swap(in_out_move_record);
        }
        unsafe { __buffer.add(448).cast::<f32>().read() }
    }
    pub fn test_depenetrating_move(
        moving_comps: &FMovingComponentSet,
        start_location: &crate::bindings::core_u_object::FVector,
        target_location: &crate::bindings::core_u_object::FVector,
        start_rotation: &crate::bindings::core_u_object::FQuat,
        target_rotation: &crate::bindings::core_u_object::FQuat,
        b_should_sweep: bool,
        out_hit: &mut crate::bindings::engine::FHitResult,
        in_out_move_record: &mut FMovementRecord,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<489>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_movement_utils_test_depenetrating_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                start_location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_location,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                start_rotation,
                __buffer.add(80).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_rotation,
                __buffer.add(112).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_sweep,
                __buffer.add(144).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hit,
                __buffer.add(152).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_move_record,
                __buffer.add(416).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UAsyncMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_async_movement_utils_test_depenetrating_move,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(152)
                .cast::<crate::bindings::engine::FHitResult>()
                .swap(out_hit);
        }
        unsafe {
            __buffer.add(416).cast::<FMovementRecord>().swap(in_out_move_record);
        }
        unsafe { __buffer.add(488).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBasedMovementUtils {
    __padding_end: [u8; 48],
}
impl UBasedMovementUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBasedMovementUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBasedMovementUtils")
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
    pub fn transform_world_rotator_to_based(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        bone_name: FName,
        world_space_rotator: crate::bindings::core_u_object::FRotator,
        out_local_rotator: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_rotator_to_based,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_rotator,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_local_rotator,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_rotator_to_based,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(out_local_rotator);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn transform_world_location_to_based(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        bone_name: FName,
        world_space_location: crate::bindings::core_u_object::FVector,
        out_local_location: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_location_to_based,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_local_location,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_location_to_based,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_local_location);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn transform_world_direction_to_based(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        bone_name: FName,
        world_space_direction: crate::bindings::core_u_object::FVector,
        out_local_direction: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_direction_to_based,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_direction,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_local_direction,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_world_direction_to_based,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_local_direction);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn transform_rotator_to_world(
        base_quat: crate::bindings::core_u_object::FQuat,
        local_rotator: crate::bindings::core_u_object::FRotator,
        out_world_space_rotator: &mut crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_rotator_to_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_quat,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_rotator,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_world_space_rotator,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_rotator_to_world,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(out_world_space_rotator);
        }
    }
    pub fn transform_rotator_to_local(
        base_quat: crate::bindings::core_u_object::FQuat,
        world_space_rotator: crate::bindings::core_u_object::FRotator,
        out_local_rotator: &mut crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_rotator_to_local,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_quat,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_rotator,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_local_rotator,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_rotator_to_local,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(out_local_rotator);
        }
    }
    pub fn transform_location_to_world(
        base_pos: crate::bindings::core_u_object::FVector,
        base_quat: crate::bindings::core_u_object::FQuat,
        local_location: crate::bindings::core_u_object::FVector,
        out_location_world_space: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_location_to_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_pos,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_quat,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_location,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_location_world_space,
                __buffer.add(88).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_location_to_world,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_location_world_space);
        }
    }
    pub fn transform_location_to_local(
        base_pos: crate::bindings::core_u_object::FVector,
        base_quat: crate::bindings::core_u_object::FQuat,
        world_space_location: crate::bindings::core_u_object::FVector,
        out_local_location: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_location_to_local,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_pos,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_quat,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_location,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_local_location,
                __buffer.add(88).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_location_to_local,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_local_location);
        }
    }
    pub fn transform_direction_to_world(
        base_quat: crate::bindings::core_u_object::FQuat,
        local_direction: crate::bindings::core_u_object::FVector,
        out_direction_world_space: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_direction_to_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_quat,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_direction,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_direction_world_space,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_direction_to_world,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_direction_world_space);
        }
    }
    pub fn transform_direction_to_local(
        base_quat: crate::bindings::core_u_object::FQuat,
        world_space_direction: crate::bindings::core_u_object::FVector,
        out_local_direction: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_direction_to_local,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_quat,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world_space_direction,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_local_direction,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_direction_to_local,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_local_direction);
        }
    }
    pub fn transform_based_rotator_to_world(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        bone_name: FName,
        local_rotator: crate::bindings::core_u_object::FRotator,
        out_world_space_rotator: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_rotator_to_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_rotator,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_world_space_rotator,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_rotator_to_world,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(out_world_space_rotator);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn transform_based_location_to_world(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        bone_name: FName,
        local_location: crate::bindings::core_u_object::FVector,
        out_location_world_space: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_location_to_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_location_world_space,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_location_to_world,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_location_world_space);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn transform_based_direction_to_world(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        bone_name: FName,
        local_direction: crate::bindings::core_u_object::FVector,
        out_direction_world_space: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_direction_to_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_direction,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_direction_world_space,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_transform_based_direction_to_world,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_direction_world_space);
        }
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn is_base_simulating_physics(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_is_base_simulating_physics,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_is_base_simulating_physics,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_a_dynamic_base(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_is_a_dynamic_base,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_is_a_dynamic_base,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_movement_base_transform(
        movement_base: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        bone_name: FName,
        out_location: &mut crate::bindings::core_u_object::FVector,
        out_quat: &mut crate::bindings::core_u_object::FQuat,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_get_movement_base_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movement_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_quat,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UBasedMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_based_movement_utils_get_movement_base_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_location);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::core_u_object::FQuat>()
                .swap(out_quat);
        }
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPlanarConstraintUtils {
    __padding_end: [u8; 48],
}
impl UPlanarConstraintUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlanarConstraintUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlanarConstraintUtils")
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
    pub fn set_plane_constraint_origin(
        constraint: &mut FPlanarConstraint,
        plane_origin: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_set_plane_constraint_origin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                constraint,
                __buffer.add(0).cast::<FPlanarConstraint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &plane_origin,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UPlanarConstraintUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_set_plane_constraint_origin,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPlanarConstraint>().swap(constraint);
        }
    }
    pub fn set_planar_constraint_normal(
        constraint: &mut FPlanarConstraint,
        plane_normal: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_set_planar_constraint_normal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                constraint,
                __buffer.add(0).cast::<FPlanarConstraint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &plane_normal,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UPlanarConstraintUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_set_planar_constraint_normal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPlanarConstraint>().swap(constraint);
        }
    }
    pub fn set_planar_constraint_enabled(
        constraint: &mut FPlanarConstraint,
        b_enabled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_set_planar_constraint_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                constraint,
                __buffer.add(0).cast::<FPlanarConstraint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enabled,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UPlanarConstraintUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_set_planar_constraint_enabled,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FPlanarConstraint>().swap(constraint);
        }
    }
    pub fn constrain_normal_to_plane(
        constraint: &FPlanarConstraint,
        normal: crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_normal_to_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                constraint,
                __buffer.add(0).cast::<FPlanarConstraint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &normal,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UPlanarConstraintUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_normal_to_plane,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn constrain_location_to_plane(
        constraint: &FPlanarConstraint,
        location: crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_location_to_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                constraint,
                __buffer.add(0).cast::<FPlanarConstraint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UPlanarConstraintUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_location_to_plane,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(80).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn constrain_direction_to_plane(
        constraint: &FPlanarConstraint,
        direction: crate::bindings::core_u_object::FVector,
        b_maintain_magnitude: bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_direction_to_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                constraint,
                __buffer.add(0).cast::<FPlanarConstraint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &direction,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_magnitude,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UPlanarConstraintUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_planar_constraint_utils_constrain_direction_to_plane,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UFloorQueryUtils {
    __padding_end: [u8; 48],
}
impl UFloorQueryUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloorQueryUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloorQueryUtils")
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
    pub fn is_hit_surface_walkable(
        hit: &crate::bindings::engine::FHitResult,
        up_direction: &crate::bindings::core_u_object::FVector,
        max_walk_slope_cosine: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<293>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_floor_query_utils_is_hit_surface_walkable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(0).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                up_direction,
                __buffer.add(264).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(288).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UFloorQueryUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_floor_query_utils_is_hit_surface_walkable,
                __buffer,
            )
        };
        unsafe { __buffer.add(292).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UGroundMovementUtils {
    __padding_end: [u8; 48],
}
impl UGroundMovementUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroundMovementUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroundMovementUtils")
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
    pub fn try_walk_to_slide_along_surface(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        pct_of_delta_to_move: f32,
        rotation: crate::bindings::core_u_object::FQuat,
        normal: &crate::bindings::core_u_object::FVector,
        hit: &mut crate::bindings::engine::FHitResult,
        b_handle_impact: bool,
        move_record: &mut FMovementRecord,
        max_walk_slope_cosine: f32,
        max_step_height: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<476>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_try_walk_to_slide_along_surface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pct_of_delta_to_move,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normal,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(120).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_handle_impact,
                __buffer.add(384).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_record,
                __buffer.add(392).cast::<FMovementRecord>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(464).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_step_height,
                __buffer.add(468).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_try_walk_to_slide_along_surface,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(120).cast::<crate::bindings::engine::FHitResult>().swap(hit);
        }
        unsafe {
            __buffer.add(392).cast::<FMovementRecord>().swap(move_record);
        }
        unsafe { __buffer.add(472).cast::<f32>().read() }
    }
    pub fn test_move_to_keep_min_height_above_floor(
        moving_comps: &FMovingComponentSet,
        location: &crate::bindings::core_u_object::FVector,
        rotation: &crate::bindings::core_u_object::FQuat,
        max_walk_slope_cosine: f32,
        in_out_current_floor: &mut FFloorCheckResult,
        in_out_move_record: &mut FMovementRecord,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<464>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_test_move_to_keep_min_height_above_floor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_current_floor,
                __buffer.add(88).cast::<FFloorCheckResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_move_record,
                __buffer.add(368).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_test_move_to_keep_min_height_above_floor,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<FFloorCheckResult>().swap(in_out_current_floor);
        }
        unsafe {
            __buffer.add(368).cast::<FMovementRecord>().swap(in_out_move_record);
        }
        unsafe {
            __buffer.add(440).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn test_move_to_adjust_to_floor(
        moving_comps: &FMovingComponentSet,
        location: &crate::bindings::core_u_object::FVector,
        rotation: &crate::bindings::core_u_object::FQuat,
        max_walk_slope_cosine: f32,
        in_out_current_floor: &mut FFloorCheckResult,
        in_out_move_record: &mut FMovementRecord,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<464>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_test_move_to_adjust_to_floor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_current_floor,
                __buffer.add(88).cast::<FFloorCheckResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_move_record,
                __buffer.add(368).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_test_move_to_adjust_to_floor,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<FFloorCheckResult>().swap(in_out_current_floor);
        }
        unsafe {
            __buffer.add(368).cast::<FMovementRecord>().swap(in_out_move_record);
        }
        unsafe {
            __buffer.add(440).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn test_grounded_move_along_hit_surface(
        moving_comps: &FMovingComponentSet,
        original_move_delta: &crate::bindings::core_u_object::FVector,
        location_at_hit: &crate::bindings::core_u_object::FVector,
        target_rotation: &crate::bindings::core_u_object::FQuat,
        b_handle_impact: bool,
        max_step_height: f32,
        max_walk_slope_cosine: f32,
        in_out_hit: &mut crate::bindings::engine::FHitResult,
        in_out_move_record: &mut FMovementRecord,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<468>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_test_grounded_move_along_hit_surface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                original_move_delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location_at_hit,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_rotation,
                __buffer.add(80).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_handle_impact,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_step_height,
                __buffer.add(116).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(120).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_hit,
                __buffer.add(128).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_move_record,
                __buffer.add(392).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_test_grounded_move_along_hit_surface,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(128)
                .cast::<crate::bindings::engine::FHitResult>()
                .swap(in_out_hit);
        }
        unsafe {
            __buffer.add(392).cast::<FMovementRecord>().swap(in_out_move_record);
        }
        unsafe { __buffer.add(464).cast::<f32>().read() }
    }
    pub fn compute_deflected_move_onto_ramp(
        orig_move_delta: &crate::bindings::core_u_object::FVector,
        up_direction: &crate::bindings::core_u_object::FVector,
        ramp_hit_result: &crate::bindings::engine::FHitResult,
        max_walk_slope_cosine: f32,
        b_hit_from_line_trace: bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<344>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_compute_deflected_move_onto_ramp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                orig_move_delta,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                up_direction,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                ramp_hit_result,
                __buffer.add(48).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &max_walk_slope_cosine,
                __buffer.add(312).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_hit_from_line_trace,
                __buffer.add(316).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_compute_deflected_move_onto_ramp,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(320).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_controlled_ground_move(
        in_params: &FGroundMoveParams,
    ) -> FProposedMove {
        let mut __stack = crate::core_data::StackAlloc::<336>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_compute_controlled_ground_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_params,
                __buffer.add(0).cast::<FGroundMoveParams>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_compute_controlled_ground_move,
                __buffer,
            )
        };
        unsafe { __buffer.add(240).cast::<FProposedMove>().read() }
    }
    pub fn can_step_up_on_hit_surface(
        hit: &crate::bindings::engine::FHitResult,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<265>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_can_step_up_on_hit_surface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(0).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UGroundMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_ground_movement_utils_can_step_up_on_hit_surface,
                __buffer,
            )
        };
        unsafe { __buffer.add(264).cast::<bool>().read() }
    }
}
pub struct ITurnGeneratorInterface {}
#[repr(C, align(8))]
pub struct UTurnGeneratorInterface {
    __padding_end: [u8; 48],
}
impl UTurnGeneratorInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTurnGeneratorInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTurnGeneratorInterface")
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
    pub fn get_turn(
        &mut self,
        target_orientation: crate::bindings::core_u_object::FRotator,
        full_start_state: &FMoverTickStartData,
        mover_state: &FMoverDefaultSyncState,
        time_step: &FMoverTimeStep,
        proposed_move: &FProposedMove,
        sim_blackboard: UPtr<UMoverBlackboard>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<648>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_turn_generator_interface_get_turn,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_orientation,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                full_start_state,
                __buffer.add(24).cast::<FMoverTickStartData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                mover_state,
                __buffer.add(288).cast::<FMoverDefaultSyncState>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_step,
                __buffer.add(496).cast::<FMoverTimeStep>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                proposed_move,
                __buffer.add(520).cast::<FProposedMove>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_blackboard,
                __buffer.add(616).cast::<UPtr<UMoverBlackboard>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_turn_generator_interface_get_turn,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(624).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct ULinearTurnGenerator {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub heading_rate: f32,
    pub pitch_rate: f32,
    pub roll_rate: f32,
}
impl ULinearTurnGenerator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearTurnGenerator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearTurnGenerator")
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
pub struct UExactDampedTurnGenerator {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub half_life_seconds: f32,
}
impl UExactDampedTurnGenerator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExactDampedTurnGenerator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExactDampedTurnGenerator")
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
pub struct UBlueprintableTurnGenerator {
    __padding_end: [u8; 56],
}
impl UBlueprintableTurnGenerator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintableTurnGenerator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintableTurnGenerator")
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
pub struct UMovementMixer {
    __padding_end: [u8; 64],
}
impl UMovementMixer {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementMixer")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementMixer")
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
pub struct UMovementRecordUtils {
    __padding_end: [u8; 48],
}
impl UMovementRecordUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementRecordUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementRecordUtils")
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
    pub fn k2_set_delta_seconds(
        out_movement_record: &mut FMovementRecord,
        delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_set_delta_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_movement_record,
                __buffer.add(0).cast::<FMovementRecord>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(72).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementRecordUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_set_delta_seconds,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FMovementRecord>().swap(out_movement_record);
        }
    }
    pub fn k2_get_total_move_delta(
        movement_record: &FMovementRecord,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_get_total_move_delta,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                movement_record,
                __buffer.add(0).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementRecordUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_get_total_move_delta,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn k2_get_relevant_velocity(
        movement_record: &FMovementRecord,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_get_relevant_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                movement_record,
                __buffer.add(0).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementRecordUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_get_relevant_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn k2_get_relevant_move_delta(
        movement_record: &FMovementRecord,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_get_relevant_move_delta,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                movement_record,
                __buffer.add(0).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementRecordUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_record_utils_k2_get_relevant_move_delta,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovementUtils {
    __padding_end: [u8; 48],
}
impl UMovementUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementUtils")
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
    pub fn try_safe_move_updated_component_no_movement_record(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        new_rotation: &crate::bindings::core_u_object::FQuat,
        b_sweep: bool,
        out_hit: &mut crate::bindings::engine::FHitResult,
        teleport: crate::bindings::engine::ETeleportType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<354>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_updated_component_no_movement_record,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_sweep, __buffer.add(80).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hit,
                __buffer.add(88).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &teleport,
                __buffer.add(352).cast::<crate::bindings::engine::ETeleportType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_updated_component_no_movement_record,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::engine::FHitResult>().swap(out_hit);
        }
        unsafe { __buffer.add(353).cast::<bool>().read() }
    }
    pub fn try_safe_move_updated_component(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        new_rotation: &crate::bindings::core_u_object::FQuat,
        b_sweep: bool,
        out_hit: &mut crate::bindings::engine::FHitResult,
        teleport: crate::bindings::engine::ETeleportType,
        move_record: &mut FMovementRecord,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<433>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_updated_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_sweep, __buffer.add(80).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hit,
                __buffer.add(88).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &teleport,
                __buffer.add(352).cast::<crate::bindings::engine::ETeleportType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_record,
                __buffer.add(360).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_updated_component,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::engine::FHitResult>().swap(out_hit);
        }
        unsafe {
            __buffer.add(360).cast::<FMovementRecord>().swap(move_record);
        }
        unsafe { __buffer.add(432).cast::<bool>().read() }
    }
    pub fn try_safe_move_and_slide_updated_component_no_movement_record(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        new_rotation: &crate::bindings::core_u_object::FQuat,
        b_sweep: bool,
        out_hit: &mut crate::bindings::engine::FHitResult,
        teleport: crate::bindings::engine::ETeleportType,
        b_slide_along_surface: bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<360>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_and_slide_updated_component_no_movement_record,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_sweep, __buffer.add(80).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hit,
                __buffer.add(88).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &teleport,
                __buffer.add(352).cast::<crate::bindings::engine::ETeleportType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_slide_along_surface,
                __buffer.add(353).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_and_slide_updated_component_no_movement_record,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::engine::FHitResult>().swap(out_hit);
        }
        unsafe { __buffer.add(356).cast::<f32>().read() }
    }
    pub fn try_safe_move_and_slide_updated_component(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        new_rotation: &crate::bindings::core_u_object::FQuat,
        b_sweep: bool,
        out_hit: &mut crate::bindings::engine::FHitResult,
        teleport: crate::bindings::engine::ETeleportType,
        move_record: &mut FMovementRecord,
        b_slide_along_surface: bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<440>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_and_slide_updated_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_rotation,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_sweep, __buffer.add(80).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_hit,
                __buffer.add(88).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &teleport,
                __buffer.add(352).cast::<crate::bindings::engine::ETeleportType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_record,
                __buffer.add(360).cast::<FMovementRecord>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_slide_along_surface,
                __buffer.add(432).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_safe_move_and_slide_updated_component,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::engine::FHitResult>().swap(out_hit);
        }
        unsafe {
            __buffer.add(360).cast::<FMovementRecord>().swap(move_record);
        }
        unsafe { __buffer.add(436).cast::<f32>().read() }
    }
    pub fn try_move_to_slide_along_surface_no_movement_record(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        pct_of_delta_to_move: f32,
        rotation: crate::bindings::core_u_object::FQuat,
        normal: &crate::bindings::core_u_object::FVector,
        hit: &mut crate::bindings::engine::FHitResult,
        b_handle_impact: bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<392>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_move_to_slide_along_surface_no_movement_record,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pct_of_delta_to_move,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normal,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(120).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_handle_impact,
                __buffer.add(384).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_move_to_slide_along_surface_no_movement_record,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(120).cast::<crate::bindings::engine::FHitResult>().swap(hit);
        }
        unsafe { __buffer.add(388).cast::<f32>().read() }
    }
    pub fn try_move_to_slide_along_surface(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        pct_of_delta_to_move: f32,
        rotation: crate::bindings::core_u_object::FQuat,
        normal: &crate::bindings::core_u_object::FVector,
        hit: &mut crate::bindings::engine::FHitResult,
        b_handle_impact: bool,
        move_record: &mut FMovementRecord,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<468>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_move_to_slide_along_surface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pct_of_delta_to_move,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normal,
                __buffer.add(96).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(120).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_handle_impact,
                __buffer.add(384).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_record,
                __buffer.add(392).cast::<FMovementRecord>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_try_move_to_slide_along_surface,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(120).cast::<crate::bindings::engine::FHitResult>().swap(hit);
        }
        unsafe {
            __buffer.add(392).cast::<FMovementRecord>().swap(move_record);
        }
        unsafe { __buffer.add(464).cast::<f32>().read() }
    }
    pub fn test_encroachment_and_adjust(
        mover_comp: UPtr<UMoverComponent>,
        test_location: crate::bindings::core_u_object::FVector,
        test_rotation: crate::bindings::core_u_object::FRotator,
        out_proposed_adjustment: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_test_encroachment_and_adjust,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mover_comp,
                __buffer.add(0).cast::<UPtr<UMoverComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &test_location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &test_rotation,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_proposed_adjustment,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_test_encroachment_and_adjust,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_proposed_adjustment);
        }
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn test_encroachment(
        mover_comp: UPtr<UMoverComponent>,
        test_location: crate::bindings::core_u_object::FVector,
        test_rotation: crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_test_encroachment,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mover_comp,
                __buffer.add(0).cast::<UPtr<UMoverComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &test_location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &test_rotation,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_test_encroachment,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn project_to_gravity_floor(
        vector: &crate::bindings::core_u_object::FVector,
        up_direction: &crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_project_to_gravity_floor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                vector,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                up_direction,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_project_to_gravity_floor,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn is_exceeding_max_speed(
        velocity: &crate::bindings::core_u_object::FVector,
        in_max_speed: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_is_exceeding_max_speed,
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
                &in_max_speed,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_is_exceeding_max_speed,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn is_angular_velocity_zero(
        angular_velocity: &crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_is_angular_velocity_zero,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                angular_velocity,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_is_angular_velocity_zero,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_gravity_vertical_component(
        vector: &crate::bindings::core_u_object::FVector,
        up_direction: &crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_get_gravity_vertical_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                vector,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                up_direction,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_get_gravity_vertical_component,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn find_teleport_spot(
        mover_comp: UPtr<UMoverComponent>,
        test_location: &mut crate::bindings::core_u_object::FVector,
        test_rotation: crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_find_teleport_spot,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mover_comp,
                __buffer.add(0).cast::<UPtr<UMoverComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                test_location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &test_rotation,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_find_teleport_spot,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(test_location);
        }
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn deduce_up_direction_from_gravity(
        gravity_acceleration: &crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_deduce_up_direction_from_gravity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                gravity_acceleration,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_deduce_up_direction_from_gravity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn constrain_to_plane(
        vector: &crate::bindings::core_u_object::FVector,
        movement_plane: &crate::bindings::core_u_object::FPlane,
        b_maintain_magnitude: bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_constrain_to_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                vector,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                movement_plane,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FPlane>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_magnitude,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_constrain_to_plane,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_velocity_from_positions(
        from_pos: &crate::bindings::core_u_object::FVector,
        to_pos: &crate::bindings::core_u_object::FVector,
        delta_seconds: f32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_velocity_from_positions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from_pos,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to_pos,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_velocity_from_positions,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_velocity_from_gravity(
        gravity_accel: &crate::bindings::core_u_object::FVector,
        delta_seconds: f32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_velocity_from_gravity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                gravity_accel,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(24).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_velocity_from_gravity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_velocity(
        in_params: &FComputeVelocityParams,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_params,
                __buffer.add(0).cast::<FComputeVelocityParams>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_slide_delta(
        moving_comps: &FMovingComponentSet,
        delta: &crate::bindings::core_u_object::FVector,
        pct_of_delta_to_move: f32,
        normal: &crate::bindings::core_u_object::FVector,
        hit: &crate::bindings::engine::FHitResult,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<368>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_slide_delta,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                moving_comps,
                __buffer.add(0).cast::<FMovingComponentSet>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                delta,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pct_of_delta_to_move,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                normal,
                __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                hit,
                __buffer.add(80).cast::<crate::bindings::engine::FHitResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_slide_delta,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(344).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_direction_intent(
        move_input: &crate::bindings::core_u_object::FVector,
        move_input_type: EMoveInputType,
        max_speed: f32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_direction_intent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                move_input,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &move_input_type,
                __buffer.add(24).cast::<EMoveInputType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&max_speed, __buffer.add(28).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_direction_intent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_combined_velocity(
        in_params: &FComputeCombinedVelocityParams,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_combined_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_params,
                __buffer.add(0).cast::<FComputeCombinedVelocityParams>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_combined_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_angular_velocity_degrees(
        from: &crate::bindings::core_u_object::FRotator,
        to: &crate::bindings::core_u_object::FRotator,
        delta_seconds: f32,
        turning_rate_limit: f32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_angular_velocity_degrees,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turning_rate_limit,
                __buffer.add(52).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_angular_velocity_degrees,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn compute_angular_velocity(
        from: &crate::bindings::core_u_object::FRotator,
        to: &crate::bindings::core_u_object::FRotator,
        world_to_gravity: &crate::bindings::core_u_object::FQuat,
        delta_seconds: f32,
        turning_rate_limit: f32,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                world_to_gravity,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(80).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &turning_rate_limit,
                __buffer.add(84).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_compute_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn can_escape_gravity(
        prior_velocity: &crate::bindings::core_u_object::FVector,
        new_velocity: &crate::bindings::core_u_object::FVector,
        gravity_accel: &crate::bindings::core_u_object::FVector,
        delta_seconds: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<77>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_can_escape_gravity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                prior_velocity,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_velocity,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                gravity_accel,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(72).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_can_escape_gravity,
                __buffer,
            )
        };
        unsafe { __buffer.add(76).cast::<bool>().read() }
    }
    pub fn apply_gravity_to_orientation_intent(
        intended_orientation: &crate::bindings::core_u_object::FRotator,
        world_to_gravity: &crate::bindings::core_u_object::FQuat,
        b_stay_vertical: bool,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_gravity_to_orientation_intent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                intended_orientation,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                world_to_gravity,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_stay_vertical,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_gravity_to_orientation_intent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn apply_angular_velocity_to_rotator(
        starting_orient: &crate::bindings::core_u_object::FRotator,
        angular_velocity_degrees: &crate::bindings::core_u_object::FVector,
        delta_seconds: f32,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_angular_velocity_to_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                starting_orient,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                angular_velocity_degrees,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_angular_velocity_to_rotator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn apply_angular_velocity_to_quat(
        starting_orient: &crate::bindings::core_u_object::FQuat,
        angular_velocity_degrees: &crate::bindings::core_u_object::FVector,
        delta_seconds: f32,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_angular_velocity_to_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                starting_orient,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                angular_velocity_degrees,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(56).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_angular_velocity_to_quat,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn apply_angular_velocity(
        starting_orient: &crate::bindings::core_u_object::FRotator,
        angular_velocity: &crate::bindings::core_u_object::FRotator,
        delta_seconds: f32,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                starting_orient,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                angular_velocity,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delta_seconds,
                __buffer.add(48).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_movement_utils_apply_angular_velocity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMoverBlackboard {
    __padding_end: [u8; 136],
}
impl UMoverBlackboard {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverBlackboard")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverBlackboard")
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
pub struct UPlayMoverMontageCallbackProxy {
    __padding_end: [u8; 232],
}
impl UPlayMoverMontageCallbackProxy {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlayMoverMontageCallbackProxy")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPlayMoverMontageCallbackProxy")
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
    pub fn create_proxy_object_for_play_mover_montage(
        in_mover_component: UPtr<UMoverComponent>,
        montage_to_play: UPtr<crate::bindings::engine::UAnimMontage>,
        play_rate: f32,
        starting_position: f32,
        starting_section: FName,
    ) -> UPtr<UPlayMoverMontageCallbackProxy> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_play_mover_montage_callback_proxy_create_proxy_object_for_play_mover_montage,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mover_component,
                __buffer.add(0).cast::<UPtr<UMoverComponent>>(),
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
        let __object_ptr = crate::bindings::mover::UPlayMoverMontageCallbackProxy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_play_mover_montage_callback_proxy_create_proxy_object_for_play_mover_montage,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<UPlayMoverMontageCallbackProxy>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URollbackBlackboard {
    __padding_end: [u8; 176],
}
impl URollbackBlackboard {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URollbackBlackboard")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URollbackBlackboard")
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
pub struct URollbackBlackboard_InternalWrapper {
    __padding_end: [u8; 56],
}
impl URollbackBlackboard_InternalWrapper {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URollbackBlackboard_InternalWrapper")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URollbackBlackboard_InternalWrapper")
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
pub struct UWaterMovementUtils {
    __padding_end: [u8; 48],
}
impl UWaterMovementUtils {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterMovementUtils")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWaterMovementUtils")
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
    pub fn update_water_spline_data(
        update_water_spline_data_params: &FUpdateWaterSplineDataParams,
        water_check_result: &mut FWaterCheckResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<576>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_water_movement_utils_update_water_spline_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_water_spline_data_params,
                __buffer.add(0).cast::<FUpdateWaterSplineDataParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                water_check_result,
                __buffer.add(72).cast::<FWaterCheckResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UWaterMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_water_movement_utils_update_water_spline_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<FWaterCheckResult>().swap(water_check_result);
        }
    }
    pub fn compute_controlled_water_move(in_params: &FWaterMoveParams) -> FProposedMove {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_water_movement_utils_compute_controlled_water_move,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_params,
                __buffer.add(0).cast::<FWaterMoveParams>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UWaterMovementUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_water_movement_utils_compute_controlled_water_move,
                __buffer,
            )
        };
        unsafe { __buffer.add(208).cast::<FProposedMove>().read() }
    }
}
pub struct IMovementSettingsInterface {}
#[repr(C, align(8))]
pub struct UMovementSettingsInterface {
    __padding_end: [u8; 48],
}
impl UMovementSettingsInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementSettingsInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementSettingsInterface")
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
pub struct UNullMovementMode {
    __padding_end: [u8; 120],
}
impl UNullMovementMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNullMovementMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNullMovementMode")
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
pub struct UMovementModeStateMachine {
    __padding_end: [u8; 1000],
}
impl UMovementModeStateMachine {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementModeStateMachine")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovementModeStateMachine")
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
pub struct UBaseMovementModeTransition {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub b_allow_mode_reentry: bool,
    pub b_first_sub_step_only: bool,
    pub b_supports_async: bool,
}
impl UBaseMovementModeTransition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMovementModeTransition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseMovementModeTransition")
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
    pub fn trigger(&mut self, params: &FSimulationTickParams) {
        let mut __stack = crate::core_data::StackAlloc::<408>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_trigger,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(0).cast::<FSimulationTickParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_trigger,
                __buffer,
            )
        };
    }
    pub fn on_unregistered(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_on_unregistered,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_on_unregistered,
                __buffer,
            )
        };
    }
    pub fn on_registered(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_on_registered,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_on_registered,
                __buffer,
            )
        };
    }
    pub fn k2_get_mover_component(&self) -> UPtr<UMoverComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_get_mover_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_k2_get_mover_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMoverComponent>>().read() }
    }
    pub fn evaluate(&self, params: &FSimulationTickParams) -> FTransitionEvalResult {
        let mut __stack = crate::core_data::StackAlloc::<420>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_evaluate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(0).cast::<FSimulationTickParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_base_movement_mode_transition_evaluate,
                __buffer,
            )
        };
        unsafe { __buffer.add(408).cast::<FTransitionEvalResult>().read() }
    }
}
#[repr(C, align(8))]
pub struct UImmediateMovementModeTransition {
    __padding_end: [u8; 88],
}
impl UImmediateMovementModeTransition {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImmediateMovementModeTransition")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImmediateMovementModeTransition")
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
pub struct UMoverDataModelBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UMoverDataModelBlueprintLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDataModelBlueprintLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDataModelBlueprintLibrary")
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
    pub fn set_velocity_input(
        inputs: &mut FCharacterDefaultInputs,
        velocity_input: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_set_velocity_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                inputs,
                __buffer.add(0).cast::<FCharacterDefaultInputs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                velocity_input,
                __buffer.add(128).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_set_velocity_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FCharacterDefaultInputs>().swap(inputs);
        }
    }
    pub fn set_directional_input(
        inputs: &mut FCharacterDefaultInputs,
        direction_input: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_set_directional_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                inputs,
                __buffer.add(0).cast::<FCharacterDefaultInputs>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                direction_input,
                __buffer.add(128).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_set_directional_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FCharacterDefaultInputs>().swap(inputs);
        }
    }
    pub fn get_velocity_from_sync_state(
        sync_state: &FMoverDefaultSyncState,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_velocity_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sync_state,
                __buffer.add(0).cast::<FMoverDefaultSyncState>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_velocity_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_orientation_from_sync_state(
        sync_state: &FMoverDefaultSyncState,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_orientation_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sync_state,
                __buffer.add(0).cast::<FMoverDefaultSyncState>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_orientation_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_move_direction_intent_from_sync_state(
        sync_state: &FMoverDefaultSyncState,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_move_direction_intent_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sync_state,
                __buffer.add(0).cast::<FMoverDefaultSyncState>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_move_direction_intent_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_move_direction_intent_from_inputs(
        inputs: &FCharacterDefaultInputs,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_move_direction_intent_from_inputs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                inputs,
                __buffer.add(0).cast::<FCharacterDefaultInputs>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_move_direction_intent_from_inputs,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(128).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_location_from_sync_state(
        sync_state: &FMoverDefaultSyncState,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_location_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sync_state,
                __buffer.add(0).cast::<FMoverDefaultSyncState>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_location_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_angular_velocity_degrees_from_sync_state(
        sync_state: &FMoverDefaultSyncState,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_angular_velocity_degrees_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                sync_state,
                __buffer.add(0).cast::<FMoverDefaultSyncState>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataModelBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_model_blueprint_library_get_angular_velocity_degrees_from_sync_state,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMoverDeveloperSettings {
    __padding_end: [u8; 112],
}
impl UMoverDeveloperSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDeveloperSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDeveloperSettings")
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
pub struct UMoverTrajectoryPredictor {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub mover_sampling_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
impl UMoverTrajectoryPredictor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverTrajectoryPredictor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverTrajectoryPredictor")
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
    pub fn setup(&mut self, in_mover_component: UPtr<UMoverComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_trajectory_predictor_setup,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mover_component,
                __buffer.add(0).cast::<UPtr<UMoverComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_trajectory_predictor_setup,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMoverSimulation {
    __padding_end: [u8; 64],
}
impl UMoverSimulation {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverSimulation")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverSimulation")
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
    pub fn get_rollback_blackboard_mutable(
        &mut self,
    ) -> UPtr<URollbackBlackboard_InternalWrapper> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_rollback_blackboard_mutable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_rollback_blackboard_mutable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<URollbackBlackboard_InternalWrapper>>().read()
        }
    }
    pub fn get_rollback_blackboard(&self) -> UPtr<URollbackBlackboard_InternalWrapper> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_rollback_blackboard,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_rollback_blackboard,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<URollbackBlackboard_InternalWrapper>>().read()
        }
    }
    pub fn get_blackboard_mutable(&mut self) -> UPtr<UMoverBlackboard> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_blackboard_mutable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_blackboard_mutable,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMoverBlackboard>>().read() }
    }
    pub fn get_blackboard(&self) -> UPtr<UMoverBlackboard> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_blackboard,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_get_blackboard,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UMoverBlackboard>>().read() }
    }
    pub fn attempt_teleport(
        &mut self,
        time_step: &FMoverTimeStep,
        target_transform: &crate::bindings::core_u_object::FTransform,
        b_use_actor_rotation: bool,
        output_state: &mut FMoverSyncState,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<360>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_attempt_teleport,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_step,
                __buffer.add(0).cast::<FMoverTimeStep>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_actor_rotation,
                __buffer.add(128).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_state,
                __buffer.add(136).cast::<FMoverSyncState>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_simulation_attempt_teleport,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<FMoverSyncState>().swap(output_state);
        }
    }
}
pub struct IMoverInputProducerInterface {}
#[repr(C, align(8))]
pub struct UMoverInputProducerInterface {
    __padding_end: [u8; 48],
}
impl UMoverInputProducerInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverInputProducerInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverInputProducerInterface")
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
    pub fn produce_input(
        &mut self,
        sim_time_ms: i32,
        input_cmd_result: &mut FMoverInputCmdContext,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_input_producer_interface_produce_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_time_ms,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_cmd_result,
                __buffer.add(8).cast::<FMoverInputCmdContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_input_producer_interface_produce_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FMoverInputCmdContext>().swap(input_cmd_result);
        }
    }
}
#[repr(C, align(8))]
pub struct UMoverDataCollectionLibrary {
    __padding_end: [u8; 48],
}
impl UMoverDataCollectionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDataCollectionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMoverDataCollectionLibrary")
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
    pub fn k2_get_data_from_collection(
        did_succeed: &mut bool,
        collection: &FMoverDataCollection,
        target_as_raw_bytes: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_collection_library_k2_get_data_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                did_succeed,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(8).cast::<FMoverDataCollection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                target_as_raw_bytes,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataCollectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_collection_library_k2_get_data_from_collection,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(did_succeed);
        }
        unsafe {
            __buffer.add(24).cast::<i32>().swap(target_as_raw_bytes);
        }
    }
    pub fn k2_add_data_to_collection(
        collection: &mut FMoverDataCollection,
        source_as_raw_bytes: &i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_collection_library_k2_add_data_to_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FMoverDataCollection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_as_raw_bytes,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataCollectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_collection_library_k2_add_data_to_collection,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FMoverDataCollection>().swap(collection);
        }
    }
    pub fn clear_data_from_collection(collection: &mut FMoverDataCollection) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_collection_library_clear_data_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FMoverDataCollection>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::mover::UMoverDataCollectionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_mover_data_collection_library_clear_data_from_collection,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FMoverDataCollection>().swap(collection);
        }
    }
}
#[repr(C, align(8))]
pub struct UPhysicsDrivenFallingMode {
    __padding_end: [u8; 224],
}
impl UPhysicsDrivenFallingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenFallingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenFallingMode")
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
pub struct UPhysicsDrivenFlyingMode {
    __padding_end: [u8; 152],
}
impl UPhysicsDrivenFlyingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenFlyingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenFlyingMode")
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
pub struct UPhysicsDrivenSwimmingMode {
    __padding_end: [u8; 328],
}
impl UPhysicsDrivenSwimmingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenSwimmingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenSwimmingMode")
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
pub struct UPhysicsDrivenWalkingMode {
    __padding_end: [u8; 208],
}
impl UPhysicsDrivenWalkingMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenWalkingMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsDrivenWalkingMode")
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
pub struct UPathedPhysicsMovementMode {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub b_use_joint_constraint: bool,
    #[doc(hidden)]
    pub(crate) __padding_480: [u8; 359],
    pub playback_behavior_override: TOptional<EPathedPhysicsPlaybackBehavior>,
    pub one_way_trip_duration: f32,
    pub path_patterns: TArray<UPtr<UPathedMovementPatternBase>>,
    pub easing: crate::bindings::engine::EAlphaBlendOption,
    pub custom_easing_curve: UPtr<crate::bindings::engine::UCurveFloat>,
    __padding_end: [u8; 168],
}
impl UPathedPhysicsMovementMode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedPhysicsMovementMode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedPhysicsMovementMode")
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
    pub fn set_use_joint_constraint(&mut self, b_use_joint: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_set_use_joint_constraint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_joint,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_set_use_joint_constraint,
                __buffer,
            )
        };
    }
    pub fn set_path_duration_begin_play_only(&mut self, new_duration: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_set_path_duration_begin_play_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_duration,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_set_path_duration_begin_play_only,
                __buffer,
            )
        };
    }
    pub fn bp_find_pattern(
        &self,
        pattern_type: TSubclassOf<UPathedMovementPatternBase>,
    ) -> UPtr<UPathedMovementPatternBase> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_bp_find_pattern,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pattern_type,
                __buffer.add(0).cast::<TSubclassOf<UPathedMovementPatternBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_pathed_physics_movement_mode_bp_find_pattern,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UPathedMovementPatternBase>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPathedMovementPatternBase {
    __padding_end: [u8; 88],
}
impl UPathedMovementPatternBase {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedMovementPatternBase")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedMovementPatternBase")
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
pub struct UArcRotationPattern {
    __padding_end: [u8; 120],
}
impl UArcRotationPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UArcRotationPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UArcRotationPattern")
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
pub struct UEllipticalMovementPathPattern {
    __padding_end: [u8; 128],
}
impl UEllipticalMovementPathPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEllipticalMovementPathPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEllipticalMovementPathPattern")
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
pub struct ULookAtRotationPattern {
    __padding_end: [u8; 112],
}
impl ULookAtRotationPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULookAtRotationPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULookAtRotationPattern")
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
    pub fn set_relative_look_at_location(
        &mut self,
        relative_look_at: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_look_at_rotation_pattern_set_relative_look_at_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                relative_look_at,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_look_at_rotation_pattern_set_relative_look_at_location,
                __buffer,
            )
        };
    }
    pub fn set_look_at_location(
        &mut self,
        look_at: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_look_at_rotation_pattern_set_look_at_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                look_at,
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
                crate::bindings::mover::__FUNCTION_PTRS
                    .u_look_at_rotation_pattern_set_look_at_location,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UPathedPhysicsDebugDrawComponent {
    __padding_end: [u8; 1696],
}
impl UPathedPhysicsDebugDrawComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedPhysicsDebugDrawComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPathedPhysicsDebugDrawComponent")
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
pub struct UPointMovementPathPattern {
    #[doc(hidden)]
    pub(crate) __padding_88: [u8; 88],
    pub path_points: TArray<FPointMovementPathPoint>,
    __padding_end: [u8; 8],
}
impl UPointMovementPathPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPointMovementPathPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPointMovementPathPattern")
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
pub struct USplineMovementPathPattern {
    __padding_end: [u8; 160],
}
impl USplineMovementPathPattern {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineMovementPathPattern")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineMovementPathPattern")
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
pub struct UPhysicsCharacterMoverComponent {
    __padding_end: [u8; 1936],
}
impl UPhysicsCharacterMoverComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsCharacterMoverComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsCharacterMoverComponent")
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
pub struct UPhysicsMoverManager {
    __padding_end: [u8; 104],
}
impl UPhysicsMoverManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsMoverManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsMoverManager")
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
pub struct IPhysicsCharacterMovementModeInterface {}
#[repr(C, align(8))]
pub struct UPhysicsCharacterMovementModeInterface {
    __padding_end: [u8; 48],
}
impl UPhysicsCharacterMovementModeInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsCharacterMovementModeInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsCharacterMovementModeInterface")
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
pub struct UPhysicsJumpCheck {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub jump_upwards_speed: f32,
    pub fractional_ground_reaction_impulse: f32,
    pub transition_to_mode: FName,
}
impl UPhysicsJumpCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsJumpCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsJumpCheck")
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
pub struct UPhysicsLaunchCheck {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub transition_to_mode: FName,
}
impl UPhysicsLaunchCheck {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsLaunchCheck")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsLaunchCheck")
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
pub struct FBindProcessGeneratedMovement_ProcessGeneratedMovementEvent {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnPreSimulationTick {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnPostMovement {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnPostSimulationTick {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnPostSimulationRollback {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnMovementModeChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnTeleportSucceeded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnTeleportFailed {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnMovementTransitionTriggered {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnPostFinalize {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FMoverComponent_OnPreMovement {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FCharacterMoverComponent_OnStanceChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncFallingMode_OnLanded {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FFallingMode_OnLanded {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct EMoveMixMode(pub u8);
impl EMoveMixMode {
    pub const ADDITIVE_VELOCITY: EMoveMixMode = EMoveMixMode(0);
    pub const OVERRIDE_VELOCITY: EMoveMixMode = EMoveMixMode(1);
    pub const OVERRIDE_ALL: EMoveMixMode = EMoveMixMode(2);
    pub const OVERRIDE_ALL_EXCEPT_VERTICAL_VELOCITY: EMoveMixMode = EMoveMixMode(3);
}
#[repr(transparent)]
pub struct ELayeredMoveFinishVelocityMode(pub u8);
impl ELayeredMoveFinishVelocityMode {
    pub const MAINTAIN_LAST_ROOT_MOTION_VELOCITY: ELayeredMoveFinishVelocityMode = ELayeredMoveFinishVelocityMode(
        0,
    );
    pub const SET_VELOCITY: ELayeredMoveFinishVelocityMode = ELayeredMoveFinishVelocityMode(
        1,
    );
    pub const CLAMP_VELOCITY: ELayeredMoveFinishVelocityMode = ELayeredMoveFinishVelocityMode(
        2,
    );
}
#[repr(transparent)]
pub struct EMoveInputType(pub u8);
impl EMoveInputType {
    pub const INVALID: EMoveInputType = EMoveInputType(0);
    pub const DIRECTIONAL_INTENT: EMoveInputType = EMoveInputType(1);
    pub const VELOCITY: EMoveInputType = EMoveInputType(2);
    pub const NONE: EMoveInputType = EMoveInputType(3);
}
#[repr(transparent)]
pub struct EPathedPhysicsPlaybackBehavior(pub u8);
impl EPathedPhysicsPlaybackBehavior {
    pub const ONE_SHOT: EPathedPhysicsPlaybackBehavior = EPathedPhysicsPlaybackBehavior(
        0,
    );
    pub const THERE_AND_BACK: EPathedPhysicsPlaybackBehavior = EPathedPhysicsPlaybackBehavior(
        1,
    );
    pub const LOOPING: EPathedPhysicsPlaybackBehavior = EPathedPhysicsPlaybackBehavior(
        2,
    );
    pub const PING_PONG: EPathedPhysicsPlaybackBehavior = EPathedPhysicsPlaybackBehavior(
        3,
    );
}
#[repr(transparent)]
pub struct EPointMovementLocationBasis(pub u8);
impl EPointMovementLocationBasis {
    pub const PREVIOUS_POINT: EPointMovementLocationBasis = EPointMovementLocationBasis(
        0,
    );
    pub const PATH_ORIGIN: EPointMovementLocationBasis = EPointMovementLocationBasis(1);
    pub const WORLD: EPointMovementLocationBasis = EPointMovementLocationBasis(2);
}
#[repr(transparent)]
pub struct EMoverLaunchVelocityMode(pub u8);
impl EMoverLaunchVelocityMode {
    pub const ADDITIVE: EMoverLaunchVelocityMode = EMoverLaunchVelocityMode(0);
    pub const OVERRIDE: EMoverLaunchVelocityMode = EMoverLaunchVelocityMode(1);
}
#[repr(transparent)]
pub struct EMoverTickDependencyOrder(pub u8);
impl EMoverTickDependencyOrder {
    pub const BEFORE: EMoverTickDependencyOrder = EMoverTickDependencyOrder(0);
    pub const AFTER: EMoverTickDependencyOrder = EMoverTickDependencyOrder(1);
}
#[repr(transparent)]
pub struct EMoverTickPhase(pub u8);
impl EMoverTickPhase {
    pub const INVALID: EMoverTickPhase = EMoverTickPhase(0);
    pub const PRODUCE_INPUT: EMoverTickPhase = EMoverTickPhase(1);
    pub const SIMULATE_MOVEMENT: EMoverTickPhase = EMoverTickPhase(2);
    pub const APPLY_STATE: EMoverTickPhase = EMoverTickPhase(3);
}
#[repr(transparent)]
pub struct ETeleportFailureReason(pub u8);
impl ETeleportFailureReason {
    pub const REASON_NOT_AVAILABLE: ETeleportFailureReason = ETeleportFailureReason(0);
}
#[repr(transparent)]
pub struct EMoverSmoothingMode(pub u8);
impl EMoverSmoothingMode {
    pub const NONE: EMoverSmoothingMode = EMoverSmoothingMode(0);
    pub const VISUAL_COMPONENT_OFFSET: EMoverSmoothingMode = EMoverSmoothingMode(1);
}
#[repr(transparent)]
pub struct EStanceMode(pub u8);
impl EStanceMode {
    pub const INVALID: EStanceMode = EStanceMode(0);
    pub const CROUCH: EStanceMode = EStanceMode(1);
    pub const PRONE: EStanceMode = EStanceMode(2);
}
#[repr(transparent)]
pub struct EOffNavMeshBehavior(pub u8);
impl EOffNavMeshBehavior {
    pub const SWITCH_TO_WALKING: EOffNavMeshBehavior = EOffNavMeshBehavior(0);
    pub const MOVE_WITHOUT_NAV_MESH: EOffNavMeshBehavior = EOffNavMeshBehavior(1);
    pub const DO_NOT_MOVE: EOffNavMeshBehavior = EOffNavMeshBehavior(2);
    pub const ROTATE_ONLY: EOffNavMeshBehavior = EOffNavMeshBehavior(3);
}
#[repr(transparent)]
pub struct EStaticFloorCheckPolicy(pub u8);
impl EStaticFloorCheckPolicy {
    pub const ALWAYS: EStaticFloorCheckPolicy = EStaticFloorCheckPolicy(0);
    pub const ON_DYNAMIC_BASE_ONLY: EStaticFloorCheckPolicy = EStaticFloorCheckPolicy(1);
}
