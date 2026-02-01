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
    pub u_camera_rig_instance_functions_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_run_child_camera_director: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_run_camera_director: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_resolve_camera_rig_proxy: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_remove_child_evaluation_context: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_get_initial_context_result: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_get_conditional_context_result: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_find_evaluation_context_owner_actor: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_deactivate_persistent_visual_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_deactivate_persistent_global_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_deactivate_persistent_base_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_deactivate_camera_director: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_add_child_evaluation_context: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_activate_persistent_visual_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_activate_persistent_global_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_activate_persistent_base_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_activate_camera_rig_via_proxy: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_activate_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_director_evaluator_activate_camera_director: *mut crate::ffi::UFunctionOpague,
    pub u_activate_camera_rig_functions_activate_persistent_visual_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_activate_camera_rig_functions_activate_persistent_global_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_activate_camera_rig_functions_activate_persistent_base_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_evaluation_data_function_library_set_default_camera_rig_parameters: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_evaluation_data_function_library_set_camera_pose: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_evaluation_data_function_library_make_camera_evaluation_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_evaluation_data_function_library_get_camera_pose: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_evaluation_data_function_library_blend_camera_evaluation_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_vector4_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_vector3_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_vector2_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_transform_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_rotator_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_integer32_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_float_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_double_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_set_boolean_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_vector4_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_vector3_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_vector2_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_transform_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_rotator_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_integer32_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_float_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_double_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_variable_table_function_library_get_boolean_camera_variable: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_set_struct_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_set_string_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_set_object_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_set_name_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_set_enum_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_set_class_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_get_struct_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_get_string_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_get_object_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_get_name_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_get_enum_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_context_data_table_function_library_get_class_data: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_set_transform: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_set_target_distance: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_set_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_set_location: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_set_focal_length: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_set_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_make_camera_pose_from_cine_camera_component: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_make_camera_pose_from_camera_component: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_transform: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_target_distance: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_target_at_distance: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_target: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_sensor_aspect_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_location: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_focal_length: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_effective_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_aim_ray: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_pose_function_library_get_aim_dir: *mut crate::ffi::UFunctionOpague,
    pub u_camera_rig_parameter_interop_set_camera_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_camera_rig_parameter_interop_get_camera_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_camera_rig_parameter_interop_library_make_literal_vector3f: *mut crate::ffi::UFunctionOpague,
    pub u_camera_rig_parameter_interop_library_make_literal_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_camera_rig_parameter_interop_library_make_literal_vector: *mut crate::ffi::UFunctionOpague,
    pub u_camera_rig_parameter_interop_library_make_literal_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_camera_rig_parameter_interop_library_make_literal_linear_color: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_camera_actor_get_camera_component: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_get_output_camera_component: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_get_initial_result: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_get_evaluated_camera_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_get_conditional_result: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_deactivate_camera: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_activate_persistent_visual_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_activate_persistent_global_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_activate_persistent_base_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_activate_camera_for_player_index: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_base_activate_camera_for_player_controller: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_component_notify_change_camera_reference: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_parameter_setter_component_stop_parameter_setters: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_parameter_setter_component_start_parameter_setters: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_parameter_setter_component_on_actor_end_overlap: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_parameter_setter_component_on_actor_begin_overlap: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_camera_rig_actor_get_camera_rig_component: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_rig_component_notify_change_camera_rig_reference: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_stop_camera_shake_asset: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_stop_camera_modifier_rig: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_steal_player_controller: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_start_visual_camera_modifier_rig: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_start_global_camera_modifier_rig: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_start_camera_shake_asset: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_release_player_controller: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_is_camera_shake_asset_playing: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_activate_persistent_visual_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_activate_persistent_global_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_cameras_player_camera_manager_activate_persistent_base_camera_rig: *mut crate::ffi::UFunctionOpague,
    pub a_gameplay_camera_system_actor_get_camera_system_component: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_system_component_stop_camera_modifier_rig: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_system_component_start_visual_camera_modifier_rig: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_system_component_start_global_camera_modifier_rig: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_system_component_is_camera_system_active_for_play_controller: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_system_component_deactivate_camera_system: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_system_component_activate_camera_system_for_player_index: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_camera_system_component_activate_camera_system_for_player_controller: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_control_rotation_component_deactivate_control_rotation_management: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_index: *mut crate::ffi::UFunctionOpague,
    pub u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_controller: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_tick_camera_node: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_set_default_owning_camera_rig_parameters: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_set_current_camera_pose: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_set_camera_pose: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_initialize_camera_node: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_get_player_controller: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_get_current_camera_pose: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_get_camera_pose: *mut crate::ffi::UFunctionOpague,
    pub u_blueprint_camera_node_evaluator_find_evaluation_context_owner_actor: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_camera_rig_instance_functions_is_valid: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_run_child_camera_director: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_run_camera_director: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_resolve_camera_rig_proxy: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_remove_child_evaluation_context: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_get_initial_context_result: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_get_conditional_context_result: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_find_evaluation_context_owner_actor: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_deactivate_persistent_visual_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_deactivate_persistent_global_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_deactivate_persistent_base_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_deactivate_camera_director: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_add_child_evaluation_context: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_activate_persistent_visual_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_activate_persistent_global_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_activate_persistent_base_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_activate_camera_rig_via_proxy: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_activate_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_director_evaluator_activate_camera_director: std::ptr::null_mut(),
            u_activate_camera_rig_functions_activate_persistent_visual_camera_rig: std::ptr::null_mut(),
            u_activate_camera_rig_functions_activate_persistent_global_camera_rig: std::ptr::null_mut(),
            u_activate_camera_rig_functions_activate_persistent_base_camera_rig: std::ptr::null_mut(),
            u_blueprint_camera_evaluation_data_function_library_set_default_camera_rig_parameters: std::ptr::null_mut(),
            u_blueprint_camera_evaluation_data_function_library_set_camera_pose: std::ptr::null_mut(),
            u_blueprint_camera_evaluation_data_function_library_make_camera_evaluation_data: std::ptr::null_mut(),
            u_blueprint_camera_evaluation_data_function_library_get_camera_pose: std::ptr::null_mut(),
            u_blueprint_camera_evaluation_data_function_library_blend_camera_evaluation_data: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_vector4_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_vector3_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_vector2_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_transform_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_rotator_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_integer32_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_float_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_double_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_set_boolean_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_vector4_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_vector3_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_vector2_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_transform_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_rotator_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_integer32_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_float_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_double_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_variable_table_function_library_get_boolean_camera_variable: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_set_struct_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_set_string_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_set_object_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_set_name_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_set_enum_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_set_class_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_get_struct_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_get_string_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_get_object_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_get_name_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_get_enum_data: std::ptr::null_mut(),
            u_blueprint_camera_context_data_table_function_library_get_class_data: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_set_transform: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_set_target_distance: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_set_rotation: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_set_location: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_set_focal_length: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_set_field_of_view: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_make_camera_pose_from_cine_camera_component: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_make_camera_pose_from_camera_component: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_transform: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_target_distance: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_target_at_distance: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_target: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_sensor_aspect_ratio: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_rotation: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_location: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_focal_length: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_field_of_view: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_effective_field_of_view: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_aim_ray: std::ptr::null_mut(),
            u_blueprint_camera_pose_function_library_get_aim_dir: std::ptr::null_mut(),
            u_camera_rig_parameter_interop_set_camera_parameter: std::ptr::null_mut(),
            u_camera_rig_parameter_interop_get_camera_parameter: std::ptr::null_mut(),
            u_camera_rig_parameter_interop_library_make_literal_vector3f: std::ptr::null_mut(),
            u_camera_rig_parameter_interop_library_make_literal_vector2_d: std::ptr::null_mut(),
            u_camera_rig_parameter_interop_library_make_literal_vector: std::ptr::null_mut(),
            u_camera_rig_parameter_interop_library_make_literal_rotator: std::ptr::null_mut(),
            u_camera_rig_parameter_interop_library_make_literal_linear_color: std::ptr::null_mut(),
            a_gameplay_camera_actor_get_camera_component: std::ptr::null_mut(),
            u_gameplay_camera_component_base_get_output_camera_component: std::ptr::null_mut(),
            u_gameplay_camera_component_base_get_initial_result: std::ptr::null_mut(),
            u_gameplay_camera_component_base_get_evaluated_camera_rotation: std::ptr::null_mut(),
            u_gameplay_camera_component_base_get_conditional_result: std::ptr::null_mut(),
            u_gameplay_camera_component_base_deactivate_camera: std::ptr::null_mut(),
            u_gameplay_camera_component_base_activate_persistent_visual_camera_rig: std::ptr::null_mut(),
            u_gameplay_camera_component_base_activate_persistent_global_camera_rig: std::ptr::null_mut(),
            u_gameplay_camera_component_base_activate_persistent_base_camera_rig: std::ptr::null_mut(),
            u_gameplay_camera_component_base_activate_camera_for_player_index: std::ptr::null_mut(),
            u_gameplay_camera_component_base_activate_camera_for_player_controller: std::ptr::null_mut(),
            u_gameplay_camera_component_notify_change_camera_reference: std::ptr::null_mut(),
            u_gameplay_camera_parameter_setter_component_stop_parameter_setters: std::ptr::null_mut(),
            u_gameplay_camera_parameter_setter_component_start_parameter_setters: std::ptr::null_mut(),
            u_gameplay_camera_parameter_setter_component_on_actor_end_overlap: std::ptr::null_mut(),
            u_gameplay_camera_parameter_setter_component_on_actor_begin_overlap: std::ptr::null_mut(),
            a_gameplay_camera_rig_actor_get_camera_rig_component: std::ptr::null_mut(),
            u_gameplay_camera_rig_component_notify_change_camera_rig_reference: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_stop_camera_shake_asset: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_stop_camera_modifier_rig: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_steal_player_controller: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_start_visual_camera_modifier_rig: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_start_global_camera_modifier_rig: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_start_camera_shake_asset: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_release_player_controller: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_is_camera_shake_asset_playing: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_activate_persistent_visual_camera_rig: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_activate_persistent_global_camera_rig: std::ptr::null_mut(),
            a_gameplay_cameras_player_camera_manager_activate_persistent_base_camera_rig: std::ptr::null_mut(),
            a_gameplay_camera_system_actor_get_camera_system_component: std::ptr::null_mut(),
            u_gameplay_camera_system_component_stop_camera_modifier_rig: std::ptr::null_mut(),
            u_gameplay_camera_system_component_start_visual_camera_modifier_rig: std::ptr::null_mut(),
            u_gameplay_camera_system_component_start_global_camera_modifier_rig: std::ptr::null_mut(),
            u_gameplay_camera_system_component_is_camera_system_active_for_play_controller: std::ptr::null_mut(),
            u_gameplay_camera_system_component_deactivate_camera_system: std::ptr::null_mut(),
            u_gameplay_camera_system_component_activate_camera_system_for_player_index: std::ptr::null_mut(),
            u_gameplay_camera_system_component_activate_camera_system_for_player_controller: std::ptr::null_mut(),
            u_gameplay_control_rotation_component_deactivate_control_rotation_management: std::ptr::null_mut(),
            u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_index: std::ptr::null_mut(),
            u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_controller: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_tick_camera_node: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_set_default_owning_camera_rig_parameters: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_set_current_camera_pose: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_set_camera_pose: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_initialize_camera_node: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_get_player_controller: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_get_current_camera_pose: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_get_camera_pose: std::ptr::null_mut(),
            u_blueprint_camera_node_evaluator_find_evaluation_context_owner_actor: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCameraRigInstanceFunctions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValid"),
                &raw mut __FUNCTION_PTRS.u_camera_rig_instance_functions_is_valid,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintCameraDirectorEvaluator::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RunChildCameraDirector"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_run_child_camera_director,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RunCameraDirector"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_run_camera_director,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResolveCameraRigProxy"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_resolve_camera_rig_proxy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveChildEvaluationContext"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_remove_child_evaluation_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInitialContextResult"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_get_initial_context_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConditionalContextResult"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_get_conditional_context_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindEvaluationContextOwnerActor"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_find_evaluation_context_owner_actor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeactivatePersistentVisualCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_visual_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeactivatePersistentGlobalCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_global_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeactivatePersistentBaseCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_base_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeactivateCameraDirector"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_camera_director,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddChildEvaluationContext"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_add_child_evaluation_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_visual_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_global_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_base_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivateCameraRigViaProxy"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_rig_via_proxy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivateCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivateCameraDirector"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_director,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UActivateCameraRigFunctions::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_visual_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_global_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_base_camera_rig,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintCameraEvaluationDataFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDefaultCameraRigParameters"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_set_default_camera_rig_parameters,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCameraPose"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_set_camera_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeCameraEvaluationData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_make_camera_evaluation_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraPose"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_get_camera_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlendCameraEvaluationData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_blend_camera_evaluation_data,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintCameraVariableTableFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVector4CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector4_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVector3CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector3_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetVector2CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector2_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTransformCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_transform_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRotatorCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_rotator_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInteger32CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_integer32_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFloatCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_float_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDoubleCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_double_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetBooleanCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_boolean_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVector4CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector4_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVector3CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector3_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVector2CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector2_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransformCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_transform_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRotatorCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_rotator_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInteger32CameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_integer32_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_float_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDoubleCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_double_camera_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBooleanCameraVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_boolean_camera_variable,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintCameraContextDataTableFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStructData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_struct_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetStringData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_string_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetObjectData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_object_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNameData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_name_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnumData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_enum_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetClassData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_class_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStructData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_struct_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStringData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_string_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObjectData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_object_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNameData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_name_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnumData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_enum_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetClassData"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_class_data,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintCameraPoseFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetTargetDistance"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_target_distance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRotation"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFocalLength"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_focal_length,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFieldOfView"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_field_of_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeCameraPoseFromCineCameraComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_make_camera_pose_from_cine_camera_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeCameraPoseFromCameraComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_make_camera_pose_from_camera_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetDistance"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target_distance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetAtDistance"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target_at_distance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTarget"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSensorAspectRatio"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_sensor_aspect_ratio,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRotation"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocation"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_location,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFocalLength"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_focal_length,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFieldOfView"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_field_of_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEffectiveFieldOfView"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_effective_field_of_view,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAimRay"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_aim_ray,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAimDir"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_aim_dir,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCameraRigParameterInterop::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCameraParameter"),
                &raw mut __FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_set_camera_parameter,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraParameter"),
                &raw mut __FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_get_camera_parameter,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UCameraRigParameterInteropLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeLiteralVector3f"),
                &raw mut __FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector3f,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeLiteralVector2D"),
                &raw mut __FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector2_d,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeLiteralVector"),
                &raw mut __FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeLiteralRotator"),
                &raw mut __FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeLiteralLinearColor"),
                &raw mut __FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_linear_color,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayCameraActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraComponent"),
                &raw mut __FUNCTION_PTRS.a_gameplay_camera_actor_get_camera_component,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCameraComponentBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOutputCameraComponent"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_output_camera_component,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInitialResult"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_initial_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEvaluatedCameraRotation"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_evaluated_camera_rotation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConditionalResult"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_conditional_result,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeactivateCamera"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_deactivate_camera,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_visual_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_global_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_base_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivateCameraForPlayerIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_camera_for_player_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivateCameraForPlayerController"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_camera_for_player_controller,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCameraComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotifyChangeCameraReference"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_component_notify_change_camera_reference,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCameraParameterSetterComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopParameterSetters"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_stop_parameter_setters,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartParameterSetters"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_start_parameter_setters,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnActorEndOverlap"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_on_actor_end_overlap,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnActorBeginOverlap"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_on_actor_begin_overlap,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayCameraRigActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraRigComponent"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_camera_rig_actor_get_camera_rig_component,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCameraRigComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("NotifyChangeCameraRigReference"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_rig_component_notify_change_camera_rig_reference,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayCamerasPlayerCameraManager::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopCameraShakeAsset"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_stop_camera_shake_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopCameraModifierRig"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_stop_camera_modifier_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StealPlayerController"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_steal_player_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartVisualCameraModifierRig"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_visual_camera_modifier_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartGlobalCameraModifierRig"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_global_camera_modifier_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartCameraShakeAsset"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_camera_shake_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReleasePlayerController"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_release_player_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCameraShakeAssetPlaying"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_is_camera_shake_asset_playing,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentVisualCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_visual_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentGlobalCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_global_camera_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivatePersistentBaseCameraRig"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_base_camera_rig,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = AGameplayCameraSystemActor::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraSystemComponent"),
                &raw mut __FUNCTION_PTRS
                    .a_gameplay_camera_system_actor_get_camera_system_component,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayCameraSystemComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StopCameraModifierRig"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_system_component_stop_camera_modifier_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartVisualCameraModifierRig"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_system_component_start_visual_camera_modifier_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("StartGlobalCameraModifierRig"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_system_component_start_global_camera_modifier_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsCameraSystemActiveForPlayController"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_system_component_is_camera_system_active_for_play_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeactivateCameraSystem"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_system_component_deactivate_camera_system,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivateCameraSystemForPlayerIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_system_component_activate_camera_system_for_player_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ActivateCameraSystemForPlayerController"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_camera_system_component_activate_camera_system_for_player_controller,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UGameplayControlRotationComponent::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeactivateControlRotationManagement"),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_deactivate_control_rotation_management,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "ActivateControlRotationManagementForPlayerIndex",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "ActivateControlRotationManagementForPlayerController",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_controller,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UBlueprintCameraNodeEvaluator::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TickCameraNode"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_tick_camera_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDefaultOwningCameraRigParameters"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_default_owning_camera_rig_parameters,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCurrentCameraPose"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_current_camera_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCameraPose"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_camera_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InitializeCameraNode"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_initialize_camera_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPlayerController"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_player_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentCameraPose"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_current_camera_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCameraPose"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_camera_pose,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindEvaluationContextOwnerActor"),
                &raw mut __FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_find_evaluation_context_owner_actor,
            );
        }
    }
}
#[repr(C, align(4))]
pub struct FCameraContextDataID {
    pub(crate) __padding_end: [u8; 4],
}
impl FCameraContextDataID {}
#[repr(C, align(4))]
pub struct FCameraRigInstanceID {
    pub(crate) __padding_end: [u8; 8],
}
impl FCameraRigInstanceID {}
#[repr(C, align(4))]
pub struct FCameraShakeInstanceID {
    pub(crate) __padding_end: [u8; 4],
}
impl FCameraShakeInstanceID {}
#[repr(C, align(8))]
pub struct FBaseCameraObjectReference {
    pub(crate) __padding_end: [u8; 56],
}
impl FBaseCameraObjectReference {}
#[repr(C, align(8))]
pub struct FCameraAssetReference {
    pub(crate) __padding_end: [u8; 56],
}
impl FCameraAssetReference {}
#[repr(C, align(8))]
pub struct FBooleanCameraParameter {
    pub value: bool,
    pub(crate) __padding_end: [u8; 15],
}
impl FBooleanCameraParameter {}
#[repr(C, align(8))]
pub struct FInteger32CameraParameter {
    pub value: i32,
    pub(crate) __padding_end: [u8; 12],
}
impl FInteger32CameraParameter {}
#[repr(C, align(8))]
pub struct FFloatCameraParameter {
    pub value: f32,
    pub(crate) __padding_end: [u8; 12],
}
impl FFloatCameraParameter {}
#[repr(C, align(8))]
pub struct FDoubleCameraParameter {
    pub value: f64,
    pub(crate) __padding_end: [u8; 16],
}
impl FDoubleCameraParameter {}
#[repr(C, align(8))]
pub struct FVector2fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector2f,
    pub(crate) __padding_end: [u8; 16],
}
impl FVector2fCameraParameter {}
#[repr(C, align(8))]
pub struct FVector2dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector2D,
    pub(crate) __padding_end: [u8; 16],
}
impl FVector2dCameraParameter {}
#[repr(C, align(8))]
pub struct FVector3fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector3f,
    pub(crate) __padding_end: [u8; 12],
}
impl FVector3fCameraParameter {}
#[repr(C, align(8))]
pub struct FVector3dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector,
    pub(crate) __padding_end: [u8; 16],
}
impl FVector3dCameraParameter {}
#[repr(C, align(16))]
pub struct FVector4fCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4f,
    pub(crate) __padding_end: [u8; 16],
}
impl FVector4fCameraParameter {}
#[repr(C, align(16))]
pub struct FVector4dCameraParameter {
    pub value: crate::bindings::core_u_object::FVector4,
    pub(crate) __padding_end: [u8; 16],
}
impl FVector4dCameraParameter {}
#[repr(C, align(8))]
pub struct FRotator3fCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator3f,
    pub(crate) __padding_end: [u8; 12],
}
impl FRotator3fCameraParameter {}
#[repr(C, align(8))]
pub struct FRotator3dCameraParameter {
    pub value: crate::bindings::core_u_object::FRotator,
    pub(crate) __padding_end: [u8; 16],
}
impl FRotator3dCameraParameter {}
#[repr(C, align(16))]
pub struct FTransform3fCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform3f,
    pub(crate) __padding_end: [u8; 16],
}
impl FTransform3fCameraParameter {}
#[repr(C, align(16))]
pub struct FTransform3dCameraParameter {
    pub value: crate::bindings::core_u_object::FTransform,
    pub(crate) __padding_end: [u8; 16],
}
impl FTransform3dCameraParameter {}
#[repr(C, align(8))]
pub struct FCameraRigAssetReference {
    pub(crate) __padding_end: [u8; 304],
}
impl FCameraRigAssetReference {}
#[repr(C, align(8))]
pub struct FCameraShakeAssetReference {
    pub(crate) __padding_end: [u8; 64],
}
impl FCameraShakeAssetReference {}
#[repr(C, align(4))]
pub struct FCameraVariableSetterHandle {
    pub(crate) __padding_end: [u8; 8],
}
impl FCameraVariableSetterHandle {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorEvaluationParams {
    pub delta_time: f32,
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorEvaluationParams {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorActivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorActivateParams {}
#[repr(C, align(8))]
pub struct FBlueprintCameraDirectorDeactivateParams {
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
}
impl FBlueprintCameraDirectorDeactivateParams {}
#[repr(C, align(8))]
pub struct FCameraDirectorStateTreeEvaluationData {
    pub(crate) __padding_end: [u8; 32],
}
impl FCameraDirectorStateTreeEvaluationData {}
#[repr(C, align(8))]
pub struct FBlueprintCameraEvaluationDataRef {
    pub(crate) __padding_end: [u8; 24],
}
impl FBlueprintCameraEvaluationDataRef {}
#[repr(C, align(8))]
pub struct FBlueprintCameraPose {
    pub location: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub target_distance: f64,
    pub field_of_view: f32,
    pub focal_length: f32,
    pub orthographic_width: f32,
    pub aperture: f32,
    pub shutter_speed: f32,
    pub focus_distance: f32,
    pub sensor_width: f32,
    pub sensor_height: f32,
    pub sensor_horizontal_offset: f32,
    pub sensor_vertical_offset: f32,
    pub iso: f32,
    pub squeeze_factor: f32,
    pub overscan: f32,
    pub diaphragm_blade_count: i32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
    #[doc(hidden)]
    pub(crate) __padding_124: [u8; 4],
    pub enable_physical_camera: bool,
    pub constrain_aspect_ratio: bool,
    #[doc(hidden)]
    pub(crate) __padding_127: [u8; 1],
    pub aspect_ratio_axis_constraint: crate::bindings::engine::EAspectRatioAxisConstraint,
    pub projection_mode: crate::bindings::engine::ECameraProjectionMode,
}
impl FBlueprintCameraPose {}
#[repr(C, align(8))]
pub struct FCameraActorAttachmentInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub weight: f32,
}
impl FCameraActorAttachmentInfo {}
#[repr(C, align(8))]
pub struct FCameraActorTargetInfo {
    pub actor: UPtr<crate::bindings::engine::AActor>,
    pub socket_name: FName,
    pub bone_name: FName,
    pub target_shape: ECameraTargetShape,
    pub target_size: f32,
    pub weight: f32,
}
impl FCameraActorTargetInfo {}
#[repr(C, align(8))]
pub struct FCameraFramingZone {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl FCameraFramingZone {}
pub struct IHasCameraBuildStatus {}
#[repr(C, align(8))]
pub struct UHasCameraBuildStatus {
    __padding_end: [u8; 48],
}
impl UHasCameraBuildStatus {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHasCameraBuildStatus")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHasCameraBuildStatus")
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
pub struct UCameraRigInstanceFunctions {
    __padding_end: [u8; 48],
}
impl UCameraRigInstanceFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInstanceFunctions")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInstanceFunctions")
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
    pub fn is_valid(instance_id: FCameraRigInstanceID) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_instance_functions_is_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_id,
                __buffer.add(0).cast::<FCameraRigInstanceID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigInstanceFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_instance_functions_is_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
pub struct IAssetReferenceCameraNode {}
#[repr(C, align(8))]
pub struct UAssetReferenceCameraNode {
    __padding_end: [u8; 48],
}
impl UAssetReferenceCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetReferenceCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetReferenceCameraNode")
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
pub struct IObjectTreeGraphObject {}
#[repr(C, align(8))]
pub struct UObjectTreeGraphObject {
    __padding_end: [u8; 48],
}
impl UObjectTreeGraphObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphObject")
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
pub struct IObjectTreeGraphRootObject {}
#[repr(C, align(8))]
pub struct UObjectTreeGraphRootObject {
    __padding_end: [u8; 48],
}
impl UObjectTreeGraphRootObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphRootObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphRootObject")
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
pub struct UBaseCameraObject {
    __padding_end: [u8; 176],
}
impl UBaseCameraObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCameraObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseCameraObject")
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
pub struct UCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraNode")
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
pub struct UBlendCameraNode {
    __padding_end: [u8; 104],
}
impl UBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendCameraNode")
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
pub struct UBlendStackCameraNode {
    __padding_end: [u8; 112],
}
impl UBlendStackCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackCameraNode")
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
pub struct UBlendStackRootCameraNode {
    __padding_end: [u8; 120],
}
impl UBlendStackRootCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackRootCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendStackRootCameraNode")
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
pub struct UCameraAsset {
    __padding_end: [u8; 248],
}
impl UCameraAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAsset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraAsset")
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
pub struct UCameraDirector {
    __padding_end: [u8; 72],
}
impl UCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraDirector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraDirector")
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
pub struct UCameraObjectInterfaceParameterBase {
    __padding_end: [u8; 120],
}
impl UCameraObjectInterfaceParameterBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceParameterBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceParameterBase")
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
pub struct UCameraObjectInterfaceBlendableParameter {
    __padding_end: [u8; 152],
}
impl UCameraObjectInterfaceBlendableParameter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceBlendableParameter")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceBlendableParameter")
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
pub struct UCameraObjectInterfaceDataParameter {
    __padding_end: [u8; 144],
}
impl UCameraObjectInterfaceDataParameter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceDataParameter")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraObjectInterfaceDataParameter")
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
pub struct UCameraRigAsset {
    __padding_end: [u8; 400],
}
impl UCameraRigAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigAsset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigAsset")
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
pub struct UCombinedCameraRigsCameraNode {
    __padding_end: [u8; 120],
}
impl UCombinedCameraRigsCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombinedCameraRigsCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCombinedCameraRigsCameraNode")
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
pub struct UCameraRigProxyAsset {
    __padding_end: [u8; 64],
}
impl UCameraRigProxyAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigProxyAsset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigProxyAsset")
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
pub struct UCameraRigProxyTable {
    __padding_end: [u8; 64],
}
impl UCameraRigProxyTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigProxyTable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigProxyTable")
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
pub struct UCameraRigTransitionCondition {
    __padding_end: [u8; 88],
}
impl UCameraRigTransitionCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigTransitionCondition")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigTransitionCondition")
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
pub struct UCameraRigTransition {
    __padding_end: [u8; 120],
}
impl UCameraRigTransition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigTransition")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigTransition")
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
pub struct UCameraShakeAsset {
    __padding_end: [u8; 288],
}
impl UCameraShakeAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeAsset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeAsset")
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
pub struct UCameraValueInterpolator {
    __padding_end: [u8; 48],
}
impl UCameraValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraValueInterpolator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraValueInterpolator")
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
pub struct UPopValueInterpolator {
    __padding_end: [u8; 48],
}
impl UPopValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPopValueInterpolator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPopValueInterpolator")
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
pub struct UCameraVariableAsset {
    __padding_end: [u8; 88],
}
impl UCameraVariableAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraVariableAsset")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraVariableAsset")
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
pub struct UBooleanCameraVariable {
    __padding_end: [u8; 96],
}
impl UBooleanCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBooleanCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBooleanCameraVariable")
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
pub struct UInteger32CameraVariable {
    __padding_end: [u8; 96],
}
impl UInteger32CameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInteger32CameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInteger32CameraVariable")
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
pub struct UFloatCameraVariable {
    __padding_end: [u8; 96],
}
impl UFloatCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFloatCameraVariable")
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
pub struct UDoubleCameraVariable {
    __padding_end: [u8; 96],
}
impl UDoubleCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleCameraVariable")
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
pub struct UVector2fCameraVariable {
    __padding_end: [u8; 96],
}
impl UVector2fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector2fCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector2fCameraVariable")
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
pub struct UVector2dCameraVariable {
    __padding_end: [u8; 104],
}
impl UVector2dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector2dCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector2dCameraVariable")
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
pub struct UVector3fCameraVariable {
    __padding_end: [u8; 104],
}
impl UVector3fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector3fCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector3fCameraVariable")
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
pub struct UVector3dCameraVariable {
    __padding_end: [u8; 112],
}
impl UVector3dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector3dCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector3dCameraVariable")
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
pub struct UVector4fCameraVariable {
    __padding_end: [u8; 112],
}
impl UVector4fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector4fCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector4fCameraVariable")
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
pub struct UVector4dCameraVariable {
    __padding_end: [u8; 128],
}
impl UVector4dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector4dCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVector4dCameraVariable")
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
pub struct URotator3fCameraVariable {
    __padding_end: [u8; 104],
}
impl URotator3fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotator3fCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotator3fCameraVariable")
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
pub struct URotator3dCameraVariable {
    __padding_end: [u8; 112],
}
impl URotator3dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotator3dCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URotator3dCameraVariable")
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
pub struct UTransform3fCameraVariable {
    __padding_end: [u8; 144],
}
impl UTransform3fCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransform3fCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransform3fCameraVariable")
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
pub struct UTransform3dCameraVariable {
    __padding_end: [u8; 192],
}
impl UTransform3dCameraVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransform3dCameraVariable")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransform3dCameraVariable")
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
pub struct UCameraVariableCollection {
    __padding_end: [u8; 64],
}
impl UCameraVariableCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraVariableCollection")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraVariableCollection")
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
pub struct URootCameraNode {
    __padding_end: [u8; 104],
}
impl URootCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URootCameraNode")
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
pub struct UDefaultRootCameraNode {
    __padding_end: [u8; 136],
}
impl UDefaultRootCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDefaultRootCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDefaultRootCameraNode")
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
pub struct ICustomCameraNodeParameterProvider {}
#[repr(C, align(8))]
pub struct UCustomCameraNodeParameterProvider {
    __padding_end: [u8; 48],
}
impl UCustomCameraNodeParameterProvider {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomCameraNodeParameterProvider")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCustomCameraNodeParameterProvider")
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
pub struct UObjectTreeGraphComment {
    __padding_end: [u8; 104],
}
impl UObjectTreeGraphComment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphComment")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectTreeGraphComment")
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
pub struct UShakeCameraNode {
    __padding_end: [u8; 104],
}
impl UShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShakeCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShakeCameraNode")
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
pub struct UBlueprintCameraDirectorEvaluator {
    __padding_end: [u8; 336],
}
impl UBlueprintCameraDirectorEvaluator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraDirectorEvaluator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraDirectorEvaluator")
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
    pub fn run_child_camera_director(
        &mut self,
        delta_time: f32,
        child_slot_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_run_child_camera_director,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_slot_name,
                __buffer.add(4).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_run_child_camera_director,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn run_camera_director(
        &mut self,
        delta_time: f32,
        evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        params: &FBlueprintCameraDirectorEvaluationParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_run_camera_director,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&delta_time, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_context_owner,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(16).cast::<FBlueprintCameraDirectorEvaluationParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_run_camera_director,
                __buffer,
            )
        };
    }
    pub fn resolve_camera_rig_proxy(
        &self,
        camera_rig_proxy: UPtr<UCameraRigProxyAsset>,
    ) -> UPtr<UCameraRigAsset> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_resolve_camera_rig_proxy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_proxy,
                __buffer.add(0).cast::<UPtr<UCameraRigProxyAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_resolve_camera_rig_proxy,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UCameraRigAsset>>().read() }
    }
    pub fn remove_child_evaluation_context(
        &mut self,
        child_evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        child_slot_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_remove_child_evaluation_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_evaluation_context_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_slot_name,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_remove_child_evaluation_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_initial_context_result(&self) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_get_initial_context_result,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_get_initial_context_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn get_conditional_context_result(
        &self,
        condition: ECameraEvaluationDataCondition,
    ) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_get_conditional_context_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &condition,
                __buffer.add(0).cast::<ECameraEvaluationDataCondition>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_get_conditional_context_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn find_evaluation_context_owner_actor(
        &self,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_find_evaluation_context_owner_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(0).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_find_evaluation_context_owner_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
    pub fn deactivate_persistent_visual_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_visual_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_visual_camera_rig,
                __buffer,
            )
        };
    }
    pub fn deactivate_persistent_global_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_global_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_global_camera_rig,
                __buffer,
            )
        };
    }
    pub fn deactivate_persistent_base_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_base_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_persistent_base_camera_rig,
                __buffer,
            )
        };
    }
    pub fn deactivate_camera_director(
        &mut self,
        evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        params: &FBlueprintCameraDirectorDeactivateParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_camera_director,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_context_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(8).cast::<FBlueprintCameraDirectorDeactivateParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_deactivate_camera_director,
                __buffer,
            )
        };
    }
    pub fn add_child_evaluation_context(
        &mut self,
        child_evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_add_child_evaluation_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_evaluation_context_owner,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_add_child_evaluation_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FName>().read() }
    }
    pub fn activate_persistent_visual_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_visual_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_visual_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_global_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_global_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        &mut self,
        camera_rig_prefab: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_base_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_prefab,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_persistent_base_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_camera_rig_via_proxy(
        &mut self,
        camera_rig_proxy: UPtr<UCameraRigProxyAsset>,
        b_force_new_instance: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_rig_via_proxy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig_proxy,
                __buffer.add(0).cast::<UPtr<UCameraRigProxyAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_new_instance,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_rig_via_proxy,
                __buffer,
            )
        };
    }
    pub fn activate_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        b_force_new_instance: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_new_instance,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_camera_director(
        &mut self,
        evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
        params: &FBlueprintCameraDirectorActivateParams,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_director,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_context_owner,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                params,
                __buffer.add(8).cast::<FBlueprintCameraDirectorActivateParams>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_director_evaluator_activate_camera_director,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraDirector {
    __padding_end: [u8; 80],
}
impl UBlueprintCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraDirector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraDirector")
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
pub struct UCameraDirectorStateTreeSchema {
    __padding_end: [u8; 64],
}
impl UCameraDirectorStateTreeSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraDirectorStateTreeSchema")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraDirectorStateTreeSchema")
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
pub struct UPriorityQueueCameraDirector {
    __padding_end: [u8; 72],
}
impl UPriorityQueueCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPriorityQueueCameraDirector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPriorityQueueCameraDirector")
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
pub struct USingleCameraDirector {
    __padding_end: [u8; 80],
}
impl USingleCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleCameraDirector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USingleCameraDirector")
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
pub struct UStateTreeCameraDirector {
    __padding_end: [u8; 112],
}
impl UStateTreeCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeCameraDirector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStateTreeCameraDirector")
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
pub struct UActivateCameraRigFunctions {
    __padding_end: [u8; 48],
}
impl UActivateCameraRigFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActivateCameraRigFunctions")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActivateCameraRigFunctions")
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
    pub fn activate_persistent_visual_camera_rig(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_visual_camera_rig,
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
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(16).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UActivateCameraRigFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_visual_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_global_camera_rig,
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
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(16).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UActivateCameraRigFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_global_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_base_camera_rig,
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
                &player_controller,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(16).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UActivateCameraRigFunctions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_activate_camera_rig_functions_activate_persistent_base_camera_rig,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UCameraComponentCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraComponentCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraComponentCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraComponentCameraNode")
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
pub struct UCalcCameraActorCameraNode {
    __padding_end: [u8; 104],
}
impl UCalcCameraActorCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCalcCameraActorCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCalcCameraActorCameraNode")
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
pub struct UBlueprintCameraEvaluationDataFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraEvaluationDataFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraEvaluationDataFunctionLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraEvaluationDataFunctionLibrary")
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
    pub fn set_default_camera_rig_parameters(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_set_default_camera_rig_parameters,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(24).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_set_default_camera_rig_parameters,
                __buffer,
            )
        };
    }
    pub fn set_camera_pose(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_pose: &FBlueprintCameraPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_set_camera_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(24).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_set_camera_pose,
                __buffer,
            )
        };
    }
    pub fn make_camera_evaluation_data() -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_make_camera_evaluation_data,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_make_camera_evaluation_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn get_camera_pose(
        camera_data: &FBlueprintCameraEvaluationDataRef,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_get_camera_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_get_camera_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn blend_camera_evaluation_data(
        from_camera_data: &FBlueprintCameraEvaluationDataRef,
        to_camera_data: &FBlueprintCameraEvaluationDataRef,
        factor: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_blend_camera_evaluation_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from_camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to_camera_data,
                __buffer.add(24).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&factor, __buffer.add(48).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraEvaluationDataFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_evaluation_data_function_library_blend_camera_evaluation_data,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraVariableTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraVariableTableFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraVariableTableFunctionLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraVariableTableFunctionLibrary")
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
    pub fn set_vector4_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector4dCameraVariable>,
        value: &crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector4_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector4dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector4_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_vector3_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector3dCameraVariable>,
        value: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector3_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector3dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector3_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_vector2_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector2dCameraVariable>,
        value: &crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector2_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector2dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_vector2_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_transform_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UTransform3dCameraVariable>,
        value: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_transform_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UTransform3dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_transform_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_rotator_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<URotator3dCameraVariable>,
        value: &crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_rotator_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<URotator3dCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_rotator_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_integer32_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UInteger32CameraVariable>,
        value: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_integer32_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UInteger32CameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_integer32_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_float_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UFloatCameraVariable>,
        value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_float_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UFloatCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_float_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_double_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UDoubleCameraVariable>,
        value: f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_double_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UDoubleCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<f64>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_double_camera_variable,
                __buffer,
            )
        };
    }
    pub fn set_boolean_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UBooleanCameraVariable>,
        value: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_boolean_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UBooleanCameraVariable>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_set_boolean_camera_variable,
                __buffer,
            )
        };
    }
    pub fn get_vector4_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector4dCameraVariable>,
    ) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector4_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector4dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector4_camera_variable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn get_vector3_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector3dCameraVariable>,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector3_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector3dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector3_camera_variable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_vector2_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UVector2dCameraVariable>,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector2_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UVector2dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_vector2_camera_variable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_transform_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UTransform3dCameraVariable>,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_transform_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UTransform3dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_transform_camera_variable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_rotator_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<URotator3dCameraVariable>,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_rotator_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<URotator3dCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_rotator_camera_variable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_integer32_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UInteger32CameraVariable>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_integer32_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UInteger32CameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_integer32_camera_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<i32>().read() }
    }
    pub fn get_float_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UFloatCameraVariable>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_float_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UFloatCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_float_camera_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f32>().read() }
    }
    pub fn get_double_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UDoubleCameraVariable>,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_double_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UDoubleCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_double_camera_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<f64>().read() }
    }
    pub fn get_boolean_camera_variable(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        variable: UPtr<UBooleanCameraVariable>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_boolean_camera_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable,
                __buffer.add(24).cast::<UPtr<UBooleanCameraVariable>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraVariableTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_variable_table_function_library_get_boolean_camera_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraContextDataTableFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraContextDataTableFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraContextDataTableFunctionLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraContextDataTableFunctionLibrary")
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
    pub fn set_struct_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: &crate::bindings::core_u_object::FInstancedStruct,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_struct_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::core_u_object::FInstancedStruct>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_struct_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_string_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_string_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&data, __buffer.add(32).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_string_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn set_object_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_object_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_object_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_name_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_name_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(data, __buffer.add(28).cast::<FName>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_name_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_enum_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        enum_type: UPtr<crate::bindings::core_u_object::UEnum>,
        data: u8,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_enum_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enum_type,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UEnum>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&data, __buffer.add(40).cast::<u8>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_enum_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn set_class_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_class_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_set_class_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_struct_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        data_struct_type: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) -> crate::bindings::core_u_object::FInstancedStruct {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_struct_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_struct_type,
                __buffer
                    .add(32)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_struct_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::core_u_object::FInstancedStruct>()
                .read()
        }
    }
    pub fn get_string_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_string_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_string_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn get_object_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_object_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_object_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_name_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_name_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_name_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<FName>().read() }
    }
    pub fn get_enum_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
        enum_type: UPtr<crate::bindings::core_u_object::UEnum>,
    ) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_enum_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &enum_type,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UEnum>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_enum_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<u8>().read() }
    }
    pub fn get_class_data(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        data_id: FCameraContextDataID,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_class_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_id,
                __buffer.add(24).cast::<FCameraContextDataID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraContextDataTableFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_context_data_table_function_library_get_class_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraPoseFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UBlueprintCameraPoseFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraPoseFunctionLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraPoseFunctionLibrary")
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
    pub fn set_transform(
        camera_pose: &FBlueprintCameraPose,
        transform: &crate::bindings::core_u_object::FTransform,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<376>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                transform,
                __buffer.add(144).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(240).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_target_distance(
        camera_pose: &FBlueprintCameraPose,
        target_distance: f64,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<280>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_target_distance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_distance,
                __buffer.add(136).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_target_distance,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_rotation(
        camera_pose: &FBlueprintCameraPose,
        rotation: &crate::bindings::core_u_object::FRotator,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<296>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation,
                __buffer.add(136).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_rotation,
                __buffer,
            )
        };
        unsafe { __buffer.add(160).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_location(
        camera_pose: &FBlueprintCameraPose,
        location: &crate::bindings::core_u_object::FVector,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<296>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                location,
                __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_location,
                __buffer,
            )
        };
        unsafe { __buffer.add(160).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_focal_length(
        camera_pose: &FBlueprintCameraPose,
        focal_length: f32,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<280>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_focal_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &focal_length,
                __buffer.add(136).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_focal_length,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn set_field_of_view(
        camera_pose: &FBlueprintCameraPose,
        field_of_view: f32,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<280>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_field_of_view,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &field_of_view,
                __buffer.add(136).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_set_field_of_view,
                __buffer,
            )
        };
        unsafe { __buffer.add(144).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn make_camera_pose_from_cine_camera_component(
        camera_component: UPtr<crate::bindings::cinematic_camera::UCineCameraComponent>,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_make_camera_pose_from_cine_camera_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_component,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::cinematic_camera::UCineCameraComponent>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_make_camera_pose_from_cine_camera_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn make_camera_pose_from_camera_component(
        camera_component: UPtr<crate::bindings::engine::UCameraComponent>,
    ) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_make_camera_pose_from_camera_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_component,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UCameraComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_make_camera_pose_from_camera_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn get_transform(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(144).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_target_distance(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target_distance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target_distance,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_target_at_distance(
        camera_pose: &FBlueprintCameraPose,
        target_distance: f64,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target_at_distance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_distance,
                __buffer.add(136).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target_at_distance,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(144).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_target(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_target,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_sensor_aspect_ratio(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_sensor_aspect_ratio,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_sensor_aspect_ratio,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_rotation(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_rotation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_location(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_location,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_focal_length(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_focal_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_focal_length,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_field_of_view(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_field_of_view,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_field_of_view,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_effective_field_of_view(camera_pose: &FBlueprintCameraPose) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_effective_field_of_view,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_effective_field_of_view,
                __buffer,
            )
        };
        unsafe { __buffer.add(136).cast::<f64>().read() }
    }
    pub fn get_aim_ray(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FRay {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_aim_ray,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_aim_ray,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FRay>().read()
        }
    }
    pub fn get_aim_dir(
        camera_pose: &FBlueprintCameraPose,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_aim_dir,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UBlueprintCameraPoseFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_pose_function_library_get_aim_dir,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigParameterInterop {
    __padding_end: [u8; 48],
}
impl UCameraRigParameterInterop {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigParameterInterop")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigParameterInterop")
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
    pub fn set_camera_parameter(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_rig: UPtr<UCameraRigAsset>,
        parameter_name: FName,
        new_value: &i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_set_camera_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(24).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(new_value, __buffer.add(44).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInterop::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_set_camera_parameter,
                __buffer,
            )
        };
    }
    pub fn get_camera_parameter(
        camera_data: &FBlueprintCameraEvaluationDataRef,
        camera_rig: UPtr<UCameraRigAsset>,
        parameter_name: FName,
        return_value: &mut i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_get_camera_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(24).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                return_value,
                __buffer.add(44).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInterop::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_get_camera_parameter,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UCameraRigParameterInteropLibrary {
    __padding_end: [u8; 48],
}
impl UCameraRigParameterInteropLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigParameterInteropLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigParameterInteropLibrary")
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
    pub fn make_literal_vector3f(
        value: crate::bindings::core_u_object::FVector3f,
    ) -> crate::bindings::core_u_object::FVector3f {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector3f,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector3f>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector3f,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<crate::bindings::core_u_object::FVector3f>().read()
        }
    }
    pub fn make_literal_vector2_d(
        value: crate::bindings::core_u_object::FVector2D,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn make_literal_vector(
        value: crate::bindings::core_u_object::FVector,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn make_literal_rotator(
        value: crate::bindings::core_u_object::FRotator,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_rotator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn make_literal_linear_color(
        value: crate::bindings::core_u_object::FLinearColor,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_linear_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::gameplay_cameras::UCameraRigParameterInteropLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_camera_rig_parameter_interop_library_make_literal_linear_color,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UControllerGameplayCameraEvaluationComponent {
    __padding_end: [u8; 288],
}
impl UControllerGameplayCameraEvaluationComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControllerGameplayCameraEvaluationComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControllerGameplayCameraEvaluationComponent")
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
pub struct AGameplayCameraActorBase {
    __padding_end: [u8; 1136],
}
impl AGameplayCameraActorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraActorBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraActorBase")
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
pub struct AGameplayCameraActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub camera_component: UPtr<UGameplayCameraComponent>,
}
impl AGameplayCameraActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraActor")
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
    pub fn get_camera_component(&self) -> UPtr<UGameplayCameraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_camera_actor_get_camera_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_camera_actor_get_camera_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGameplayCameraComponent>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraComponentBase {
    #[doc(hidden)]
    pub(crate) __padding_688: [u8; 688],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_control_rotation_when_view_target: bool,
    __padding_end: [u8; 62],
}
impl UGameplayCameraComponentBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraComponentBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraComponentBase")
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
    pub fn get_output_camera_component(
        &self,
    ) -> UPtr<crate::bindings::cinematic_camera::UCineCameraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_output_camera_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_output_camera_component,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::cinematic_camera::UCineCameraComponent>>()
                .read()
        }
    }
    pub fn get_initial_result(&self) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_initial_result,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_initial_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn get_evaluated_camera_rotation(
        &self,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_evaluated_camera_rotation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_evaluated_camera_rotation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_conditional_result(
        &self,
        condition: ECameraEvaluationDataCondition,
    ) -> FBlueprintCameraEvaluationDataRef {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_conditional_result,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &condition,
                __buffer.add(0).cast::<ECameraEvaluationDataCondition>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_get_conditional_result,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FBlueprintCameraEvaluationDataRef>().read() }
    }
    pub fn deactivate_camera(&mut self, b_immediately: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_deactivate_camera,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_deactivate_camera,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_visual_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_visual_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_visual_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_global_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_global_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_base_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_persistent_base_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_camera_for_player_index(
        &mut self,
        player_index: i32,
        b_set_as_view_target: bool,
        activation_mode: EGameplayCameraComponentActivationMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_camera_for_player_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_as_view_target,
                __buffer.add(4).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &activation_mode,
                __buffer.add(5).cast::<EGameplayCameraComponentActivationMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_camera_for_player_index,
                __buffer,
            )
        };
    }
    pub fn activate_camera_for_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
        b_set_as_view_target: bool,
        activation_mode: EGameplayCameraComponentActivationMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_camera_for_player_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_as_view_target,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &activation_mode,
                __buffer.add(9).cast::<EGameplayCameraComponentActivationMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_component_base_activate_camera_for_player_controller,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraComponent {
    #[doc(hidden)]
    pub(crate) __padding_744: [u8; 744],
    pub camera_reference: FCameraAssetReference,
    __padding_end: [u8; 32],
}
impl UGameplayCameraComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraComponent")
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
pub struct UGameplayCameraParameterSetterComponent {
    #[doc(hidden)]
    pub(crate) __padding_240: [u8; 240],
    pub camera_rig_reference: FCameraRigAssetReference,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub blend_type: ECameraVariableSetterBlendType,
    __padding_end: [u8; 23],
}
impl UGameplayCameraParameterSetterComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraParameterSetterComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraParameterSetterComponent")
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
    pub fn stop_parameter_setters(&mut self, b_immediately: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_stop_parameter_setters,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_stop_parameter_setters,
                __buffer,
            )
        };
    }
    pub fn start_parameter_setters(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_start_parameter_setters,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_parameter_setter_component_start_parameter_setters,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct AGameplayCameraRigActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub camera_rig_component: UPtr<UGameplayCameraRigComponent>,
}
impl AGameplayCameraRigActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraRigActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraRigActor")
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
    pub fn get_camera_rig_component(&self) -> UPtr<UGameplayCameraRigComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_camera_rig_actor_get_camera_rig_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_camera_rig_actor_get_camera_rig_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGameplayCameraRigComponent>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraRigComponent {
    #[doc(hidden)]
    pub(crate) __padding_744: [u8; 744],
    pub camera_rig_reference: FCameraRigAssetReference,
    __padding_end: [u8; 40],
}
impl UGameplayCameraRigComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraRigComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraRigComponent")
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
pub struct AGameplayCamerasPlayerCameraManager {
    __padding_end: [u8; 11040],
}
impl AGameplayCamerasPlayerCameraManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCamerasPlayerCameraManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCamerasPlayerCameraManager")
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
    pub fn stop_camera_shake_asset(
        &mut self,
        in_instance_id: FCameraShakeInstanceID,
        b_immediately: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_stop_camera_shake_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_instance_id,
                __buffer.add(0).cast::<FCameraShakeInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_stop_camera_shake_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn stop_camera_modifier_rig(
        &mut self,
        instance_id: FCameraRigInstanceID,
        b_immediately: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_stop_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_id,
                __buffer.add(0).cast::<FCameraRigInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_stop_camera_modifier_rig,
                __buffer,
            )
        };
    }
    pub fn steal_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_steal_player_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_steal_player_controller,
                __buffer,
            )
        };
    }
    pub fn start_visual_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_visual_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_visual_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn start_global_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_global_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_global_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn start_camera_shake_asset(
        &mut self,
        camera_shake: UPtr<UCameraShakeAsset>,
        shake_scale: f32,
        play_space: crate::bindings::engine::ECameraShakePlaySpace,
        user_play_space_rotation: crate::bindings::core_u_object::FRotator,
    ) -> FCameraShakeInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_camera_shake_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_shake,
                __buffer.add(0).cast::<UPtr<UCameraShakeAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &shake_scale,
                __buffer.add(8).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &play_space,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::engine::ECameraShakePlaySpace>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_play_space_rotation,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_start_camera_shake_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FCameraShakeInstanceID>().read() }
    }
    pub fn release_player_controller(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_release_player_controller,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_release_player_controller,
                __buffer,
            )
        };
    }
    pub fn is_camera_shake_asset_playing(
        &self,
        in_instance_id: FCameraShakeInstanceID,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_is_camera_shake_asset_playing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_instance_id,
                __buffer.add(0).cast::<FCameraShakeInstanceID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_is_camera_shake_asset_playing,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn activate_persistent_visual_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_visual_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_visual_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_global_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_global_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_global_camera_rig,
                __buffer,
            )
        };
    }
    pub fn activate_persistent_base_camera_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_base_camera_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_cameras_player_camera_manager_activate_persistent_base_camera_rig,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct AGameplayCameraSystemActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub camera_system_component: UPtr<UGameplayCameraSystemComponent>,
}
impl AGameplayCameraSystemActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraSystemActor")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGameplayCameraSystemActor")
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
    pub fn get_camera_system_component(&self) -> UPtr<UGameplayCameraSystemComponent> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_camera_system_actor_get_camera_system_component,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .a_gameplay_camera_system_actor_get_camera_system_component,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UGameplayCameraSystemComponent>>().read() }
    }
}
#[repr(C, align(16))]
pub struct UGameplayCameraSystemComponent {
    #[doc(hidden)]
    pub(crate) __padding_688: [u8; 688],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    pub b_set_player_controller_rotation: bool,
}
impl UGameplayCameraSystemComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraSystemComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraSystemComponent")
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
    pub fn stop_camera_modifier_rig(
        &mut self,
        instance_id: FCameraRigInstanceID,
        b_immediately: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_stop_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &instance_id,
                __buffer.add(0).cast::<FCameraRigInstanceID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_immediately,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_stop_camera_modifier_rig,
                __buffer,
            )
        };
    }
    pub fn start_visual_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_start_visual_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_start_visual_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn start_global_camera_modifier_rig(
        &mut self,
        camera_rig: UPtr<UCameraRigAsset>,
        order_key: i32,
    ) -> FCameraRigInstanceID {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_start_global_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rig,
                __buffer.add(0).cast::<UPtr<UCameraRigAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&order_key, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_start_global_camera_modifier_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FCameraRigInstanceID>().read() }
    }
    pub fn is_camera_system_active_for_play_controller(
        &self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_is_camera_system_active_for_play_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_is_camera_system_active_for_play_controller,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn deactivate_camera_system(
        &mut self,
        next_view_target: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_deactivate_camera_system,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &next_view_target,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_deactivate_camera_system,
                __buffer,
            )
        };
    }
    pub fn activate_camera_system_for_player_index(&mut self, player_index: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_activate_camera_system_for_player_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_activate_camera_system_for_player_index,
                __buffer,
            )
        };
    }
    pub fn activate_camera_system_for_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_activate_camera_system_for_player_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_camera_system_component_activate_camera_system_for_player_controller,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UGameplayControlRotationComponent {
    #[doc(hidden)]
    pub(crate) __padding_264: [u8; 264],
    pub auto_activate_for_player: crate::bindings::engine::EAutoReceiveInput,
    __padding_end: [u8; 47],
}
impl UGameplayControlRotationComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayControlRotationComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayControlRotationComponent")
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
    pub fn deactivate_control_rotation_management(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_deactivate_control_rotation_management,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_deactivate_control_rotation_management,
                __buffer,
            )
        };
    }
    pub fn activate_control_rotation_management_for_player_index(
        &mut self,
        player_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_index,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_index,
                __buffer,
            )
        };
    }
    pub fn activate_control_rotation_management_for_player_controller(
        &mut self,
        player_controller: UPtr<crate::bindings::engine::APlayerController>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &player_controller,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::APlayerController>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_gameplay_control_rotation_component_activate_control_rotation_management_for_player_controller,
                __buffer,
            )
        };
    }
}
pub struct IGameplayCameraSystemHost {}
#[repr(C, align(8))]
pub struct UGameplayCameraSystemHost {
    __padding_end: [u8; 48],
}
impl UGameplayCameraSystemHost {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraSystemHost")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCameraSystemHost")
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
pub struct USimpleBlendCameraNode {
    __padding_end: [u8; 104],
}
impl USimpleBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleBlendCameraNode")
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
pub struct UViewTargetTransitionParamsBlendCameraNode {
    __padding_end: [u8; 120],
}
impl UViewTargetTransitionParamsBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewTargetTransitionParamsBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewTargetTransitionParamsBlendCameraNode")
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
pub struct UGameplayCamerasSettings {
    __padding_end: [u8; 152],
}
impl UGameplayCamerasSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCamerasSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayCamerasSettings")
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
pub struct UMovieSceneCameraFramingZonePropertySystem {
    __padding_end: [u8; 104],
}
impl UMovieSceneCameraFramingZonePropertySystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZonePropertySystem")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZonePropertySystem")
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
pub struct UMovieSceneCameraFramingZoneSection {
    __padding_end: [u8; 1616],
}
impl UMovieSceneCameraFramingZoneSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZoneSection")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZoneSection")
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
pub struct UMovieSceneCameraFramingZoneTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneCameraFramingZoneTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZoneTrack")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneCameraFramingZoneTrack")
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
pub struct UAttachToActorCameraNode {
    __padding_end: [u8; 184],
}
impl UAttachToActorCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToActorCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToActorCameraNode")
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
pub struct UAttachToActorGroupCameraNode {
    __padding_end: [u8; 128],
}
impl UAttachToActorGroupCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToActorGroupCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToActorGroupCameraNode")
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
pub struct UAttachToPlayerPawnCameraNode {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub socket_name: FName,
    pub bone_name: FName,
}
impl UAttachToPlayerPawnCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToPlayerPawnCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAttachToPlayerPawnCameraNode")
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
pub struct USimpleFixedTimeBlendCameraNode {
    __padding_end: [u8; 120],
}
impl USimpleFixedTimeBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleFixedTimeBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USimpleFixedTimeBlendCameraNode")
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
pub struct ULinearBlendCameraNode {
    __padding_end: [u8; 120],
}
impl ULinearBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearBlendCameraNode")
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
pub struct ULocationRotationBlendCameraNode {
    __padding_end: [u8; 128],
}
impl ULocationRotationBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationRotationBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULocationRotationBlendCameraNode")
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
pub struct UOrbitBlendCameraNode {
    __padding_end: [u8; 112],
}
impl UOrbitBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOrbitBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOrbitBlendCameraNode")
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
pub struct UPopBlendCameraNode {
    __padding_end: [u8; 104],
}
impl UPopBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPopBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPopBlendCameraNode")
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
pub struct USmoothBlendCameraNode {
    __padding_end: [u8; 128],
}
impl USmoothBlendCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothBlendCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USmoothBlendCameraNode")
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
pub struct UCollisionPushCameraNode {
    __padding_end: [u8; 240],
}
impl UCollisionPushCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollisionPushCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollisionPushCameraNode")
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
pub struct UOcclusionMaterialCameraNode {
    __padding_end: [u8; 176],
}
impl UOcclusionMaterialCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOcclusionMaterialCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOcclusionMaterialCameraNode")
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
pub struct UArrayCameraNode {
    __padding_end: [u8; 120],
}
impl UArrayCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UArrayCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UArrayCameraNode")
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
pub struct UAutoFocusCameraNode {
    __padding_end: [u8; 136],
}
impl UAutoFocusCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutoFocusCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutoFocusCameraNode")
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
pub struct UBodyParametersCameraNode {
    __padding_end: [u8; 136],
}
impl UBodyParametersCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBodyParametersCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBodyParametersCameraNode")
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
pub struct UBoomArmCameraNode {
    __padding_end: [u8; 208],
}
impl UBoomArmCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoomArmCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBoomArmCameraNode")
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
pub struct UCameraRigCameraNode {
    __padding_end: [u8; 424],
}
impl UCameraRigCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigCameraNode")
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
pub struct UClippingPlanesCameraNode {
    __padding_end: [u8; 152],
}
impl UClippingPlanesCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClippingPlanesCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClippingPlanesCameraNode")
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
pub struct UDampenPositionCameraNode {
    __padding_end: [u8; 160],
}
impl UDampenPositionCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDampenPositionCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDampenPositionCameraNode")
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
pub struct UDampenRotationCameraNode {
    __padding_end: [u8; 152],
}
impl UDampenRotationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDampenRotationCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDampenRotationCameraNode")
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
pub struct UFieldOfViewCameraNode {
    __padding_end: [u8; 120],
}
impl UFieldOfViewCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldOfViewCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFieldOfViewCameraNode")
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
pub struct UFilmbackCameraNode {
    __padding_end: [u8; 224],
}
impl UFilmbackCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFilmbackCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFilmbackCameraNode")
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
pub struct ULensParametersCameraNode {
    __padding_end: [u8; 168],
}
impl ULensParametersCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULensParametersCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULensParametersCameraNode")
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
pub struct UOffsetCameraNode {
    __padding_end: [u8; 192],
}
impl UOffsetCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOffsetCameraNode")
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
pub struct UOrthographicCameraNode {
    __padding_end: [u8; 136],
}
impl UOrthographicCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOrthographicCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UOrthographicCameraNode")
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
pub struct UPostProcessCameraNode {
    __padding_end: [u8; 2064],
}
impl UPostProcessCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPostProcessCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPostProcessCameraNode")
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
pub struct USetLocationCameraNode {
    __padding_end: [u8; 152],
}
impl USetLocationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetLocationCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetLocationCameraNode")
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
pub struct USetRotationCameraNode {
    __padding_end: [u8; 152],
}
impl USetRotationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetRotationCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USetRotationCameraNode")
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
pub struct USplineFieldOfViewCameraNode {
    __padding_end: [u8; 248],
}
impl USplineFieldOfViewCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineFieldOfViewCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineFieldOfViewCameraNode")
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
pub struct USplineOffsetCameraNode {
    __padding_end: [u8; 896],
}
impl USplineOffsetCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineOffsetCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineOffsetCameraNode")
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
pub struct USplineOrbitCameraNode {
    __padding_end: [u8; 1288],
}
impl USplineOrbitCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineOrbitCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USplineOrbitCameraNode")
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
pub struct UTargetRayCastCameraNode {
    __padding_end: [u8; 128],
}
impl UTargetRayCastCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetRayCastCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTargetRayCastCameraNode")
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
pub struct UBaseFramingCameraNode {
    __padding_end: [u8; 424],
}
impl UBaseFramingCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseFramingCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseFramingCameraNode")
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
pub struct UDollyFramingCameraNode {
    __padding_end: [u8; 456],
}
impl UDollyFramingCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDollyFramingCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDollyFramingCameraNode")
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
pub struct UPanningFramingCameraNode {
    __padding_end: [u8; 456],
}
impl UPanningFramingCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPanningFramingCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPanningFramingCameraNode")
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
pub struct UInput2DCameraNode {
    __padding_end: [u8; 104],
}
impl UInput2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInput2DCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInput2DCameraNode")
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
pub struct UAutoRotateInput2DCameraNode {
    __padding_end: [u8; 240],
}
impl UAutoRotateInput2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutoRotateInput2DCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutoRotateInput2DCameraNode")
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
pub struct UInput1DCameraNode {
    __padding_end: [u8; 104],
}
impl UInput1DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInput1DCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInput1DCameraNode")
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
pub struct UCameraRigInput1DSlot {
    __padding_end: [u8; 224],
}
impl UCameraRigInput1DSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInput1DSlot")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInput1DSlot")
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
pub struct UCameraRigInput2DSlot {
    __padding_end: [u8; 288],
}
impl UCameraRigInput2DSlot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInput2DSlot")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraRigInput2DSlot")
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
pub struct UDrivenControlRotationCameraNode {
    __padding_end: [u8; 104],
}
impl UDrivenControlRotationCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrivenControlRotationCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDrivenControlRotationCameraNode")
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
pub struct UInputAccumulator2DCameraNode {
    __padding_end: [u8; 296],
}
impl UInputAccumulator2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputAccumulator2DCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputAccumulator2DCameraNode")
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
pub struct UInputAxisBinding2DCameraNode {
    __padding_end: [u8; 344],
}
impl UInputAxisBinding2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputAxisBinding2DCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInputAxisBinding2DCameraNode")
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
pub struct URawInputAxisBinding2DCameraNode {
    __padding_end: [u8; 152],
}
impl URawInputAxisBinding2DCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URawInputAxisBinding2DCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URawInputAxisBinding2DCameraNode")
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
pub struct UCameraShakeCameraNode {
    __padding_end: [u8; 184],
}
impl UCameraShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeCameraNode")
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
pub struct UCompositeShakeCameraNode {
    __padding_end: [u8; 120],
}
impl UCompositeShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeShakeCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeShakeCameraNode")
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
pub struct UEnvelopeShakeCameraNode {
    __padding_end: [u8; 160],
}
impl UEnvelopeShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvelopeShakeCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnvelopeShakeCameraNode")
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
pub struct UPerlinNoiseLocationShakeCameraNode {
    __padding_end: [u8; 176],
}
impl UPerlinNoiseLocationShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPerlinNoiseLocationShakeCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPerlinNoiseLocationShakeCameraNode")
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
pub struct UPerlinNoiseRotationShakeCameraNode {
    __padding_end: [u8; 176],
}
impl UPerlinNoiseRotationShakeCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPerlinNoiseRotationShakeCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPerlinNoiseRotationShakeCameraNode")
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
pub struct UBlueprintCameraNodeEvaluator {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub b_is_first_frame: bool,
    pub b_is_active_camera_rig: bool,
    pub evaluation_context_owner: UPtr<crate::bindings::core_u_object::UObject>,
    pub camera_data: FBlueprintCameraEvaluationDataRef,
    pub camera_pose: FBlueprintCameraPose,
    pub variable_table: FBlueprintCameraEvaluationDataRef,
    __padding_end: [u8; 32],
}
impl UBlueprintCameraNodeEvaluator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraNodeEvaluator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraNodeEvaluator")
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
    pub fn tick_camera_node(&mut self, delta_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_tick_camera_node,
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
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_tick_camera_node,
                __buffer,
            )
        };
    }
    pub fn set_default_owning_camera_rig_parameters(
        &self,
        target_camera_data: FBlueprintCameraEvaluationDataRef,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_default_owning_camera_rig_parameters,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_camera_data,
                __buffer.add(0).cast::<FBlueprintCameraEvaluationDataRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_default_owning_camera_rig_parameters,
                __buffer,
            )
        };
    }
    pub fn set_current_camera_pose(&mut self, camera_pose: &FBlueprintCameraPose) {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_current_camera_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_current_camera_pose,
                __buffer,
            )
        };
    }
    pub fn set_camera_pose(&mut self, in_camera_pose: &FBlueprintCameraPose) {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_camera_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_camera_pose,
                __buffer.add(0).cast::<FBlueprintCameraPose>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_set_camera_pose,
                __buffer,
            )
        };
    }
    pub fn initialize_camera_node(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_initialize_camera_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_initialize_camera_node,
                __buffer,
            )
        };
    }
    pub fn get_player_controller(
        &self,
    ) -> UPtr<crate::bindings::engine::APlayerController> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_player_controller,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_player_controller,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::engine::APlayerController>>()
                .read()
        }
    }
    pub fn get_current_camera_pose(&self) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_current_camera_pose,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_current_camera_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn get_camera_pose(&self) -> FBlueprintCameraPose {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_camera_pose,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_get_camera_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FBlueprintCameraPose>().read() }
    }
    pub fn find_evaluation_context_owner_actor(
        &self,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_find_evaluation_context_owner_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(0).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::gameplay_cameras::__FUNCTION_PTRS
                    .u_blueprint_camera_node_evaluator_find_evaluation_context_owner_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintCameraNode {
    __padding_end: [u8; 160],
}
impl UBlueprintCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintCameraNode")
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
pub struct UCameraShakeServiceCameraNode {
    __padding_end: [u8; 104],
}
impl UCameraShakeServiceCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeServiceCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCameraShakeServiceCameraNode")
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
pub struct UUpdateTrackerCameraNode {
    __padding_end: [u8; 168],
}
impl UUpdateTrackerCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUpdateTrackerCameraNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUpdateTrackerCameraNode")
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
pub struct UFixedTestCameraDirector {
    __padding_end: [u8; 104],
}
impl UFixedTestCameraDirector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedTestCameraDirector")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixedTestCameraDirector")
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
pub struct UIsCameraRigTransitionCondition {
    __padding_end: [u8; 104],
}
impl UIsCameraRigTransitionCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIsCameraRigTransitionCondition")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIsCameraRigTransitionCondition")
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
pub struct UGameplayTagTransitionCondition {
    __padding_end: [u8; 232],
}
impl UGameplayTagTransitionCondition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagTransitionCondition")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGameplayTagTransitionCondition")
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
pub struct UAccelerationDecelerationValueInterpolator {
    __padding_end: [u8; 64],
}
impl UAccelerationDecelerationValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAccelerationDecelerationValueInterpolator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAccelerationDecelerationValueInterpolator")
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
pub struct UCriticalDamperValueInterpolator {
    __padding_end: [u8; 56],
}
impl UCriticalDamperValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCriticalDamperValueInterpolator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCriticalDamperValueInterpolator")
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
pub struct UDoubleIIRValueInterpolator {
    __padding_end: [u8; 64],
}
impl UDoubleIIRValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleIIRValueInterpolator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDoubleIIRValueInterpolator")
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
pub struct UIIRValueInterpolator {
    __padding_end: [u8; 56],
}
impl UIIRValueInterpolator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIIRValueInterpolator")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIIRValueInterpolator")
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
#[repr(transparent)]
pub struct ECameraObjectInterfaceParameterType(pub u8);
impl ECameraObjectInterfaceParameterType {
    pub const BLENDABLE: ECameraObjectInterfaceParameterType = ECameraObjectInterfaceParameterType(
        0,
    );
    pub const DATA: ECameraObjectInterfaceParameterType = ECameraObjectInterfaceParameterType(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraVariableType(pub i32);
impl ECameraVariableType {
    pub const BOOLEAN: ECameraVariableType = ECameraVariableType(0);
    pub const INTEGER32: ECameraVariableType = ECameraVariableType(1);
    pub const FLOAT: ECameraVariableType = ECameraVariableType(2);
    pub const DOUBLE: ECameraVariableType = ECameraVariableType(3);
    pub const VECTOR2F: ECameraVariableType = ECameraVariableType(4);
    pub const VECTOR2D: ECameraVariableType = ECameraVariableType(5);
    pub const VECTOR3F: ECameraVariableType = ECameraVariableType(6);
    pub const VECTOR3D: ECameraVariableType = ECameraVariableType(7);
    pub const VECTOR4F: ECameraVariableType = ECameraVariableType(8);
    pub const VECTOR4D: ECameraVariableType = ECameraVariableType(9);
    pub const ROTATOR3F: ECameraVariableType = ECameraVariableType(10);
    pub const ROTATOR3D: ECameraVariableType = ECameraVariableType(11);
    pub const TRANSFORM3F: ECameraVariableType = ECameraVariableType(12);
    pub const TRANSFORM3D: ECameraVariableType = ECameraVariableType(13);
    pub const BLENDABLE_STRUCT: ECameraVariableType = ECameraVariableType(14);
}
#[repr(transparent)]
pub struct ECameraContextDataType(pub i32);
impl ECameraContextDataType {
    pub const NAME: ECameraContextDataType = ECameraContextDataType(0);
    pub const STRING: ECameraContextDataType = ECameraContextDataType(1);
    pub const ENUM: ECameraContextDataType = ECameraContextDataType(2);
    pub const STRUCT: ECameraContextDataType = ECameraContextDataType(3);
    pub const OBJECT: ECameraContextDataType = ECameraContextDataType(4);
    pub const CLASS: ECameraContextDataType = ECameraContextDataType(5);
    pub const COUNT: ECameraContextDataType = ECameraContextDataType(6);
}
#[repr(transparent)]
pub struct ECameraContextDataContainerType(pub i32);
impl ECameraContextDataContainerType {
    pub const NONE: ECameraContextDataContainerType = ECameraContextDataContainerType(0);
    pub const ARRAY: ECameraContextDataContainerType = ECameraContextDataContainerType(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraRigLayer(pub u8);
impl ECameraRigLayer {
    pub const NONE: ECameraRigLayer = ECameraRigLayer(0);
    pub const BASE: ECameraRigLayer = ECameraRigLayer(1);
    pub const MAIN: ECameraRigLayer = ECameraRigLayer(2);
    pub const GLOBAL: ECameraRigLayer = ECameraRigLayer(3);
    pub const VISUAL: ECameraRigLayer = ECameraRigLayer(4);
}
#[repr(transparent)]
pub struct ECameraTargetShape(pub u8);
impl ECameraTargetShape {
    pub const POINT: ECameraTargetShape = ECameraTargetShape(0);
    pub const AUTOMATIC_BOUNDS: ECameraTargetShape = ECameraTargetShape(1);
    pub const MANUAL_BOUNDS: ECameraTargetShape = ECameraTargetShape(2);
}
#[repr(transparent)]
pub struct ECameraEvaluationDataCondition(pub u8);
impl ECameraEvaluationDataCondition {
    pub const ACTIVE_CAMERA_RIG: ECameraEvaluationDataCondition = ECameraEvaluationDataCondition(
        0,
    );
}
#[repr(transparent)]
pub struct EGameplayCameraComponentActivationMode(pub u8);
impl EGameplayCameraComponentActivationMode {
    pub const PUSH: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        0,
    );
    pub const PUSH_AND_INSERT: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        1,
    );
    pub const INSERT_OR_PUSH: EGameplayCameraComponentActivationMode = EGameplayCameraComponentActivationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ECameraBlendStackType(pub i32);
impl ECameraBlendStackType {
    pub const ISOLATED_TRANSIENT: ECameraBlendStackType = ECameraBlendStackType(0);
    pub const ADDITIVE_PERSISTENT: ECameraBlendStackType = ECameraBlendStackType(1);
}
#[repr(transparent)]
pub struct ECameraBuildStatus(pub u8);
impl ECameraBuildStatus {
    pub const CLEAN: ECameraBuildStatus = ECameraBuildStatus(0);
    pub const CLEAN_WITH_WARNINGS: ECameraBuildStatus = ECameraBuildStatus(1);
    pub const WITH_ERRORS: ECameraBuildStatus = ECameraBuildStatus(2);
    pub const DIRTY: ECameraBuildStatus = ECameraBuildStatus(3);
}
#[repr(transparent)]
pub struct ECameraRigInitialOrientation(pub i32);
impl ECameraRigInitialOrientation {
    pub const NONE: ECameraRigInitialOrientation = ECameraRigInitialOrientation(0);
    pub const CONTEXT_YAW_PITCH: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        1,
    );
    pub const PREVIOUS_YAW_PITCH: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        2,
    );
    pub const PREVIOUS_ABSOLUTE_TARGET: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        3,
    );
    pub const PREVIOUS_RELATIVE_TARGET: ECameraRigInitialOrientation = ECameraRigInitialOrientation(
        4,
    );
}
#[repr(transparent)]
pub struct ECameraVariableSetterBlendType(pub u8);
impl ECameraVariableSetterBlendType {
    pub const NONE: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(0);
    pub const LINEAR: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(1);
    pub const SMOOTH_STEP: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(
        2,
    );
    pub const SMOOTHER_STEP: ECameraVariableSetterBlendType = ECameraVariableSetterBlendType(
        3,
    );
}
#[repr(transparent)]
pub struct EGameplayCamerasViewRotationMode(pub i32);
impl EGameplayCamerasViewRotationMode {
    pub const NONE: EGameplayCamerasViewRotationMode = EGameplayCamerasViewRotationMode(
        0,
    );
    pub const PREVIEW_UPDATE: EGameplayCamerasViewRotationMode = EGameplayCamerasViewRotationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ESmoothCameraBlendType(pub i32);
impl ESmoothCameraBlendType {
    pub const SMOOTH_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(0);
    pub const SMOOTHER_STEP: ESmoothCameraBlendType = ESmoothCameraBlendType(1);
}
#[repr(transparent)]
pub struct ECollisionSafePosition(pub u8);
impl ECollisionSafePosition {
    pub const ACTIVE_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(0);
    pub const OWNING_CONTEXT: ECollisionSafePosition = ECollisionSafePosition(1);
    pub const PIVOT: ECollisionSafePosition = ECollisionSafePosition(2);
    pub const PAWN: ECollisionSafePosition = ECollisionSafePosition(3);
}
#[repr(transparent)]
pub struct ECollisionSafePositionOffsetSpace(pub u8);
impl ECollisionSafePositionOffsetSpace {
    pub const ACTIVE_CONTEXT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        0,
    );
    pub const OWNING_CONTEXT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        1,
    );
    pub const PIVOT: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        2,
    );
    pub const CAMERA_POSE: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        3,
    );
    pub const PAWN: ECollisionSafePositionOffsetSpace = ECollisionSafePositionOffsetSpace(
        4,
    );
}
#[repr(transparent)]
pub struct ECameraNodeOriginPosition(pub u8);
impl ECameraNodeOriginPosition {
    pub const CAMERA_POSE: ECameraNodeOriginPosition = ECameraNodeOriginPosition(0);
    pub const ACTIVE_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(1);
    pub const OWNING_CONTEXT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(2);
    pub const PIVOT: ECameraNodeOriginPosition = ECameraNodeOriginPosition(3);
    pub const PAWN: ECameraNodeOriginPosition = ECameraNodeOriginPosition(4);
}
#[repr(transparent)]
pub struct ECameraNodeSpace(pub u8);
impl ECameraNodeSpace {
    pub const CAMERA_POSE: ECameraNodeSpace = ECameraNodeSpace(0);
    pub const ACTIVE_CONTEXT: ECameraNodeSpace = ECameraNodeSpace(1);
    pub const OWNING_CONTEXT: ECameraNodeSpace = ECameraNodeSpace(2);
    pub const PIVOT: ECameraNodeSpace = ECameraNodeSpace(3);
    pub const PAWN: ECameraNodeSpace = ECameraNodeSpace(4);
    pub const WORLD: ECameraNodeSpace = ECameraNodeSpace(5);
}
#[repr(transparent)]
pub struct ECameraAutoRotateDirection(pub i32);
impl ECameraAutoRotateDirection {
    pub const FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(0);
    pub const MOVEMENT: ECameraAutoRotateDirection = ECameraAutoRotateDirection(1);
    pub const MOVEMENT_OR_FACING: ECameraAutoRotateDirection = ECameraAutoRotateDirection(
        2,
    );
}
#[repr(transparent)]
pub struct EBuiltInDoubleCameraVariable(pub i32);
impl EBuiltInDoubleCameraVariable {
    pub const NONE: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(0);
    pub const YAW: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(1);
    pub const PITCH: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(2);
    pub const ROLL: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(3);
    pub const ZOOM: EBuiltInDoubleCameraVariable = EBuiltInDoubleCameraVariable(4);
}
#[repr(transparent)]
pub struct EBuiltInVector2dCameraVariable(pub i32);
impl EBuiltInVector2dCameraVariable {
    pub const NONE: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(0);
    pub const YAW_PITCH: EBuiltInVector2dCameraVariable = EBuiltInVector2dCameraVariable(
        1,
    );
}
#[repr(transparent)]
pub struct ECameraShakeEvaluationMode(pub u8);
impl ECameraShakeEvaluationMode {
    pub const INLINE: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(0);
    pub const VISUAL_LAYER: ECameraShakeEvaluationMode = ECameraShakeEvaluationMode(1);
}
