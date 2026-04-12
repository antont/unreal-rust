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
    pub u_anim_node_control_rig_library_set_control_rig_class: *mut crate::ffi::UFunctionOpague,
    pub u_anim_node_control_rig_library_convert_to_control_rig_pure: *mut crate::ffi::UFunctionOpague,
    pub u_anim_node_control_rig_library_convert_to_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_supports_backwards_solve: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_set_interaction_rig_class: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_set_interaction_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_select_control: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_request_construction: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_on_control_selected_bp_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_is_control_selected: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_get_interaction_rig_class: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_get_interaction_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_get_hosting_actor: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_get_hierarchy: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_find_control_rigs: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_current_control_selection: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_create_transformable_control_handle: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_clear_control_selection: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_shape_library_link_set_shape_library: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_shape_library_link_get_shape_library: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_update: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_object_binding: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_mapped_elements: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_initial_space_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_initial_bone_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_scale: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_rig_class: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_position: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_offset: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_int: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_float: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_control_bool: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_bone_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_set_bone_initial_transforms_from_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_on_pre_initialize: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_on_pre_forwards_solve: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_on_pre_construction: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_on_post_initialize: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_on_post_forwards_solve: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_on_post_construction: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_initialize: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_space_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_initial_space_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_initial_bone_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_element_names: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_scale: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_position: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_offset: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_int: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_float: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_control_bool: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_bone_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_get_absolute_time: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_does_element_exist: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_clear_mapped_elements: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_can_execute: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_add_mapped_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_add_mapped_elements: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_add_mapped_components: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_component_add_mapped_complete_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_control_actor_reset_control_actor: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_control_actor_refresh: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_control_actor_clear: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_set_selected: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_set_selectable: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_set_hovered: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_set_global_transform: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_set_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_on_transform_changed: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_on_selection_changed: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_on_manipulating_changed: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_on_hovered_changed: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_on_enabled_changed: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_is_selected_in_editor: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_is_hovered: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_is_enabled: *mut crate::ffi::UFunctionOpague,
    pub a_control_rig_shape_actor_get_global_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_stop_replay: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_stop_recording: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_start_replay: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_start_recording: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_set_playback_mode: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_pause_replay: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_is_replaying: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_is_recording: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_is_paused: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_get_time_range: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_get_playback_mode: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_replay_create_new_asset: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_test_data_get_frame_index_for_time: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_parent_path_for_bp: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_parent_module_name_for_bp: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_module_rig_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_module_rig: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_module_paths: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_module_names: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_events_for_module_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_events_for_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_get_events_for_all_modules: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_execute_event_on_module_for_bp: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_execute_event_on_module_by_name_for_bp: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_execute_event_on_all_modules: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_un_bind_module_variable: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_swap_modules_of_class: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_swap_module_class: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_set_module_selection: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_set_config_value_in_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_select_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_reset_config_value_in_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_reparent_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_reorder_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_rename_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_mirror_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_import_module_settings_from_string: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_get_selected_modules: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_get_module_reference: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_get_connectors_for_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_get_all_modules: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_export_module_settings_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_disconnect_cyclic_connectors: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_disconnect_connector: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_deselect_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_delete_module: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_connect_connector_to_elements: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_connect_connector_to_element: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_can_connect_connector_to_elements: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_can_connect_connector_to_element: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_bind_module_variable: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_auto_connect_secondary_connectors: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_auto_connect_modules: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_add_target_to_array_connector: *mut crate::ffi::UFunctionOpague,
    pub u_modular_rig_controller_add_module: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_unset_curve_value_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_unset_curve_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_switch_to_world_space: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_switch_to_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_switch_to_default_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_sort_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_vector_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_vector_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_transform_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_transform_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_tag: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_rotator_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_rotator_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_rig_element_key_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_rig_element_key_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_quat_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_quat_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_pose_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_parent_weight_array: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_parent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_name_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_name_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_local_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_linear_color_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_linear_color_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_int32_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_int32_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_global_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_global_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_float_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_float_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_curve_value_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_curve_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_visibility_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_value_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_shape_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_shape_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_settings_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_settings: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_preferred_rotator_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_preferred_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_preferred_rotation_order_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_preferred_rotation_order: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_preferred_euler_angles_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_preferred_euler_angles: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_offset_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_control_offset_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_connector_settings_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_connector_settings: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_bool_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_set_bool_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_send_auto_key_event: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_restore_sockets_from_states: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_restore_connectors_from_states: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_reset_to_default: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_reset_pose_to_initial: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_reset_curve_values: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_reset: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_remove_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_remove_all_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_num_components: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_num: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_vector: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_transform_no_scale: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_int: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_float: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_euler_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_make_control_value_from_bool: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_valid_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_selected_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_selected: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_procedural: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_parented_to: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_curve_value_set_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_curve_value_set: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_controller_available: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_is_component_selected: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_has_tag: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_vector_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_vector_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_vector_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_vector2_d_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_transform_no_scale_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_transform_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_transform_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_transform_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_tags: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_socket_states: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_socket_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_selected_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_selected_hierarchy_keys_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_rule_manager: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_rotator_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_rotator_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_rotator_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_root_element_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_rig_element_key_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_rig_element_key_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_reference_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_quat_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_quat_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_previous_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_previous_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_previous_hierarchy_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_previous_hierarchy_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_pose: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_parent_weight_array: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_parent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_parent_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_parent_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_parents: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_number_of_parents: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_null_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_name_space_f_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_name_space: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_name_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_name_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_module_prefix: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_module_path_f_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_module_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_module_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_module_f_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_metadata_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_metadata_names: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_local_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_local_index_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_local_control_shape_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_local_control_shape_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_linear_color_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_linear_color_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_key: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_int_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_int32_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_int32_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_index_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_global_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_global_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_global_control_shape_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_global_control_shape_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_global_control_offset_transform_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_global_control_offset_transform: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_float_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_float_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_float_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_first_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_euler_transform_from_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_default_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_curve_value_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_curve_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_curve_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_value_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_preferred_rotator_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_preferred_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_preferred_euler_rotation_order_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_preferred_euler_rotation_order: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_preferred_euler_angles_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_preferred_euler_angles: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_controller: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_control_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_connector_states: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_connector_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_component_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_component_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_component_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_component_key: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_component_content: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_children: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_bool_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_bool_array_metadata: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_bone_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_all_keys_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_get_all_component_keys: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_find_null_for_blueprint_only: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_find_control_for_blueprint_only: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_find_bone_for_blueprint_only: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_copy_pose: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_copy_hierarchy: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_contains_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_selection: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_hierarchy_selection: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_hierarchy: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_control_settings: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_component_selection: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_component_content: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_available_space_label: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_set_available_space_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_select_hierarchy_key: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_select_element: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_select_component: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_reparent_component: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_reorder_element: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_rename_element: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_rename_component: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_remove_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_remove_element: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_remove_component: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_remove_channel_host: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_remove_available_space: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_remove_all_parents: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_mirror_elements: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_sockets_from_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_preview_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_from_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_curves_from_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_curves_from_asset: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_curves: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_bones_from_skeletal_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_bones_from_asset: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_import_bones: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_get_hierarchy: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_get_control_settings: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_generate_python_commands: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_export_to_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_export_selection_to_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_duplicate_elements: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_deselect_hierarchy_key: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_deselect_element: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_deselect_component: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_clear_selection: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_socket: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_parent: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_null: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_curve: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_control_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_connector: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_component: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_channel_host: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_bone: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_available_space: *mut crate::ffi::UFunctionOpague,
    pub u_rig_hierarchy_controller_add_animation_channel_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_set_up_mirror_match_table: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_select_controls: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_save_pose: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_replace_control_name: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_paste_pose: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_get_current_pose: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_get_control_names: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_pose_asset_does_mirror_match: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_workflow_options_ensure_at_least_one_rig_element_selected: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_transform_workflow_options_provide_workflows: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_anim_node_control_rig_library_set_control_rig_class: std::ptr::null_mut(),
            u_anim_node_control_rig_library_convert_to_control_rig_pure: std::ptr::null_mut(),
            u_anim_node_control_rig_library_convert_to_control_rig: std::ptr::null_mut(),
            u_control_rig_supports_backwards_solve: std::ptr::null_mut(),
            u_control_rig_set_interaction_rig_class: std::ptr::null_mut(),
            u_control_rig_set_interaction_rig: std::ptr::null_mut(),
            u_control_rig_select_control: std::ptr::null_mut(),
            u_control_rig_request_construction: std::ptr::null_mut(),
            u_control_rig_on_control_selected_bp_delegate_signature: std::ptr::null_mut(),
            u_control_rig_is_control_selected: std::ptr::null_mut(),
            u_control_rig_get_interaction_rig_class: std::ptr::null_mut(),
            u_control_rig_get_interaction_rig: std::ptr::null_mut(),
            u_control_rig_get_hosting_actor: std::ptr::null_mut(),
            u_control_rig_get_hierarchy: std::ptr::null_mut(),
            u_control_rig_find_control_rigs: std::ptr::null_mut(),
            u_control_rig_current_control_selection: std::ptr::null_mut(),
            u_control_rig_create_transformable_control_handle: std::ptr::null_mut(),
            u_control_rig_clear_control_selection: std::ptr::null_mut(),
            u_control_rig_shape_library_link_set_shape_library: std::ptr::null_mut(),
            u_control_rig_shape_library_link_get_shape_library: std::ptr::null_mut(),
            u_control_rig_component_update: std::ptr::null_mut(),
            u_control_rig_component_set_object_binding: std::ptr::null_mut(),
            u_control_rig_component_set_mapped_elements: std::ptr::null_mut(),
            u_control_rig_component_set_initial_space_transform: std::ptr::null_mut(),
            u_control_rig_component_set_initial_bone_transform: std::ptr::null_mut(),
            u_control_rig_component_set_control_vector2_d: std::ptr::null_mut(),
            u_control_rig_component_set_control_transform: std::ptr::null_mut(),
            u_control_rig_component_set_control_scale: std::ptr::null_mut(),
            u_control_rig_component_set_control_rotator: std::ptr::null_mut(),
            u_control_rig_component_set_control_rig_class: std::ptr::null_mut(),
            u_control_rig_component_set_control_position: std::ptr::null_mut(),
            u_control_rig_component_set_control_offset: std::ptr::null_mut(),
            u_control_rig_component_set_control_int: std::ptr::null_mut(),
            u_control_rig_component_set_control_float: std::ptr::null_mut(),
            u_control_rig_component_set_control_bool: std::ptr::null_mut(),
            u_control_rig_component_set_bone_transform: std::ptr::null_mut(),
            u_control_rig_component_set_bone_initial_transforms_from_skeletal_mesh: std::ptr::null_mut(),
            u_control_rig_component_on_pre_initialize: std::ptr::null_mut(),
            u_control_rig_component_on_pre_forwards_solve: std::ptr::null_mut(),
            u_control_rig_component_on_pre_construction: std::ptr::null_mut(),
            u_control_rig_component_on_post_initialize: std::ptr::null_mut(),
            u_control_rig_component_on_post_forwards_solve: std::ptr::null_mut(),
            u_control_rig_component_on_post_construction: std::ptr::null_mut(),
            u_control_rig_component_initialize: std::ptr::null_mut(),
            u_control_rig_component_get_space_transform: std::ptr::null_mut(),
            u_control_rig_component_get_initial_space_transform: std::ptr::null_mut(),
            u_control_rig_component_get_initial_bone_transform: std::ptr::null_mut(),
            u_control_rig_component_get_element_names: std::ptr::null_mut(),
            u_control_rig_component_get_control_vector2_d: std::ptr::null_mut(),
            u_control_rig_component_get_control_transform: std::ptr::null_mut(),
            u_control_rig_component_get_control_scale: std::ptr::null_mut(),
            u_control_rig_component_get_control_rotator: std::ptr::null_mut(),
            u_control_rig_component_get_control_rig: std::ptr::null_mut(),
            u_control_rig_component_get_control_position: std::ptr::null_mut(),
            u_control_rig_component_get_control_offset: std::ptr::null_mut(),
            u_control_rig_component_get_control_int: std::ptr::null_mut(),
            u_control_rig_component_get_control_float: std::ptr::null_mut(),
            u_control_rig_component_get_control_bool: std::ptr::null_mut(),
            u_control_rig_component_get_bone_transform: std::ptr::null_mut(),
            u_control_rig_component_get_absolute_time: std::ptr::null_mut(),
            u_control_rig_component_does_element_exist: std::ptr::null_mut(),
            u_control_rig_component_clear_mapped_elements: std::ptr::null_mut(),
            u_control_rig_component_can_execute: std::ptr::null_mut(),
            u_control_rig_component_add_mapped_skeletal_mesh: std::ptr::null_mut(),
            u_control_rig_component_add_mapped_elements: std::ptr::null_mut(),
            u_control_rig_component_add_mapped_components: std::ptr::null_mut(),
            u_control_rig_component_add_mapped_complete_skeletal_mesh: std::ptr::null_mut(),
            a_control_rig_control_actor_reset_control_actor: std::ptr::null_mut(),
            a_control_rig_control_actor_refresh: std::ptr::null_mut(),
            a_control_rig_control_actor_clear: std::ptr::null_mut(),
            a_control_rig_shape_actor_set_selected: std::ptr::null_mut(),
            a_control_rig_shape_actor_set_selectable: std::ptr::null_mut(),
            a_control_rig_shape_actor_set_hovered: std::ptr::null_mut(),
            a_control_rig_shape_actor_set_global_transform: std::ptr::null_mut(),
            a_control_rig_shape_actor_set_enabled: std::ptr::null_mut(),
            a_control_rig_shape_actor_on_transform_changed: std::ptr::null_mut(),
            a_control_rig_shape_actor_on_selection_changed: std::ptr::null_mut(),
            a_control_rig_shape_actor_on_manipulating_changed: std::ptr::null_mut(),
            a_control_rig_shape_actor_on_hovered_changed: std::ptr::null_mut(),
            a_control_rig_shape_actor_on_enabled_changed: std::ptr::null_mut(),
            a_control_rig_shape_actor_is_selected_in_editor: std::ptr::null_mut(),
            a_control_rig_shape_actor_is_hovered: std::ptr::null_mut(),
            a_control_rig_shape_actor_is_enabled: std::ptr::null_mut(),
            a_control_rig_shape_actor_get_global_transform: std::ptr::null_mut(),
            u_control_rig_replay_stop_replay: std::ptr::null_mut(),
            u_control_rig_replay_stop_recording: std::ptr::null_mut(),
            u_control_rig_replay_start_replay: std::ptr::null_mut(),
            u_control_rig_replay_start_recording: std::ptr::null_mut(),
            u_control_rig_replay_set_playback_mode: std::ptr::null_mut(),
            u_control_rig_replay_pause_replay: std::ptr::null_mut(),
            u_control_rig_replay_is_replaying: std::ptr::null_mut(),
            u_control_rig_replay_is_recording: std::ptr::null_mut(),
            u_control_rig_replay_is_paused: std::ptr::null_mut(),
            u_control_rig_replay_get_time_range: std::ptr::null_mut(),
            u_control_rig_replay_get_playback_mode: std::ptr::null_mut(),
            u_control_rig_replay_create_new_asset: std::ptr::null_mut(),
            u_control_rig_test_data_get_frame_index_for_time: std::ptr::null_mut(),
            u_modular_rig_get_parent_path_for_bp: std::ptr::null_mut(),
            u_modular_rig_get_parent_module_name_for_bp: std::ptr::null_mut(),
            u_modular_rig_get_module_rig_by_name: std::ptr::null_mut(),
            u_modular_rig_get_module_rig: std::ptr::null_mut(),
            u_modular_rig_get_module_paths: std::ptr::null_mut(),
            u_modular_rig_get_module_names: std::ptr::null_mut(),
            u_modular_rig_get_events_for_module_by_name: std::ptr::null_mut(),
            u_modular_rig_get_events_for_module: std::ptr::null_mut(),
            u_modular_rig_get_events_for_all_modules: std::ptr::null_mut(),
            u_modular_rig_execute_event_on_module_for_bp: std::ptr::null_mut(),
            u_modular_rig_execute_event_on_module_by_name_for_bp: std::ptr::null_mut(),
            u_modular_rig_execute_event_on_all_modules: std::ptr::null_mut(),
            u_modular_rig_controller_un_bind_module_variable: std::ptr::null_mut(),
            u_modular_rig_controller_swap_modules_of_class: std::ptr::null_mut(),
            u_modular_rig_controller_swap_module_class: std::ptr::null_mut(),
            u_modular_rig_controller_set_module_selection: std::ptr::null_mut(),
            u_modular_rig_controller_set_config_value_in_module: std::ptr::null_mut(),
            u_modular_rig_controller_select_module: std::ptr::null_mut(),
            u_modular_rig_controller_reset_config_value_in_module: std::ptr::null_mut(),
            u_modular_rig_controller_reparent_module: std::ptr::null_mut(),
            u_modular_rig_controller_reorder_module: std::ptr::null_mut(),
            u_modular_rig_controller_rename_module: std::ptr::null_mut(),
            u_modular_rig_controller_mirror_module: std::ptr::null_mut(),
            u_modular_rig_controller_import_module_settings_from_string: std::ptr::null_mut(),
            u_modular_rig_controller_get_selected_modules: std::ptr::null_mut(),
            u_modular_rig_controller_get_module_reference: std::ptr::null_mut(),
            u_modular_rig_controller_get_connectors_for_module: std::ptr::null_mut(),
            u_modular_rig_controller_get_all_modules: std::ptr::null_mut(),
            u_modular_rig_controller_export_module_settings_to_string: std::ptr::null_mut(),
            u_modular_rig_controller_disconnect_cyclic_connectors: std::ptr::null_mut(),
            u_modular_rig_controller_disconnect_connector: std::ptr::null_mut(),
            u_modular_rig_controller_deselect_module: std::ptr::null_mut(),
            u_modular_rig_controller_delete_module: std::ptr::null_mut(),
            u_modular_rig_controller_connect_connector_to_elements: std::ptr::null_mut(),
            u_modular_rig_controller_connect_connector_to_element: std::ptr::null_mut(),
            u_modular_rig_controller_can_connect_connector_to_elements: std::ptr::null_mut(),
            u_modular_rig_controller_can_connect_connector_to_element: std::ptr::null_mut(),
            u_modular_rig_controller_bind_module_variable: std::ptr::null_mut(),
            u_modular_rig_controller_auto_connect_secondary_connectors: std::ptr::null_mut(),
            u_modular_rig_controller_auto_connect_modules: std::ptr::null_mut(),
            u_modular_rig_controller_add_target_to_array_connector: std::ptr::null_mut(),
            u_modular_rig_controller_add_module: std::ptr::null_mut(),
            u_rig_hierarchy_unset_curve_value_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_unset_curve_value: std::ptr::null_mut(),
            u_rig_hierarchy_switch_to_world_space: std::ptr::null_mut(),
            u_rig_hierarchy_switch_to_parent: std::ptr::null_mut(),
            u_rig_hierarchy_switch_to_default_parent: std::ptr::null_mut(),
            u_rig_hierarchy_sort_keys: std::ptr::null_mut(),
            u_rig_hierarchy_set_vector_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_vector_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_transform_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_transform_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_tag: std::ptr::null_mut(),
            u_rig_hierarchy_set_rotator_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_rotator_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_rig_element_key_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_rig_element_key_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_quat_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_quat_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_pose_for_blueprint: std::ptr::null_mut(),
            u_rig_hierarchy_set_parent_weight_array: std::ptr::null_mut(),
            u_rig_hierarchy_set_parent_weight: std::ptr::null_mut(),
            u_rig_hierarchy_set_name_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_name_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_local_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_local_transform: std::ptr::null_mut(),
            u_rig_hierarchy_set_linear_color_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_linear_color_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_int32_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_int32_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_global_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_global_transform: std::ptr::null_mut(),
            u_rig_hierarchy_set_float_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_float_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_curve_value_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_curve_value: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_visibility_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_visibility: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_value_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_shape_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_shape_transform: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_settings_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_settings: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_preferred_rotator_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_preferred_rotator: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_preferred_rotation_order_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_preferred_rotation_order: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_preferred_euler_angles_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_preferred_euler_angles: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_offset_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_control_offset_transform: std::ptr::null_mut(),
            u_rig_hierarchy_set_connector_settings_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_set_connector_settings: std::ptr::null_mut(),
            u_rig_hierarchy_set_bool_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_set_bool_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_send_auto_key_event: std::ptr::null_mut(),
            u_rig_hierarchy_restore_sockets_from_states: std::ptr::null_mut(),
            u_rig_hierarchy_restore_connectors_from_states: std::ptr::null_mut(),
            u_rig_hierarchy_reset_to_default: std::ptr::null_mut(),
            u_rig_hierarchy_reset_pose_to_initial: std::ptr::null_mut(),
            u_rig_hierarchy_reset_curve_values: std::ptr::null_mut(),
            u_rig_hierarchy_reset: std::ptr::null_mut(),
            u_rig_hierarchy_remove_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_remove_all_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_num_components: std::ptr::null_mut(),
            u_rig_hierarchy_num: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_vector2_d: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_vector: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_transform_no_scale: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_transform: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_rotator: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_int: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_float: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_euler_transform: std::ptr::null_mut(),
            u_rig_hierarchy_make_control_value_from_bool: std::ptr::null_mut(),
            u_rig_hierarchy_is_valid_index: std::ptr::null_mut(),
            u_rig_hierarchy_is_selected_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_is_selected: std::ptr::null_mut(),
            u_rig_hierarchy_is_procedural: std::ptr::null_mut(),
            u_rig_hierarchy_is_parented_to: std::ptr::null_mut(),
            u_rig_hierarchy_is_curve_value_set_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_is_curve_value_set: std::ptr::null_mut(),
            u_rig_hierarchy_is_controller_available: std::ptr::null_mut(),
            u_rig_hierarchy_is_component_selected: std::ptr::null_mut(),
            u_rig_hierarchy_has_tag: std::ptr::null_mut(),
            u_rig_hierarchy_get_vector_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_vector_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_vector_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_vector2_d_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_transform_no_scale_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_transform_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_transform_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_transform_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_tags: std::ptr::null_mut(),
            u_rig_hierarchy_get_socket_states: std::ptr::null_mut(),
            u_rig_hierarchy_get_socket_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_selected_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_selected_hierarchy_keys_for_blueprint: std::ptr::null_mut(),
            u_rig_hierarchy_get_rule_manager: std::ptr::null_mut(),
            u_rig_hierarchy_get_rotator_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_rotator_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_rotator_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_root_element_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_rig_element_key_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_rig_element_key_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_reference_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_quat_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_quat_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_previous_parent: std::ptr::null_mut(),
            u_rig_hierarchy_get_previous_name: std::ptr::null_mut(),
            u_rig_hierarchy_get_previous_hierarchy_parent: std::ptr::null_mut(),
            u_rig_hierarchy_get_previous_hierarchy_name: std::ptr::null_mut(),
            u_rig_hierarchy_get_pose: std::ptr::null_mut(),
            u_rig_hierarchy_get_parent_weight_array: std::ptr::null_mut(),
            u_rig_hierarchy_get_parent_weight: std::ptr::null_mut(),
            u_rig_hierarchy_get_parent_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_parent_transform: std::ptr::null_mut(),
            u_rig_hierarchy_get_parents: std::ptr::null_mut(),
            u_rig_hierarchy_get_number_of_parents: std::ptr::null_mut(),
            u_rig_hierarchy_get_null_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_name_space_f_name: std::ptr::null_mut(),
            u_rig_hierarchy_get_name_space: std::ptr::null_mut(),
            u_rig_hierarchy_get_name_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_name_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_module_prefix: std::ptr::null_mut(),
            u_rig_hierarchy_get_module_path_f_name: std::ptr::null_mut(),
            u_rig_hierarchy_get_module_path: std::ptr::null_mut(),
            u_rig_hierarchy_get_module_name: std::ptr::null_mut(),
            u_rig_hierarchy_get_module_f_name: std::ptr::null_mut(),
            u_rig_hierarchy_get_metadata_type: std::ptr::null_mut(),
            u_rig_hierarchy_get_metadata_names: std::ptr::null_mut(),
            u_rig_hierarchy_get_local_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_local_transform: std::ptr::null_mut(),
            u_rig_hierarchy_get_local_index_for_blueprint: std::ptr::null_mut(),
            u_rig_hierarchy_get_local_control_shape_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_local_control_shape_transform: std::ptr::null_mut(),
            u_rig_hierarchy_get_linear_color_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_linear_color_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_key: std::ptr::null_mut(),
            u_rig_hierarchy_get_int_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_int32_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_int32_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_index_for_blueprint: std::ptr::null_mut(),
            u_rig_hierarchy_get_global_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_global_transform: std::ptr::null_mut(),
            u_rig_hierarchy_get_global_control_shape_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_global_control_shape_transform: std::ptr::null_mut(),
            u_rig_hierarchy_get_global_control_offset_transform_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_global_control_offset_transform: std::ptr::null_mut(),
            u_rig_hierarchy_get_float_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_float_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_float_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_first_parent: std::ptr::null_mut(),
            u_rig_hierarchy_get_euler_transform_from_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_default_parent: std::ptr::null_mut(),
            u_rig_hierarchy_get_curve_value_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_curve_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_curve_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_value_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_value: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_preferred_rotator_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_preferred_rotator: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_preferred_euler_rotation_order_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_preferred_euler_rotation_order: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_preferred_euler_angles_by_index: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_preferred_euler_angles: std::ptr::null_mut(),
            u_rig_hierarchy_get_controller: std::ptr::null_mut(),
            u_rig_hierarchy_get_control_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_connector_states: std::ptr::null_mut(),
            u_rig_hierarchy_get_connector_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_component_type: std::ptr::null_mut(),
            u_rig_hierarchy_get_component_name: std::ptr::null_mut(),
            u_rig_hierarchy_get_component_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_component_key: std::ptr::null_mut(),
            u_rig_hierarchy_get_component_content: std::ptr::null_mut(),
            u_rig_hierarchy_get_children: std::ptr::null_mut(),
            u_rig_hierarchy_get_bool_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_bool_array_metadata: std::ptr::null_mut(),
            u_rig_hierarchy_get_bone_keys: std::ptr::null_mut(),
            u_rig_hierarchy_get_all_keys_for_blueprint: std::ptr::null_mut(),
            u_rig_hierarchy_get_all_component_keys: std::ptr::null_mut(),
            u_rig_hierarchy_find_null_for_blueprint_only: std::ptr::null_mut(),
            u_rig_hierarchy_find_control_for_blueprint_only: std::ptr::null_mut(),
            u_rig_hierarchy_find_bone_for_blueprint_only: std::ptr::null_mut(),
            u_rig_hierarchy_copy_pose: std::ptr::null_mut(),
            u_rig_hierarchy_copy_hierarchy: std::ptr::null_mut(),
            u_rig_hierarchy_contains_for_blueprint: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_selection: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_parent: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_hierarchy_selection: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_hierarchy: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_display_name: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_control_settings: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_component_selection: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_component_content: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_available_space_label: std::ptr::null_mut(),
            u_rig_hierarchy_controller_set_available_space_index: std::ptr::null_mut(),
            u_rig_hierarchy_controller_select_hierarchy_key: std::ptr::null_mut(),
            u_rig_hierarchy_controller_select_element: std::ptr::null_mut(),
            u_rig_hierarchy_controller_select_component: std::ptr::null_mut(),
            u_rig_hierarchy_controller_reparent_component: std::ptr::null_mut(),
            u_rig_hierarchy_controller_reorder_element: std::ptr::null_mut(),
            u_rig_hierarchy_controller_rename_element: std::ptr::null_mut(),
            u_rig_hierarchy_controller_rename_component: std::ptr::null_mut(),
            u_rig_hierarchy_controller_remove_parent: std::ptr::null_mut(),
            u_rig_hierarchy_controller_remove_element: std::ptr::null_mut(),
            u_rig_hierarchy_controller_remove_component: std::ptr::null_mut(),
            u_rig_hierarchy_controller_remove_channel_host: std::ptr::null_mut(),
            u_rig_hierarchy_controller_remove_available_space: std::ptr::null_mut(),
            u_rig_hierarchy_controller_remove_all_parents: std::ptr::null_mut(),
            u_rig_hierarchy_controller_mirror_elements: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_sockets_from_skeletal_mesh: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_preview_skeletal_mesh: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_from_text: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_curves_from_skeletal_mesh: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_curves_from_asset: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_curves: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_bones_from_skeletal_mesh: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_bones_from_asset: std::ptr::null_mut(),
            u_rig_hierarchy_controller_import_bones: std::ptr::null_mut(),
            u_rig_hierarchy_controller_get_hierarchy: std::ptr::null_mut(),
            u_rig_hierarchy_controller_get_control_settings: std::ptr::null_mut(),
            u_rig_hierarchy_controller_generate_python_commands: std::ptr::null_mut(),
            u_rig_hierarchy_controller_export_to_text: std::ptr::null_mut(),
            u_rig_hierarchy_controller_export_selection_to_text: std::ptr::null_mut(),
            u_rig_hierarchy_controller_duplicate_elements: std::ptr::null_mut(),
            u_rig_hierarchy_controller_deselect_hierarchy_key: std::ptr::null_mut(),
            u_rig_hierarchy_controller_deselect_element: std::ptr::null_mut(),
            u_rig_hierarchy_controller_deselect_component: std::ptr::null_mut(),
            u_rig_hierarchy_controller_clear_selection: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_socket: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_parent: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_null: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_curve: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_control_for_blueprint: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_connector: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_component: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_channel_host: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_bone: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_available_space: std::ptr::null_mut(),
            u_rig_hierarchy_controller_add_animation_channel_for_blueprint: std::ptr::null_mut(),
            u_control_rig_pose_asset_set_up_mirror_match_table: std::ptr::null_mut(),
            u_control_rig_pose_asset_select_controls: std::ptr::null_mut(),
            u_control_rig_pose_asset_save_pose: std::ptr::null_mut(),
            u_control_rig_pose_asset_replace_control_name: std::ptr::null_mut(),
            u_control_rig_pose_asset_paste_pose: std::ptr::null_mut(),
            u_control_rig_pose_asset_get_current_pose: std::ptr::null_mut(),
            u_control_rig_pose_asset_get_control_names: std::ptr::null_mut(),
            u_control_rig_pose_asset_does_mirror_match: std::ptr::null_mut(),
            u_control_rig_workflow_options_ensure_at_least_one_rig_element_selected: std::ptr::null_mut(),
            u_control_rig_transform_workflow_options_provide_workflows: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimNodeControlRigLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigClass"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_node_control_rig_library_set_control_rig_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToControlRigPure"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_node_control_rig_library_convert_to_control_rig_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConvertToControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_anim_node_control_rig_library_convert_to_control_rig,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRig::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SupportsBackwardsSolve"),
                &raw mut __FUNCTION_PTRS.u_control_rig_supports_backwards_solve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInteractionRigClass"),
                &raw mut __FUNCTION_PTRS.u_control_rig_set_interaction_rig_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInteractionRig"),
                &raw mut __FUNCTION_PTRS.u_control_rig_set_interaction_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectControl"),
                &raw mut __FUNCTION_PTRS.u_control_rig_select_control,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestConstruction"),
                &raw mut __FUNCTION_PTRS.u_control_rig_request_construction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnControlSelectedBP__DelegateSignature"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_on_control_selected_bp_delegate_signature,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsControlSelected"),
                &raw mut __FUNCTION_PTRS.u_control_rig_is_control_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInteractionRigClass"),
                &raw mut __FUNCTION_PTRS.u_control_rig_get_interaction_rig_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInteractionRig"),
                &raw mut __FUNCTION_PTRS.u_control_rig_get_interaction_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHostingActor"),
                &raw mut __FUNCTION_PTRS.u_control_rig_get_hosting_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHierarchy"),
                &raw mut __FUNCTION_PTRS.u_control_rig_get_hierarchy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindControlRigs"),
                &raw mut __FUNCTION_PTRS.u_control_rig_find_control_rigs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CurrentControlSelection"),
                &raw mut __FUNCTION_PTRS.u_control_rig_current_control_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateTransformableControlHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_create_transformable_control_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearControlSelection"),
                &raw mut __FUNCTION_PTRS.u_control_rig_clear_control_selection,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigShapeLibraryLink::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetShapeLibrary"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_shape_library_link_set_shape_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetShapeLibrary"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_shape_library_link_get_shape_library,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Update"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetObjectBinding"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_object_binding,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetMappedElements"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_mapped_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInitialSpaceTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_component_set_initial_space_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInitialBoneTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_component_set_initial_bone_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlVector2D"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_vector2_d,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlTransform"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlScale"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRotator"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigClass"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_rig_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlPosition"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlOffset"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlInt"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlFloat"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlBool"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_control_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBoneTransform"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_set_bone_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBoneInitialTransformsFromSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_component_set_bone_initial_transforms_from_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPreInitialize"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_on_pre_initialize,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPreForwardsSolve"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_on_pre_forwards_solve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPreConstruction"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_on_pre_construction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPostInitialize"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_on_post_initialize,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPostForwardsSolve"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_on_post_forwards_solve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPostConstruction"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_on_post_construction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Initialize"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_initialize,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSpaceTransform"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_space_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInitialSpaceTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_component_get_initial_space_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInitialBoneTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_component_get_initial_bone_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetElementNames"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_element_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlVector2D"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_vector2_d,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlTransform"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlScale"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRotator"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRig"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlPosition"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlOffset"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_offset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlInt"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlFloat"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlBool"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_control_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoneTransform"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_bone_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAbsoluteTime"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_get_absolute_time,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoesElementExist"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_does_element_exist,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearMappedElements"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_clear_mapped_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanExecute"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_can_execute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMappedSkeletalMesh"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_add_mapped_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMappedElements"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_add_mapped_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMappedComponents"),
                &raw mut __FUNCTION_PTRS.u_control_rig_component_add_mapped_components,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMappedCompleteSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_complete_skeletal_mesh,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AControlRigControlActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetControlActor"),
                &raw mut __FUNCTION_PTRS.a_control_rig_control_actor_reset_control_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Refresh"),
                &raw mut __FUNCTION_PTRS.a_control_rig_control_actor_refresh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Clear"),
                &raw mut __FUNCTION_PTRS.a_control_rig_control_actor_clear,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AControlRigShapeActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSelected"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_set_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSelectable"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_set_selectable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHovered"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_set_hovered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGlobalTransform"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_set_global_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnabled"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_set_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnTransformChanged"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_on_transform_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnSelectionChanged"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_on_selection_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnManipulatingChanged"),
                &raw mut __FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_manipulating_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnHoveredChanged"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_on_hovered_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnEnabledChanged"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_on_enabled_changed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSelectedInEditor"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_is_selected_in_editor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsHovered"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_is_hovered,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnabled"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_is_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalTransform"),
                &raw mut __FUNCTION_PTRS.a_control_rig_shape_actor_get_global_transform,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigReplay::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopReplay"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_stop_replay,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopRecording"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_stop_recording,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartReplay"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_start_replay,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartRecording"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_start_recording,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPlaybackMode"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_set_playback_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PauseReplay"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_pause_replay,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReplaying"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_is_replaying,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRecording"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_is_recording,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPaused"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_is_paused,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTimeRange"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_get_time_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlaybackMode"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_get_playback_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateNewAsset"),
                &raw mut __FUNCTION_PTRS.u_control_rig_replay_create_new_asset,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigTestData::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFrameIndexForTime"),
                &raw mut __FUNCTION_PTRS.u_control_rig_test_data_get_frame_index_for_time,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UModularRig::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentPathForBP"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_parent_path_for_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentModuleNameForBP"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_parent_module_name_for_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModuleRigByName"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_module_rig_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModuleRig"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_module_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModulePaths"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_module_paths,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModuleNames"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_module_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEventsForModuleByName"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_events_for_module_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEventsForModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_events_for_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEventsForAllModules"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_get_events_for_all_modules,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExecuteEventOnModuleForBP"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_execute_event_on_module_for_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExecuteEventOnModuleByNameForBP"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_execute_event_on_module_by_name_for_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExecuteEventOnAllModules"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_execute_event_on_all_modules,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UModularRigController::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnBindModuleVariable"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_un_bind_module_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwapModulesOfClass"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_swap_modules_of_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwapModuleClass"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_swap_module_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetModuleSelection"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_set_module_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetConfigValueInModule"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_set_config_value_in_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_select_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetConfigValueInModule"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_reset_config_value_in_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReparentModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_reparent_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReorderModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_reorder_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_rename_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MirrorModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_mirror_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportModuleSettingsFromString"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_import_module_settings_from_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedModules"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_get_selected_modules,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModuleReference"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_get_module_reference,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConnectorsForModule"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_get_connectors_for_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllModules"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_get_all_modules,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportModuleSettingsToString"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_export_module_settings_to_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DisconnectCyclicConnectors"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_disconnect_cyclic_connectors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DisconnectConnector"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_disconnect_connector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_deselect_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_delete_module,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConnectConnectorToElements"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_connect_connector_to_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ConnectConnectorToElement"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_connect_connector_to_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanConnectConnectorToElements"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_can_connect_connector_to_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanConnectConnectorToElement"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_can_connect_connector_to_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BindModuleVariable"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_bind_module_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AutoConnectSecondaryConnectors"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_auto_connect_secondary_connectors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AutoConnectModules"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_auto_connect_modules,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTargetToArrayConnector"),
                &raw mut __FUNCTION_PTRS
                    .u_modular_rig_controller_add_target_to_array_connector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddModule"),
                &raw mut __FUNCTION_PTRS.u_modular_rig_controller_add_module,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigHierarchy::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnsetCurveValueByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_unset_curve_value_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnsetCurveValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_unset_curve_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwitchToWorldSpace"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_switch_to_world_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwitchToParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_switch_to_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwitchToDefaultParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_switch_to_default_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SortKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_sort_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVectorMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_vector_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVectorArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_vector_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTransformMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_transform_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTransformArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_transform_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTag"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRotatorMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_rotator_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRotatorArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_rotator_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRigElementKeyMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_rig_element_key_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRigElementKeyArrayMetadata"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_rig_element_key_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetQuatMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_quat_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetQuatArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_quat_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPose_ForBlueprint"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_pose_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetParentWeightArray"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_parent_weight_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetParentWeight"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_parent_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNameMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_name_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNameArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_name_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalTransformByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_local_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalTransform"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_local_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLinearColorMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_linear_color_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLinearColorArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_linear_color_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInt32Metadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_int32_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInt32ArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_int32_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGlobalTransformByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_global_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGlobalTransform"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_global_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFloatMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_float_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFloatArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_float_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCurveValueByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_curve_value_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCurveValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_curve_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlVisibilityByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_visibility_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlVisibility"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_visibility,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlValueByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_value_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlShapeTransformByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_shape_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlShapeTransform"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_shape_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlSettingsByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_settings_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlSettings"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlPreferredRotatorByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotator_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlPreferredRotator"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_preferred_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlPreferredRotationOrderByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotation_order_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlPreferredRotationOrder"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotation_order,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlPreferredEulerAnglesByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_euler_angles_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlPreferredEulerAngles"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_euler_angles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlOffsetTransformByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_offset_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlOffsetTransform"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_control_offset_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetConnectorSettingsByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_connector_settings_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetConnectorSettings"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_connector_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBoolMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_bool_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBoolArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_set_bool_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SendAutoKeyEvent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_send_auto_key_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RestoreSocketsFromStates"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_restore_sockets_from_states,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RestoreConnectorsFromStates"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_restore_connectors_from_states,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetToDefault"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_reset_to_default,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetPoseToInitial"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_reset_pose_to_initial,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetCurveValues"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_reset_curve_values,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reset"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_reset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_remove_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_remove_all_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NumComponents"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_num_components,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Num"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_num,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromVector2D"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_vector2_d,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromVector"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_make_control_value_from_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromTransformNoScale"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_transform_no_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromRotator"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_make_control_value_from_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromInt"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_make_control_value_from_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromFloat"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_make_control_value_from_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromEulerTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_euler_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeControlValueFromBool"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_make_control_value_from_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_valid_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSelectedByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_selected_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSelected"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsProcedural"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_procedural,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsParentedTo"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_parented_to,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCurveValueSetByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_curve_value_set_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCurveValueSet"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_curve_value_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsControllerAvailable"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_controller_available,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsComponentSelected"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_is_component_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasTag"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_has_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVectorMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_vector_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVectorFromControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_vector_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVectorArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_vector_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVector2DFromControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_vector2_d_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformNoScaleFromControlValue"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_no_scale_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_transform_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformFromControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_transform_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_transform_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTags"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_tags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSocketStates"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_socket_states,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSocketKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_socket_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_selected_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectedHierarchyKeys_ForBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_selected_hierarchy_keys_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRuleManager"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_rule_manager,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRotatorMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_rotator_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRotatorFromControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_rotator_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRotatorArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_rotator_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRootElementKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_root_element_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRigElementKeyMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_rig_element_key_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRigElementKeyArrayMetadata"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_rig_element_key_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReferenceKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_reference_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetQuatMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_quat_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetQuatArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_quat_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviousParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_previous_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviousName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_previous_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviousHierarchyParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_previous_hierarchy_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviousHierarchyName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_previous_hierarchy_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPose"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentWeightArray"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_parent_weight_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentWeight"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_parent_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentTransformByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_parent_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentTransform"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_parent_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParents"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_parents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNumberOfParents"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_number_of_parents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNullKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_null_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNameSpaceFName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_name_space_f_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNameSpace"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_name_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNameMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_name_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNameArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_name_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModulePrefix"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_module_prefix,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModulePathFName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_module_path_f_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModulePath"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_module_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModuleName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_module_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModuleFName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_module_f_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMetadataType"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_metadata_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMetadataNames"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_metadata_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalTransformByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_local_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalTransform"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_local_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalIndex_ForBlueprint"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_local_index_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlShapeTransformByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_control_shape_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlShapeTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_control_shape_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinearColorMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_linear_color_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinearColorArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_linear_color_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetKey"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIntFromControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_int_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInt32Metadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_int32_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInt32ArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_int32_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIndex_ForBlueprint"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_index_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalTransformByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_global_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalTransform"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_global_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalControlShapeTransformByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_shape_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalControlShapeTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_shape_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalControlOffsetTransformByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_offset_transform_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGlobalControlOffsetTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_offset_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_float_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatFromControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_float_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_float_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFirstParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_first_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEulerTransformFromControlValue"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_euler_transform_from_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_default_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurveValueByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_curve_value_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurveValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_curve_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurveKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_curve_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlValueByIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_control_value_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlValue"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_control_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlPreferredRotatorByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_rotator_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlPreferredRotator"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_control_preferred_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "GetControlPreferredEulerRotationOrderByIndex",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_rotation_order_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlPreferredEulerRotationOrder"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_rotation_order,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlPreferredEulerAnglesByIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_angles_by_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlPreferredEulerAngles"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_angles,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetController"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_control_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConnectorStates"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_connector_states,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConnectorKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_connector_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetComponentType"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_component_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetComponentName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_component_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetComponentKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_component_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetComponentKey"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_component_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetComponentContent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_component_content,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetChildren"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_children,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoolMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_bool_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoolArrayMetadata"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_bool_array_metadata,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBoneKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_bone_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllKeys_ForBlueprint"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_all_keys_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllComponentKeys"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_get_all_component_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindNull_ForBlueprintOnly"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_find_null_for_blueprint_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindControl_ForBlueprintOnly"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_find_control_for_blueprint_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindBone_ForBlueprintOnly"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_find_bone_for_blueprint_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CopyPose"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_copy_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CopyHierarchy"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_copy_hierarchy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Contains_ForBlueprint"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_contains_for_blueprint,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigHierarchyController::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSelection"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_set_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_set_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHierarchySelection"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_hierarchy_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHierarchy"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_set_hierarchy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayName"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_set_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlSettings"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_set_control_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetComponentSelection"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_component_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetComponentContent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_set_component_content,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAvailableSpaceLabel"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_available_space_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAvailableSpaceIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_available_space_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectHierarchyKey"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_select_hierarchy_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectElement"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_select_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectComponent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_select_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReparentComponent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_reparent_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReorderElement"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_reorder_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameElement"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_rename_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameComponent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_rename_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_remove_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveElement"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_remove_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveComponent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_remove_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveChannelHost"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_remove_channel_host,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAvailableSpace"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_available_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllParents"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_remove_all_parents,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MirrorElements"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_mirror_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportSocketsFromSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_sockets_from_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportPreviewSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_preview_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportFromText"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_import_from_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportCurvesFromSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves_from_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportCurvesFromAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves_from_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportCurves"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_import_curves,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportBonesFromSkeletalMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones_from_skeletal_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportBonesFromAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones_from_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportBones"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_import_bones,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHierarchy"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_get_hierarchy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlSettings"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_get_control_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GeneratePythonCommands"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_generate_python_commands,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportToText"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_export_to_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportSelectionToText"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_export_selection_to_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DuplicateElements"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_duplicate_elements,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectHierarchyKey"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_deselect_hierarchy_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectElement"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_deselect_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeselectComponent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_deselect_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearSelection"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_clear_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSocket"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_socket,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddParent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_parent,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddNull"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_null,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddCurve"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_curve,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddControl_ForBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_control_for_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddConnector"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_connector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddComponent"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddChannelHost"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_channel_host,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBone"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_bone,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddAvailableSpace"),
                &raw mut __FUNCTION_PTRS.u_rig_hierarchy_controller_add_available_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddAnimationChannel_ForBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_animation_channel_for_blueprint,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigPoseAsset::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUpMirrorMatchTable"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_pose_asset_set_up_mirror_match_table,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectControls"),
                &raw mut __FUNCTION_PTRS.u_control_rig_pose_asset_select_controls,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SavePose"),
                &raw mut __FUNCTION_PTRS.u_control_rig_pose_asset_save_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceControlName"),
                &raw mut __FUNCTION_PTRS.u_control_rig_pose_asset_replace_control_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PastePose"),
                &raw mut __FUNCTION_PTRS.u_control_rig_pose_asset_paste_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentPose"),
                &raw mut __FUNCTION_PTRS.u_control_rig_pose_asset_get_current_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlNames"),
                &raw mut __FUNCTION_PTRS.u_control_rig_pose_asset_get_control_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoesMirrorMatch"),
                &raw mut __FUNCTION_PTRS.u_control_rig_pose_asset_does_mirror_match,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigWorkflowOptions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnsureAtLeastOneRigElementSelected"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_workflow_options_ensure_at_least_one_rig_element_selected,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigTransformWorkflowOptions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ProvideWorkflows"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_transform_workflow_options_provide_workflows,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FRigElementKey {
    pub ty: ERigElementType,
    pub name: FName,
}
impl FRigElementKey {}
#[repr(C, align(8))]
pub struct FRigBaseElement {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub key: FRigElementKey,
    pub index: i32,
    pub sub_index: i32,
    pub created_at_instruction_index: i32,
    pub spawn_index: i32,
    pub b_selected: bool,
    pub(crate) __padding_end: [u8; 47],
}
impl FRigBaseElement {}
#[repr(C, align(8))]
pub struct FRigTransformElement {
    pub(crate) __padding_end: [u8; 288],
}
impl FRigTransformElement {}
#[repr(C, align(16))]
pub struct FRigMultiParentElement {
    pub(crate) __padding_end: [u8; 560],
}
impl FRigMultiParentElement {}
#[repr(C, align(16))]
pub struct FRigControlElement {
    #[doc(hidden)]
    pub(crate) __padding_560: [u8; 560],
    pub settings: FRigControlSettings,
    pub preferred_euler_angles: FRigPreferredEulerAngles,
    pub(crate) __padding_end: [u8; 264],
}
impl FRigControlElement {}
#[repr(C, align(8))]
pub struct FRigPreferredEulerAngles {
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub current: crate::bindings::core_u_object::FVector,
    pub initial: crate::bindings::core_u_object::FVector,
}
impl FRigPreferredEulerAngles {}
#[repr(C, align(16))]
pub struct FRigControlSettings {
    pub animation_type: ERigControlAnimationType,
    pub control_type: ERigControlType,
    pub display_name: FName,
    pub primary_axis: ERigControlAxis,
    pub limit_enabled: TArray<FRigControlLimitEnabled>,
    pub b_draw_limits: bool,
    pub minimum_value: FRigControlValue,
    pub maximum_value: FRigControlValue,
    pub b_shape_visible: bool,
    pub shape_visibility: ERigControlVisibility,
    pub shape_name: FName,
    pub shape_color: crate::bindings::core_u_object::FLinearColor,
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
    pub customization: FRigControlElementCustomization,
    pub driven_controls: TArray<FRigElementKey>,
    #[doc(hidden)]
    pub(crate) __padding_640: [u8; 16],
    pub b_group_with_parent_control: bool,
    pub b_restrict_space_switching: bool,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub b_use_preferred_rotation_order: bool,
    pub shape_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigControlSettings {}
#[repr(C, align(8))]
pub struct FRigControlElementCustomization {
    pub available_spaces: TArray<FRigElementKeyWithLabel>,
    pub removed_spaces: TArray<FRigElementKey>,
}
impl FRigControlElementCustomization {}
#[repr(C, align(4))]
pub struct FRigElementKeyWithLabel {
    pub key: FRigElementKey,
    pub label: FName,
}
impl FRigElementKeyWithLabel {}
#[repr(C, align(16))]
pub struct FRigControlValue {
    pub(crate) __padding_end: [u8; 240],
}
impl FRigControlValue {}
#[repr(C, align(4))]
pub struct FRigControlValueStorage {
    pub(crate) __padding_end: [u8; 132],
}
impl FRigControlValueStorage {}
#[repr(C, align(1))]
pub struct FRigControlLimitEnabled {
    pub b_minimum: bool,
    pub b_maximum: bool,
}
impl FRigControlLimitEnabled {}
#[repr(C, align(8))]
pub struct FRigModuleSettings {
    pub identifier: FRigModuleIdentifier,
    pub icon: crate::bindings::core_u_object::FSoftObjectPath,
    pub category: FString,
    pub keywords: FString,
    pub description: FString,
    pub exposed_connectors: TArray<FRigModuleConnector>,
}
impl FRigModuleSettings {}
#[repr(C, align(8))]
pub struct FRigModuleConnector {
    pub name: FString,
    pub settings: FRigConnectorSettings,
}
impl FRigModuleConnector {}
#[repr(C, align(8))]
pub struct FRigConnectorSettings {
    pub description: FString,
    pub ty: EConnectorType,
    pub b_optional: bool,
    pub b_is_array: bool,
    pub b_post_construction: bool,
    pub rules: TArray<FRigConnectionRuleStash>,
}
impl FRigConnectorSettings {}
#[repr(C, align(8))]
pub struct FRigConnectionRuleStash {
    pub script_struct_path: FString,
    pub exported_text: FString,
}
impl FRigConnectionRuleStash {}
#[repr(C, align(8))]
pub struct FRigModuleIdentifier {
    pub name: FString,
    pub ty: FString,
}
impl FRigModuleIdentifier {}
#[repr(C, align(8))]
pub struct FRigInfluenceMapPerEvent {
    pub(crate) __padding_end: [u8; 96],
}
impl FRigInfluenceMapPerEvent {}
#[repr(C, align(8))]
pub struct FRigInfluenceMap {
    pub(crate) __padding_end: [u8; 112],
}
impl FRigInfluenceMap {}
#[repr(C, align(8))]
pub struct FRigInfluenceEntry {
    pub(crate) __padding_end: [u8; 32],
}
impl FRigInfluenceEntry {}
#[repr(C, align(8))]
pub struct FRigUnit {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigUnit {}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub debug_draw_settings: crate::bindings::rig_vm::FRigVMDebugDrawSettings,
}
impl FRigUnit_DebugBase {}
#[repr(C, align(8))]
pub struct FRigUnitMutable {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnitMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_DebugBaseMutable {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub debug_draw_settings: crate::bindings::rig_vm::FRigVMDebugDrawSettings,
}
impl FRigUnit_DebugBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigUnit_HighlevelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_HighlevelBaseMutable {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_HighlevelBaseMutable {}
#[repr(C, align(8))]
pub struct FControlRigReference {
    pub(crate) __padding_end: [u8; 16],
}
impl FControlRigReference {}
#[repr(C, align(16))]
pub struct FControlRigComponentMappedElement {
    pub component_reference: crate::bindings::engine::FSoftComponentReference,
    pub transform_index: i32,
    pub transform_name: FName,
    pub element_type: ERigElementType,
    pub element_name: FName,
    pub direction: EControlRigComponentMapDirection,
    pub offset: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub space: EControlRigComponentSpace,
    pub(crate) __padding_end: [u8; 27],
}
impl FControlRigComponentMappedElement {}
#[repr(C, align(8))]
pub struct FControlRigComponentMappedComponent {
    pub component: UPtr<crate::bindings::engine::USceneComponent>,
    pub element_name: FName,
    pub element_type: ERigElementType,
    pub direction: EControlRigComponentMapDirection,
}
impl FControlRigComponentMappedComponent {}
#[repr(C, align(4))]
pub struct FControlRigComponentMappedBone {
    pub source: FName,
    pub target: FName,
}
impl FControlRigComponentMappedBone {}
#[repr(C, align(4))]
pub struct FControlRigComponentMappedCurve {
    pub source: FName,
    pub target: FName,
}
impl FControlRigComponentMappedCurve {}
#[repr(C, align(16))]
pub struct FControlRigShapeDefinition {
    pub shape_name: FName,
    pub static_mesh: TSoftObjectPtr<crate::bindings::engine::UStaticMesh>,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 16],
}
impl FControlRigShapeDefinition {}
#[repr(C, align(8))]
pub struct FControlRigOverrideValue {
    pub(crate) __padding_end: [u8; 88],
}
impl FControlRigOverrideValue {}
#[repr(C, align(8))]
pub struct FControlRigOverrideContainer {
    pub(crate) __padding_end: [u8; 344],
}
impl FControlRigOverrideContainer {}
#[repr(C, align(8))]
pub struct FControlRigReplayVariable {
    pub name: FName,
    pub cpp_type: FName,
    pub value: FString,
}
impl FControlRigReplayVariable {}
#[repr(C, align(8))]
pub struct FControlRigTestDataFrame {
    pub absolute_time: f64,
    pub delta_time: f64,
    pub variables: TArray<FControlRigReplayVariable>,
    pub pose: FRigPose,
    pub metadata: TArray<u8>,
    pub(crate) __padding_end: [u8; 104],
}
impl FControlRigTestDataFrame {}
#[repr(C, align(8))]
pub struct FRigPose {
    pub(crate) __padding_end: [u8; 112],
}
impl FRigPose {}
#[repr(C, align(16))]
pub struct FRigPoseElement {
    pub(crate) __padding_end: [u8; 272],
}
impl FRigPoseElement {}
#[repr(C, align(8))]
pub struct FCachedRigElement {
    pub(crate) __padding_end: [u8; 32],
}
impl FCachedRigElement {}
#[repr(C, align(4))]
pub struct FCRSimLinearSpring {
    pub(crate) __padding_end: [u8; 16],
}
impl FCRSimLinearSpring {}
#[repr(C, align(16))]
pub struct FCRSimSoftCollision {
    pub(crate) __padding_end: [u8; 128],
}
impl FCRSimSoftCollision {}
#[repr(C, align(8))]
pub struct FCRSimPointForce {
    pub(crate) __padding_end: [u8; 40],
}
impl FCRSimPointForce {}
#[repr(C, align(8))]
pub struct FRigModuleInstance {
    pub(crate) __padding_end: [u8; 208],
}
impl FRigModuleInstance {}
#[repr(C, align(8))]
pub struct FRigModuleExecutionElement {
    pub(crate) __padding_end: [u8; 48],
}
impl FRigModuleExecutionElement {}
#[repr(C, align(8))]
pub struct FRigModuleExecutionQueue {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigModuleExecutionQueue {}
#[repr(C, align(8))]
pub struct FRigModuleReference {
    pub(crate) __padding_end: [u8; 752],
}
impl FRigModuleReference {}
#[repr(C, align(8))]
pub struct FModularRigSingleConnection {
    pub(crate) __padding_end: [u8; 48],
}
impl FModularRigSingleConnection {}
#[repr(C, align(8))]
pub struct FModularRigConnections {
    pub(crate) __padding_end: [u8; 96],
}
impl FModularRigConnections {}
#[repr(C, align(8))]
pub struct FModularRigModel {
    pub(crate) __padding_end: [u8; 272],
}
impl FModularRigModel {}
#[repr(C, align(8))]
pub struct FRigHierarchyModulePath {
    pub(crate) __padding_end: [u8; 72],
}
impl FRigHierarchyModulePath {}
#[repr(C, align(8))]
pub struct FModularRigModuleSettingsForClipboard {
    pub module_class: crate::bindings::core_u_object::FSoftObjectPath,
    pub defaults: TMap<FString, FString>,
    pub overrides: TMap<FString, FString>,
    pub bindings: TMap<FName, FString>,
}
impl FModularRigModuleSettingsForClipboard {}
#[repr(C, align(8))]
pub struct FModularRigModuleSettingsSetForClipboard {
    pub settings: TMap<FName, FModularRigModuleSettingsForClipboard>,
}
impl FModularRigModuleSettingsSetForClipboard {}
#[repr(C, align(8))]
pub struct FRigElement {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub index: i32,
}
impl FRigElement {}
#[repr(C, align(16))]
pub struct FRigBone {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub parent_name: FName,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    #[doc(hidden)]
    pub(crate) __padding_352: [u8; 16],
    pub ty: ERigBoneType,
}
impl FRigBone {}
#[repr(C, align(8))]
pub struct FRigAndConnectionRule {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child_rules: TArray<FRigConnectionRuleStash>,
}
impl FRigAndConnectionRule {}
#[repr(C, align(8))]
pub struct FRigOrConnectionRule {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child_rules: TArray<FRigConnectionRuleStash>,
}
impl FRigOrConnectionRule {}
#[repr(C, align(8))]
pub struct FRigTypeConnectionRule {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub element_type: ERigElementType,
}
impl FRigTypeConnectionRule {}
#[repr(C, align(8))]
pub struct FRigTagConnectionRule {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub tag: FName,
}
impl FRigTagConnectionRule {}
#[repr(C, align(8))]
pub struct FRigChildOfPrimaryConnectionRule {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigChildOfPrimaryConnectionRule {}
#[repr(C, align(16))]
pub struct FRigControl {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub control_type: ERigControlType,
    pub display_name: FName,
    pub parent_name: FName,
    pub parent_index: i32,
    pub space_name: FName,
    pub space_index: i32,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub initial_value: FRigControlValue,
    pub value: FRigControlValue,
    pub primary_axis: ERigControlAxis,
    #[doc(hidden)]
    pub(crate) __padding_658: [u8; 1],
    pub b_animatable: bool,
    pub b_limit_translation: bool,
    pub b_limit_rotation: bool,
    pub b_limit_scale: bool,
    pub b_draw_limits: bool,
    pub minimum_value: FRigControlValue,
    pub maximum_value: FRigControlValue,
    pub b_gizmo_enabled: bool,
    pub b_gizmo_visible: bool,
    pub gizmo_name: FName,
    pub gizmo_transform: crate::bindings::core_u_object::FTransform,
    pub gizmo_color: crate::bindings::core_u_object::FLinearColor,
    #[doc(hidden)]
    pub(crate) __padding_1296: [u8; 16],
    pub b_is_transient_control: bool,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
}
impl FRigControl {}
#[repr(C, align(8))]
pub struct FRigCurve {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub value: f32,
}
impl FRigCurve {}
#[repr(C, align(8))]
pub struct FCachedRigComponent {
    pub(crate) __padding_end: [u8; 56],
}
impl FCachedRigComponent {}
#[repr(C, align(8))]
pub struct FRigBaseComponent {
    pub(crate) __padding_end: [u8; 88],
}
impl FRigBaseComponent {}
#[repr(C, align(16))]
pub struct FRigSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub space_type: ERigSpaceType,
    pub parent_name: FName,
    pub parent_index: i32,
    pub initial_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigSpace {}
#[repr(C, align(4))]
pub struct FRigControlModifiedContext {
    pub(crate) __padding_end: [u8; 28],
}
impl FRigControlModifiedContext {}
#[repr(C, align(4))]
pub struct FRigComponentKey {
    pub element_key: FRigElementKey,
    pub name: FName,
}
impl FRigComponentKey {}
#[repr(C, align(4))]
pub struct FRigHierarchyKey {
    pub(crate) __padding_end: [u8; 52],
}
impl FRigHierarchyKey {}
#[repr(C, align(8))]
pub struct FRigElementKeyCollection {
    pub keys: TArray<FRigElementKey>,
}
impl FRigElementKeyCollection {}
#[repr(C, align(8))]
pub struct FRigEventContext {
    pub(crate) __padding_end: [u8; 48],
}
impl FRigEventContext {}
#[repr(C, align(8))]
pub struct FRigElementResolveResult {
    pub(crate) __padding_end: [u8; 40],
}
impl FRigElementResolveResult {}
#[repr(C, align(8))]
pub struct FModularRigResolveResult {
    pub(crate) __padding_end: [u8; 72],
}
impl FModularRigResolveResult {}
#[repr(C, align(8))]
pub struct FRigTransformDirtyState {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigTransformDirtyState {}
#[repr(C, align(8))]
pub struct FRigLocalAndGlobalDirtyState {
    pub global: FRigTransformDirtyState,
    pub local: FRigTransformDirtyState,
}
impl FRigLocalAndGlobalDirtyState {}
#[repr(C, align(8))]
pub struct FRigCurrentAndInitialDirtyState {
    pub current: FRigLocalAndGlobalDirtyState,
    pub initial: FRigLocalAndGlobalDirtyState,
}
impl FRigCurrentAndInitialDirtyState {}
#[repr(C, align(8))]
pub struct FRigComputedTransform {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigComputedTransform {}
#[repr(C, align(8))]
pub struct FRigLocalAndGlobalTransform {
    pub local: FRigComputedTransform,
    pub global: FRigComputedTransform,
}
impl FRigLocalAndGlobalTransform {}
#[repr(C, align(8))]
pub struct FRigCurrentAndInitialTransform {
    pub current: FRigLocalAndGlobalTransform,
    pub initial: FRigLocalAndGlobalTransform,
}
impl FRigCurrentAndInitialTransform {}
#[repr(C, align(8))]
pub struct FRigSingleParentElement {
    pub(crate) __padding_end: [u8; 296],
}
impl FRigSingleParentElement {}
#[repr(C, align(4))]
pub struct FRigElementWeight {
    pub location: f32,
    pub rotation: f32,
    pub scale: f32,
}
impl FRigElementWeight {}
#[repr(C, align(8))]
pub struct FRigBoneElement {
    #[doc(hidden)]
    pub(crate) __padding_296: [u8; 296],
    pub bone_type: ERigBoneType,
}
impl FRigBoneElement {}
#[repr(C, align(16))]
pub struct FRigNullElement {
    pub(crate) __padding_end: [u8; 560],
}
impl FRigNullElement {}
#[repr(C, align(8))]
pub struct FRigCurveElement {
    pub(crate) __padding_end: [u8; 112],
}
impl FRigCurveElement {}
#[repr(C, align(8))]
pub struct FRigReferenceElement {
    pub(crate) __padding_end: [u8; 320],
}
impl FRigReferenceElement {}
#[repr(C, align(8))]
pub struct FRigConnectorState {
    pub name: FName,
    pub resolved_target: FRigElementKey,
    pub settings: FRigConnectorSettings,
}
impl FRigConnectorState {}
#[repr(C, align(8))]
pub struct FRigConnectorElement {
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 96],
    pub settings: FRigConnectorSettings,
}
impl FRigConnectorElement {}
#[repr(C, align(16))]
pub struct FRigSocketState {
    pub name: FName,
    pub parent: FRigElementKey,
    pub initial_local_transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub description: FString,
}
impl FRigSocketState {}
#[repr(C, align(8))]
pub struct FRigSocketElement {
    pub(crate) __padding_end: [u8; 296],
}
impl FRigSocketElement {}
#[repr(C, align(1))]
pub struct FModularRigSettings {
    pub b_auto_resolve: bool,
}
impl FModularRigSettings {}
#[repr(C, align(8))]
pub struct FRigModuleDescription {
    pub path: crate::bindings::core_u_object::FSoftObjectPath,
    pub settings: FRigModuleSettings,
}
impl FRigModuleDescription {}
#[repr(C, align(8))]
pub struct FModuleReferenceData {
    pub(crate) __padding_end: [u8; 56],
}
impl FModuleReferenceData {}
#[repr(C, align(4))]
pub struct FRigPhysicsSolverID {
    pub guid: crate::bindings::core_u_object::FGuid,
}
impl FRigPhysicsSolverID {}
#[repr(C, align(16))]
pub struct FRigControlCopy {
    pub name: FName,
    pub control_type: ERigControlType,
    pub parent_key: FRigElementKey,
    #[doc(hidden)]
    pub(crate) __padding_272: [u8; 240],
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub parent_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigControlCopy {}
#[repr(C, align(8))]
pub struct FControlRigControlPose {
    pub copy_of_controls: TArray<FRigControlCopy>,
    pub(crate) __padding_end: [u8; 80],
}
impl FControlRigControlPose {}
#[repr(C, align(8))]
pub struct FRigDispatchFactory {
    pub(crate) __padding_end: [u8; 168],
}
impl FRigDispatchFactory {}
#[repr(C, align(8))]
pub struct FRigDispatch_AnimAttributeBase {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_AnimAttributeBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetAnimAttribute {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_GetAnimAttribute {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetAnimAttribute {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_SetAnimAttribute {}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceWorld {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub channel: crate::bindings::engine::ECollisionChannel,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 56],
}
impl FRigUnit_SphereTraceWorld {}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByTraceChannel {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub trace_channel: crate::bindings::engine::ETraceTypeQuery,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 56],
}
impl FRigUnit_SphereTraceByTraceChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByTraceChannel {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub trace_channel: crate::bindings::engine::ETraceTypeQuery,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 56],
}
impl FRigUnit_LineTraceByTraceChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SphereTraceByObjectTypes {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub object_types: TArray<crate::bindings::engine::EObjectTypeQuery>,
    pub radius: f32,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 56],
}
impl FRigUnit_SphereTraceByObjectTypes {}
#[repr(C, align(8))]
pub struct FRigUnit_LineTraceByObjectTypes {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start: crate::bindings::core_u_object::FVector,
    pub end: crate::bindings::core_u_object::FVector,
    pub object_types: TArray<crate::bindings::engine::EObjectTypeQuery>,
    pub b_hit: bool,
    pub hit_location: crate::bindings::core_u_object::FVector,
    pub hit_normal: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 56],
}
impl FRigUnit_LineTraceByObjectTypes {}
#[repr(C, align(16))]
pub struct FRigUnit_Control {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub transform: crate::bindings::animation_core::FEulerTransform,
    pub base: crate::bindings::core_u_object::FTransform,
    pub init_transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub filter: crate::bindings::animation_core::FTransformFilter,
}
impl FRigUnit_Control {}
#[repr(C, align(16))]
pub struct FRigUnit_Control_StaticMesh {
    #[doc(hidden)]
    pub(crate) __padding_384: [u8; 384],
    pub mesh_transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_Control_StaticMesh {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetUserData {
    pub(crate) __padding_end: [u8; 168],
}
impl FRigDispatch_GetUserData {}
#[repr(C, align(8))]
pub struct FRigUnit_SetupShapeLibraryFromUserData {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub name_space: FString,
    pub path: FString,
    pub library_name: FString,
    pub log_shape_libraries: bool,
}
impl FRigUnit_SetupShapeLibraryFromUserData {}
#[repr(C, align(8))]
pub struct FRigUnit_ShapeExists {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub shape_name: FName,
    pub result: bool,
}
impl FRigUnit_ShapeExists {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugBezier {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugBezier {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugBezierItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub minimum_u: f32,
    pub maximum_u: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub detail: i32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugBezierItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugHierarchy {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugHierarchy {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugPose {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
    pub pose: FRigPose,
    pub items: TArray<FRigElementKey>,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugPose {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLine {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugLine {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub a: crate::bindings::core_u_object::FVector,
    pub b: crate::bindings::core_u_object::FVector,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugLineItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStrip {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugLineStrip {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugLineStripItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub points: TArray<crate::bindings::core_u_object::FVector>,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugLineStripItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangle {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugRectangle {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugRectangleItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub scale: f32,
    pub thickness: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugRectangleItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArc {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugArc {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugArcItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub radius: f32,
    pub minimum_degrees: f32,
    pub maximum_degrees: f32,
    pub thickness: f32,
    pub detail: i32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugArcItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutable {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugTransformMutable {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformMutableItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugTransformMutableItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutable {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FName,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub(crate) __padding_end: [u8; 31],
}
impl FRigUnit_DebugTransformArrayMutable {}
#[repr(C, align(16))]
pub struct FRigUnit_DebugTransformArrayMutableItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub parent_indices: TArray<i32>,
    pub mode: crate::bindings::rig_vm::ERigUnitDebugTransformMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
    pub world_offset: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
}
impl FRigUnit_DebugTransformArrayMutableItemSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_StartProfilingTimer {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigUnit_StartProfilingTimer {}
#[repr(C, align(8))]
pub struct FRigUnit_EndProfilingTimer {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub number_of_measurements: i32,
    pub prefix: FString,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_EndProfilingTimer {}
#[repr(C, align(8))]
pub struct FRigUnit_VisualDebugVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: crate::bindings::rig_vm::ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
impl FRigUnit_VisualDebugVector {}
#[repr(C, align(8))]
pub struct FRigUnit_VisualDebugVectorItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FVector,
    pub b_enabled: bool,
    pub mode: crate::bindings::rig_vm::ERigUnitVisualDebugPointMode,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
impl FRigUnit_VisualDebugVectorItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
impl FRigUnit_VisualDebugQuat {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugQuatItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
impl FRigUnit_VisualDebugQuatItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub bone_space: FName,
}
impl FRigUnit_VisualDebugTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_VisualDebugTransformItemSpace {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub b_enabled: bool,
    pub thickness: f32,
    pub scale: f32,
    pub space: FRigElementKey,
}
impl FRigUnit_VisualDebugTransformItemSpace {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::animation_core::FEulerTransform,
}
impl FRigUnit_ConvertTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertEulerTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub input: crate::bindings::animation_core::FEulerTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ConvertEulerTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertRotation {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ConvertRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorRotation {
    pub(crate) __padding_end: [u8; 64],
}
impl FRigUnit_ConvertVectorRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternion {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_ConvertQuaternion {}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertVectorToRotation {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_ConvertVectorToRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertVectorToQuaternion {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ConvertVectorToQuaternion {}
#[repr(C, align(8))]
pub struct FRigUnit_ConvertRotationToVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub input: crate::bindings::core_u_object::FRotator,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_ConvertRotationToVector {}
#[repr(C, align(16))]
pub struct FRigUnit_ConvertQuaternionToVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_ConvertQuaternionToVector {}
#[repr(C, align(16))]
pub struct FRigUnit_ToSwingAndTwist {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FQuat,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub swing: crate::bindings::core_u_object::FQuat,
    pub twist: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ToSwingAndTwist {}
#[repr(C, align(8))]
pub struct FRigUnit_BinaryFloatOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub argument0: f32,
    pub argument1: f32,
    pub result: f32,
}
impl FRigUnit_BinaryFloatOp {}
#[repr(C, align(8))]
pub struct FRigUnit_Multiply_FloatFloat {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigUnit_Multiply_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Add_FloatFloat {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigUnit_Add_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Subtract_FloatFloat {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigUnit_Subtract_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Divide_FloatFloat {
    pub(crate) __padding_end: [u8; 24],
}
impl FRigUnit_Divide_FloatFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_Clamp_Float {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub result: f32,
}
impl FRigUnit_Clamp_Float {}
#[repr(C, align(8))]
pub struct FRigUnit_MapRange_Float {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: f32,
    pub min_in: f32,
    pub max_in: f32,
    pub min_out: f32,
    pub max_out: f32,
    pub result: f32,
}
impl FRigUnit_MapRange_Float {}
#[repr(C, align(16))]
pub struct FRigUnit_BinaryQuaternionOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub argument0: crate::bindings::core_u_object::FQuat,
    pub argument1: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_BinaryQuaternionOp {}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyQuaternion {
    pub(crate) __padding_end: [u8; 112],
}
impl FRigUnit_MultiplyQuaternion {}
#[repr(C, align(16))]
pub struct FRigUnit_UnaryQuaternionOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub argument: crate::bindings::core_u_object::FQuat,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_UnaryQuaternionOp {}
#[repr(C, align(16))]
pub struct FRigUnit_InverseQuaterion {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_InverseQuaterion {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAxisAndAngle {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub argument: crate::bindings::core_u_object::FQuat,
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
}
impl FRigUnit_QuaternionToAxisAndAngle {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionFromAxisAndAngle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub axis: crate::bindings::core_u_object::FVector,
    pub angle: f32,
    pub result: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_QuaternionFromAxisAndAngle {}
#[repr(C, align(16))]
pub struct FRigUnit_QuaternionToAngle {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub axis: crate::bindings::core_u_object::FVector,
    pub argument: crate::bindings::core_u_object::FQuat,
    pub angle: f32,
}
impl FRigUnit_QuaternionToAngle {}
#[repr(C, align(16))]
pub struct FRigUnit_BinaryTransformOp {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub argument0: crate::bindings::core_u_object::FTransform,
    pub argument1: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_BinaryTransformOp {}
#[repr(C, align(16))]
pub struct FRigUnit_MultiplyTransform {
    pub(crate) __padding_end: [u8; 304],
}
impl FRigUnit_MultiplyTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransform {
    pub(crate) __padding_end: [u8; 304],
}
impl FRigUnit_GetRelativeTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_BinaryVectorOp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub argument0: crate::bindings::core_u_object::FVector,
    pub argument1: crate::bindings::core_u_object::FVector,
    pub result: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_BinaryVectorOp {}
#[repr(C, align(8))]
pub struct FRigUnit_Multiply_VectorVector {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_Multiply_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Add_VectorVector {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_Add_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Subtract_VectorVector {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_Subtract_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Divide_VectorVector {
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_Divide_VectorVector {}
#[repr(C, align(8))]
pub struct FRigUnit_Distance_VectorVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub argument0: crate::bindings::core_u_object::FVector,
    pub argument1: crate::bindings::core_u_object::FVector,
    pub result: f32,
}
impl FRigUnit_Distance_VectorVector {}
#[repr(C, align(16))]
pub struct FAimTarget {
    pub(crate) __padding_end: [u8; 144],
}
impl FAimTarget {}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub joint: FName,
    pub aim_mode: EAimMode,
    pub up_mode: EAimMode,
    pub aim_vector: crate::bindings::core_u_object::FVector,
    pub up_vector: crate::bindings::core_u_object::FVector,
    pub aim_targets: TArray<FAimTarget>,
    pub up_targets: TArray<FAimTarget>,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_AimConstraint {}
#[repr(C, align(16))]
pub struct FRigUnit_ApplyFK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub joint: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub filter: crate::bindings::animation_core::FTransformFilter,
    pub apply_transform_mode: EApplyTransformMode,
    pub apply_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_joint: FName,
}
impl FRigUnit_ApplyFK {}
#[repr(C, align(16))]
pub struct FBlendTarget {
    pub(crate) __padding_end: [u8; 112],
}
impl FBlendTarget {}
#[repr(C, align(16))]
pub struct FRigUnit_BlendTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub source: crate::bindings::core_u_object::FTransform,
    pub targets: TArray<FBlendTarget>,
    pub result: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_BlendTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetJointTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub joint: FName,
    pub ty: ETransformGetterType,
    pub transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_joint: FName,
    pub output: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_GetJointTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKFK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_joint: FName,
    pub end_joint: FName,
    pub pole_target: crate::bindings::core_u_object::FVector,
    pub spin: f32,
    pub end_effector: crate::bindings::core_u_object::FTransform,
    pub ik_blend: f32,
    pub(crate) __padding_end: [u8; 636],
}
impl FRigUnit_TwoBoneIKFK {}
#[repr(C, align(16))]
pub struct FRigUnit_DrawContainerGetInstruction {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub instruction_name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_DrawContainerGetInstruction {}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetColor {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub instruction_name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
}
impl FRigUnit_DrawContainerSetColor {}
#[repr(C, align(8))]
pub struct FRigUnit_DrawContainerSetThickness {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub instruction_name: FName,
    pub thickness: f32,
}
impl FRigUnit_DrawContainerSetThickness {}
#[repr(C, align(16))]
pub struct FRigUnit_DrawContainerSetTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub instruction_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_DrawContainerSetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_BeginExecution {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_BeginExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_PreBeginExecution {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PreBeginExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_PostBeginExecution {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PostBeginExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigUnit_CollectionBase {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionBaseMutable {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_CollectionBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChain {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub first_item: FRigElementKey,
    pub last_item: FRigElementKey,
    pub reverse: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionChain {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChainArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub first_item: FRigElementKey,
    pub last_item: FRigElementKey,
    pub reverse: bool,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionChainArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionNameSearch {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub partial_name: FName,
    pub type_to_search: ERigElementType,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionNameSearch {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionNameSearchArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub partial_name: FName,
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionNameSearchArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChildren {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub type_to_search: ERigElementType,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionChildren {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionChildrenArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub b_default_children: bool,
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionChildrenArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetAll {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionGetAll {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReplaceItems {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: FRigElementKeyCollection,
    pub old: FName,
    pub new: FName,
    pub remove_invalid_items: bool,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionReplaceItems {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReplaceItemsArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub old: FName,
    pub new: FName,
    pub remove_invalid_items: bool,
    pub b_allow_duplicates: bool,
    pub result: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionReplaceItemsArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionItems {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionItems {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetItems {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_CollectionGetItems {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetParentIndices {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub parent_indices: TArray<i32>,
}
impl FRigUnit_CollectionGetParentIndices {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionGetParentIndicesItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub parent_indices: TArray<i32>,
}
impl FRigUnit_CollectionGetParentIndicesItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionUnion {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub b_allow_duplicates: bool,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionUnion {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionIntersection {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionIntersection {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionDifference {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FRigElementKeyCollection,
    pub b: FRigElementKeyCollection,
    pub collection: FRigElementKeyCollection,
}
impl FRigUnit_CollectionDifference {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionReverse {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub reversed: FRigElementKeyCollection,
}
impl FRigUnit_CollectionReverse {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionCount {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub count: i32,
}
impl FRigUnit_CollectionCount {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionItemAtIndex {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub index: i32,
    pub item: FRigElementKey,
}
impl FRigUnit_CollectionItemAtIndex {}
#[repr(C, align(16))]
pub struct FRigUnit_CollectionLoop {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub collection: FRigElementKeyCollection,
    pub item: FRigElementKey,
    pub index: i32,
    pub count: i32,
    pub ratio: f32,
    pub completed: FControlRigExecuteContext,
}
impl FRigUnit_CollectionLoop {}
#[repr(C, align(16))]
pub struct FControlRigExecuteContext {
    pub(crate) __padding_end: [u8; 608],
}
impl FControlRigExecuteContext {}
#[repr(C, align(8))]
pub struct FRigUnit_CollectionAddItem {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub collection: FRigElementKeyCollection,
    pub item: FRigElementKey,
    pub result: FRigElementKeyCollection,
}
impl FRigUnit_CollectionAddItem {}
#[repr(C, align(8))]
pub struct FRigUnit_DynamicHierarchyBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigUnit_DynamicHierarchyBase {}
#[repr(C, align(8))]
pub struct FRigUnit_DynamicHierarchyBaseMutable {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_DynamicHierarchyBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_AddParent {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub display_label: FName,
}
impl FRigUnit_AddParent {}
#[repr(C, align(8))]
pub struct FRigUnit_SetDefaultParent {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
}
impl FRigUnit_SetDefaultParent {}
#[repr(C, align(8))]
pub struct FRigUnit_AddAvailableSpaces {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FRigElementKey,
    pub spaces: TArray<FRigElementKeyWithLabel>,
}
impl FRigUnit_AddAvailableSpaces {}
#[repr(C, align(8))]
pub struct FRigUnit_SetChannelHosts {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub channel: FRigElementKey,
    pub hosts: TArray<FRigElementKey>,
}
impl FRigUnit_SetChannelHosts {}
#[repr(C, align(8))]
pub struct FRigUnit_SwitchParent {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub mode: ERigSwitchParentMode,
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_maintain_global: bool,
}
impl FRigUnit_SwitchParent {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentWeights {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
    pub parents: FRigElementKeyCollection,
}
impl FRigUnit_HierarchyGetParentWeights {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentWeightsArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
    pub parents: TArray<FRigElementKey>,
}
impl FRigUnit_HierarchyGetParentWeightsArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetParentWeights {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub weights: TArray<FRigElementWeight>,
}
impl FRigUnit_HierarchySetParentWeights {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyReset {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_HierarchyReset {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyImportFromSkeleton {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub name_space: FName,
    pub b_include_curves: bool,
    pub b_include_mesh_sockets: bool,
    pub b_include_virtual_bones: bool,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_HierarchyImportFromSkeleton {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyRemoveElement {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub b_success: bool,
}
impl FRigUnit_HierarchyRemoveElement {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddElement {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub parent: FRigElementKey,
    pub name: FName,
    pub item: FRigElementKey,
}
impl FRigUnit_HierarchyAddElement {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddBone {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
}
impl FRigUnit_HierarchyAddBone {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddNull {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
}
impl FRigUnit_HierarchyAddNull {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControl_Settings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub display_name: FName,
    pub b_selectable: bool,
}
impl FRigUnit_HierarchyAddControl_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControl_ShapeSettings {
    pub b_visible: bool,
    pub name: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_HierarchyAddControl_ShapeSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControl_ProxySettings {
    pub b_is_proxy: bool,
    pub driven_controls: TArray<FRigElementKey>,
    pub shape_visibility: ERigControlVisibility,
}
impl FRigUnit_HierarchyAddControl_ProxySettings {}
#[repr(C, align(4))]
pub struct FRigUnit_HierarchyAddControlFloat_LimitSettings {
    pub limit: FRigControlLimitEnabled,
    pub min_value: f32,
    pub max_value: f32,
    pub b_draw_limits: bool,
}
impl FRigUnit_HierarchyAddControlFloat_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlFloat_Settings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub primary_axis: ERigControlAxis,
    pub b_is_scale: bool,
    pub limits: FRigUnit_HierarchyAddControlFloat_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
}
impl FRigUnit_HierarchyAddControlFloat_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlElement {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub offset_space: crate::bindings::rig_vm::ERigVMTransformSpace,
}
impl FRigUnit_HierarchyAddControlElement {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlFloat {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub initial_value: f32,
    pub settings: FRigUnit_HierarchyAddControlFloat_Settings,
}
impl FRigUnit_HierarchyAddControlFloat {}
#[repr(C, align(4))]
pub struct FRigUnit_HierarchyAddControlInteger_LimitSettings {
    pub limit: FRigControlLimitEnabled,
    pub min_value: i32,
    pub max_value: i32,
    pub b_draw_limits: bool,
}
impl FRigUnit_HierarchyAddControlInteger_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlInteger_Settings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub primary_axis: ERigControlAxis,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
    pub limits: FRigUnit_HierarchyAddControlInteger_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
}
impl FRigUnit_HierarchyAddControlInteger_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlInteger {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub initial_value: i32,
    pub settings: FRigUnit_HierarchyAddControlInteger_Settings,
}
impl FRigUnit_HierarchyAddControlInteger {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlVector2D_LimitSettings {
    pub limit_x: FRigControlLimitEnabled,
    pub limit_y: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FVector2D,
    pub max_value: crate::bindings::core_u_object::FVector2D,
    pub b_draw_limits: bool,
}
impl FRigUnit_HierarchyAddControlVector2D_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector2D_Settings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub primary_axis: ERigControlAxis,
    pub limits: FRigUnit_HierarchyAddControlVector2D_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
impl FRigUnit_HierarchyAddControlVector2D_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector2D {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub initial_value: crate::bindings::core_u_object::FVector2D,
    pub settings: FRigUnit_HierarchyAddControlVector2D_Settings,
}
impl FRigUnit_HierarchyAddControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlVector_LimitSettings {
    pub limit_x: FRigControlLimitEnabled,
    pub limit_y: FRigControlLimitEnabled,
    pub limit_z: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FVector,
    pub max_value: crate::bindings::core_u_object::FVector,
    pub b_draw_limits: bool,
}
impl FRigUnit_HierarchyAddControlVector_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector_Settings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_is_position: bool,
    pub limits: FRigUnit_HierarchyAddControlVector_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
impl FRigUnit_HierarchyAddControlVector_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlVector {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub settings: FRigUnit_HierarchyAddControlVector_Settings,
}
impl FRigUnit_HierarchyAddControlVector {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlRotator_LimitSettings {
    pub limit_pitch: FRigControlLimitEnabled,
    pub limit_yaw: FRigControlLimitEnabled,
    pub limit_roll: FRigControlLimitEnabled,
    pub min_value: crate::bindings::core_u_object::FRotator,
    pub max_value: crate::bindings::core_u_object::FRotator,
    pub b_draw_limits: bool,
}
impl FRigUnit_HierarchyAddControlRotator_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator_Settings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub limits: FRigUnit_HierarchyAddControlRotator_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
}
impl FRigUnit_HierarchyAddControlRotator_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlRotator {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub initial_value: crate::bindings::core_u_object::FRotator,
    pub settings: FRigUnit_HierarchyAddControlRotator_Settings,
}
impl FRigUnit_HierarchyAddControlRotator {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddControlTransform_LimitSettings {
    pub limit_translation_x: FRigControlLimitEnabled,
    pub limit_translation_y: FRigControlLimitEnabled,
    pub limit_translation_z: FRigControlLimitEnabled,
    pub limit_pitch: FRigControlLimitEnabled,
    pub limit_yaw: FRigControlLimitEnabled,
    pub limit_roll: FRigControlLimitEnabled,
    pub limit_scale_x: FRigControlLimitEnabled,
    pub limit_scale_y: FRigControlLimitEnabled,
    pub limit_scale_z: FRigControlLimitEnabled,
    pub min_value: crate::bindings::animation_core::FEulerTransform,
    pub max_value: crate::bindings::animation_core::FEulerTransform,
    pub b_draw_limits: bool,
}
impl FRigUnit_HierarchyAddControlTransform_LimitSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform_Settings {
    #[doc(hidden)]
    pub(crate) __padding_24: [u8; 24],
    pub initial_space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_use_preferred_rotation_order: bool,
    pub preferred_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub limits: FRigUnit_HierarchyAddControlTransform_LimitSettings,
    pub shape: FRigUnit_HierarchyAddControl_ShapeSettings,
    pub proxy: FRigUnit_HierarchyAddControl_ProxySettings,
    pub filtered_channels: TArray<ERigControlTransformChannel>,
}
impl FRigUnit_HierarchyAddControlTransform_Settings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddControlTransform {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub initial_value: crate::bindings::core_u_object::FTransform,
    pub settings: FRigUnit_HierarchyAddControlTransform_Settings,
}
impl FRigUnit_HierarchyAddControlTransform {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings {
    pub(crate) __padding_end: [u8; 1],
}
impl FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelBool {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: bool,
    pub minimum_value: bool,
    pub maximum_value: bool,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelEmptyLimitSettings,
}
impl FRigUnit_HierarchyAddAnimationChannelBool {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings {
    pub enabled: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelFloat {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: f32,
    pub minimum_value: f32,
    pub maximum_value: f32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
}
impl FRigUnit_HierarchyAddAnimationChannelFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelScaleFloat {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: f32,
    pub minimum_value: f32,
    pub maximum_value: f32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
}
impl FRigUnit_HierarchyAddAnimationChannelScaleFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelInteger {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: i32,
    pub minimum_value: i32,
    pub maximum_value: i32,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelSingleLimitSettings,
    pub control_enum: UPtr<crate::bindings::core_u_object::UEnum>,
}
impl FRigUnit_HierarchyAddAnimationChannelInteger {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannel2DLimitSettings {
    pub x: FRigControlLimitEnabled,
    pub y: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannel2DLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelVector2D {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FVector2D,
    pub minimum_value: crate::bindings::core_u_object::FVector2D,
    pub maximum_value: crate::bindings::core_u_object::FVector2D,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannel2DLimitSettings,
}
impl FRigUnit_HierarchyAddAnimationChannelVector2D {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings {
    pub x: FRigControlLimitEnabled,
    pub y: FRigControlLimitEnabled,
    pub z: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelVector {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub minimum_value: crate::bindings::core_u_object::FVector,
    pub maximum_value: crate::bindings::core_u_object::FVector,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings,
}
impl FRigUnit_HierarchyAddAnimationChannelVector {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelScaleVector {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FVector,
    pub minimum_value: crate::bindings::core_u_object::FVector,
    pub maximum_value: crate::bindings::core_u_object::FVector,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelVectorLimitSettings,
}
impl FRigUnit_HierarchyAddAnimationChannelScaleVector {}
#[repr(C, align(1))]
pub struct FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings {
    pub pitch: FRigControlLimitEnabled,
    pub yaw: FRigControlLimitEnabled,
    pub roll: FRigControlLimitEnabled,
}
impl FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddAnimationChannelRotator {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub initial_value: crate::bindings::core_u_object::FRotator,
    pub minimum_value: crate::bindings::core_u_object::FRotator,
    pub maximum_value: crate::bindings::core_u_object::FRotator,
    pub limits_enabled: FRigUnit_HierarchyAddAnimationChannelRotatorLimitSettings,
}
impl FRigUnit_HierarchyAddAnimationChannelRotator {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyGetShapeSettings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub settings: FRigUnit_HierarchyAddControl_ShapeSettings,
}
impl FRigUnit_HierarchyGetShapeSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchySetShapeSettings {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub settings: FRigUnit_HierarchyAddControl_ShapeSettings,
}
impl FRigUnit_HierarchySetShapeSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddSocket {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub description: FString,
}
impl FRigUnit_HierarchyAddSocket {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigUnit_HierarchyBase {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyBaseMutable {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_HierarchyBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParent {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_default_parent: bool,
    pub parent: FRigElementKey,
    pub(crate) __padding_end: [u8; 68],
}
impl FRigUnit_HierarchyGetParent {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParents {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_include_child: bool,
    pub b_reverse: bool,
    pub parents: FRigElementKeyCollection,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetParents {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetParentsItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_include_child: bool,
    pub b_reverse: bool,
    pub b_default_parent: bool,
    pub parents: TArray<FRigElementKey>,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetParentsItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetChildren {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub parent: FRigElementKey,
    pub b_include_parent: bool,
    pub b_recursive: bool,
    pub children: FRigElementKeyCollection,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetChildren {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetSiblings {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_include_item: bool,
    pub siblings: FRigElementKeyCollection,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetSiblings {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetSiblingsItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_include_item: bool,
    pub b_default_siblings: bool,
    pub siblings: TArray<FRigElementKey>,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_HierarchyGetSiblingsItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetChainItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub start: FRigElementKey,
    pub end: FRigElementKey,
    pub b_include_start: bool,
    pub b_include_end: bool,
    pub b_reverse: bool,
    pub chain: TArray<FRigElementKey>,
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_HierarchyGetChainItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetPose {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub initial: bool,
    pub element_type: ERigElementType,
    pub items_to_get: FRigElementKeyCollection,
    pub pose: FRigPose,
}
impl FRigUnit_HierarchyGetPose {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyGetPoseItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub initial: bool,
    pub element_type: ERigElementType,
    pub items_to_get: TArray<FRigElementKey>,
    pub pose: FRigPose,
}
impl FRigUnit_HierarchyGetPoseItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetPose {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_set: FRigElementKeyCollection,
    pub weight: f32,
}
impl FRigUnit_HierarchySetPose {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchySetPoseItemArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_set: TArray<FRigElementKey>,
    pub weight: f32,
}
impl FRigUnit_HierarchySetPoseItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseIsEmpty {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub is_empty: bool,
}
impl FRigUnit_PoseIsEmpty {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetItems {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub items: FRigElementKeyCollection,
}
impl FRigUnit_PoseGetItems {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetItemsItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub element_type: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_PoseGetItemsItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetDelta {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub pose_a: FRigPose,
    pub pose_b: FRigPose,
    pub position_threshold: f32,
    pub rotation_threshold: f32,
    pub scale_threshold: f32,
    pub curve_threshold: f32,
    pub element_type: ERigElementType,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub items_to_compare: FRigElementKeyCollection,
    pub poses_are_equal: bool,
    pub items_with_delta: FRigElementKeyCollection,
}
impl FRigUnit_PoseGetDelta {}
#[repr(C, align(16))]
pub struct FRigUnit_PoseGetTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub valid: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub curve_value: f32,
}
impl FRigUnit_PoseGetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetTransformArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub valid: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
}
impl FRigUnit_PoseGetTransformArray {}
#[repr(C, align(8))]
pub struct FRigUnit_PoseGetCurve {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub pose: FRigPose,
    pub curve: FName,
    pub valid: bool,
    pub curve_value: f32,
    pub(crate) __padding_end: [u8; 12],
}
impl FRigUnit_PoseGetCurve {}
#[repr(C, align(16))]
pub struct FRigUnit_PoseLoop {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub pose: FRigPose,
    pub item: FRigElementKey,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub curve_value: f32,
    pub index: i32,
    pub count: i32,
    pub ratio: f32,
    pub completed: FControlRigExecuteContext,
}
impl FRigUnit_PoseLoop {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyCreatePoseItemArray_Entry {
    pub item: FRigElementKey,
    pub local_transform: crate::bindings::core_u_object::FTransform,
    pub global_transform: crate::bindings::core_u_object::FTransform,
    pub use_euler_angles: bool,
    pub euler_angles: crate::bindings::core_u_object::FVector,
    pub curve_value: f32,
}
impl FRigUnit_HierarchyCreatePoseItemArray_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyCreatePoseItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub entries: TArray<FRigUnit_HierarchyCreatePoseItemArray_Entry>,
    pub pose: FRigPose,
}
impl FRigUnit_HierarchyCreatePoseItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_InteractionExecution {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_InteractionExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_InverseExecution {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_InverseExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_IsInteracting {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub b_is_interacting: bool,
    pub b_is_translating: bool,
    pub b_is_rotating: bool,
    pub b_is_scaling: bool,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_IsInteracting {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigUnit_ItemBase {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemBaseMutable {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_ItemBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemExists {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub exists: bool,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_ItemExists {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemReplace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub old: FName,
    pub new: FName,
    pub result: FRigElementKey,
}
impl FRigUnit_ItemReplace {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
impl FRigUnit_ItemEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
impl FRigUnit_ItemNotEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemTypeEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
impl FRigUnit_ItemTypeEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemTypeNotEquals {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub a: FRigElementKey,
    pub b: FRigElementKey,
    pub result: bool,
}
impl FRigUnit_ItemTypeNotEquals {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemToName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: FRigElementKey,
    pub result: FName,
}
impl FRigUnit_ItemToName {}
#[repr(C, align(8))]
pub struct FRigUnit_HierarchyAddPhysicsSolver {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub name: FName,
    pub solver: FRigPhysicsSolverID,
}
impl FRigUnit_HierarchyAddPhysicsSolver {}
#[repr(C, align(16))]
pub struct FRigUnit_HierarchyAddPhysicsJoint {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub transform: crate::bindings::core_u_object::FTransform,
    pub solver: FRigPhysicsSolverID,
}
impl FRigUnit_HierarchyAddPhysicsJoint {}
#[repr(C, align(8))]
pub struct FRigUnit_PrepareForExecution {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PrepareForExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_PostPrepareForExecution {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_PostPrepareForExecution {}
#[repr(C, align(8))]
pub struct FRigUnit_RigModulesBase {
    pub(crate) __padding_end: [u8; 8],
}
impl FRigUnit_RigModulesBase {}
#[repr(C, align(8))]
pub struct FRigUnit_RigModulesBaseMutable {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_RigModulesBaseMutable {}
#[repr(C, align(8))]
pub struct FRigUnit_ResolveConnector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub connector: FRigElementKey,
    pub skip_socket: bool,
    pub result: FRigElementKey,
    pub b_is_connected: bool,
}
impl FRigUnit_ResolveConnector {}
#[repr(C, align(8))]
pub struct FRigUnit_ResolveArrayConnector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub connector: FRigElementKey,
    pub skip_socket: bool,
    pub result: TArray<FRigElementKey>,
    pub b_is_connected: bool,
}
impl FRigUnit_ResolveArrayConnector {}
#[repr(C, align(8))]
pub struct FRigUnit_GetCurrentNameSpace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name_space: FString,
}
impl FRigUnit_GetCurrentNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemShortName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub short_name: FName,
}
impl FRigUnit_GetItemShortName {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemNameSpace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub has_name_space: bool,
    pub name_space: FString,
}
impl FRigUnit_GetItemNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_IsItemInCurrentNameSpace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub result: bool,
}
impl FRigUnit_IsItemInCurrentNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemsInNameSpace {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_GetItemsInNameSpace {}
#[repr(C, align(8))]
pub struct FRigUnit_GetModuleName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub module: FString,
}
impl FRigUnit_GetModuleName {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemModuleName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub is_part_of_module: bool,
    pub module: FString,
}
impl FRigUnit_GetItemModuleName {}
#[repr(C, align(8))]
pub struct FRigUnit_IsItemInCurrentModule {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub result: bool,
}
impl FRigUnit_IsItemInCurrentModule {}
#[repr(C, align(8))]
pub struct FRigUnit_GetItemsInModule {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub type_to_search: ERigElementType,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_GetItemsInModule {}
#[repr(C, align(16))]
pub struct FRigUnit_SequenceExecution {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub execute_context: FControlRigExecuteContext,
    pub a: FControlRigExecuteContext,
    pub b: FControlRigExecuteContext,
    pub c: FControlRigExecuteContext,
    pub d: FControlRigExecuteContext,
}
impl FRigUnit_SequenceExecution {}
#[repr(C, align(16))]
pub struct FRigUnit_AddBoneTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_post_multiply: bool,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 42],
}
impl FRigUnit_AddBoneTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_Item {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
}
impl FRigUnit_Item {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_ItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_BoneName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bone: FName,
}
impl FRigUnit_BoneName {}
#[repr(C, align(8))]
pub struct FRigUnit_SpaceName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub space: FName,
}
impl FRigUnit_SpaceName {}
#[repr(C, align(8))]
pub struct FRigUnit_ControlName {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
}
impl FRigUnit_ControlName {}
#[repr(C, align(8))]
pub struct FRigDispatch_ComponentBase {
    pub(crate) __padding_end: [u8; 208],
}
impl FRigDispatch_ComponentBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_SpawnComponent {
    pub(crate) __padding_end: [u8; 208],
}
impl FRigDispatch_SpawnComponent {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetComponentContent {
    pub(crate) __padding_end: [u8; 208],
}
impl FRigDispatch_GetComponentContent {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetComponentContent {
    pub(crate) __padding_end: [u8; 208],
}
impl FRigDispatch_SetComponentContent {}
#[repr(C, align(8))]
pub struct FRigUnit_GetAnimationChannelBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub channel: FName,
    pub b_initial: bool,
    pub(crate) __padding_end: [u8; 23],
}
impl FRigUnit_GetAnimationChannelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_GetBoolAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub value: bool,
}
impl FRigUnit_GetBoolAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetFloatAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub value: f32,
}
impl FRigUnit_GetFloatAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetIntAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub value: i32,
}
impl FRigUnit_GetIntAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVector2DAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_GetVector2DAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_GetVectorAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_GetRotatorAnimationChannel {}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_GetTransformAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBase {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_SetAnimationChannelBase {}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoolAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: bool,
}
impl FRigUnit_SetBoolAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetFloatAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: f32,
}
impl FRigUnit_SetFloatAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetIntAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: i32,
}
impl FRigUnit_SetIntAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVector2DAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_SetVector2DAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_SetVectorAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_SetRotatorAnimationChannel {}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannel {
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 64],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_SetTransformAnimationChannel {}
#[repr(C, align(8))]
pub struct FRigUnit_GetAnimationChannelFromItemBase {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_initial: bool,
}
impl FRigUnit_GetAnimationChannelFromItemBase {}
#[repr(C, align(8))]
pub struct FRigUnit_GetBoolAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub value: bool,
}
impl FRigUnit_GetBoolAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetFloatAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub value: f32,
}
impl FRigUnit_GetFloatAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetIntAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub value: i32,
}
impl FRigUnit_GetIntAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVector2DAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_GetVector2DAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetVectorAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_GetVectorAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_GetRotatorAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_GetRotatorAnimationChannelFromItem {}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransformAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_GetTransformAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetAnimationChannelBaseFromItem {
    #[doc(hidden)]
    pub(crate) __padding_32: [u8; 32],
    pub execute_pin: crate::bindings::rig_vm::FRigVMExecutePin,
}
impl FRigUnit_SetAnimationChannelBaseFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoolAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub value: bool,
}
impl FRigUnit_SetBoolAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetFloatAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub value: f32,
}
impl FRigUnit_SetFloatAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetIntAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub value: i32,
}
impl FRigUnit_SetIntAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVector2DAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub value: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_SetVector2DAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetVectorAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub value: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_SetVectorAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetRotatorAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 40],
    pub value: crate::bindings::core_u_object::FRotator,
}
impl FRigUnit_SetRotatorAnimationChannelFromItem {}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransformAnimationChannelFromItem {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub value: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_SetTransformAnimationChannelFromItem {}
#[repr(C, align(8))]
pub struct FRigUnit_CurveExists {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub curve: FName,
    pub exists: bool,
    pub(crate) __padding_end: [u8; 35],
}
impl FRigUnit_CurveExists {}
#[repr(C, align(8))]
pub struct FRigUnit_FindClosestItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub point: crate::bindings::core_u_object::FVector,
    pub item: FRigElementKey,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_FindClosestItem {}
#[repr(C, align(16))]
pub struct FRigUnit_GetBoneTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bone: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_GetBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlInitialTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlInitialTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlOffset {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlBool {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub bool_value: bool,
    pub(crate) __padding_end: [u8; 35],
}
impl FRigUnit_GetControlBool {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlFloat {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub float_value: f32,
    pub minimum: f32,
    pub maximum: f32,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlInteger {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub integer_value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlInteger {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVector2D {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub vector: crate::bindings::core_u_object::FVector2D,
    pub minimum: crate::bindings::core_u_object::FVector2D,
    pub maximum: crate::bindings::core_u_object::FVector2D,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub vector: crate::bindings::core_u_object::FVector,
    pub minimum: crate::bindings::core_u_object::FVector,
    pub maximum: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlVector {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlRotator {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub minimum: crate::bindings::core_u_object::FRotator,
    pub maximum: crate::bindings::core_u_object::FRotator,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlRotator {}
#[repr(C, align(16))]
pub struct FRigUnit_GetControlTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub minimum: crate::bindings::core_u_object::FTransform,
    pub maximum: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_GetCurveValue {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub curve: FName,
    pub valid: bool,
    pub value: f32,
    pub(crate) __padding_end: [u8; 36],
}
impl FRigUnit_GetCurveValue {}
#[repr(C, align(16))]
pub struct FRigUnit_GetInitialBoneTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bone: FName,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetInitialBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeBoneTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub bone: FName,
    pub space: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 64],
}
impl FRigUnit_GetRelativeBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetRelativeTransformForItem {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub relative_transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 64],
}
impl FRigUnit_GetRelativeTransformForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_GetSpaceTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub space: FName,
    pub space_type: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetSpaceTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_GetTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: FRigElementKeyCollection,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_GetTransformArray {}
#[repr(C, align(8))]
pub struct FRigUnit_GetTransformItemArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_GetTransformItemArray {}
#[repr(C, align(8))]
pub struct FRigDispatch_MetadataBase {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_MetadataBase {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetMetadata {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_GetMetadata {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetMetadata {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_SetMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveMetadata {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub name: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    pub(crate) __padding_end: [u8; 34],
}
impl FRigUnit_RemoveMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveAllMetadata {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    pub(crate) __padding_end: [u8; 38],
}
impl FRigUnit_RemoveAllMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadata {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub name: FName,
    pub ty: ERigMetadataType,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    pub(crate) __padding_end: [u8; 33],
}
impl FRigUnit_HasMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadata {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub name: FName,
    pub ty: ERigMetadataType,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_FindItemsWithMetadata {}
#[repr(C, align(8))]
pub struct FRigUnit_GetMetadataTags {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetMetadataTags {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMetadataTag {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub(crate) __padding_end: [u8; 35],
}
impl FRigUnit_SetMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMetadataTagArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetMetadataTagArray {}
#[repr(C, align(8))]
pub struct FRigUnit_RemoveMetadataTag {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub removed: bool,
    pub(crate) __padding_end: [u8; 34],
}
impl FRigUnit_RemoveMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadataTag {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    pub(crate) __padding_end: [u8; 34],
}
impl FRigUnit_HasMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_HasMetadataTagArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub found: bool,
    pub(crate) __padding_end: [u8; 38],
}
impl FRigUnit_HasMetadataTagArray {}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadataTag {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub tag: FName,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_FindItemsWithMetadataTag {}
#[repr(C, align(8))]
pub struct FRigUnit_FindItemsWithMetadataTagArray {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub items: TArray<FRigElementKey>,
}
impl FRigUnit_FindItemsWithMetadataTagArray {}
#[repr(C, align(8))]
pub struct FRigUnit_FilterItemsByMetadataTags {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub tags: TArray<FName>,
    pub name_space: ERigMetaDataNameSpace,
    pub inclusive: bool,
    pub result: TArray<FRigElementKey>,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_FilterItemsByMetadataTags {}
#[repr(C, align(8))]
pub struct FRigDispatch_GetModuleMetadata {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_GetModuleMetadata {}
#[repr(C, align(8))]
pub struct FRigDispatch_SetModuleMetadata {
    pub(crate) __padding_end: [u8; 216],
}
impl FRigDispatch_SetModuleMetadata {}
#[repr(C, align(16))]
pub struct FRigUnit_OffsetTransformForItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub offset_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 43],
}
impl FRigUnit_OffsetTransformForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: FRigElementKeyCollection,
    pub initial_global_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub switched: bool,
    pub(crate) __padding_end: [u8; 175],
}
impl FRigUnit_ParentSwitchConstraint {}
#[repr(C, align(16))]
pub struct FRigUnit_ParentSwitchConstraintArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub subject: FRigElementKey,
    pub parent_index: i32,
    pub parents: TArray<FRigElementKey>,
    pub initial_global_transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub switched: bool,
    pub(crate) __padding_end: [u8; 175],
}
impl FRigUnit_ParentSwitchConstraintArray {}
#[repr(C, align(16))]
pub struct FRigUnit_ProjectTransformToNewParent {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub child: FRigElementKey,
    pub b_child_initial: bool,
    pub old_parent: FRigElementKey,
    pub b_old_parent_initial: bool,
    pub new_parent: FRigElementKey,
    pub b_new_parent_initial: bool,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 96],
}
impl FRigUnit_ProjectTransformToNewParent {}
#[repr(C, align(8))]
pub struct FRigUnit_PropagateTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub b_recompute_global: bool,
    pub b_apply_to_children: bool,
    pub b_recursive: bool,
    pub(crate) __padding_end: [u8; 37],
}
impl FRigUnit_PropagateTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SendEvent {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub event: ERigEvent,
    pub item: FRigElementKey,
    pub offset_in_seconds: f32,
    pub b_enable: bool,
    pub b_only_during_interaction: bool,
}
impl FRigUnit_SendEvent {}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneInitialTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 46],
}
impl FRigUnit_SetBoneInitialTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneRotation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetBoneRotation {}
#[repr(C, align(16))]
pub struct FRigUnit_SetBoneTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetBoneTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SetBoneTranslation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub translation: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetBoneTranslation {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlColor {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub(crate) __padding_end: [u8; 36],
}
impl FRigUnit_GetControlColor {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlColor {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub(crate) __padding_end: [u8; 36],
}
impl FRigUnit_SetControlColor {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlDrivenList {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub driven: TArray<FRigElementKey>,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetControlDrivenList {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlDrivenList {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub driven: TArray<FRigElementKey>,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_SetControlDrivenList {}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 47],
}
impl FRigUnit_SetControlOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlTranslationOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetControlTranslationOffset {}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlRotationOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub offset: crate::bindings::core_u_object::FQuat,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 47],
}
impl FRigUnit_SetControlRotationOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlScaleOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub scale: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetControlScaleOffset {}
#[repr(C, align(16))]
pub struct FRigUnit_GetShapeTransform {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub control: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_GetShapeTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetShapeTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_SetShapeTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlBool {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub bool_value: bool,
    pub(crate) __padding_end: [u8; 35],
}
impl FRigUnit_SetControlBool {}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlBool_Entry {
    pub control: FName,
    pub bool_value: bool,
}
impl FRigUnit_SetMultiControlBool_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlBool {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlBool_Entry>,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_SetMultiControlBool {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlFloat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub float_value: f32,
    pub(crate) __padding_end: [u8; 36],
}
impl FRigUnit_SetControlFloat {}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlFloat_Entry {
    pub control: FName,
    pub float_value: f32,
}
impl FRigUnit_SetMultiControlFloat_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlFloat {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlFloat_Entry>,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlFloat {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlInteger {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub weight: i32,
    pub integer_value: i32,
    pub(crate) __padding_end: [u8; 36],
}
impl FRigUnit_SetControlInteger {}
#[repr(C, align(4))]
pub struct FRigUnit_SetMultiControlInteger_Entry {
    pub control: FName,
    pub integer_value: i32,
}
impl FRigUnit_SetMultiControlInteger_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlInteger {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlInteger_Entry>,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlInteger {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVector2D {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub vector: crate::bindings::core_u_object::FVector2D,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_SetControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlVector2D_Entry {
    pub control: FName,
    pub vector: crate::bindings::core_u_object::FVector2D,
}
impl FRigUnit_SetMultiControlVector2D_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlVector2D {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlVector2D_Entry>,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlVector2D {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVector {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub vector: crate::bindings::core_u_object::FVector,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetControlVector {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlRotator {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_SetControlRotator {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlRotator_Entry {
    pub control: FName,
    pub rotator: crate::bindings::core_u_object::FRotator,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
}
impl FRigUnit_SetMultiControlRotator_Entry {}
#[repr(C, align(8))]
pub struct FRigUnit_SetMultiControlRotator {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub entries: TArray<FRigUnit_SetMultiControlRotator_Entry>,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 20],
}
impl FRigUnit_SetMultiControlRotator {}
#[repr(C, align(16))]
pub struct FRigUnit_SetControlTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub control: FName,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 47],
}
impl FRigUnit_SetControlTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_GetControlVisibility {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub item: FRigElementKey,
    pub b_visible: bool,
    pub(crate) __padding_end: [u8; 39],
}
impl FRigUnit_GetControlVisibility {}
#[repr(C, align(8))]
pub struct FRigUnit_SetControlVisibility {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub pattern: FString,
    pub b_visible: bool,
    pub(crate) __padding_end: [u8; 23],
}
impl FRigUnit_SetControlVisibility {}
#[repr(C, align(8))]
pub struct FRigUnit_SetCurveValue {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub curve: FName,
    pub value: f32,
    pub(crate) __padding_end: [u8; 32],
}
impl FRigUnit_SetCurveValue {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeBoneTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub space: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 75],
}
impl FRigUnit_SetRelativeBoneTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeTransformForItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 75],
}
impl FRigUnit_SetRelativeTransformForItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SetRelativeTranslationForItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 67],
}
impl FRigUnit_SetRelativeTranslationForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRelativeRotationForItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub parent: FRigElementKey,
    pub b_parent_initial: bool,
    pub value: crate::bindings::core_u_object::FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 75],
}
impl FRigUnit_SetRelativeRotationForItem {}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceInitialTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub space_name: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub result: crate::bindings::core_u_object::FTransform,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 47],
}
impl FRigUnit_SetSpaceInitialTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetSpaceTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub space: FName,
    pub weight: f32,
    pub transform: crate::bindings::core_u_object::FTransform,
    pub space_type: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub(crate) __padding_end: [u8; 47],
}
impl FRigUnit_SetSpaceTransform {}
#[repr(C, align(16))]
pub struct FRigUnit_SetTransform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FTransform,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 43],
}
impl FRigUnit_SetTransform {}
#[repr(C, align(8))]
pub struct FRigUnit_SetTranslation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 35],
}
impl FRigUnit_SetTranslation {}
#[repr(C, align(16))]
pub struct FRigUnit_SetRotation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub value: crate::bindings::core_u_object::FQuat,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 43],
}
impl FRigUnit_SetRotation {}
#[repr(C, align(8))]
pub struct FRigUnit_SetScale {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub scale: crate::bindings::core_u_object::FVector,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 35],
}
impl FRigUnit_SetScale {}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 19],
}
impl FRigUnit_SetTransformArray {}
#[repr(C, align(8))]
pub struct FRigUnit_SetTransformItemArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub space: crate::bindings::rig_vm::ERigVMTransformSpace,
    pub b_initial: bool,
    pub transforms: TArray<crate::bindings::core_u_object::FTransform>,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 19],
}
impl FRigUnit_SetTransformItemArray {}
#[repr(C, align(8))]
pub struct FRigUnit_UnsetCurveValue {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub curve: FName,
    pub(crate) __padding_end: [u8; 36],
}
impl FRigUnit_UnsetCurveValue {}
#[repr(C, align(16))]
pub struct FRigUnit_ToWorldSpace_Transform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub world: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ToWorldSpace_Transform {}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Transform {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FTransform,
    pub global: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ToRigSpace_Transform {}
#[repr(C, align(8))]
pub struct FRigUnit_ToWorldSpace_Location {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub world: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_ToWorldSpace_Location {}
#[repr(C, align(8))]
pub struct FRigUnit_ToRigSpace_Location {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub value: crate::bindings::core_u_object::FVector,
    pub global: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_ToRigSpace_Location {}
#[repr(C, align(16))]
pub struct FRigUnit_ToWorldSpace_Rotation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub world: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ToWorldSpace_Rotation {}
#[repr(C, align(16))]
pub struct FRigUnit_ToRigSpace_Rotation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub value: crate::bindings::core_u_object::FQuat,
    pub global: crate::bindings::core_u_object::FQuat,
}
impl FRigUnit_ToRigSpace_Rotation {}
#[repr(C, align(4))]
pub struct FRigUnit_BoneHarmonics_BoneTarget {
    pub bone: FName,
    pub ratio: f32,
}
impl FRigUnit_BoneHarmonics_BoneTarget {}
#[repr(C, align(4))]
pub struct FRigUnit_Harmonics_TargetItem {
    pub item: FRigElementKey,
    pub ratio: f32,
}
impl FRigUnit_Harmonics_TargetItem {}
#[repr(C, align(8))]
pub struct FRigUnit_BoneHarmonics {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bones: TArray<FRigUnit_BoneHarmonics_BoneTarget>,
    pub wave_speed: crate::bindings::core_u_object::FVector,
    pub wave_frequency: crate::bindings::core_u_object::FVector,
    pub wave_amplitude: crate::bindings::core_u_object::FVector,
    pub wave_offset: crate::bindings::core_u_object::FVector,
    pub wave_noise: crate::bindings::core_u_object::FVector,
    pub wave_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 42],
}
impl FRigUnit_BoneHarmonics {}
#[repr(C, align(8))]
pub struct FRigUnit_ItemHarmonics {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub targets: TArray<FRigUnit_Harmonics_TargetItem>,
    pub wave_speed: crate::bindings::core_u_object::FVector,
    pub wave_frequency: crate::bindings::core_u_object::FVector,
    pub wave_amplitude: crate::bindings::core_u_object::FVector,
    pub wave_offset: crate::bindings::core_u_object::FVector,
    pub wave_noise: crate::bindings::core_u_object::FVector,
    pub wave_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    pub(crate) __padding_end: [u8; 43],
}
impl FRigUnit_ItemHarmonics {}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Reach {
    pub b_enabled: bool,
    pub reach_target: crate::bindings::core_u_object::FVector,
    pub reach_axis: crate::bindings::core_u_object::FVector,
    pub reach_minimum: f32,
    pub reach_maximum: f32,
    pub reach_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
}
impl FRigUnit_ChainHarmonics_Reach {}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Wave {
    pub b_enabled: bool,
    pub wave_frequency: crate::bindings::core_u_object::FVector,
    pub wave_amplitude: crate::bindings::core_u_object::FVector,
    pub wave_offset: crate::bindings::core_u_object::FVector,
    pub wave_noise: crate::bindings::core_u_object::FVector,
    pub wave_minimum: f32,
    pub wave_maximum: f32,
    pub wave_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
}
impl FRigUnit_ChainHarmonics_Wave {}
#[repr(C, align(8))]
pub struct FRigUnit_ChainHarmonics_Pendulum {
    pub b_enabled: bool,
    pub pendulum_stiffness: f32,
    pub pendulum_gravity: crate::bindings::core_u_object::FVector,
    pub pendulum_blend: f32,
    pub pendulum_drag: f32,
    pub pendulum_minimum: f32,
    pub pendulum_maximum: f32,
    pub pendulum_ease: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub unwind_axis: crate::bindings::core_u_object::FVector,
    pub unwind_minimum: f32,
    pub unwind_maximum: f32,
}
impl FRigUnit_ChainHarmonics_Pendulum {}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonics {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub chain_root: FName,
    pub speed: crate::bindings::core_u_object::FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 160],
}
impl FRigUnit_ChainHarmonics {}
#[repr(C, align(16))]
pub struct FRigUnit_ChainHarmonicsPerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub chain_root: FRigElementKey,
    pub speed: crate::bindings::core_u_object::FVector,
    pub reach: FRigUnit_ChainHarmonics_Reach,
    pub wave: FRigUnit_ChainHarmonics_Wave,
    pub wave_curve: crate::bindings::engine::FRuntimeFloatCurve,
    pub pendulum: FRigUnit_ChainHarmonics_Pendulum,
    pub b_draw_debug: bool,
    pub draw_world_offset: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 160],
}
impl FRigUnit_ChainHarmonicsPerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_AimBone_Target {
    pub weight: f32,
    pub axis: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FName,
}
impl FRigUnit_AimBone_Target {}
#[repr(C, align(8))]
pub struct FRigUnit_AimItem_Target {
    pub weight: f32,
    pub axis: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
}
impl FRigUnit_AimItem_Target {}
#[repr(C, align(16))]
pub struct FRigUnit_AimBone_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_AimBone_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_AimBoneMath {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input_transform: crate::bindings::core_u_object::FTransform,
    pub primary: FRigUnit_AimItem_Target,
    pub secondary: FRigUnit_AimItem_Target,
    pub weight: f32,
    pub result: crate::bindings::core_u_object::FTransform,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_AimBoneMath {}
#[repr(C, align(16))]
pub struct FRigUnit_AimBone {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub primary: FRigUnit_AimBone_Target,
    pub secondary: FRigUnit_AimBone_Target,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub(crate) __padding_end: [u8; 112],
}
impl FRigUnit_AimBone {}
#[repr(C, align(16))]
pub struct FRigUnit_AimItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub primary: FRigUnit_AimItem_Target,
    pub secondary: FRigUnit_AimItem_Target,
    pub weight: f32,
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub(crate) __padding_end: [u8; 112],
}
impl FRigUnit_AimItem {}
#[repr(C, align(8))]
pub struct FRigUnit_AimConstraint_WorldUp {
    pub target: crate::bindings::core_u_object::FVector,
    pub kind: EControlRigVectorKind,
    pub space: FRigElementKey,
}
impl FRigUnit_AimConstraint_WorldUp {}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraint_AdvancedSettings {
    pub debug_settings: FRigUnit_AimBone_DebugSettings,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
impl FRigUnit_AimConstraint_AdvancedSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_AimConstraintLocalSpaceOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub aim_axis: crate::bindings::core_u_object::FVector,
    pub up_axis: crate::bindings::core_u_object::FVector,
    pub world_up: FRigUnit_AimConstraint_WorldUp,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_AimConstraint_AdvancedSettings,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 92],
}
impl FRigUnit_AimConstraintLocalSpaceOffset {}
#[repr(C, align(4))]
pub struct FConstraintParent {
    pub item: FRigElementKey,
    pub weight: f32,
}
impl FConstraintParent {}
#[repr(C, align(4))]
pub struct FRigUnit_CCDIK_RotationLimit {
    pub bone: FName,
    pub limit: f32,
}
impl FRigUnit_CCDIK_RotationLimit {}
#[repr(C, align(4))]
pub struct FRigUnit_CCDIK_RotationLimitPerItem {
    pub item: FRigElementKey,
    pub limit: f32,
}
impl FRigUnit_CCDIK_RotationLimitPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_bone: FName,
    pub effector_bone: FName,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimit>,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 103],
}
impl FRigUnit_CCDIK {}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIKPerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimitPerItem>,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 103],
}
impl FRigUnit_CCDIKPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_CCDIKItemArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub max_iterations: i32,
    pub b_start_from_tail: bool,
    pub base_rotation_limit: f32,
    pub rotation_limits: TArray<FRigUnit_CCDIK_RotationLimitPerItem>,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 103],
}
impl FRigUnit_CCDIKItemArray {}
#[repr(C, align(4))]
pub struct FRigUnit_ChainInfo_SegmentInfo {
    pub(crate) __padding_end: [u8; 60],
}
impl FRigUnit_ChainInfo_SegmentInfo {}
#[repr(C, align(16))]
pub struct FRigUnit_ChainInfo {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub items: TArray<FRigElementKey>,
    pub param: f32,
    pub b_calculate_stretch: bool,
    pub b_initial: bool,
    pub b_debug: bool,
    pub debug_scale: f32,
    pub interpolated_transform: crate::bindings::core_u_object::FTransform,
    pub chain_length: f32,
    pub param_length: f32,
    pub chain_stretch_factor: f32,
    pub segment_info: FRigUnit_ChainInfo_SegmentInfo,
    pub(crate) __padding_end: [u8; 24],
}
impl FRigUnit_ChainInfo {}
#[repr(C, align(16))]
pub struct FRigUnit_DistributeRotation_Rotation {
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub ratio: f32,
}
impl FRigUnit_DistributeRotation_Rotation {}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 87],
}
impl FRigUnit_DistributeRotation {}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForCollection {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_DistributeRotationForCollection {}
#[repr(C, align(8))]
pub struct FRigUnit_DistributeRotationForItemArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub rotations: TArray<FRigUnit_DistributeRotation_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_DistributeRotationForItemArray {}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_bone: FName,
    pub effector_bone: FName,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    #[doc(hidden)]
    pub(crate) __padding_224: [u8; 64],
    pub b_set_effector_transform: bool,
}
impl FRigUnit_FABRIK {}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIKPerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    #[doc(hidden)]
    pub(crate) __padding_208: [u8; 64],
    pub b_set_effector_transform: bool,
}
impl FRigUnit_FABRIKPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_FABRIKItemArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub effector_transform: crate::bindings::core_u_object::FTransform,
    pub precision: f32,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    #[doc(hidden)]
    pub(crate) __padding_208: [u8; 64],
    pub b_set_effector_transform: bool,
}
impl FRigUnit_FABRIKItemArray {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_Rotation {
    pub rotation: crate::bindings::core_u_object::FQuat,
    pub ratio: f32,
}
impl FRigUnit_FitChainToCurve_Rotation {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub curve_color: crate::bindings::core_u_object::FLinearColor,
    pub segments_color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_FitChainToCurve_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurve {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub(crate) __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToCurve {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurvePerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub(crate) __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToCurvePerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_FitChainToCurveItemArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub alignment: EControlRigCurveAlignment,
    pub minimum: f32,
    pub maximum: f32,
    pub sampling_precision: i32,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub pole_vector_position: crate::bindings::core_u_object::FVector,
    pub rotations: TArray<FRigUnit_FitChainToCurve_Rotation>,
    pub rotation_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_FitChainToCurve_DebugSettings,
    pub(crate) __padding_end: [u8; 160],
}
impl FRigUnit_FitChainToCurveItemArray {}
#[repr(C, align(16))]
pub struct FRigUnit_ModifyBoneTransforms_PerBone {
    pub bone: FName,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ModifyBoneTransforms_PerBone {}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyBoneTransforms {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone_to_modify: TArray<FRigUnit_ModifyBoneTransforms_PerBone>,
    pub weight: f32,
    pub weight_minimum: f32,
    pub weight_maximum: f32,
    pub mode: EControlRigModifyBoneMode,
    pub(crate) __padding_end: [u8; 19],
}
impl FRigUnit_ModifyBoneTransforms {}
#[repr(C, align(16))]
pub struct FRigUnit_ModifyTransforms_PerItem {
    pub item: FRigElementKey,
    pub transform: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_ModifyTransforms_PerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_ModifyTransforms {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item_to_modify: TArray<FRigUnit_ModifyTransforms_PerItem>,
    pub weight: f32,
    pub weight_minimum: f32,
    pub weight_maximum: f32,
    pub mode: EControlRigModifyBoneMode,
    pub(crate) __padding_end: [u8; 19],
}
impl FRigUnit_ModifyTransforms {}
#[repr(C, align(8))]
pub struct FRigUnit_MultiFABRIK_EndEffector {
    pub bone: FName,
    pub location: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_MultiFABRIK_EndEffector {}
#[repr(C, align(8))]
pub struct FRigUnit_MultiFABRIK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub root_bone: FName,
    pub effectors: TArray<FRigUnit_MultiFABRIK_EndEffector>,
    pub precision: f32,
    pub b_propagate_to_children: bool,
    pub max_iterations: i32,
    pub(crate) __padding_end: [u8; 116],
}
impl FRigUnit_MultiFABRIK {}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChain {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 75],
}
impl FRigUnit_SlideChain {}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChainPerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 75],
}
impl FRigUnit_SlideChainPerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_SlideChainItemArray {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: TArray<FRigElementKey>,
    pub slide_amount: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 75],
}
impl FRigUnit_SlideChainItemArray {}
#[repr(C, align(4))]
pub struct FRegionScaleFactors {
    pub positive_width: f32,
    pub negative_width: f32,
    pub positive_height: f32,
    pub negative_height: f32,
}
impl FRegionScaleFactors {}
#[repr(C, align(4))]
pub struct FSphericalPoseReaderDebugSettings {
    pub b_draw_debug: bool,
    pub b_draw2_d: bool,
    pub b_draw_local_axes: bool,
    pub debug_scale: f32,
    pub debug_segments: i32,
    pub debug_thickness: f32,
}
impl FSphericalPoseReaderDebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_SphericalPoseReader {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub output_param: f32,
    pub driver_item: FRigElementKey,
    pub driver_axis: crate::bindings::core_u_object::FVector,
    pub rotation_offset: crate::bindings::core_u_object::FVector,
    pub active_region_size: f32,
    pub active_region_scale_factors: FRegionScaleFactors,
    pub falloff_size: f32,
    pub falloff_region_scale_factors: FRegionScaleFactors,
    pub flip_width_scaling: bool,
    pub flip_height_scaling: bool,
    pub optional_parent_item: FRigElementKey,
    pub debug: FSphericalPoseReaderDebugSettings,
    pub(crate) __padding_end: [u8; 284],
}
impl FRigUnit_SphericalPoseReader {}
#[repr(C, align(16))]
pub struct FRigUnit_SpringIK_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_SpringIK_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_SpringIK {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub hierarchy_strength: f32,
    pub effector_strength: f32,
    pub effector_ratio: f32,
    pub root_strength: f32,
    pub root_ratio: f32,
    pub damping: f32,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub b_flip_pole_plane: bool,
    pub pole_vector_kind: EControlRigVectorKind,
    pub pole_vector_space: FName,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub b_live_simulation: bool,
    pub iterations: i32,
    pub b_limit_local_position: bool,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_SpringIK_DebugSettings,
    pub(crate) __padding_end: [u8; 192],
}
impl FRigUnit_SpringIK {}
#[repr(C, align(16))]
pub struct FConstraintTarget {
    #[doc(hidden)]
    pub(crate) __padding_101: [u8; 101],
    pub filter: crate::bindings::animation_core::FTransformFilter,
}
impl FConstraintTarget {}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone: FName,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_bone: FName,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    pub(crate) __padding_end: [u8; 111],
}
impl FRigUnit_TransformConstraint {}
#[repr(C, align(16))]
pub struct FRigUnit_TransformConstraintPerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item: FRigElementKey,
    pub base_transform_space: ETransformSpaceMode,
    pub base_transform: crate::bindings::core_u_object::FTransform,
    pub base_item: FRigElementKey,
    pub targets: TArray<FConstraintTarget>,
    pub b_use_initial_transforms: bool,
    pub(crate) __padding_end: [u8; 111],
}
impl FRigUnit_TransformConstraintPerItem {}
#[repr(C, align(1))]
pub struct FRigUnit_ParentConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
impl FRigUnit_ParentConstraint_AdvancedSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_ParentConstraint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FTransformFilter,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_ParentConstraint_AdvancedSettings,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_ParentConstraint {}
#[repr(C, align(1))]
pub struct FRigUnit_ParentConstraintMath_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
}
impl FRigUnit_ParentConstraintMath_AdvancedSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_ParentConstraintMath {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub input: crate::bindings::core_u_object::FTransform,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_ParentConstraintMath_AdvancedSettings,
    pub output: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_ParentConstraintMath {}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
}
impl FRigUnit_PositionConstraint {}
#[repr(C, align(8))]
pub struct FRigUnit_PositionConstraintLocalSpaceOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 52],
}
impl FRigUnit_PositionConstraintLocalSpaceOffset {}
#[repr(C, align(1))]
pub struct FRigUnit_RotationConstraint_AdvancedSettings {
    pub interpolation_type: EConstraintInterpType,
    pub rotation_order_for_filter: crate::bindings::animation_core::EEulerRotationOrder,
}
impl FRigUnit_RotationConstraint_AdvancedSettings {}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_RotationConstraint_AdvancedSettings,
    pub weight: f32,
}
impl FRigUnit_RotationConstraint {}
#[repr(C, align(8))]
pub struct FRigUnit_RotationConstraintLocalSpaceOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub advanced_settings: FRigUnit_RotationConstraint_AdvancedSettings,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 48],
}
impl FRigUnit_RotationConstraintLocalSpaceOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_ScaleConstraint {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
}
impl FRigUnit_ScaleConstraint {}
#[repr(C, align(8))]
pub struct FRigUnit_ScaleConstraintLocalSpaceOffset {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub child: FRigElementKey,
    pub b_maintain_offset: bool,
    pub filter: crate::bindings::animation_core::FFilterOptionPerAxis,
    pub parents: TArray<FConstraintParent>,
    pub weight: f32,
    pub(crate) __padding_end: [u8; 52],
}
impl FRigUnit_ScaleConstraintLocalSpaceOffset {}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBones {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub start_bone: FName,
    pub end_bone: FName,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub pole_axis: crate::bindings::core_u_object::FVector,
    pub twist_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 55],
}
impl FRigUnit_TwistBones {}
#[repr(C, align(8))]
pub struct FRigUnit_TwistBonesPerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub items: FRigElementKeyCollection,
    pub twist_axis: crate::bindings::core_u_object::FVector,
    pub pole_axis: crate::bindings::core_u_object::FVector,
    pub twist_ease_type: crate::bindings::rig_vm::ERigVMAnimEasingType,
    pub weight: f32,
    pub b_propagate_to_children: bool,
    pub(crate) __padding_end: [u8; 55],
}
impl FRigUnit_TwistBonesPerItem {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_TwoBoneIKSimple_DebugSettings {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimple {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub bone_a: FName,
    pub bone_b: FName,
    pub effector_bone: FName,
    pub effector: crate::bindings::core_u_object::FTransform,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis_weight: f32,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub pole_vector_kind: EControlRigVectorKind,
    pub pole_vector_space: FName,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub weight: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_TwoBoneIKSimple_DebugSettings,
    pub(crate) __padding_end: [u8; 128],
}
impl FRigUnit_TwoBoneIKSimple {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimplePerItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item_a: FRigElementKey,
    pub item_b: FRigElementKey,
    pub effector_item: FRigElementKey,
    pub effector: crate::bindings::core_u_object::FTransform,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis_weight: f32,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub pole_vector_kind: EControlRigVectorKind,
    pub pole_vector_space: FRigElementKey,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub weight: f32,
    pub item_a_length: f32,
    pub item_b_length: f32,
    pub b_propagate_to_children: bool,
    pub debug_settings: FRigUnit_TwoBoneIKSimple_DebugSettings,
    pub(crate) __padding_end: [u8; 128],
}
impl FRigUnit_TwoBoneIKSimplePerItem {}
#[repr(C, align(8))]
pub struct FRigUnit_TwoBoneIKSimpleVectors {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub root: crate::bindings::core_u_object::FVector,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub effector: crate::bindings::core_u_object::FVector,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub elbow: crate::bindings::core_u_object::FVector,
}
impl FRigUnit_TwoBoneIKSimpleVectors {}
#[repr(C, align(16))]
pub struct FRigUnit_TwoBoneIKSimpleTransforms {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub root: crate::bindings::core_u_object::FTransform,
    pub pole_vector: crate::bindings::core_u_object::FVector,
    pub effector: crate::bindings::core_u_object::FTransform,
    pub primary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis: crate::bindings::core_u_object::FVector,
    pub secondary_axis_weight: f32,
    pub b_enable_stretch: bool,
    pub stretch_start_ratio: f32,
    pub stretch_maximum_ratio: f32,
    pub bone_a_length: f32,
    pub bone_b_length: f32,
    pub elbow: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_TwoBoneIKSimpleTransforms {}
#[repr(C, align(8))]
pub struct FRigUnit_GetCandidates {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub connector: FRigElementKey,
    pub candidates: TArray<FRigElementKey>,
}
impl FRigUnit_GetCandidates {}
#[repr(C, align(8))]
pub struct FRigUnit_DiscardMatches {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub excluded: TArray<FRigElementKey>,
    pub message: FString,
}
impl FRigUnit_DiscardMatches {}
#[repr(C, align(8))]
pub struct FRigUnit_SetDefaultMatch {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub default: FRigElementKey,
}
impl FRigUnit_SetDefaultMatch {}
#[repr(C, align(16))]
pub struct FRigUnit_ConnectorExecution {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub execute_context: FControlRigExecuteContext,
}
impl FRigUnit_ConnectorExecution {}
#[repr(C, align(16))]
pub struct FRigUnit_PointSimulation_DebugSettings {
    pub b_enabled: bool,
    pub scale: f32,
    pub collision_scale: f32,
    pub b_draw_points_as_spheres: bool,
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub world_offset: crate::bindings::core_u_object::FTransform,
}
impl FRigUnit_PointSimulation_DebugSettings {}
#[repr(C, align(4))]
pub struct FRigUnit_PointSimulation_BoneTarget {
    pub bone: FName,
    pub translation_point: i32,
    pub primary_aim_point: i32,
    pub secondary_aim_point: i32,
}
impl FRigUnit_PointSimulation_BoneTarget {}
#[repr(C, align(16))]
pub struct FRigUnit_PointSimulation {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub points: TArray<crate::bindings::rig_vm::FRigVMSimPoint>,
    pub links: TArray<FCRSimLinearSpring>,
    pub forces: TArray<FCRSimPointForce>,
    pub collision_volumes: TArray<FCRSimSoftCollision>,
    pub simulated_steps_per_second: f32,
    pub integrator_type: crate::bindings::rig_vm::ERigVMSimPointIntegrateType,
    pub verlet_blend: f32,
    pub bone_targets: TArray<FRigUnit_PointSimulation_BoneTarget>,
    pub b_limit_local_position: bool,
    pub b_propagate_to_children: bool,
    pub primary_aim_axis: crate::bindings::core_u_object::FVector,
    pub secondary_aim_axis: crate::bindings::core_u_object::FVector,
    pub debug_settings: FRigUnit_PointSimulation_DebugSettings,
    pub bezier: crate::bindings::rig_vm::FRigVMFourPointBezier,
    pub(crate) __padding_end: [u8; 144],
}
impl FRigUnit_PointSimulation {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterp {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub current: f32,
    pub target: f32,
    pub stiffness: f32,
    pub critical_damping: f32,
    pub mass: f32,
    pub result: f32,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_SpringInterp {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVector {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub current: crate::bindings::core_u_object::FVector,
    pub target: crate::bindings::core_u_object::FVector,
    pub stiffness: f32,
    pub critical_damping: f32,
    pub mass: f32,
    pub result: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 56],
}
impl FRigUnit_SpringInterpVector {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpV2 {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target: f32,
    pub strength: f32,
    pub critical_damping: f32,
    pub force: f32,
    pub b_use_current_input: bool,
    pub current: f32,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: f32,
    pub velocity: f32,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigUnit_SpringInterpV2 {}
#[repr(C, align(8))]
pub struct FRigUnit_SpringInterpVectorV2 {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub target: crate::bindings::core_u_object::FVector,
    pub strength: f32,
    pub critical_damping: f32,
    pub force: crate::bindings::core_u_object::FVector,
    pub b_use_current_input: bool,
    pub current: crate::bindings::core_u_object::FVector,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: crate::bindings::core_u_object::FVector,
    pub velocity: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 80],
}
impl FRigUnit_SpringInterpVectorV2 {}
#[repr(C, align(16))]
pub struct FRigUnit_SpringInterpQuaternionV2 {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub target: crate::bindings::core_u_object::FQuat,
    pub strength: f32,
    pub critical_damping: f32,
    pub torque: crate::bindings::core_u_object::FVector,
    pub b_use_current_input: bool,
    pub current: crate::bindings::core_u_object::FQuat,
    pub target_velocity_amount: f32,
    pub b_initialize_from_target: bool,
    pub result: crate::bindings::core_u_object::FQuat,
    pub angular_velocity: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 104],
}
impl FRigUnit_SpringInterpQuaternionV2 {}
#[repr(C, align(8))]
pub struct UAnimNodeControlRigLibrary {
    __padding_end: [u8; 48],
}
impl UAnimNodeControlRigLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNodeControlRigLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimNodeControlRigLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_control_rig_class(
        node: &FControlRigReference,
        control_rig_class: TSubclassOf<UControlRig>,
    ) -> FControlRigReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_anim_node_control_rig_library_set_control_rig_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<FControlRigReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig_class,
                __buffer.add(16).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::UAnimNodeControlRigLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_anim_node_control_rig_library_set_control_rig_class,
                __buffer,
            )
        };
        std::mem::forget(control_rig_class);
        unsafe { __buffer.add(24).cast::<FControlRigReference>().read() }
    }
    pub fn convert_to_control_rig_pure(
        node: &crate::bindings::engine::FAnimNodeReference,
        control_rig: &mut FControlRigReference,
        result: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_anim_node_control_rig_library_convert_to_control_rig_pure,
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
                control_rig,
                __buffer.add(16).cast::<FControlRigReference>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(result, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig::UAnimNodeControlRigLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_anim_node_control_rig_library_convert_to_control_rig_pure,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FControlRigReference>().swap(control_rig);
        }
        unsafe {
            __buffer.add(32).cast::<bool>().swap(result);
        }
    }
    pub fn convert_to_control_rig(
        node: &crate::bindings::engine::FAnimNodeReference,
        result: &mut crate::bindings::engine::EAnimNodeReferenceConversionResult,
    ) -> FControlRigReference {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_anim_node_control_rig_library_convert_to_control_rig,
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
        let __object_ptr = crate::bindings::control_rig::UAnimNodeControlRigLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_anim_node_control_rig_library_convert_to_control_rig,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::engine::EAnimNodeReferenceConversionResult>()
                .swap(result);
        }
        unsafe { __buffer.add(24).cast::<FControlRigReference>().read() }
    }
}
#[repr(C, align(16))]
pub struct UTransformableControlHandle {
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 112],
    pub control_rig: TSoftObjectPtr<UControlRig>,
    pub control_name: FName,
    __padding_end: [u8; 180],
}
impl UTransformableControlHandle {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableControlHandle")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformableControlHandle")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRig {
    __padding_end: [u8; 5040],
}
impl UControlRig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRig")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn supports_backwards_solve(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_supports_backwards_solve,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_supports_backwards_solve,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_interaction_rig_class(
        &mut self,
        in_interaction_rig_class: TSubclassOf<UControlRig>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_set_interaction_rig_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_interaction_rig_class,
                __buffer.add(0).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_set_interaction_rig_class,
                __buffer,
            )
        };
        std::mem::forget(in_interaction_rig_class);
    }
    pub fn set_interaction_rig(&mut self, in_interaction_rig: UPtr<UControlRig>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_set_interaction_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_interaction_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_set_interaction_rig,
                __buffer,
            )
        };
        std::mem::forget(in_interaction_rig);
    }
    pub fn select_control(
        &mut self,
        in_control_name: &FName,
        b_select: bool,
        b_setup_undo: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_select_control,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(12).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_select_control,
                __buffer,
            )
        };
        std::mem::forget(b_select);
        std::mem::forget(b_setup_undo);
    }
    pub fn request_construction(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_request_construction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_request_construction,
                __buffer,
            )
        };
    }
    pub fn is_control_selected(&self, in_control_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_is_control_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_is_control_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_interaction_rig_class(&self) -> TSubclassOf<UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_interaction_rig_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_interaction_rig_class,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSubclassOf<UControlRig>>().read() }
    }
    pub fn get_interaction_rig(&self) -> UPtr<UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_interaction_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_interaction_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UControlRig>>().read() }
    }
    pub fn get_hosting_actor(&self) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_hosting_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_hosting_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
    pub fn get_hierarchy(&mut self) -> UPtr<URigHierarchy> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_hierarchy,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_get_hierarchy,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigHierarchy>>().read() }
    }
    pub fn find_control_rigs(
        outer: UPtr<crate::bindings::core_u_object::UObject>,
        optional_class: TSubclassOf<UControlRig>,
    ) -> TArray<UPtr<UControlRig>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_find_control_rigs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &outer,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &optional_class,
                __buffer.add(8).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::UControlRig::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_find_control_rigs,
                __buffer,
            )
        };
        std::mem::forget(outer);
        std::mem::forget(optional_class);
        unsafe { __buffer.add(16).cast::<TArray<UPtr<UControlRig>>>().read() }
    }
    pub fn current_control_selection(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_current_control_selection,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_current_control_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn create_transformable_control_handle(
        &self,
        control_name: &FName,
    ) -> UPtr<UTransformableControlHandle> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_create_transformable_control_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_create_transformable_control_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UTransformableControlHandle>>().read() }
    }
    pub fn clear_control_selection(&mut self, b_setup_undo: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_clear_control_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_clear_control_selection,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct UControlRigAnimInstance {
    __padding_end: [u8; 1136],
}
impl UControlRigAnimInstance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigAnimInstance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigAnimInstance")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigShapeLibraryLink {
    #[doc(hidden)]
    pub(crate) __padding_264: [u8; 264],
    pub shape_library: TSoftObjectPtr<UControlRigShapeLibrary>,
    __padding_end: [u8; 24],
}
impl UControlRigShapeLibraryLink {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigShapeLibraryLink")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigShapeLibraryLink")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_shape_library(
        &mut self,
        in_shape_library: TSoftObjectPtr<UControlRigShapeLibrary>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_shape_library_link_set_shape_library,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_shape_library,
                __buffer.add(0).cast::<TSoftObjectPtr<UControlRigShapeLibrary>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_shape_library_link_set_shape_library,
                __buffer,
            )
        };
        std::mem::forget(in_shape_library);
    }
    pub fn get_shape_library(&self) -> TSoftObjectPtr<UControlRigShapeLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_shape_library_link_get_shape_library,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_shape_library_link_get_shape_library,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TSoftObjectPtr<UControlRigShapeLibrary>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigBlueprintGeneratedClass {
    __padding_end: [u8; 1960],
}
impl UControlRigBlueprintGeneratedClass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintGeneratedClass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintGeneratedClass")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigComponent {
    #[doc(hidden)]
    pub(crate) __padding_1504: [u8; 1504],
    pub control_rig_class: TSubclassOf<UControlRig>,
    __padding_end: [u8; 552],
}
impl UControlRigComponent {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigComponent")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_update,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_update,
                __buffer,
            )
        };
        std::mem::forget(delta_time);
    }
    pub fn set_object_binding(
        &mut self,
        in_object_to_bind: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_object_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object_to_bind,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_object_binding,
                __buffer,
            )
        };
        std::mem::forget(in_object_to_bind);
    }
    pub fn set_mapped_elements(
        &mut self,
        new_mapped_elements: TArray<FControlRigComponentMappedElement>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_mapped_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mapped_elements,
                __buffer.add(0).cast::<TArray<FControlRigComponentMappedElement>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_mapped_elements,
                __buffer,
            )
        };
        std::mem::forget(new_mapped_elements);
    }
    pub fn set_initial_space_transform(
        &mut self,
        space_name: FName,
        initial_transform: crate::bindings::core_u_object::FTransform,
        space: EControlRigComponentSpace,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_initial_space_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &initial_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(112).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_initial_space_transform,
                __buffer,
            )
        };
        std::mem::forget(space_name);
        std::mem::forget(initial_transform);
        std::mem::forget(space);
    }
    pub fn set_initial_bone_transform(
        &mut self,
        bone_name: FName,
        initial_transform: crate::bindings::core_u_object::FTransform,
        space: EControlRigComponentSpace,
        b_propagate_to_children: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_initial_bone_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &initial_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(112).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_propagate_to_children,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_initial_bone_transform,
                __buffer,
            )
        };
        std::mem::forget(bone_name);
        std::mem::forget(initial_transform);
        std::mem::forget(space);
        std::mem::forget(b_propagate_to_children);
    }
    pub fn set_control_vector2_d(
        &mut self,
        control_name: FName,
        value: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_vector2_d,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
    }
    pub fn set_control_transform(
        &mut self,
        control_name: FName,
        value: crate::bindings::core_u_object::FTransform,
        space: EControlRigComponentSpace,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(112).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_transform,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
        std::mem::forget(space);
    }
    pub fn set_control_scale(
        &mut self,
        control_name: FName,
        value: crate::bindings::core_u_object::FVector,
        space: EControlRigComponentSpace,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(40).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_scale,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
        std::mem::forget(space);
    }
    pub fn set_control_rotator(
        &mut self,
        control_name: FName,
        value: crate::bindings::core_u_object::FRotator,
        space: EControlRigComponentSpace,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(40).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_rotator,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
        std::mem::forget(space);
    }
    pub fn set_control_rig_class(
        &mut self,
        in_control_rig_class: TSubclassOf<UControlRig>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_rig_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig_class,
                __buffer.add(0).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_rig_class,
                __buffer,
            )
        };
        std::mem::forget(in_control_rig_class);
    }
    pub fn set_control_position(
        &mut self,
        control_name: FName,
        value: crate::bindings::core_u_object::FVector,
        space: EControlRigComponentSpace,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(40).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_position,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
        std::mem::forget(space);
    }
    pub fn set_control_offset(
        &mut self,
        control_name: FName,
        offset_transform: crate::bindings::core_u_object::FTransform,
        space: EControlRigComponentSpace,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(112).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_offset,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(offset_transform);
        std::mem::forget(space);
    }
    pub fn set_control_int(&mut self, control_name: FName, value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_int,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
    }
    pub fn set_control_float(&mut self, control_name: FName, value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_float,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
    }
    pub fn set_control_bool(&mut self, control_name: FName, value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(12).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_control_bool,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(value);
    }
    pub fn set_bone_transform(
        &mut self,
        bone_name: FName,
        transform: crate::bindings::core_u_object::FTransform,
        space: EControlRigComponentSpace,
        weight: f32,
        b_propagate_to_children: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<121>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_bone_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(112).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&weight, __buffer.add(116).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_propagate_to_children,
                __buffer.add(120).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_bone_transform,
                __buffer,
            )
        };
        std::mem::forget(bone_name);
        std::mem::forget(transform);
        std::mem::forget(space);
        std::mem::forget(weight);
        std::mem::forget(b_propagate_to_children);
    }
    pub fn set_bone_initial_transforms_from_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_bone_initial_transforms_from_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_set_bone_initial_transforms_from_skeletal_mesh,
                __buffer,
            )
        };
        std::mem::forget(in_skeletal_mesh);
    }
    pub fn on_pre_initialize(&mut self, component: UPtr<UControlRigComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_pre_initialize,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UControlRigComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_pre_initialize,
                __buffer,
            )
        };
        std::mem::forget(component);
    }
    pub fn on_pre_forwards_solve(&mut self, component: UPtr<UControlRigComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_pre_forwards_solve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UControlRigComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_pre_forwards_solve,
                __buffer,
            )
        };
        std::mem::forget(component);
    }
    pub fn on_pre_construction(&mut self, component: UPtr<UControlRigComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_pre_construction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UControlRigComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_pre_construction,
                __buffer,
            )
        };
        std::mem::forget(component);
    }
    pub fn on_post_initialize(&mut self, component: UPtr<UControlRigComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_post_initialize,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UControlRigComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_post_initialize,
                __buffer,
            )
        };
        std::mem::forget(component);
    }
    pub fn on_post_forwards_solve(&mut self, component: UPtr<UControlRigComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_post_forwards_solve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UControlRigComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_post_forwards_solve,
                __buffer,
            )
        };
        std::mem::forget(component);
    }
    pub fn on_post_construction(&mut self, component: UPtr<UControlRigComponent>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_post_construction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UControlRigComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_on_post_construction,
                __buffer,
            )
        };
        std::mem::forget(component);
    }
    pub fn initialize(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_initialize,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_initialize,
                __buffer,
            )
        };
    }
    pub fn get_space_transform(
        &mut self,
        space_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_space_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_space_transform,
                __buffer,
            )
        };
        std::mem::forget(space_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_initial_space_transform(
        &mut self,
        space_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_initial_space_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_initial_space_transform,
                __buffer,
            )
        };
        std::mem::forget(space_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_initial_bone_transform(
        &mut self,
        bone_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_initial_bone_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_initial_bone_transform,
                __buffer,
            )
        };
        std::mem::forget(bone_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_element_names(&mut self, element_type: ERigElementType) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_element_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_type,
                __buffer.add(0).cast::<ERigElementType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_element_names,
                __buffer,
            )
        };
        std::mem::forget(element_type);
        unsafe { __buffer.add(8).cast::<TArray<FName>>().read() }
    }
    pub fn get_control_vector2_d(
        &mut self,
        control_name: FName,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_vector2_d,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_control_transform(
        &mut self,
        control_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_transform,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_control_scale(
        &mut self,
        control_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_scale,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_control_rotator(
        &mut self,
        control_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_rotator,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_control_rig(&mut self) -> UPtr<UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UControlRig>>().read() }
    }
    pub fn get_control_position(
        &mut self,
        control_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_position,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_control_offset(
        &mut self,
        control_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_offset,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_control_int(&mut self, control_name: FName) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_int,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn get_control_float(&mut self, control_name: FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_float,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        unsafe { __buffer.add(12).cast::<f32>().read() }
    }
    pub fn get_control_bool(&mut self, control_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_control_bool,
                __buffer,
            )
        };
        std::mem::forget(control_name);
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_bone_transform(
        &mut self,
        bone_name: FName,
        space: EControlRigComponentSpace,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_bone_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(12).cast::<EControlRigComponentSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_bone_transform,
                __buffer,
            )
        };
        std::mem::forget(bone_name);
        std::mem::forget(space);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_absolute_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_absolute_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_get_absolute_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn does_element_exist(
        &mut self,
        name: FName,
        element_type: ERigElementType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_does_element_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &element_type,
                __buffer.add(12).cast::<ERigElementType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_does_element_exist,
                __buffer,
            )
        };
        std::mem::forget(name);
        std::mem::forget(element_type);
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn clear_mapped_elements(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_clear_mapped_elements,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_clear_mapped_elements,
                __buffer,
            )
        };
    }
    pub fn can_execute(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_can_execute,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_can_execute,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_mapped_skeletal_mesh(
        &mut self,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        bones: TArray<FControlRigComponentMappedBone>,
        curves: TArray<FControlRigComponentMappedCurve>,
        in_direction: EControlRigComponentMapDirection,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bones,
                __buffer.add(8).cast::<TArray<FControlRigComponentMappedBone>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curves,
                __buffer.add(24).cast::<TArray<FControlRigComponentMappedCurve>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_direction,
                __buffer.add(40).cast::<EControlRigComponentMapDirection>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_skeletal_mesh,
                __buffer,
            )
        };
        std::mem::forget(skeletal_mesh_component);
        std::mem::forget(bones);
        std::mem::forget(curves);
        std::mem::forget(in_direction);
    }
    pub fn add_mapped_elements(
        &mut self,
        new_mapped_elements: TArray<FControlRigComponentMappedElement>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_mapped_elements,
                __buffer.add(0).cast::<TArray<FControlRigComponentMappedElement>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_elements,
                __buffer,
            )
        };
        std::mem::forget(new_mapped_elements);
    }
    pub fn add_mapped_components(
        &mut self,
        components: TArray<FControlRigComponentMappedComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &components,
                __buffer.add(0).cast::<TArray<FControlRigComponentMappedComponent>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_components,
                __buffer,
            )
        };
        std::mem::forget(components);
    }
    pub fn add_mapped_complete_skeletal_mesh(
        &mut self,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        in_direction: EControlRigComponentMapDirection,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_complete_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_direction,
                __buffer.add(8).cast::<EControlRigComponentMapDirection>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_component_add_mapped_complete_skeletal_mesh,
                __buffer,
            )
        };
        std::mem::forget(skeletal_mesh_component);
        std::mem::forget(in_direction);
    }
}
#[repr(C, align(8))]
pub struct AControlRigControlActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub actor_to_track: UPtr<crate::bindings::engine::AActor>,
    pub control_rig_class: TSubclassOf<UControlRig>,
    pub b_refresh_on_tick: bool,
    pub b_is_selectable: bool,
    pub material_override: UPtr<crate::bindings::engine::UMaterialInterface>,
    pub color_parameter: FString,
    pub b_cast_shadows: bool,
    __padding_end: [u8; 143],
}
impl AControlRigControlActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AControlRigControlActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AControlRigControlActor")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn reset_control_actor(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_control_actor_reset_control_actor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_control_actor_reset_control_actor,
                __buffer,
            )
        };
    }
    pub fn refresh(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_control_actor_refresh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_control_actor_refresh,
                __buffer,
            )
        };
    }
    pub fn clear(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_control_actor_clear,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_control_actor_clear,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct AControlRigShapeActor {
    #[doc(hidden)]
    pub(crate) __padding_1152: [u8; 1152],
    pub static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    #[doc(hidden)]
    pub(crate) __padding_1360: [u8; 200],
    pub flags_1360: u8,
    __padding_end: [u8; 31],
}
impl AControlRigShapeActor {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AControlRigShapeActor")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AControlRigShapeActor")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_selected(&mut self, b_in_selected: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_selected,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_selected,
                __buffer,
            )
        };
        std::mem::forget(b_in_selected);
    }
    pub fn set_selectable(&mut self, b_in_selectable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_selectable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_selectable,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_selectable,
                __buffer,
            )
        };
        std::mem::forget(b_in_selectable);
    }
    pub fn set_hovered(&mut self, b_in_hovered: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_hovered,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_hovered,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_hovered,
                __buffer,
            )
        };
        std::mem::forget(b_in_hovered);
    }
    pub fn set_global_transform(
        &mut self,
        in_transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_global_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_transform,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_global_transform,
                __buffer,
            )
        };
    }
    pub fn set_enabled(&mut self, b_in_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_enabled,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_set_enabled,
                __buffer,
            )
        };
        std::mem::forget(b_in_enabled);
    }
    pub fn on_transform_changed(
        &mut self,
        new_transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_transform_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_transform,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_transform_changed,
                __buffer,
            )
        };
    }
    pub fn on_selection_changed(&mut self, b_is_selected: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_selection_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_selected,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_selection_changed,
                __buffer,
            )
        };
        std::mem::forget(b_is_selected);
    }
    pub fn on_manipulating_changed(&mut self, b_is_manipulating: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_manipulating_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_manipulating,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_manipulating_changed,
                __buffer,
            )
        };
        std::mem::forget(b_is_manipulating);
    }
    pub fn on_hovered_changed(&mut self, b_is_selected: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_hovered_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_selected,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_hovered_changed,
                __buffer,
            )
        };
        std::mem::forget(b_is_selected);
    }
    pub fn on_enabled_changed(&mut self, b_is_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_enabled_changed,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_enabled,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_on_enabled_changed,
                __buffer,
            )
        };
        std::mem::forget(b_is_enabled);
    }
    pub fn is_selected_in_editor(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_is_selected_in_editor,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_is_selected_in_editor,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_hovered(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_is_hovered,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_is_hovered,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_is_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_is_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_global_transform(&self) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_get_global_transform,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .a_control_rig_shape_actor_get_global_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
}
#[repr(C, align(16))]
pub struct UControlRigShapeLibrary {
    __padding_end: [u8; 368],
}
impl UControlRigShapeLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigShapeLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigShapeLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigOverrideAsset {
    __padding_end: [u8; 424],
}
impl UControlRigOverrideAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigOverrideAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigOverrideAsset")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigReplay {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub description: FText,
    pub control_rig_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    pub preview_skeletal_mesh_object_path: crate::bindings::core_u_object::FSoftObjectPath,
    #[doc(hidden)]
    pub(crate) __padding_896: [u8; 752],
    pub tolerance: f64,
    pub b_validate_hierarchy_topology: bool,
    pub b_validate_pose: bool,
    pub b_validate_metadata: bool,
    pub b_validate_variables: bool,
    pub frames_to_skip: TArray<i32>,
    pub enable_test: bool,
    __padding_end: [u8; 95],
}
impl UControlRigReplay {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigReplay")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigReplay")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn stop_replay(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_stop_replay,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_stop_replay,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn stop_recording(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_stop_recording,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_stop_recording,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn start_replay(
        &mut self,
        in_control_rig: UPtr<UControlRig>,
        in_mode: EControlRigReplayPlaybackMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_start_replay,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mode,
                __buffer.add(8).cast::<EControlRigReplayPlaybackMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_start_replay,
                __buffer,
            )
        };
        std::mem::forget(in_control_rig);
        std::mem::forget(in_mode);
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn start_recording(&mut self, in_control_rig: UPtr<UControlRig>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_start_recording,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_start_recording,
                __buffer,
            )
        };
        std::mem::forget(in_control_rig);
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_playback_mode(&mut self, in_mode: EControlRigReplayPlaybackMode) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_set_playback_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mode,
                __buffer.add(0).cast::<EControlRigReplayPlaybackMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_set_playback_mode,
                __buffer,
            )
        };
        std::mem::forget(in_mode);
    }
    pub fn pause_replay(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_pause_replay,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_pause_replay,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_replaying(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_is_replaying,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_is_replaying,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_recording(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_is_recording,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_is_recording,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_is_paused,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_is_paused,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_time_range(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_get_time_range,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_get_time_range,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_playback_mode(&self) -> EControlRigReplayPlaybackMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_get_playback_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_get_playback_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EControlRigReplayPlaybackMode>().read() }
    }
    pub fn create_new_asset(
        in_desired_package_path: FString,
        in_blueprint_path_name: FString,
        in_asset_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<UControlRigReplay> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_create_new_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_desired_package_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_blueprint_path_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_class,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::UControlRigReplay::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_replay_create_new_asset,
                __buffer,
            )
        };
        std::mem::forget(in_desired_package_path);
        std::mem::forget(in_blueprint_path_name);
        std::mem::forget(in_asset_class);
        unsafe { __buffer.add(40).cast::<UPtr<UControlRigReplay>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UControlRigTestData {
    #[doc(hidden)]
    pub(crate) __padding_1024: [u8; 1024],
    pub initial: FControlRigTestDataFrame,
    pub input_frames: TArray<FControlRigTestDataFrame>,
    pub output_frames: TArray<FControlRigTestDataFrame>,
    pub event_queue: TArray<FName>,
    __padding_end: [u8; 8],
}
impl UControlRigTestData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigTestData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigTestData")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_frame_index_for_time(&self, in_seconds: f64, b_input: bool) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_test_data_get_frame_index_for_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_seconds, __buffer.add(0).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_input, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_test_data_get_frame_index_for_time,
                __buffer,
            )
        };
        std::mem::forget(in_seconds);
        std::mem::forget(b_input);
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UControlRigValidator {
    __padding_end: [u8; 128],
}
impl UControlRigValidator {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigValidator")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigValidator")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigValidationPass {
    __padding_end: [u8; 48],
}
impl UControlRigValidationPass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigValidationPass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigValidationPass")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UModularRig {
    __padding_end: [u8; 5504],
}
impl UModularRig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularRig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularRig")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_parent_path(&self, in_module_path: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_parent_path_for_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_path,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_parent_path_for_bp,
                __buffer,
            )
        };
        std::mem::forget(in_module_path);
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_parent_module_name(&self, in_module_name: FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_parent_module_name_for_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_parent_module_name_for_bp,
                __buffer,
            )
        };
        std::mem::forget(in_module_name);
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_module_rig_by_name(
        &mut self,
        in_module_name: FName,
    ) -> UPtr<UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_rig_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_rig_by_name,
                __buffer,
            )
        };
        std::mem::forget(in_module_name);
        unsafe { __buffer.add(16).cast::<UPtr<UControlRig>>().read() }
    }
    pub fn get_module_rig(&mut self, in_module_path: FString) -> UPtr<UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_path,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_rig,
                __buffer,
            )
        };
        std::mem::forget(in_module_path);
        unsafe { __buffer.add(16).cast::<UPtr<UControlRig>>().read() }
    }
    pub fn get_module_paths(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_paths,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_paths,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_module_names(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_names,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_module_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_events_for_module_by_name(&self, in_module_name: FName) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_events_for_module_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_events_for_module_by_name,
                __buffer,
            )
        };
        std::mem::forget(in_module_name);
        unsafe { __buffer.add(16).cast::<TArray<FName>>().read() }
    }
    pub fn get_events_for_module(&self, in_module_path: FString) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_events_for_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_path,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_events_for_module,
                __buffer,
            )
        };
        std::mem::forget(in_module_path);
        unsafe { __buffer.add(16).cast::<TArray<FName>>().read() }
    }
    pub fn get_events_for_all_modules(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_events_for_all_modules,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_get_events_for_all_modules,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn execute_event_on_module(
        &mut self,
        in_event: FName,
        in_module_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_execute_event_on_module_for_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_event, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_path,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_execute_event_on_module_for_bp,
                __buffer,
            )
        };
        std::mem::forget(in_event);
        std::mem::forget(in_module_path);
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn execute_event_on_module_by_name(
        &mut self,
        in_event: FName,
        in_module_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_execute_event_on_module_by_name_for_bp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_event, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_execute_event_on_module_by_name_for_bp,
                __buffer,
            )
        };
        std::mem::forget(in_event);
        std::mem::forget(in_module_name);
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn execute_event_on_all_modules(&mut self, in_event: FName) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_execute_event_on_all_modules,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_event, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_execute_event_on_all_modules,
                __buffer,
            )
        };
        std::mem::forget(in_event);
        unsafe { __buffer.add(16).cast::<TArray<FName>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UModularRigController {
    __padding_end: [u8; 96],
}
impl UModularRigController {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularRigController")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularRigController")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn un_bind_module_variable(
        &mut self,
        in_module_name: &FName,
        in_variable_name: &FName,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_un_bind_module_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_un_bind_module_variable,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn swap_modules_of_class(
        &mut self,
        in_old_class: TSubclassOf<UControlRig>,
        in_new_class: TSubclassOf<UControlRig>,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_swap_modules_of_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_old_class,
                __buffer.add(0).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_class,
                __buffer.add(8).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_swap_modules_of_class,
                __buffer,
            )
        };
        std::mem::forget(in_old_class);
        std::mem::forget(in_new_class);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn swap_module_class(
        &mut self,
        in_module_name: &FName,
        in_new_class: TSubclassOf<UControlRig>,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_swap_module_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_class,
                __buffer.add(16).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_swap_module_class,
                __buffer,
            )
        };
        std::mem::forget(in_new_class);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn set_module_selection(&mut self, in_module_names: &TArray<FName>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_set_module_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_set_module_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_config_value_in_module(
        &mut self,
        in_module_name: &FName,
        in_variable_name: &FName,
        in_value: FString,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_set_config_value_in_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_set_config_value_in_module,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn select_module(&mut self, in_module_name: &FName, in_selected: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_select_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selected,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_select_module,
                __buffer,
            )
        };
        std::mem::forget(in_selected);
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn reset_config_value_in_module(
        &mut self,
        in_module_name: &FName,
        in_path: FString,
        b_clear_override: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_reset_config_value_in_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_override,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_reset_config_value_in_module,
                __buffer,
            )
        };
        std::mem::forget(in_path);
        std::mem::forget(b_clear_override);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn reparent_module(
        &mut self,
        in_module_name: &FName,
        in_new_parent_module_name: &FName,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_reparent_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_parent_module_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_reparent_module,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn reorder_module(
        &mut self,
        in_module_name: &FName,
        in_module_index: i32,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_reorder_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_index,
                __buffer.add(12).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_reorder_module,
                __buffer,
            )
        };
        std::mem::forget(in_module_index);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn rename_module(
        &mut self,
        in_module_name: &FName,
        in_new_name: &FName,
        b_setup_undo: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_rename_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_rename_module,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(28).cast::<FName>().read() }
    }
    pub fn mirror_module(
        &mut self,
        in_module_name: &FName,
        in_settings: &crate::bindings::rig_vm::FRigVMMirrorSettings,
        b_setup_undo: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_mirror_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer.add(16).cast::<crate::bindings::rig_vm::FRigVMMirrorSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_mirror_module,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(60).cast::<FName>().read() }
    }
    pub fn import_module_settings_from_string(
        &mut self,
        in_content: FString,
        in_optional_module_names: TArray<FName>,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_import_module_settings_from_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_content,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_optional_module_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_import_module_settings_from_string,
                __buffer,
            )
        };
        std::mem::forget(in_content);
        std::mem::forget(in_optional_module_names);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn get_selected_modules(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_selected_modules,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_selected_modules,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_module_reference(&self, in_module_name: FName) -> FRigModuleReference {
        let mut __stack = crate::core_data::StackAlloc::<768>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_module_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_module_reference,
                __buffer,
            )
        };
        std::mem::forget(in_module_name);
        unsafe { __buffer.add(16).cast::<FRigModuleReference>().read() }
    }
    pub fn get_connectors_for_module(
        &self,
        in_module_name: FName,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_connectors_for_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_connectors_for_module,
                __buffer,
            )
        };
        std::mem::forget(in_module_name);
        unsafe { __buffer.add(16).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_all_modules(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_all_modules,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_get_all_modules,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn export_module_settings_to_string(
        &self,
        in_module_names: TArray<FName>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_export_module_settings_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_module_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_export_module_settings_to_string,
                __buffer,
            )
        };
        std::mem::forget(in_module_names);
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn disconnect_cyclic_connectors(
        &mut self,
        b_setup_undo: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_disconnect_cyclic_connectors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_disconnect_cyclic_connectors,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn disconnect_connector(
        &mut self,
        in_connector_key: &FRigElementKey,
        b_disconnect_sub_modules: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_disconnect_connector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_connector_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_disconnect_sub_modules,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_disconnect_connector,
                __buffer,
            )
        };
        std::mem::forget(b_disconnect_sub_modules);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn deselect_module(&mut self, in_module_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_deselect_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_deselect_module,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn delete_module(&mut self, in_module_name: &FName, b_setup_undo: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_delete_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_delete_module,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn connect_connector_to_elements(
        &mut self,
        in_connector_key: &FRigElementKey,
        in_target_keys: &TArray<FRigElementKey>,
        b_setup_undo: bool,
        b_auto_resolve_other_connectors: bool,
        b_check_valid_connection: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_connect_connector_to_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_connector_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_target_keys,
                __buffer.add(16).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_resolve_other_connectors,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_check_valid_connection,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_connect_connector_to_elements,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_auto_resolve_other_connectors);
        std::mem::forget(b_check_valid_connection);
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn connect_connector_to_element(
        &mut self,
        in_connector_key: &FRigElementKey,
        in_target_key: &FRigElementKey,
        b_setup_undo: bool,
        b_auto_resolve_other_connectors: bool,
        b_check_valid_connection: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_connect_connector_to_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_connector_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_target_key,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_resolve_other_connectors,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_check_valid_connection,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_connect_connector_to_element,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_auto_resolve_other_connectors);
        std::mem::forget(b_check_valid_connection);
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn can_connect_connector_to_elements(
        &mut self,
        in_connector_key: &FRigElementKey,
        in_target_keys: &TArray<FRigElementKey>,
        out_error_message: &mut FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_can_connect_connector_to_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_connector_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_target_keys,
                __buffer.add(16).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error_message,
                __buffer.add(32).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_can_connect_connector_to_elements,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FText>().swap(out_error_message);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn can_connect_connector_to_element(
        &mut self,
        in_connector_key: &FRigElementKey,
        in_target_key: &FRigElementKey,
        out_error_message: &mut FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_can_connect_connector_to_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_connector_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_target_key,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error_message,
                __buffer.add(32).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_can_connect_connector_to_element,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FText>().swap(out_error_message);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn bind_module_variable(
        &mut self,
        in_module_name: &FName,
        in_variable_name: &FName,
        in_source_path: FString,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_bind_module_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_bind_module_variable,
                __buffer,
            )
        };
        std::mem::forget(in_source_path);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn auto_connect_secondary_connectors(
        &mut self,
        in_connector_keys: &TArray<FRigElementKey>,
        b_replace_existing_connections: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_auto_connect_secondary_connectors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_connector_keys,
                __buffer.add(0).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_connections,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_auto_connect_secondary_connectors,
                __buffer,
            )
        };
        std::mem::forget(b_replace_existing_connections);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn auto_connect_modules(
        &mut self,
        in_module_names: &TArray<FName>,
        b_replace_existing_connections: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_auto_connect_modules,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_connections,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_auto_connect_modules,
                __buffer,
            )
        };
        std::mem::forget(b_replace_existing_connections);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn add_target_to_array_connector(
        &mut self,
        in_connector_key: &FRigElementKey,
        in_target_key: &FRigElementKey,
        b_setup_undo: bool,
        b_auto_resolve_other_connectors: bool,
        b_check_valid_connection: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_add_target_to_array_connector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_connector_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_target_key,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_resolve_other_connectors,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_check_valid_connection,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_add_target_to_array_connector,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_auto_resolve_other_connectors);
        std::mem::forget(b_check_valid_connection);
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn add_module(
        &mut self,
        in_module_name: &FName,
        in_class: TSubclassOf<UControlRig>,
        in_parent_module_name: &FName,
        b_setup_undo: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_add_module,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_module_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_class,
                __buffer.add(16).cast::<TSubclassOf<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parent_module_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_modular_rig_controller_add_module,
                __buffer,
            )
        };
        std::mem::forget(in_class);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(40).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct UModularRigRuleManager {
    __padding_end: [u8; 56],
}
impl UModularRigRuleManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularRigRuleManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModularRigRuleManager")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UAdditiveControlRig {
    __padding_end: [u8; 5056],
}
impl UAdditiveControlRig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAdditiveControlRig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAdditiveControlRig")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UFKControlRig {
    __padding_end: [u8; 5104],
}
impl UFKControlRig {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFKControlRig")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFKControlRig")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigHierarchy {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub modified_event_dynamic: FRigHierarchy_ModifiedEventDynamic,
    __padding_end: [u8; 1656],
}
impl URigHierarchy {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigHierarchy")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigHierarchy")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn unset_curve_value_by_index(
        &mut self,
        in_element_index: i32,
        b_setup_undo: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_unset_curve_value_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_unset_curve_value_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_setup_undo);
    }
    pub fn unset_curve_value(&mut self, in_key: FRigElementKey, b_setup_undo: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_unset_curve_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_unset_curve_value,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_setup_undo);
    }
    pub fn switch_to_world_space(
        &mut self,
        in_child: FRigElementKey,
        b_initial: bool,
        b_affect_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_switch_to_world_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_switch_to_world_space,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn switch_to_parent(
        &mut self,
        in_child: FRigElementKey,
        in_parent: FRigElementKey,
        b_initial: bool,
        b_affect_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_switch_to_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_switch_to_parent,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_parent);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn switch_to_default_parent(
        &mut self,
        in_child: FRigElementKey,
        b_initial: bool,
        b_affect_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_switch_to_default_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_switch_to_default_parent,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn sort_keys(&self, in_keys: &TArray<FRigElementKey>) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_sort_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_keys,
                __buffer.add(0).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_sort_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn set_vector_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_vector_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_vector_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn set_vector_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<crate::bindings::core_u_object::FVector>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_vector_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer
                    .add(32)
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_vector_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_transform_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<129>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_transform_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_transform_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(128).cast::<bool>().read() }
    }
    pub fn set_transform_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<crate::bindings::core_u_object::FTransform>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_transform_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_transform_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_tag(&mut self, in_item: FRigElementKey, in_tag: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_set_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_tag, __buffer.add(16).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_set_tag,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_tag);
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn set_rotator_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rotator_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rotator_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn set_rotator_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<crate::bindings::core_u_object::FRotator>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rotator_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FRotator>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rotator_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_rig_element_key_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: FRigElementKey,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rig_element_key_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(28).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rig_element_key_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn set_rig_element_key_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<FRigElementKey>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rig_element_key_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_rig_element_key_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_quat_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: crate::bindings::core_u_object::FQuat,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_quat_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_quat_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn set_quat_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<crate::bindings::core_u_object::FQuat>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_quat_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<TArray<crate::bindings::core_u_object::FQuat>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_quat_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_pose(&mut self, in_pose: FRigPose) {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_pose_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pose,
                __buffer.add(0).cast::<FRigPose>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_pose_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(in_pose);
    }
    pub fn set_parent_weight_array(
        &mut self,
        in_child: FRigElementKey,
        in_weights: TArray<FRigElementWeight>,
        b_initial: bool,
        b_affect_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_parent_weight_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_weights,
                __buffer.add(16).cast::<TArray<FRigElementWeight>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_parent_weight_array,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_weights);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_parent_weight(
        &mut self,
        in_child: FRigElementKey,
        in_parent: FRigElementKey,
        in_weight: FRigElementWeight,
        b_initial: bool,
        b_affect_children: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<47>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_parent_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_weight,
                __buffer.add(32).cast::<FRigElementWeight>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_parent_weight,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_parent);
        std::mem::forget(in_weight);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        unsafe { __buffer.add(46).cast::<bool>().read() }
    }
    pub fn set_name_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_name_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_name_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_name_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_name_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_name_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_local_transform_by_index(
        &mut self,
        in_element_index: i32,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_affect_children: bool,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_local_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(115).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_local_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_local_transform(
        &mut self,
        in_key: FRigElementKey,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_affect_children: bool,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(115).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_local_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_linear_color_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_linear_color_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_linear_color_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn set_linear_color_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<crate::bindings::core_u_object::FLinearColor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_linear_color_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_linear_color_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_int32_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_int32_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(28).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_int32_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_int32_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<i32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_int32_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_int32_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_global_transform_by_index(
        &mut self,
        in_element_index: i32,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_affect_children: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_global_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(115).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_global_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
    }
    pub fn set_global_transform(
        &mut self,
        in_key: FRigElementKey,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_affect_children: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_global_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(115).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_global_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
    }
    pub fn set_float_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_float_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(28).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_float_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_float_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<f32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_float_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_float_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_curve_value_by_index(
        &mut self,
        in_element_index: i32,
        in_value: f32,
        b_setup_undo: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_curve_value_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_curve_value_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_value);
        std::mem::forget(b_setup_undo);
    }
    pub fn set_curve_value(
        &mut self,
        in_key: FRigElementKey,
        in_value: f32,
        b_setup_undo: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_curve_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_curve_value,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_value);
        std::mem::forget(b_setup_undo);
    }
    pub fn set_control_visibility_by_index(
        &mut self,
        in_element_index: i32,
        b_visibility: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_visibility_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visibility,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_visibility_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_visibility);
    }
    pub fn set_control_visibility(
        &mut self,
        in_key: FRigElementKey,
        b_visibility: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visibility,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_visibility,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_visibility);
    }
    pub fn set_control_value_by_index(
        &mut self,
        in_element_index: i32,
        in_value: FRigControlValue,
        in_value_type: ERigControlValueType,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<259>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_value_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<FRigControlValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value_type,
                __buffer.add(256).cast::<ERigControlValueType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(257).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(258).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_value_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_value);
        std::mem::forget(in_value_type);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_control_value(
        &mut self,
        in_key: FRigElementKey,
        in_value: FRigControlValue,
        in_value_type: ERigControlValueType,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<259>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<FRigControlValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value_type,
                __buffer.add(256).cast::<ERigControlValueType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(257).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(258).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_value);
        std::mem::forget(in_value_type);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_control_shape_transform_by_index(
        &mut self,
        in_element_index: i32,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_setup_undo: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_shape_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_shape_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_setup_undo);
    }
    pub fn set_control_shape_transform(
        &mut self,
        in_key: FRigElementKey,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_setup_undo: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_shape_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_shape_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_setup_undo);
    }
    pub fn set_control_settings_by_index(
        &mut self,
        in_element_index: i32,
        in_settings: FRigControlSettings,
        b_setup_undo: bool,
        b_force: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<787>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_settings_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FRigControlSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(784).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_force, __buffer.add(785).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(786).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_settings_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_settings);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_force);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_control_settings(
        &mut self,
        in_key: FRigElementKey,
        in_settings: FRigControlSettings,
        b_setup_undo: bool,
        b_force: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<787>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FRigControlSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(784).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_force, __buffer.add(785).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(786).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_settings,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_settings);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_force);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_control_preferred_rotator_by_index(
        &mut self,
        in_element_index: i32,
        in_rotator: &crate::bindings::core_u_object::FRotator,
        b_initial: bool,
        b_fix_euler_flips: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotator_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_rotator,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_fix_euler_flips,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotator_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        std::mem::forget(b_fix_euler_flips);
    }
    pub fn set_control_preferred_rotator(
        &mut self,
        in_key: FRigElementKey,
        in_rotator: &crate::bindings::core_u_object::FRotator,
        b_initial: bool,
        b_fix_euler_flips: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_rotator,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_fix_euler_flips,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotator,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        std::mem::forget(b_fix_euler_flips);
    }
    pub fn set_control_preferred_rotation_order_by_index(
        &mut self,
        in_element_index: i32,
        in_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotation_order_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_order,
                __buffer
                    .add(4)
                    .cast::<crate::bindings::animation_core::EEulerRotationOrder>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotation_order_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_rotation_order);
    }
    pub fn set_control_preferred_rotation_order(
        &mut self,
        in_key: FRigElementKey,
        in_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotation_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_order,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::animation_core::EEulerRotationOrder>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_rotation_order,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_rotation_order);
    }
    pub fn set_control_preferred_euler_angles_by_index(
        &mut self,
        in_element_index: i32,
        in_euler_angles: &crate::bindings::core_u_object::FVector,
        in_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
        b_initial: bool,
        b_fix_euler_flips: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_euler_angles_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_euler_angles,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_order,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::animation_core::EEulerRotationOrder>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_fix_euler_flips,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_euler_angles_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_rotation_order);
        std::mem::forget(b_initial);
        std::mem::forget(b_fix_euler_flips);
    }
    pub fn set_control_preferred_euler_angles(
        &mut self,
        in_key: FRigElementKey,
        in_euler_angles: &crate::bindings::core_u_object::FVector,
        in_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
        b_initial: bool,
        b_fix_euler_flips: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_euler_angles,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_euler_angles,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_order,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::animation_core::EEulerRotationOrder>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_fix_euler_flips,
                __buffer.add(42).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_preferred_euler_angles,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_rotation_order);
        std::mem::forget(b_initial);
        std::mem::forget(b_fix_euler_flips);
    }
    pub fn set_control_offset_transform_by_index(
        &mut self,
        in_element_index: i32,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_affect_children: bool,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_offset_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(115).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_offset_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_control_offset_transform(
        &mut self,
        in_key: FRigElementKey,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_initial: bool,
        b_affect_children: bool,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_offset_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_affect_children,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(115).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_control_offset_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_transform);
        std::mem::forget(b_initial);
        std::mem::forget(b_affect_children);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_connector_settings_by_index(
        &mut self,
        in_element_index: i32,
        in_settings: FRigConnectorSettings,
        b_setup_undo: bool,
        b_force: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<51>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_connector_settings_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(8).cast::<FRigConnectorSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_force, __buffer.add(49).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(50).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_connector_settings_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_settings);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_force);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_connector_settings(
        &mut self,
        in_key: FRigElementKey,
        in_settings: FRigConnectorSettings,
        b_setup_undo: bool,
        b_force: bool,
        b_print_python_commands: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<59>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_connector_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FRigConnectorSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_force, __buffer.add(57).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(58).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_connector_settings,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_settings);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_force);
        std::mem::forget(b_print_python_commands);
    }
    pub fn set_bool_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<30>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_bool_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(28).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_bool_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(29).cast::<bool>().read() }
    }
    pub fn set_bool_array_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        in_value: TArray<bool>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_bool_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(32).cast::<TArray<bool>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_set_bool_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(in_value);
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn send_auto_key_event(
        &mut self,
        in_element: FRigElementKey,
        in_offset_in_seconds: f32,
        b_asynchronous: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_send_auto_key_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_offset_in_seconds,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_asynchronous,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_send_auto_key_event,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(in_offset_in_seconds);
        std::mem::forget(b_asynchronous);
    }
    pub fn restore_sockets_from_states(
        &mut self,
        in_states: TArray<FRigSocketState>,
        b_setup_undo_redo: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_restore_sockets_from_states,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_states,
                __buffer.add(0).cast::<TArray<FRigSocketState>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_restore_sockets_from_states,
                __buffer,
            )
        };
        std::mem::forget(in_states);
        std::mem::forget(b_setup_undo_redo);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn restore_connectors_from_states(
        &mut self,
        in_states: TArray<FRigConnectorState>,
        b_setup_undo_redo: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_restore_connectors_from_states,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_states,
                __buffer.add(0).cast::<TArray<FRigConnectorState>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_restore_connectors_from_states,
                __buffer,
            )
        };
        std::mem::forget(in_states);
        std::mem::forget(b_setup_undo_redo);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn reset_to_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_reset_to_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_reset_to_default,
                __buffer,
            )
        };
    }
    pub fn reset_pose_to_initial(&mut self, in_type_filter: ERigElementType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_reset_pose_to_initial,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type_filter,
                __buffer.add(0).cast::<ERigElementType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_reset_pose_to_initial,
                __buffer,
            )
        };
        std::mem::forget(in_type_filter);
    }
    pub fn reset_curve_values(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_reset_curve_values,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_reset_curve_values,
                __buffer,
            )
        };
    }
    pub fn reset(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_reset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_reset,
                __buffer,
            )
        };
    }
    pub fn remove_metadata(
        &mut self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_remove_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_remove_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn remove_all_metadata(&mut self, in_item: FRigElementKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_remove_all_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_remove_all_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn num_components(&self, in_element: FRigElementKey) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_num_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_num_components,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn num(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_num,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_num,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn make_control_value_from_vector2_d(
        in_value: crate::bindings::core_u_object::FVector2D,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<256>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_vector2_d,
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
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_vector2_d,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(16).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_vector(
        in_value: crate::bindings::core_u_object::FVector,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<272>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_vector,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(32).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_transform_no_scale(
        in_value: crate::bindings::animation_core::FTransformNoScale,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_transform_no_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::animation_core::FTransformNoScale>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_transform_no_scale,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(64).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_transform(
        in_value: crate::bindings::core_u_object::FTransform,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<336>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_transform,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(96).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_rotator(
        in_value: crate::bindings::core_u_object::FRotator,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<272>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_rotator,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(32).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_int(in_value: i32) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<256>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_int,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(16).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_float(in_value: f32) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<256>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_float,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(16).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_euler_transform(
        in_value: crate::bindings::animation_core::FEulerTransform,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_euler_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::animation_core::FEulerTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_euler_transform,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(80).cast::<FRigControlValue>().read() }
    }
    pub fn make_control_value_from_bool(in_value: bool) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<256>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_make_control_value_from_bool,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(16).cast::<FRigControlValue>().read() }
    }
    pub fn is_valid_index(&self, in_element_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_valid_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_valid_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_selected_by_index(&self, in_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_selected_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_selected_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_index);
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_selected(&self, in_key: FRigElementKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_selected,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_procedural(&self, in_key: &FRigElementKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_procedural,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_procedural,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_parented_to(
        &self,
        in_child: FRigElementKey,
        in_parent: FRigElementKey,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_parented_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_parented_to,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_parent);
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn is_curve_value_set_by_index(&self, in_element_index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_curve_value_set_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_curve_value_set_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn is_curve_value_set(&self, in_key: FRigElementKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_curve_value_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_curve_value_set,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_controller_available(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_controller_available,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_controller_available,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_component_selected(&self, in_key: FRigComponentKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_component_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigComponentKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_is_component_selected,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn has_tag(&self, in_item: FRigElementKey, in_tag: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_has_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_tag, __buffer.add(16).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_has_tag,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_tag);
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn get_vector_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_vector_from_control_value(
        in_value: FRigControlValue,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<264>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe {
            __buffer.add(240).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_vector_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_vector2_d_from_control_value(
        in_value: FRigControlValue,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<256>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector2_d_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_vector2_d_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe {
            __buffer.add(240).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_transform_no_scale_from_control_value(
        in_value: FRigControlValue,
    ) -> crate::bindings::animation_core::FTransformNoScale {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_no_scale_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_no_scale_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe {
            __buffer
                .add(240)
                .cast::<crate::bindings::animation_core::FTransformNoScale>()
                .read()
        }
    }
    pub fn get_transform_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: crate::bindings::core_u_object::FTransform,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe {
            __buffer.add(128).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_transform_from_control_value(
        in_value: FRigControlValue,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<336>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe {
            __buffer.add(240).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_transform_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_transform_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_tags(&self, in_item: FRigElementKey) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_tags,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<TArray<FName>>().read() }
    }
    pub fn get_socket_states(&self) -> TArray<FRigSocketState> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_socket_states,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_socket_states,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FRigSocketState>>().read() }
    }
    pub fn get_sockets(&self, b_traverse: bool) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_socket_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_traverse,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_socket_keys,
                __buffer,
            )
        };
        std::mem::forget(b_traverse);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_selected_keys(
        &self,
        in_type_filter: ERigElementType,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_selected_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type_filter,
                __buffer.add(0).cast::<ERigElementType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_selected_keys,
                __buffer,
            )
        };
        std::mem::forget(in_type_filter);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_selected_hierarchy_keys(&self) -> TArray<FRigHierarchyKey> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_selected_hierarchy_keys_for_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_selected_hierarchy_keys_for_blueprint,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FRigHierarchyKey>>().read() }
    }
    pub fn get_rule_manager(
        &mut self,
        b_create_if_needed: bool,
    ) -> UPtr<UModularRigRuleManager> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rule_manager,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_if_needed,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rule_manager,
                __buffer,
            )
        };
        std::mem::forget(b_create_if_needed);
        unsafe { __buffer.add(8).cast::<UPtr<UModularRigRuleManager>>().read() }
    }
    pub fn get_rotator_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: crate::bindings::core_u_object::FRotator,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rotator_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rotator_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe {
            __buffer.add(56).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_rotator_from_control_value(
        in_value: FRigControlValue,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<264>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rotator_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rotator_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe {
            __buffer.add(240).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_rotator_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FRotator> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rotator_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rotator_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FRotator>>()
                .read()
        }
    }
    pub fn get_root_elements(&self) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_root_element_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_root_element_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_rig_element_key_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: FRigElementKey,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rig_element_key_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(28).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rig_element_key_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe { __buffer.add(44).cast::<FRigElementKey>().read() }
    }
    pub fn get_rig_element_key_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rig_element_key_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_rig_element_key_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe { __buffer.add(32).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_references(&self, b_traverse: bool) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_reference_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_traverse,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_reference_keys,
                __buffer,
            )
        };
        std::mem::forget(b_traverse);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_quat_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: crate::bindings::core_u_object::FQuat,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_quat_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_quat_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe {
            __buffer.add(64).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_quat_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FQuat> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_quat_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_quat_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FQuat>>()
                .read()
        }
    }
    pub fn get_previous_parent(&self, in_key: &FRigElementKey) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_parent,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FRigElementKey>().read() }
    }
    pub fn get_previous_name(&self, in_key: &FRigElementKey) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn get_previous_hierarchy_parent(
        &self,
        in_key: &FRigHierarchyKey,
    ) -> FRigHierarchyKey {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_hierarchy_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigHierarchyKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_hierarchy_parent,
                __buffer,
            )
        };
        unsafe { __buffer.add(52).cast::<FRigHierarchyKey>().read() }
    }
    pub fn get_previous_hierarchy_name(&self, in_key: &FRigHierarchyKey) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_hierarchy_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigHierarchyKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_previous_hierarchy_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(52).cast::<FName>().read() }
    }
    pub fn get_pose(
        &self,
        b_initial: bool,
        b_include_transient_controls: bool,
    ) -> FRigPose {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_transient_controls,
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
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_pose,
                __buffer,
            )
        };
        std::mem::forget(b_initial);
        std::mem::forget(b_include_transient_controls);
        unsafe { __buffer.add(8).cast::<FRigPose>().read() }
    }
    pub fn get_parent_weight_array(
        &self,
        in_child: FRigElementKey,
        b_initial: bool,
    ) -> TArray<FRigElementWeight> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_weight_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_weight_array,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(b_initial);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementWeight>>().read() }
    }
    pub fn get_parent_weight(
        &self,
        in_child: FRigElementKey,
        in_parent: FRigElementKey,
        b_initial: bool,
    ) -> FRigElementWeight {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_weight,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_parent);
        std::mem::forget(b_initial);
        unsafe { __buffer.add(36).cast::<FRigElementWeight>().read() }
    }
    pub fn get_parent_transform_by_index(
        &self,
        in_element_index: i32,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_parent_transform(
        &self,
        in_key: FRigElementKey,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parent_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_parents(
        &self,
        in_key: FRigElementKey,
        b_recursive: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_parents,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_recursive);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_number_of_parents(&self, in_key: FRigElementKey) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_number_of_parents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_number_of_parents,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_nulls(&self, b_traverse: bool) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_null_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_traverse,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_null_keys,
                __buffer,
            )
        };
        std::mem::forget(b_traverse);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_name_space_f_name(&self, in_item: FRigElementKey) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_space_f_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_space_f_name,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn get_name_space(&self, in_item: FRigElementKey) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_space,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_name_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: FName,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe { __buffer.add(40).cast::<FName>().read() }
    }
    pub fn get_name_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_name_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe { __buffer.add(32).cast::<TArray<FName>>().read() }
    }
    pub fn get_module_prefix(&self, in_item: FRigElementKey) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_prefix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_prefix,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_module_path_f_name(&self, in_item: FRigElementKey) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_path_f_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_path_f_name,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn get_module_path(&self, in_item: FRigElementKey) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_path,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_module_name(&self, in_item: FRigElementKey) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_name,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_module_f_name(&self, in_item: FRigElementKey) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_f_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_module_f_name,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn get_metadata_type(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> ERigMetadataType {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_metadata_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_metadata_type,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe { __buffer.add(28).cast::<ERigMetadataType>().read() }
    }
    pub fn get_metadata_names(&self, in_item: FRigElementKey) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_metadata_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_metadata_names,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        unsafe { __buffer.add(16).cast::<TArray<FName>>().read() }
    }
    pub fn get_local_transform_by_index(
        &self,
        in_element_index: i32,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_local_transform(
        &self,
        in_key: FRigElementKey,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_local_index(&self, in_key: FRigElementKey) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_index_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_index_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_local_control_shape_transform_by_index(
        &self,
        in_element_index: i32,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_control_shape_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_control_shape_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_local_control_shape_transform(
        &self,
        in_key: FRigElementKey,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_control_shape_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_local_control_shape_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_linear_color_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: crate::bindings::core_u_object::FLinearColor,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_linear_color_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_linear_color_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe {
            __buffer
                .add(44)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn get_linear_color_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FLinearColor> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_linear_color_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_linear_color_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>()
                .read()
        }
    }
    pub fn get_keys(&self, in_element_indices: TArray<i32>) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_indices,
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
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_keys,
                __buffer,
            )
        };
        std::mem::forget(in_element_indices);
        unsafe { __buffer.add(16).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_key(&self, in_element_index: i32) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_get_key,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        unsafe { __buffer.add(4).cast::<FRigElementKey>().read() }
    }
    pub fn get_int_from_control_value(in_value: FRigControlValue) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<244>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_int_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_int_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(240).cast::<i32>().read() }
    }
    pub fn get_int32_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_int32_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(28).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_int32_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn get_int32_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_int32_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_int32_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe { __buffer.add(32).cast::<TArray<i32>>().read() }
    }
    pub fn get_index(&self, in_key: FRigElementKey) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_index_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_index_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_global_transform_by_index(
        &self,
        in_element_index: i32,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_global_transform(
        &self,
        in_key: FRigElementKey,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_global_control_shape_transform_by_index(
        &self,
        in_element_index: i32,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_shape_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_shape_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_global_control_shape_transform(
        &self,
        in_key: FRigElementKey,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_shape_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_shape_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_global_control_offset_transform_by_index(
        &self,
        in_element_index: i32,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_offset_transform_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_offset_transform_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_global_control_offset_transform(
        &self,
        in_key: FRigElementKey,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_offset_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_global_control_offset_transform,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_float_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_float_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(28).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_float_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe { __buffer.add(32).cast::<f32>().read() }
    }
    pub fn get_float_from_control_value(in_value: FRigControlValue) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<244>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_float_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_float_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe { __buffer.add(240).cast::<f32>().read() }
    }
    pub fn get_float_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_float_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_float_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe { __buffer.add(32).cast::<TArray<f32>>().read() }
    }
    pub fn get_first_parent(&self, in_key: FRigElementKey) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_first_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_first_parent,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<FRigElementKey>().read() }
    }
    pub fn get_euler_transform_from_control_value(
        in_value: FRigControlValue,
    ) -> crate::bindings::animation_core::FEulerTransform {
        let mut __stack = crate::core_data::StackAlloc::<312>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_euler_transform_from_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(0).cast::<FRigControlValue>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig::URigHierarchy::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_euler_transform_from_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_value);
        unsafe {
            __buffer
                .add(240)
                .cast::<crate::bindings::animation_core::FEulerTransform>()
                .read()
        }
    }
    pub fn get_default_parent(&self, in_key: FRigElementKey) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_default_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_default_parent,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<FRigElementKey>().read() }
    }
    pub fn get_curve_value_by_index(&self, in_element_index: i32) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_curve_value_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_curve_value_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        unsafe { __buffer.add(4).cast::<f32>().read() }
    }
    pub fn get_curve_value(&self, in_key: FRigElementKey) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_curve_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_curve_value,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_curves(&self) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_curve_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_curve_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_control_value_by_index(
        &self,
        in_element_index: i32,
        in_value_type: ERigControlValueType,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<256>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_value_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value_type,
                __buffer.add(4).cast::<ERigControlValueType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_value_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_value_type);
        unsafe { __buffer.add(16).cast::<FRigControlValue>().read() }
    }
    pub fn get_control_value(
        &self,
        in_key: FRigElementKey,
        in_value_type: ERigControlValueType,
    ) -> FRigControlValue {
        let mut __stack = crate::core_data::StackAlloc::<272>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value_type,
                __buffer.add(16).cast::<ERigControlValueType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_value,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_value_type);
        unsafe { __buffer.add(32).cast::<FRigControlValue>().read() }
    }
    pub fn get_control_preferred_rotator_by_index(
        &self,
        in_element_index: i32,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_rotator_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(4).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_rotator_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_control_preferred_rotator(
        &self,
        in_key: FRigElementKey,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_rotator,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_control_preferred_euler_rotation_order_by_index(
        &self,
        in_element_index: i32,
        b_from_settings: bool,
    ) -> crate::bindings::animation_core::EEulerRotationOrder {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_rotation_order_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_from_settings,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_rotation_order_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(b_from_settings);
        unsafe {
            __buffer
                .add(5)
                .cast::<crate::bindings::animation_core::EEulerRotationOrder>()
                .read()
        }
    }
    pub fn get_control_preferred_euler_rotation_order(
        &self,
        in_key: FRigElementKey,
        b_from_settings: bool,
    ) -> crate::bindings::animation_core::EEulerRotationOrder {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_rotation_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_from_settings,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_rotation_order,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_from_settings);
        unsafe {
            __buffer
                .add(17)
                .cast::<crate::bindings::animation_core::EEulerRotationOrder>()
                .read()
        }
    }
    pub fn get_control_preferred_euler_angles_by_index(
        &self,
        in_element_index: i32,
        in_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_angles_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_order,
                __buffer
                    .add(4)
                    .cast::<crate::bindings::animation_core::EEulerRotationOrder>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(5).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_angles_by_index,
                __buffer,
            )
        };
        std::mem::forget(in_element_index);
        std::mem::forget(in_rotation_order);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_control_preferred_euler_angles(
        &self,
        in_key: FRigElementKey,
        in_rotation_order: crate::bindings::animation_core::EEulerRotationOrder,
        b_initial: bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_angles,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rotation_order,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::animation_core::EEulerRotationOrder>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_initial,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_preferred_euler_angles,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_rotation_order);
        std::mem::forget(b_initial);
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_controller(
        &mut self,
        b_create_if_needed: bool,
    ) -> UPtr<URigHierarchyController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_if_needed,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_controller,
                __buffer,
            )
        };
        std::mem::forget(b_create_if_needed);
        unsafe { __buffer.add(8).cast::<UPtr<URigHierarchyController>>().read() }
    }
    pub fn get_controls(&self, b_traverse: bool) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_traverse,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_control_keys,
                __buffer,
            )
        };
        std::mem::forget(b_traverse);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_connector_states(&self) -> TArray<FRigConnectorState> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_connector_states,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_connector_states,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FRigConnectorState>>().read() }
    }
    pub fn get_connectors(&self, b_traverse: bool) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_connector_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_traverse,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_connector_keys,
                __buffer,
            )
        };
        std::mem::forget(b_traverse);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_component_type(
        &self,
        in_element: FRigElementKey,
        in_component_index: i32,
    ) -> UPtr<crate::bindings::core_u_object::UScriptStruct> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_type,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(in_component_index);
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>()
                .read()
        }
    }
    pub fn get_component_name(
        &self,
        in_element: FRigElementKey,
        in_component_index: i32,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_name,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(in_component_index);
        unsafe { __buffer.add(20).cast::<FName>().read() }
    }
    pub fn get_component_keys(
        &self,
        in_element: FRigElementKey,
    ) -> TArray<FRigComponentKey> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_keys,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        unsafe { __buffer.add(16).cast::<TArray<FRigComponentKey>>().read() }
    }
    pub fn get_component_key(
        &self,
        in_element: FRigElementKey,
        in_component_index: i32,
    ) -> FRigComponentKey {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_key,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(in_component_index);
        unsafe { __buffer.add(20).cast::<FRigComponentKey>().read() }
    }
    pub fn get_component_content(
        &self,
        in_element: FRigElementKey,
        in_component_index: i32,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_content,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_index,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_component_content,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(in_component_index);
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_children(
        &self,
        in_key: FRigElementKey,
        b_recursive: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_children,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_children,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_recursive);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_bool_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
        default_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<30>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_bool_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_bool_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        std::mem::forget(default_value);
        unsafe { __buffer.add(29).cast::<bool>().read() }
    }
    pub fn get_bool_array_metadata(
        &self,
        in_item: FRigElementKey,
        in_metadata_name: FName,
    ) -> TArray<bool> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_bool_array_metadata,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_item,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_metadata_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_bool_array_metadata,
                __buffer,
            )
        };
        std::mem::forget(in_item);
        std::mem::forget(in_metadata_name);
        unsafe { __buffer.add(32).cast::<TArray<bool>>().read() }
    }
    pub fn get_bones(&self, b_traverse: bool) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_bone_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_traverse,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_bone_keys,
                __buffer,
            )
        };
        std::mem::forget(b_traverse);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_all_keys(&self, b_traverse: bool) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_all_keys_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_traverse,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_all_keys_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(b_traverse);
        unsafe { __buffer.add(8).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_all_component_keys(&self) -> TArray<FRigComponentKey> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_all_component_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_get_all_component_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FRigComponentKey>>().read() }
    }
    pub fn find_null(&self, in_key: &FRigElementKey) -> FRigNullElement {
        let mut __stack = crate::core_data::StackAlloc::<576>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_find_null_for_blueprint_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_find_null_for_blueprint_only,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FRigNullElement>().read() }
    }
    pub fn find_control(&self, in_key: &FRigElementKey) -> FRigControlElement {
        let mut __stack = crate::core_data::StackAlloc::<1664>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_find_control_for_blueprint_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_find_control_for_blueprint_only,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FRigControlElement>().read() }
    }
    pub fn find_bone(&self, in_key: &FRigElementKey) -> FRigBoneElement {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_find_bone_for_blueprint_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_find_bone_for_blueprint_only,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FRigBoneElement>().read() }
    }
    pub fn copy_pose(
        &mut self,
        in_hierarchy: UPtr<URigHierarchy>,
        b_current: bool,
        b_initial: bool,
        b_weights: bool,
        b_match_pose_in_global_if_needed: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_copy_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_hierarchy,
                __buffer.add(0).cast::<UPtr<URigHierarchy>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_current, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_initial, __buffer.add(9).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_weights,
                __buffer.add(10).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_match_pose_in_global_if_needed,
                __buffer.add(11).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS.u_rig_hierarchy_copy_pose,
                __buffer,
            )
        };
        std::mem::forget(in_hierarchy);
        std::mem::forget(b_current);
        std::mem::forget(b_initial);
        std::mem::forget(b_weights);
        std::mem::forget(b_match_pose_in_global_if_needed);
    }
    pub fn copy_hierarchy(&mut self, in_hierarchy: UPtr<URigHierarchy>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_copy_hierarchy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_hierarchy,
                __buffer.add(0).cast::<UPtr<URigHierarchy>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_copy_hierarchy,
                __buffer,
            )
        };
        std::mem::forget(in_hierarchy);
    }
    pub fn contains(&self, in_key: FRigElementKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_contains_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_contains_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
pub struct IRigHierarchyProvider {}
#[repr(C, align(8))]
pub struct URigHierarchyProvider {
    __padding_end: [u8; 48],
}
impl URigHierarchyProvider {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigHierarchyProvider")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigHierarchyProvider")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigHierarchyController {
    __padding_end: [u8; 144],
}
impl URigHierarchyController {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigHierarchyController")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigHierarchyController")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_selection(
        &mut self,
        in_keys: &TArray<FRigElementKey>,
        b_print_python_command: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_keys,
                __buffer.add(0).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_selection,
                __buffer,
            )
        };
        std::mem::forget(b_print_python_command);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_parent(
        &mut self,
        in_child: FRigElementKey,
        in_parent: FRigElementKey,
        b_maintain_global_transform: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_global_transform,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_parent,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_parent);
        std::mem::forget(b_maintain_global_transform);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn set_hierarchy_selection(
        &mut self,
        in_keys: &TArray<FRigHierarchyKey>,
        b_print_python_command: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_hierarchy_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_keys,
                __buffer.add(0).cast::<TArray<FRigHierarchyKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_hierarchy_selection,
                __buffer,
            )
        };
        std::mem::forget(b_print_python_command);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_hierarchy(&mut self, in_hierarchy: UPtr<URigHierarchy>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_hierarchy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_hierarchy,
                __buffer.add(0).cast::<UPtr<URigHierarchy>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_hierarchy,
                __buffer,
            )
        };
        std::mem::forget(in_hierarchy);
    }
    pub fn set_display_name(
        &mut self,
        in_control: FRigElementKey,
        in_display_name: FName,
        b_rename_element: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_rename_element,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_display_name,
                __buffer,
            )
        };
        std::mem::forget(in_control);
        std::mem::forget(in_display_name);
        std::mem::forget(b_rename_element);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(32).cast::<FName>().read() }
    }
    pub fn set_control_settings(
        &self,
        in_key: FRigElementKey,
        in_settings: FRigControlSettings,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<786>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_control_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FRigControlSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(784).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_control_settings,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(in_settings);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(785).cast::<bool>().read() }
    }
    pub fn set_component_selection(
        &mut self,
        in_keys: &TArray<FRigComponentKey>,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_component_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_keys,
                __buffer.add(0).cast::<TArray<FRigComponentKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_component_selection,
                __buffer,
            )
        };
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn set_component_content(
        &mut self,
        in_component: FRigComponentKey,
        in_content: FString,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<51>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_component_content,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component,
                __buffer.add(0).cast::<FRigComponentKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_content,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_component_content,
                __buffer,
            )
        };
        std::mem::forget(in_component);
        std::mem::forget(in_content);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(50).cast::<bool>().read() }
    }
    pub fn set_available_space_label(
        &mut self,
        in_control: FRigElementKey,
        in_space: FRigElementKey,
        in_display_label: FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<47>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_available_space_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_space,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_label,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_available_space_label,
                __buffer,
            )
        };
        std::mem::forget(in_control);
        std::mem::forget(in_space);
        std::mem::forget(in_display_label);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(46).cast::<bool>().read() }
    }
    pub fn set_available_space_index(
        &mut self,
        in_control: FRigElementKey,
        in_space: FRigElementKey,
        in_index: i32,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<39>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_available_space_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_space,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(32).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_set_available_space_index,
                __buffer,
            )
        };
        std::mem::forget(in_control);
        std::mem::forget(in_space);
        std::mem::forget(in_index);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(38).cast::<bool>().read() }
    }
    pub fn select_hierarchy_key(
        &mut self,
        in_key: FRigHierarchyKey,
        b_select: bool,
        b_clear_selection: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_select_hierarchy_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigHierarchyKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(52).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(53).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(54).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_select_hierarchy_key,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_select);
        std::mem::forget(b_clear_selection);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(55).cast::<bool>().read() }
    }
    pub fn select_element(
        &mut self,
        in_key: FRigElementKey,
        b_select: bool,
        b_clear_selection: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_select_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(16).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_select_element,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_select);
        std::mem::forget(b_clear_selection);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn select_component(
        &mut self,
        in_key: FRigComponentKey,
        b_select: bool,
        b_clear_selection: bool,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_select_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigComponentKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(28).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_select_component,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_select);
        std::mem::forget(b_clear_selection);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(31).cast::<bool>().read() }
    }
    pub fn reparent_component(
        &mut self,
        in_component_key: FRigComponentKey,
        in_parent_element_key: FRigElementKey,
        b_setup_undo: bool,
        b_print_python_command: bool,
        b_clear_selection: bool,
    ) -> FRigComponentKey {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_reparent_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_key,
                __buffer.add(0).cast::<FRigComponentKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_element_key,
                __buffer.add(28).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(46).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_reparent_component,
                __buffer,
            )
        };
        std::mem::forget(in_component_key);
        std::mem::forget(in_parent_element_key);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        std::mem::forget(b_clear_selection);
        unsafe { __buffer.add(48).cast::<FRigComponentKey>().read() }
    }
    pub fn reorder_element(
        &mut self,
        in_element: FRigElementKey,
        in_index: i32,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<23>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_reorder_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_reorder_element,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(in_index);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(22).cast::<bool>().read() }
    }
    pub fn rename_element(
        &mut self,
        in_element: FRigElementKey,
        in_name: FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
        b_clear_selection: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_rename_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(16).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_rename_element,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(in_name);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        std::mem::forget(b_clear_selection);
        unsafe { __buffer.add(32).cast::<FRigElementKey>().read() }
    }
    pub fn rename_component(
        &mut self,
        in_component: FRigComponentKey,
        in_name: FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
        b_clear_selection: bool,
    ) -> FRigComponentKey {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_rename_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component,
                __buffer.add(0).cast::<FRigComponentKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(28).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(42).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_rename_component,
                __buffer,
            )
        };
        std::mem::forget(in_component);
        std::mem::forget(in_name);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        std::mem::forget(b_clear_selection);
        unsafe { __buffer.add(44).cast::<FRigComponentKey>().read() }
    }
    pub fn remove_parent(
        &mut self,
        in_child: FRigElementKey,
        in_parent: FRigElementKey,
        b_maintain_global_transform: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_global_transform,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_parent,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_parent);
        std::mem::forget(b_maintain_global_transform);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn remove_element(
        &mut self,
        in_element: FRigElementKey,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_element,
                __buffer,
            )
        };
        std::mem::forget(in_element);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_component(
        &mut self,
        in_component: FRigComponentKey,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<31>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component,
                __buffer.add(0).cast::<FRigComponentKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_component,
                __buffer,
            )
        };
        std::mem::forget(in_component);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(30).cast::<bool>().read() }
    }
    pub fn remove_channel_host(
        &mut self,
        in_channel: FRigElementKey,
        in_host: FRigElementKey,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_channel_host,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_channel,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_channel_host,
                __buffer,
            )
        };
        std::mem::forget(in_channel);
        std::mem::forget(in_host);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn remove_available_space(
        &mut self,
        in_control: FRigElementKey,
        in_space: FRigElementKey,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_available_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_space,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_available_space,
                __buffer,
            )
        };
        std::mem::forget(in_control);
        std::mem::forget(in_space);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn remove_all_parents(
        &mut self,
        in_child: FRigElementKey,
        b_maintain_global_transform: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_all_parents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_global_transform,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_remove_all_parents,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(b_maintain_global_transform);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn mirror_elements(
        &mut self,
        in_keys: TArray<FRigElementKey>,
        in_settings: crate::bindings::rig_vm::FRigVMMirrorSettings,
        b_select_new_elements: bool,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_mirror_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keys,
                __buffer.add(0).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<crate::bindings::rig_vm::FRigVMMirrorSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_new_elements,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(58).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_mirror_elements,
                __buffer,
            )
        };
        std::mem::forget(in_keys);
        std::mem::forget(in_settings);
        std::mem::forget(b_select_new_elements);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
        unsafe { __buffer.add(64).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_sockets_from_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        in_name_space: &FName,
        b_replace_existing_sockets: bool,
        b_remove_obsolete_sockets: bool,
        b_select_sockets: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_sockets_from_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_name_space,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_sockets,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_obsolete_sockets,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_sockets,
                __buffer.add(22).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(23).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_sockets_from_skeletal_mesh,
                __buffer,
            )
        };
        std::mem::forget(in_skeletal_mesh);
        std::mem::forget(b_replace_existing_sockets);
        std::mem::forget(b_remove_obsolete_sockets);
        std::mem::forget(b_select_sockets);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(32).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_preview_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        b_replace_existing_bones: bool,
        b_remove_obsolete_bones: bool,
        b_select_bones: bool,
        b_setup_undo: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_preview_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_bones,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_obsolete_bones,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_bones,
                __buffer.add(10).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(11).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_preview_skeletal_mesh,
                __buffer,
            )
        };
        std::mem::forget(in_skeletal_mesh);
        std::mem::forget(b_replace_existing_bones);
        std::mem::forget(b_remove_obsolete_bones);
        std::mem::forget(b_select_bones);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(16).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_from_text(
        &mut self,
        in_content: FString,
        b_replace_existing_elements: bool,
        b_select_new_elements: bool,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_from_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_content,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_elements,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_new_elements,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(19).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_from_text,
                __buffer,
            )
        };
        std::mem::forget(in_content);
        std::mem::forget(b_replace_existing_elements);
        std::mem::forget(b_select_new_elements);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_curves_from_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        in_name_space: FName,
        b_select_curves: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves_from_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name_space,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_curves,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(22).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves_from_skeletal_mesh,
                __buffer,
            )
        };
        std::mem::forget(in_skeletal_mesh);
        std::mem::forget(in_name_space);
        std::mem::forget(b_select_curves);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_curves_from_asset(
        &mut self,
        in_asset_path: FString,
        in_name_space: FName,
        b_select_curves: bool,
        b_setup_undo: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves_from_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name_space,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_curves,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves_from_asset,
                __buffer,
            )
        };
        std::mem::forget(in_asset_path);
        std::mem::forget(in_name_space);
        std::mem::forget(b_select_curves);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(32).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_curves(
        &mut self,
        in_skeleton: UPtr<crate::bindings::engine::USkeleton>,
        in_name_space: FName,
        b_select_curves: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name_space,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_curves,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(22).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_curves,
                __buffer,
            )
        };
        std::mem::forget(in_skeleton);
        std::mem::forget(in_name_space);
        std::mem::forget(b_select_curves);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_bones_from_skeletal_mesh(
        &mut self,
        in_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        in_name_space: &FName,
        b_replace_existing_bones: bool,
        b_remove_obsolete_bones: bool,
        b_select_bones: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones_from_skeletal_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeletal_mesh,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_name_space,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_bones,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_obsolete_bones,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_bones,
                __buffer.add(22).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(23).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones_from_skeletal_mesh,
                __buffer,
            )
        };
        std::mem::forget(in_skeletal_mesh);
        std::mem::forget(b_replace_existing_bones);
        std::mem::forget(b_remove_obsolete_bones);
        std::mem::forget(b_select_bones);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(32).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_bones_from_asset(
        &mut self,
        in_asset_path: FString,
        in_name_space: FName,
        b_replace_existing_bones: bool,
        b_remove_obsolete_bones: bool,
        b_select_bones: bool,
        b_setup_undo: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones_from_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name_space,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_bones,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_obsolete_bones,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_bones,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(31).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones_from_asset,
                __buffer,
            )
        };
        std::mem::forget(in_asset_path);
        std::mem::forget(in_name_space);
        std::mem::forget(b_replace_existing_bones);
        std::mem::forget(b_remove_obsolete_bones);
        std::mem::forget(b_select_bones);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(32).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn import_bones(
        &mut self,
        in_skeleton: UPtr<crate::bindings::engine::USkeleton>,
        in_name_space: FName,
        b_replace_existing_bones: bool,
        b_remove_obsolete_bones: bool,
        b_select_bones: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name_space,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_replace_existing_bones,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_obsolete_bones,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_bones,
                __buffer.add(22).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(23).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_import_bones,
                __buffer,
            )
        };
        std::mem::forget(in_skeleton);
        std::mem::forget(in_name_space);
        std::mem::forget(b_replace_existing_bones);
        std::mem::forget(b_remove_obsolete_bones);
        std::mem::forget(b_select_bones);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(32).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn get_hierarchy(&self) -> UPtr<URigHierarchy> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_get_hierarchy,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_get_hierarchy,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigHierarchy>>().read() }
    }
    pub fn get_control_settings(&self, in_key: FRigElementKey) -> FRigControlSettings {
        let mut __stack = crate::core_data::StackAlloc::<784>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_get_control_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_get_control_settings,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<FRigControlSettings>().read() }
    }
    pub fn generate_python_commands(&mut self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_generate_python_commands,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_generate_python_commands,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn export_to_text(&self, in_keys: TArray<FRigElementKey>) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_export_to_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keys,
                __buffer.add(0).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_export_to_text,
                __buffer,
            )
        };
        std::mem::forget(in_keys);
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn export_selection_to_text(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_export_selection_to_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_export_selection_to_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn duplicate_elements(
        &mut self,
        in_keys: TArray<FRigElementKey>,
        b_select_new_elements: bool,
        b_setup_undo: bool,
        b_print_python_commands: bool,
    ) -> TArray<FRigElementKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_duplicate_elements,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keys,
                __buffer.add(0).cast::<TArray<FRigElementKey>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_new_elements,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_duplicate_elements,
                __buffer,
            )
        };
        std::mem::forget(in_keys);
        std::mem::forget(b_select_new_elements);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_commands);
        unsafe { __buffer.add(24).cast::<TArray<FRigElementKey>>().read() }
    }
    pub fn deselect_hierarchy_key(
        &mut self,
        in_key: FRigHierarchyKey,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<54>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_deselect_hierarchy_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigHierarchyKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(52).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_deselect_hierarchy_key,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(53).cast::<bool>().read() }
    }
    pub fn deselect_element(&mut self, in_key: FRigElementKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_deselect_element,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_deselect_element,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn deselect_component(&mut self, in_key: FRigComponentKey) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_deselect_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_key,
                __buffer.add(0).cast::<FRigComponentKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_deselect_component,
                __buffer,
            )
        };
        std::mem::forget(in_key);
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn clear_selection(&mut self, b_setup_undo: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_clear_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_clear_selection,
                __buffer,
            )
        };
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn add_socket(
        &mut self,
        in_name: FName,
        in_parent: FRigElementKey,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_transform_in_global: bool,
        in_color: &crate::bindings::core_u_object::FLinearColor,
        in_description: FString,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<188>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_socket,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(12).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transform_in_global,
                __buffer.add(128).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(132).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(152).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(168).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(169).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_socket,
                __buffer,
            )
        };
        std::mem::forget(in_name);
        std::mem::forget(in_parent);
        std::mem::forget(in_transform);
        std::mem::forget(b_transform_in_global);
        std::mem::forget(in_description);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(172).cast::<FRigElementKey>().read() }
    }
    pub fn add_parent(
        &mut self,
        in_child: FRigElementKey,
        in_parent: FRigElementKey,
        in_weight: f32,
        b_maintain_global_transform: bool,
        in_display_label: FName,
        b_setup_undo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<54>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_weight, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_global_transform,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_label,
                __buffer.add(40).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(52).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_parent,
                __buffer,
            )
        };
        std::mem::forget(in_child);
        std::mem::forget(in_parent);
        std::mem::forget(in_weight);
        std::mem::forget(b_maintain_global_transform);
        std::mem::forget(in_display_label);
        std::mem::forget(b_setup_undo);
        unsafe { __buffer.add(53).cast::<bool>().read() }
    }
    pub fn add_null(
        &mut self,
        in_name: FName,
        in_parent: FRigElementKey,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_transform_in_global: bool,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<148>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_null,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(12).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transform_in_global,
                __buffer.add(128).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(129).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(130).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_null,
                __buffer,
            )
        };
        std::mem::forget(in_name);
        std::mem::forget(in_parent);
        std::mem::forget(in_transform);
        std::mem::forget(b_transform_in_global);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(132).cast::<FRigElementKey>().read() }
    }
    pub fn add_curve(
        &mut self,
        in_name: FName,
        in_value: f32,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(17).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_curve,
                __buffer,
            )
        };
        std::mem::forget(in_name);
        std::mem::forget(in_value);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(20).cast::<FRigElementKey>().read() }
    }
    pub fn add_control(
        &mut self,
        in_name: FName,
        in_parent: FRigElementKey,
        in_settings: FRigControlSettings,
        in_value: FRigControlValue,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<1060>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_control_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(12).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(32).cast::<FRigControlSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(800).cast::<FRigControlValue>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(1040).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(1041).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_control_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(in_name);
        std::mem::forget(in_parent);
        std::mem::forget(in_settings);
        std::mem::forget(in_value);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(1044).cast::<FRigElementKey>().read() }
    }
    pub fn add_connector(
        &mut self,
        in_name: FName,
        in_settings: FRigConnectorSettings,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_connector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(16).cast::<FRigConnectorSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_connector,
                __buffer,
            )
        };
        std::mem::forget(in_name);
        std::mem::forget(in_settings);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(60).cast::<FRigElementKey>().read() }
    }
    pub fn add_component(
        &mut self,
        in_component_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_name: FName,
        in_element: FRigElementKey,
        in_content: FString,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigComponentKey {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_component_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_element,
                __buffer.add(20).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_content,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_component,
                __buffer,
            )
        };
        std::mem::forget(in_component_struct);
        std::mem::forget(in_name);
        std::mem::forget(in_element);
        std::mem::forget(in_content);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(60).cast::<FRigComponentKey>().read() }
    }
    pub fn add_channel_host(
        &mut self,
        in_channel: FRigElementKey,
        in_host: FRigElementKey,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_channel_host,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_channel,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_channel_host,
                __buffer,
            )
        };
        std::mem::forget(in_channel);
        std::mem::forget(in_host);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn add_bone(
        &mut self,
        in_name: FName,
        in_parent: FRigElementKey,
        in_transform: crate::bindings::core_u_object::FTransform,
        b_transform_in_global: bool,
        in_bone_type: ERigBoneType,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<148>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer.add(12).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transform_in_global,
                __buffer.add(128).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_bone_type,
                __buffer.add(129).cast::<ERigBoneType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(130).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(131).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_bone,
                __buffer,
            )
        };
        std::mem::forget(in_name);
        std::mem::forget(in_parent);
        std::mem::forget(in_transform);
        std::mem::forget(b_transform_in_global);
        std::mem::forget(in_bone_type);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(132).cast::<FRigElementKey>().read() }
    }
    pub fn add_available_space(
        &mut self,
        in_control: FRigElementKey,
        in_space: FRigElementKey,
        in_display_label: FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<47>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_available_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control,
                __buffer.add(0).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_space,
                __buffer.add(16).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_label,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_available_space,
                __buffer,
            )
        };
        std::mem::forget(in_control);
        std::mem::forget(in_space);
        std::mem::forget(in_display_label);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(46).cast::<bool>().read() }
    }
    pub fn add_animation_channel(
        &mut self,
        in_name: FName,
        in_parent_control: FRigElementKey,
        in_settings: FRigControlSettings,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<820>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_animation_channel_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_control,
                __buffer.add(12).cast::<FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(32).cast::<FRigControlSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo,
                __buffer.add(800).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(801).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_rig_hierarchy_controller_add_animation_channel_for_blueprint,
                __buffer,
            )
        };
        std::mem::forget(in_name);
        std::mem::forget(in_parent_control);
        std::mem::forget(in_settings);
        std::mem::forget(b_setup_undo);
        std::mem::forget(b_print_python_command);
        unsafe { __buffer.add(804).cast::<FRigElementKey>().read() }
    }
}
#[repr(C, align(16))]
pub struct UControlRigLayerInstance {
    __padding_end: [u8; 1136],
}
impl UControlRigLayerInstance {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigLayerInstance")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigLayerInstance")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UMovieSceneControlRigParameterSection {
    __padding_end: [u8; 1256],
}
impl UMovieSceneControlRigParameterSection {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneControlRigParameterSection")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneControlRigParameterSection")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UMovieSceneControlRigParameterTrack {
    __padding_end: [u8; 808],
}
impl UMovieSceneControlRigParameterTrack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneControlRigParameterTrack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneControlRigParameterTrack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UMovieSceneControlRigParameterEvaluatorSystem {
    __padding_end: [u8; 616],
}
impl UMovieSceneControlRigParameterEvaluatorSystem {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneControlRigParameterEvaluatorSystem")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneControlRigParameterEvaluatorSystem")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigSettings {
    __padding_end: [u8; 192],
}
impl UControlRigSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigEditorSettings {
    __padding_end: [u8; 272],
}
impl UControlRigEditorSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditorSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditorSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigPoseAsset {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub pose: FControlRigControlPose,
    __padding_end: [u8; 8],
}
impl UControlRigPoseAsset {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseAsset")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseAsset")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_up_mirror_match_table(&mut self, in_control_rig: UPtr<UControlRig>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_set_up_mirror_match_table,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_set_up_mirror_match_table,
                __buffer,
            )
        };
        std::mem::forget(in_control_rig);
    }
    pub fn select_controls(
        &mut self,
        in_control_rig: UPtr<UControlRig>,
        b_do_mirror: bool,
        b_clear_selection: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_select_controls,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_do_mirror,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_select_controls,
                __buffer,
            )
        };
        std::mem::forget(in_control_rig);
        std::mem::forget(b_do_mirror);
        std::mem::forget(b_clear_selection);
    }
    pub fn save_pose(&mut self, in_control_rig: UPtr<UControlRig>, b_use_all: bool) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_save_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_use_all, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_save_pose,
                __buffer,
            )
        };
        std::mem::forget(in_control_rig);
        std::mem::forget(b_use_all);
    }
    pub fn replace_control_name(&mut self, current_name: &FName, new_name: &FName) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_replace_control_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                current_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(new_name, __buffer.add(12).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_replace_control_name,
                __buffer,
            )
        };
    }
    pub fn paste_pose(
        &mut self,
        in_control_rig: UPtr<UControlRig>,
        b_do_key: bool,
        b_do_mirror: bool,
        b_do_additive: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_paste_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_do_key, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_do_mirror,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_do_additive,
                __buffer.add(10).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_paste_pose,
                __buffer,
            )
        };
        std::mem::forget(in_control_rig);
        std::mem::forget(b_do_key);
        std::mem::forget(b_do_mirror);
        std::mem::forget(b_do_additive);
    }
    pub fn get_current_pose(
        &mut self,
        in_control_rig: UPtr<UControlRig>,
        out_pose: &mut FControlRigControlPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_get_current_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pose,
                __buffer.add(8).cast::<FControlRigControlPose>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_get_current_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FControlRigControlPose>().swap(out_pose);
        }
        std::mem::forget(in_control_rig);
    }
    pub fn get_control_names(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_get_control_names,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_get_control_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn does_mirror_match(
        &mut self,
        control_rig: UPtr<UControlRig>,
        control_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_does_mirror_match,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer.add(0).cast::<UPtr<UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                control_name,
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
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_pose_asset_does_mirror_match,
                __buffer,
            )
        };
        std::mem::forget(control_rig);
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UControlRigPoseMirrorSettings {
    #[doc(hidden)]
    pub(crate) __padding_80: [u8; 80],
    pub mirror_axis: crate::bindings::core_u_object::EAxis,
    pub axis_to_flip: crate::bindings::core_u_object::EAxis,
    pub mirror_match_tolerance: f64,
}
impl UControlRigPoseMirrorSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseMirrorSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseMirrorSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigPoseProjectSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub root_save_dirs: TArray<crate::bindings::core_u_object::FDirectoryPath>,
}
impl UControlRigPoseProjectSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseProjectSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseProjectSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigWorkflowOptions {
    #[doc(hidden)]
    pub(crate) __padding_192: [u8; 192],
    pub hierarchy: UPtr<URigHierarchy>,
    pub selection: TArray<FRigElementKey>,
}
impl UControlRigWorkflowOptions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigWorkflowOptions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigWorkflowOptions")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn ensure_at_least_one_rig_element_selected(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_workflow_options_ensure_at_least_one_rig_element_selected,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig::__FUNCTION_PTRS
                    .u_control_rig_workflow_options_ensure_at_least_one_rig_element_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UControlRigTransformWorkflowOptions {
    #[doc(hidden)]
    pub(crate) __padding_216: [u8; 216],
    pub transform_type: ERigTransformType,
}
impl UControlRigTransformWorkflowOptions {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigTransformWorkflowOptions")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigTransformWorkflowOptions")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UControlRigNumericalValidationPass {
    __padding_end: [u8; 208],
}
impl UControlRigNumericalValidationPass {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigNumericalValidationPass")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigNumericalValidationPass")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
#[repr(C, align(1))]
pub struct FControlRig_OnControlSelected_BP {
    _opague: [u8; 1],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPreInitializeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPostInitializeDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPreConstructionDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPostConstructionDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPreForwardsSolveDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FControlRigComponent_OnPostForwardsSolveDelegate {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FRigHierarchy_ModifiedEventDynamic {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ERigElementType(pub u8);
impl ERigElementType {
    pub const NONE: ERigElementType = ERigElementType(0);
    pub const BONE: ERigElementType = ERigElementType(1);
    pub const NULL: ERigElementType = ERigElementType(2);
    pub const SPACE: ERigElementType = ERigElementType(2);
    pub const CONTROL: ERigElementType = ERigElementType(4);
    pub const CURVE: ERigElementType = ERigElementType(8);
    pub const PHYSICS: ERigElementType = ERigElementType(16);
    pub const REFERENCE: ERigElementType = ERigElementType(32);
    pub const CONNECTOR: ERigElementType = ERigElementType(64);
    pub const SOCKET: ERigElementType = ERigElementType(128);
    pub const ALL: ERigElementType = ERigElementType(239);
    pub const FIRST: ERigElementType = ERigElementType(1);
    pub const LAST: ERigElementType = ERigElementType(128);
    pub const TO_RESET_AFTER_CONSTRUCTION_EVENT: ERigElementType = ERigElementType(141);
    pub const TRANSFORM_TYPES: ERigElementType = ERigElementType(167);
}
#[repr(transparent)]
pub struct ERigControlAnimationType(pub u8);
impl ERigControlAnimationType {
    pub const ANIMATION_CONTROL: ERigControlAnimationType = ERigControlAnimationType(0);
    pub const ANIMATION_CHANNEL: ERigControlAnimationType = ERigControlAnimationType(1);
    pub const PROXY_CONTROL: ERigControlAnimationType = ERigControlAnimationType(2);
    pub const VISUAL_CUE: ERigControlAnimationType = ERigControlAnimationType(3);
}
#[repr(transparent)]
pub struct ERigControlType(pub u8);
impl ERigControlType {
    pub const BOOL: ERigControlType = ERigControlType(0);
    pub const FLOAT: ERigControlType = ERigControlType(1);
    pub const INTEGER: ERigControlType = ERigControlType(2);
    pub const VECTOR2_D: ERigControlType = ERigControlType(3);
    pub const POSITION: ERigControlType = ERigControlType(4);
    pub const SCALE: ERigControlType = ERigControlType(5);
    pub const ROTATOR: ERigControlType = ERigControlType(6);
    pub const TRANSFORM: ERigControlType = ERigControlType(7);
    pub const TRANSFORM_NO_SCALE: ERigControlType = ERigControlType(8);
    pub const EULER_TRANSFORM: ERigControlType = ERigControlType(9);
    pub const SCALE_FLOAT: ERigControlType = ERigControlType(10);
}
#[repr(transparent)]
pub struct ERigControlAxis(pub u8);
impl ERigControlAxis {
    pub const X: ERigControlAxis = ERigControlAxis(0);
    pub const Y: ERigControlAxis = ERigControlAxis(1);
    pub const Z: ERigControlAxis = ERigControlAxis(2);
}
#[repr(transparent)]
pub struct ERigControlVisibility(pub u8);
impl ERigControlVisibility {
    pub const USER_DEFINED: ERigControlVisibility = ERigControlVisibility(0);
    pub const BASED_ON_SELECTION: ERigControlVisibility = ERigControlVisibility(1);
}
#[repr(transparent)]
pub struct ERigControlTransformChannel(pub u8);
impl ERigControlTransformChannel {
    pub const TRANSLATION_X: ERigControlTransformChannel = ERigControlTransformChannel(
        0,
    );
    pub const TRANSLATION_Y: ERigControlTransformChannel = ERigControlTransformChannel(
        1,
    );
    pub const TRANSLATION_Z: ERigControlTransformChannel = ERigControlTransformChannel(
        2,
    );
    pub const PITCH: ERigControlTransformChannel = ERigControlTransformChannel(3);
    pub const YAW: ERigControlTransformChannel = ERigControlTransformChannel(4);
    pub const ROLL: ERigControlTransformChannel = ERigControlTransformChannel(5);
    pub const SCALE_X: ERigControlTransformChannel = ERigControlTransformChannel(6);
    pub const SCALE_Y: ERigControlTransformChannel = ERigControlTransformChannel(7);
    pub const SCALE_Z: ERigControlTransformChannel = ERigControlTransformChannel(8);
}
#[repr(transparent)]
pub struct EConnectorType(pub u8);
impl EConnectorType {
    pub const PRIMARY: EConnectorType = EConnectorType(0);
    pub const SECONDARY: EConnectorType = EConnectorType(1);
}
#[repr(transparent)]
pub struct EElementNameDisplayMode(pub u8);
impl EElementNameDisplayMode {
    pub const ASSET_DEFAULT: EElementNameDisplayMode = EElementNameDisplayMode(0);
    pub const AUTO: EElementNameDisplayMode = EElementNameDisplayMode(1);
    pub const FORCE_SHORT: EElementNameDisplayMode = EElementNameDisplayMode(2);
    pub const FORCE_LONG: EElementNameDisplayMode = EElementNameDisplayMode(3);
}
#[repr(transparent)]
pub struct EControlRigComponentMapDirection(pub u8);
impl EControlRigComponentMapDirection {
    pub const INPUT: EControlRigComponentMapDirection = EControlRigComponentMapDirection(
        0,
    );
    pub const OUTPUT: EControlRigComponentMapDirection = EControlRigComponentMapDirection(
        1,
    );
}
#[repr(transparent)]
pub struct EControlRigComponentSpace(pub u8);
impl EControlRigComponentSpace {
    pub const WORLD_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(0);
    pub const ACTOR_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(1);
    pub const COMPONENT_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(2);
    pub const RIG_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(3);
    pub const LOCAL_SPACE: EControlRigComponentSpace = EControlRigComponentSpace(4);
    pub const MAX: EControlRigComponentSpace = EControlRigComponentSpace(5);
}
#[repr(transparent)]
pub struct ECRSimConstraintType(pub u8);
impl ECRSimConstraintType {
    pub const DISTANCE: ECRSimConstraintType = ECRSimConstraintType(0);
    pub const DISTANCE_FROM_A: ECRSimConstraintType = ECRSimConstraintType(1);
    pub const DISTANCE_FROM_B: ECRSimConstraintType = ECRSimConstraintType(2);
    pub const PLANE: ECRSimConstraintType = ECRSimConstraintType(3);
}
#[repr(transparent)]
pub struct ECRSimPointForceType(pub u8);
impl ECRSimPointForceType {
    pub const DIRECTION: ECRSimPointForceType = ECRSimPointForceType(0);
}
#[repr(transparent)]
pub struct ECRSimSoftCollisionType(pub u8);
impl ECRSimSoftCollisionType {
    pub const PLANE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(0);
    pub const SPHERE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(1);
    pub const CONE: ECRSimSoftCollisionType = ECRSimSoftCollisionType(2);
}
#[repr(transparent)]
pub struct ERigBoneType(pub u8);
impl ERigBoneType {
    pub const IMPORTED: ERigBoneType = ERigBoneType(0);
    pub const USER: ERigBoneType = ERigBoneType(1);
}
#[repr(transparent)]
pub struct ERigTransformStackEntryType(pub u8);
impl ERigTransformStackEntryType {
    pub const TRANSFORM_POSE: ERigTransformStackEntryType = ERigTransformStackEntryType(
        0,
    );
    pub const CONTROL_OFFSET: ERigTransformStackEntryType = ERigTransformStackEntryType(
        1,
    );
    pub const CONTROL_SHAPE: ERigTransformStackEntryType = ERigTransformStackEntryType(
        2,
    );
    pub const CURVE_VALUE: ERigTransformStackEntryType = ERigTransformStackEntryType(3);
}
#[repr(transparent)]
pub struct ERigTransformType(pub u8);
impl ERigTransformType {
    pub const INITIAL_LOCAL: ERigTransformType = ERigTransformType(0);
    pub const CURRENT_LOCAL: ERigTransformType = ERigTransformType(1);
    pub const INITIAL_GLOBAL: ERigTransformType = ERigTransformType(2);
    pub const CURRENT_GLOBAL: ERigTransformType = ERigTransformType(3);
    pub const NUM_TRANSFORM_TYPES: ERigTransformType = ERigTransformType(4);
}
#[repr(transparent)]
pub struct ERigSpaceType(pub u8);
impl ERigSpaceType {
    pub const GLOBAL: ERigSpaceType = ERigSpaceType(0);
    pub const BONE: ERigSpaceType = ERigSpaceType(1);
    pub const CONTROL: ERigSpaceType = ERigSpaceType(2);
    pub const SPACE: ERigSpaceType = ERigSpaceType(3);
}
#[repr(transparent)]
pub struct ERigElementResolveState(pub u8);
impl ERigElementResolveState {
    pub const UNKNOWN: ERigElementResolveState = ERigElementResolveState(0);
    pub const INVALID_TARGET: ERigElementResolveState = ERigElementResolveState(1);
    pub const POSSIBLE_TARGET: ERigElementResolveState = ERigElementResolveState(2);
    pub const DEFAULT_TARGET: ERigElementResolveState = ERigElementResolveState(3);
    pub const MAX: ERigElementResolveState = ERigElementResolveState(4);
}
#[repr(transparent)]
pub struct EModularRigResolveState(pub u8);
impl EModularRigResolveState {
    pub const SUCCESS: EModularRigResolveState = EModularRigResolveState(0);
    pub const ERROR: EModularRigResolveState = EModularRigResolveState(1);
    pub const MAX: EModularRigResolveState = EModularRigResolveState(2);
}
#[repr(transparent)]
pub struct ERigMetadataType(pub u8);
impl ERigMetadataType {
    pub const BOOL: ERigMetadataType = ERigMetadataType(0);
    pub const BOOL_ARRAY: ERigMetadataType = ERigMetadataType(1);
    pub const FLOAT: ERigMetadataType = ERigMetadataType(2);
    pub const FLOAT_ARRAY: ERigMetadataType = ERigMetadataType(3);
    pub const INT32: ERigMetadataType = ERigMetadataType(4);
    pub const INT32_ARRAY: ERigMetadataType = ERigMetadataType(5);
    pub const NAME: ERigMetadataType = ERigMetadataType(6);
    pub const NAME_ARRAY: ERigMetadataType = ERigMetadataType(7);
    pub const VECTOR: ERigMetadataType = ERigMetadataType(8);
    pub const VECTOR_ARRAY: ERigMetadataType = ERigMetadataType(9);
    pub const ROTATOR: ERigMetadataType = ERigMetadataType(10);
    pub const ROTATOR_ARRAY: ERigMetadataType = ERigMetadataType(11);
    pub const QUAT: ERigMetadataType = ERigMetadataType(12);
    pub const QUAT_ARRAY: ERigMetadataType = ERigMetadataType(13);
    pub const TRANSFORM: ERigMetadataType = ERigMetadataType(14);
    pub const TRANSFORM_ARRAY: ERigMetadataType = ERigMetadataType(15);
    pub const LINEAR_COLOR: ERigMetadataType = ERigMetadataType(16);
    pub const LINEAR_COLOR_ARRAY: ERigMetadataType = ERigMetadataType(17);
    pub const RIG_ELEMENT_KEY: ERigMetadataType = ERigMetadataType(18);
    pub const RIG_ELEMENT_KEY_ARRAY: ERigMetadataType = ERigMetadataType(19);
    pub const INVALID: ERigMetadataType = ERigMetadataType(20);
}
#[repr(transparent)]
pub struct EMovieSceneControlRigSpaceType(pub u8);
impl EMovieSceneControlRigSpaceType {
    pub const PARENT: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(0);
    pub const WORLD: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(1);
    pub const CONTROL_RIG: EMovieSceneControlRigSpaceType = EMovieSceneControlRigSpaceType(
        2,
    );
}
#[repr(transparent)]
pub struct EAimMode(pub u8);
impl EAimMode {
    pub const AIM_AT_TARGET: EAimMode = EAimMode(0);
    pub const ORIENT_TO_TARGET: EAimMode = EAimMode(1);
    pub const MAX: EAimMode = EAimMode(2);
}
#[repr(transparent)]
pub struct EApplyTransformMode(pub u8);
impl EApplyTransformMode {
    pub const OVERRIDE: EApplyTransformMode = EApplyTransformMode(0);
    pub const ADDITIVE: EApplyTransformMode = EApplyTransformMode(1);
    pub const MAX: EApplyTransformMode = EApplyTransformMode(2);
}
#[repr(transparent)]
pub struct ETransformSpaceMode(pub u8);
impl ETransformSpaceMode {
    pub const LOCAL_SPACE: ETransformSpaceMode = ETransformSpaceMode(0);
    pub const GLOBAL_SPACE: ETransformSpaceMode = ETransformSpaceMode(1);
    pub const BASE_SPACE: ETransformSpaceMode = ETransformSpaceMode(2);
    pub const BASE_JOINT: ETransformSpaceMode = ETransformSpaceMode(3);
    pub const MAX: ETransformSpaceMode = ETransformSpaceMode(4);
}
#[repr(transparent)]
pub struct ETransformGetterType(pub u8);
impl ETransformGetterType {
    pub const INITIAL: ETransformGetterType = ETransformGetterType(0);
    pub const CURRENT: ETransformGetterType = ETransformGetterType(1);
    pub const MAX: ETransformGetterType = ETransformGetterType(2);
}
#[repr(transparent)]
pub struct ERigSwitchParentMode(pub u8);
impl ERigSwitchParentMode {
    pub const WORLD: ERigSwitchParentMode = ERigSwitchParentMode(0);
    pub const DEFAULT_PARENT: ERigSwitchParentMode = ERigSwitchParentMode(1);
    pub const PARENT_ITEM: ERigSwitchParentMode = ERigSwitchParentMode(2);
}
#[repr(transparent)]
pub struct ERigMetaDataNameSpace(pub u8);
impl ERigMetaDataNameSpace {
    pub const NONE: ERigMetaDataNameSpace = ERigMetaDataNameSpace(0);
    pub const SELF: ERigMetaDataNameSpace = ERigMetaDataNameSpace(1);
    pub const PARENT: ERigMetaDataNameSpace = ERigMetaDataNameSpace(2);
    pub const ROOT: ERigMetaDataNameSpace = ERigMetaDataNameSpace(3);
    pub const LAST: ERigMetaDataNameSpace = ERigMetaDataNameSpace(4);
}
#[repr(transparent)]
pub struct ERigEvent(pub u8);
impl ERigEvent {
    pub const NONE: ERigEvent = ERigEvent(0);
    pub const REQUEST_AUTO_KEY: ERigEvent = ERigEvent(1);
    pub const OPEN_UNDO_BRACKET: ERigEvent = ERigEvent(2);
    pub const CLOSE_UNDO_BRACKET: ERigEvent = ERigEvent(3);
    pub const MAX: ERigEvent = ERigEvent(4);
}
#[repr(transparent)]
pub struct EControlRigVectorKind(pub u8);
impl EControlRigVectorKind {
    pub const DIRECTION: EControlRigVectorKind = EControlRigVectorKind(0);
    pub const LOCATION: EControlRigVectorKind = EControlRigVectorKind(1);
}
#[repr(transparent)]
pub struct EControlRigCurveAlignment(pub u8);
impl EControlRigCurveAlignment {
    pub const FRONT: EControlRigCurveAlignment = EControlRigCurveAlignment(0);
    pub const STRETCHED: EControlRigCurveAlignment = EControlRigCurveAlignment(1);
}
#[repr(transparent)]
pub struct EControlRigModifyBoneMode(pub u8);
impl EControlRigModifyBoneMode {
    pub const OVERRIDE_LOCAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(0);
    pub const OVERRIDE_GLOBAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(1);
    pub const ADDITIVE_LOCAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(2);
    pub const ADDITIVE_GLOBAL: EControlRigModifyBoneMode = EControlRigModifyBoneMode(3);
    pub const MAX: EControlRigModifyBoneMode = EControlRigModifyBoneMode(4);
}
#[repr(transparent)]
pub struct EConstraintInterpType(pub u8);
impl EConstraintInterpType {
    pub const AVERAGE: EConstraintInterpType = EConstraintInterpType(0);
    pub const SHORTEST: EConstraintInterpType = EConstraintInterpType(1);
    pub const MAX: EConstraintInterpType = EConstraintInterpType(2);
}
#[repr(transparent)]
pub struct EControlRigReplayPlaybackMode(pub u8);
impl EControlRigReplayPlaybackMode {
    pub const LIVE: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(0);
    pub const REPLAY_INPUTS: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(
        1,
    );
    pub const GROUND_TRUTH: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(
        2,
    );
    pub const MAX: EControlRigReplayPlaybackMode = EControlRigReplayPlaybackMode(3);
}
#[repr(transparent)]
pub struct ERigControlValueType(pub u8);
impl ERigControlValueType {
    pub const INITIAL: ERigControlValueType = ERigControlValueType(0);
    pub const CURRENT: ERigControlValueType = ERigControlValueType(1);
    pub const MINIMUM: ERigControlValueType = ERigControlValueType(2);
    pub const MAXIMUM: ERigControlValueType = ERigControlValueType(3);
}
#[repr(transparent)]
pub struct EControlRigFKRigExecuteMode(pub u8);
impl EControlRigFKRigExecuteMode {
    pub const REPLACE: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(0);
    pub const ADDITIVE: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(1);
    pub const DIRECT: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(2);
    pub const MAX: EControlRigFKRigExecuteMode = EControlRigFKRigExecuteMode(3);
}
#[repr(transparent)]
pub struct ERigExecutionType(pub u8);
impl ERigExecutionType {
    pub const RUNTIME: ERigExecutionType = ERigExecutionType(0);
    pub const EDITING: ERigExecutionType = ERigExecutionType(1);
    pub const MAX: ERigExecutionType = ERigExecutionType(2);
}
#[repr(transparent)]
pub struct EControlRigType(pub u8);
impl EControlRigType {
    pub const INDEPENDENT_RIG: EControlRigType = EControlRigType(0);
    pub const RIG_MODULE: EControlRigType = EControlRigType(1);
    pub const MODULAR_RIG: EControlRigType = EControlRigType(2);
    pub const MAX: EControlRigType = EControlRigType(3);
}
#[repr(transparent)]
pub struct ERigHierarchyNotification(pub u8);
impl ERigHierarchyNotification {
    pub const ELEMENT_ADDED: ERigHierarchyNotification = ERigHierarchyNotification(0);
    pub const ELEMENT_REMOVED: ERigHierarchyNotification = ERigHierarchyNotification(1);
    pub const ELEMENT_RENAMED: ERigHierarchyNotification = ERigHierarchyNotification(2);
    pub const ELEMENT_SELECTED: ERigHierarchyNotification = ERigHierarchyNotification(3);
    pub const ELEMENT_DESELECTED: ERigHierarchyNotification = ERigHierarchyNotification(
        4,
    );
    pub const PARENT_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(5);
    pub const HIERARCHY_RESET: ERigHierarchyNotification = ERigHierarchyNotification(6);
    pub const CONTROL_SETTING_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        7,
    );
    pub const CONTROL_VISIBILITY_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        8,
    );
    pub const CONTROL_DRIVEN_LIST_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        9,
    );
    pub const CONTROL_SHAPE_TRANSFORM_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        10,
    );
    pub const PARENT_WEIGHTS_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        11,
    );
    pub const INTERACTION_BRACKET_OPENED: ERigHierarchyNotification = ERigHierarchyNotification(
        12,
    );
    pub const INTERACTION_BRACKET_CLOSED: ERigHierarchyNotification = ERigHierarchyNotification(
        13,
    );
    pub const ELEMENT_REORDERED: ERigHierarchyNotification = ERigHierarchyNotification(
        14,
    );
    pub const CONNECTOR_SETTING_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        15,
    );
    pub const SOCKET_COLOR_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        16,
    );
    pub const SOCKET_DESCRIPTION_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        17,
    );
    pub const SOCKET_DESIRED_PARENT_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        18,
    );
    pub const HIERARCHY_COPIED: ERigHierarchyNotification = ERigHierarchyNotification(
        19,
    );
    pub const COMPONENT_ADDED: ERigHierarchyNotification = ERigHierarchyNotification(20);
    pub const COMPONENT_REMOVED: ERigHierarchyNotification = ERigHierarchyNotification(
        21,
    );
    pub const COMPONENT_CONTENT_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        22,
    );
    pub const COMPONENT_SELECTED: ERigHierarchyNotification = ERigHierarchyNotification(
        23,
    );
    pub const COMPONENT_DESELECTED: ERigHierarchyNotification = ERigHierarchyNotification(
        24,
    );
    pub const COMPONENT_RENAMED: ERigHierarchyNotification = ERigHierarchyNotification(
        25,
    );
    pub const COMPONENT_REPARENTED: ERigHierarchyNotification = ERigHierarchyNotification(
        26,
    );
    pub const SHORT_NAME_CHANGED: ERigHierarchyNotification = ERigHierarchyNotification(
        27,
    );
    pub const MAX: ERigHierarchyNotification = ERigHierarchyNotification(28);
}
#[repr(transparent)]
pub struct EMultiRigTreeDisplayMode(pub i32);
impl EMultiRigTreeDisplayMode {
    pub const ALL: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(0);
    pub const SELECTED_RIGS: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(1);
    pub const SELECTED_MODULES: EMultiRigTreeDisplayMode = EMultiRigTreeDisplayMode(2);
}
#[repr(transparent)]
pub struct EControlRigInteractionType(pub u8);
impl EControlRigInteractionType {
    pub const NONE: EControlRigInteractionType = EControlRigInteractionType(0);
    pub const TRANSLATE: EControlRigInteractionType = EControlRigInteractionType(1);
    pub const ROTATE: EControlRigInteractionType = EControlRigInteractionType(2);
    pub const SCALE: EControlRigInteractionType = EControlRigInteractionType(4);
    pub const ALL: EControlRigInteractionType = EControlRigInteractionType(7);
}
