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
    pub u_meta_sound_asset_subsystem_unregister_asset_classes_in_directories: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_asset_subsystem_replace_references_in_directory: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_asset_subsystem_register_asset_classes_in_directories: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_asset_subsystem_reassign_class_name: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_asset_subsystem_find_referencing_asset_class_info: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_asset_subsystem_find_asset_class_info: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_node_input_default: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_graph_output_name: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_graph_output_data_type: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_graph_output_access_type: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_graph_input_name: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_graph_input_default: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_graph_input_data_type: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_set_graph_input_access_type: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_remove_unused_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_remove_node_input_default: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_remove_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_remove_interface: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_remove_graph_variable: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_remove_graph_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_remove_graph_input: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_nodes_are_connected: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_node_output_is_connected: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_node_input_is_connected: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_is_preset: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_interface_is_declared: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_root_graph_class_name: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_referenced_preset_asset: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_node_output_is_constructor_pin: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_node_output_data: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_node_input_is_constructor_pin: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_node_input_default: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_node_input_data: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_node_input_class_default: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_graph_variable_default: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_graph_output_names: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_graph_input_names: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_get_graph_input_default: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_outputs_by_data_type: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_outputs: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_output_parent: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_output_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_inputs_by_data_type: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_inputs: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_input_parent: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_input_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_node_class_version: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_interface_output_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_interface_input_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_graph_output_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_find_graph_input_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_disconnect_nodes_by_interface_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_disconnect_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_disconnect_node_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_disconnect_node_input: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_convert_to_preset: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_convert_from_preset: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_contains_node_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_contains_node_input: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_contains_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_nodes_by_interface_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_node_output_to_graph_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_node_outputs_to_matching_graph_interface_outputs: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_node_input_to_graph_input: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_node_inputs_to_matching_graph_interface_inputs: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_named_node_output_to_named_graph_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_connect_named_node_output_to_graph_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_build_new_meta_sound: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_build_and_overwrite_meta_sound: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_build: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_node_by_class_name: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_interface: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_graph_variable_set_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_graph_variable_get_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_graph_variable_get_delayed_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_graph_variable: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_graph_output_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_base_add_graph_input_node: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_source_builder_set_sample_rate_override: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_source_builder_set_quality: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_source_builder_set_format: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_source_builder_set_block_rate_override: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_source_builder_get_live_updates_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_source_builder_audition: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_unregister_source_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_unregister_patch_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_unregister_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_set_target_page: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_register_source_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_register_patch_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_register_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_is_interface_registered: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_find_source_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_find_patch_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_find_parent_builder_of_preset: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_find_builder_of_document: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_find_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_string_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_string_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_source_preset_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_source_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_patch_preset_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_patch_builder: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_object_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_object_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_meta_sound_literal_from_param: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_int_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_int_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_float_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_float_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_bool_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_builder_subsystem_create_bool_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_type: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_string_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_string_array_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_object_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_object_array_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_int_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_int_array_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_float_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_float_array_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_bool_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_get_bool_array_value_from_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_equal_equal_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_string_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_string_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_object_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_object_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_param: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean_array: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_audio_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_int_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_int_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_float_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_float_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_bool_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_create_bool_array_meta_sound_literal: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_frontend_literal_blueprint_access_conv_meta_sound_literal_to_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_generator_handle_watch_output: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_generator_handle_update_watchers: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_generator_handle_get_cpu_core_utilization: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_generator_handle_enable_runtime_render_timing: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_generator_handle_create_meta_sound_generator_handle: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_generator_handle_apply_parameter_pack: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_cache_subsystem_touch_or_precache_meta_sound: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_cache_subsystem_remove_cached_operators_for_meta_sound: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_cache_subsystem_precache_meta_sound: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_is_time: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_is_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_is_int32: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_is_float: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_is_bool: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_get_time_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_get_string: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_get_int32: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_get_float: *mut crate::ffi::UFunctionOpague,
    pub u_metasound_output_blueprint_access_get_bool: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_output_subsystem_watch_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_output_subsystem_unwatch_output: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_quality_helper_get_quality_names: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_settings_get_quality_names: *mut crate::ffi::UFunctionOpague,
    pub u_meta_sound_settings_get_page_names: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_meta_sound_asset_subsystem_unregister_asset_classes_in_directories: std::ptr::null_mut(),
            u_meta_sound_asset_subsystem_replace_references_in_directory: std::ptr::null_mut(),
            u_meta_sound_asset_subsystem_register_asset_classes_in_directories: std::ptr::null_mut(),
            u_meta_sound_asset_subsystem_reassign_class_name: std::ptr::null_mut(),
            u_meta_sound_asset_subsystem_find_referencing_asset_class_info: std::ptr::null_mut(),
            u_meta_sound_asset_subsystem_find_asset_class_info: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_node_input_default: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_graph_output_name: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_graph_output_data_type: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_graph_output_access_type: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_graph_input_name: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_graph_input_default: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_graph_input_data_type: std::ptr::null_mut(),
            u_meta_sound_builder_base_set_graph_input_access_type: std::ptr::null_mut(),
            u_meta_sound_builder_base_remove_unused_dependencies: std::ptr::null_mut(),
            u_meta_sound_builder_base_remove_node_input_default: std::ptr::null_mut(),
            u_meta_sound_builder_base_remove_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_remove_interface: std::ptr::null_mut(),
            u_meta_sound_builder_base_remove_graph_variable: std::ptr::null_mut(),
            u_meta_sound_builder_base_remove_graph_output: std::ptr::null_mut(),
            u_meta_sound_builder_base_remove_graph_input: std::ptr::null_mut(),
            u_meta_sound_builder_base_nodes_are_connected: std::ptr::null_mut(),
            u_meta_sound_builder_base_node_output_is_connected: std::ptr::null_mut(),
            u_meta_sound_builder_base_node_input_is_connected: std::ptr::null_mut(),
            u_meta_sound_builder_base_is_preset: std::ptr::null_mut(),
            u_meta_sound_builder_base_interface_is_declared: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_root_graph_class_name: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_referenced_preset_asset: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_node_output_is_constructor_pin: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_node_output_data: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_node_input_is_constructor_pin: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_node_input_default: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_node_input_data: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_node_input_class_default: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_graph_variable_default: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_graph_output_names: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_graph_input_names: std::ptr::null_mut(),
            u_meta_sound_builder_base_get_graph_input_default: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_outputs_by_data_type: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_outputs: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_output_parent: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_output_by_name: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_inputs_by_data_type: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_inputs: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_input_parent: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_input_by_name: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_node_class_version: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_interface_output_nodes: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_interface_input_nodes: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_graph_output_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_find_graph_input_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_disconnect_nodes_by_interface_bindings: std::ptr::null_mut(),
            u_meta_sound_builder_base_disconnect_nodes: std::ptr::null_mut(),
            u_meta_sound_builder_base_disconnect_node_output: std::ptr::null_mut(),
            u_meta_sound_builder_base_disconnect_node_input: std::ptr::null_mut(),
            u_meta_sound_builder_base_convert_to_preset: std::ptr::null_mut(),
            u_meta_sound_builder_base_convert_from_preset: std::ptr::null_mut(),
            u_meta_sound_builder_base_contains_node_output: std::ptr::null_mut(),
            u_meta_sound_builder_base_contains_node_input: std::ptr::null_mut(),
            u_meta_sound_builder_base_contains_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_nodes_by_interface_bindings: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_nodes: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_node_output_to_graph_output: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_node_outputs_to_matching_graph_interface_outputs: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_node_input_to_graph_input: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_node_inputs_to_matching_graph_interface_inputs: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_named_node_output_to_named_graph_output: std::ptr::null_mut(),
            u_meta_sound_builder_base_connect_named_node_output_to_graph_output: std::ptr::null_mut(),
            u_meta_sound_builder_base_build_new_meta_sound: std::ptr::null_mut(),
            u_meta_sound_builder_base_build_and_overwrite_meta_sound: std::ptr::null_mut(),
            u_meta_sound_builder_base_build: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_node_by_class_name: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_interface: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_graph_variable_set_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_graph_variable_get_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_graph_variable_get_delayed_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_graph_variable: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_graph_output_node: std::ptr::null_mut(),
            u_meta_sound_builder_base_add_graph_input_node: std::ptr::null_mut(),
            u_meta_sound_source_builder_set_sample_rate_override: std::ptr::null_mut(),
            u_meta_sound_source_builder_set_quality: std::ptr::null_mut(),
            u_meta_sound_source_builder_set_format: std::ptr::null_mut(),
            u_meta_sound_source_builder_set_block_rate_override: std::ptr::null_mut(),
            u_meta_sound_source_builder_get_live_updates_enabled: std::ptr::null_mut(),
            u_meta_sound_source_builder_audition: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_unregister_source_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_unregister_patch_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_unregister_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_set_target_page: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_register_source_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_register_patch_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_register_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_is_interface_registered: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_find_source_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_find_patch_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_find_parent_builder_of_preset: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_find_builder_of_document: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_find_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_string_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_string_array_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_source_preset_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_source_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_patch_preset_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_patch_builder: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_object_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_object_array_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_meta_sound_literal_from_param: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_int_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_int_array_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_float_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_float_array_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_bool_meta_sound_literal: std::ptr::null_mut(),
            u_meta_sound_builder_subsystem_create_bool_array_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_type: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_string_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_string_array_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_object_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_object_array_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_int_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_int_array_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_float_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_float_array_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_bool_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_get_bool_array_value_from_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_equal_equal_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_string_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_string_array_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_object_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_object_array_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_param: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean_array: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_audio_parameter: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_int_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_int_array_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_float_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_float_array_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_bool_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_create_bool_array_meta_sound_literal: std::ptr::null_mut(),
            u_metasound_frontend_literal_blueprint_access_conv_meta_sound_literal_to_string: std::ptr::null_mut(),
            u_metasound_generator_handle_watch_output: std::ptr::null_mut(),
            u_metasound_generator_handle_update_watchers: std::ptr::null_mut(),
            u_metasound_generator_handle_get_cpu_core_utilization: std::ptr::null_mut(),
            u_metasound_generator_handle_enable_runtime_render_timing: std::ptr::null_mut(),
            u_metasound_generator_handle_create_meta_sound_generator_handle: std::ptr::null_mut(),
            u_metasound_generator_handle_apply_parameter_pack: std::ptr::null_mut(),
            u_meta_sound_cache_subsystem_touch_or_precache_meta_sound: std::ptr::null_mut(),
            u_meta_sound_cache_subsystem_remove_cached_operators_for_meta_sound: std::ptr::null_mut(),
            u_meta_sound_cache_subsystem_precache_meta_sound: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_is_time: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_is_string: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_is_int32: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_is_float: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_is_bool: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_get_time_seconds: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_get_string: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_get_int32: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_get_float: std::ptr::null_mut(),
            u_metasound_output_blueprint_access_get_bool: std::ptr::null_mut(),
            u_meta_sound_output_subsystem_watch_output: std::ptr::null_mut(),
            u_meta_sound_output_subsystem_unwatch_output: std::ptr::null_mut(),
            u_meta_sound_quality_helper_get_quality_names: std::ptr::null_mut(),
            u_meta_sound_settings_get_quality_names: std::ptr::null_mut(),
            u_meta_sound_settings_get_page_names: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundAssetSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterAssetClassesInDirectories"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_asset_subsystem_unregister_asset_classes_in_directories,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceReferencesInDirectory"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_asset_subsystem_replace_references_in_directory,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterAssetClassesInDirectories"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_asset_subsystem_register_asset_classes_in_directories,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReassignClassName"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_asset_subsystem_reassign_class_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindReferencingAssetClassInfo"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_asset_subsystem_find_referencing_asset_class_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAssetClassInfo"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_asset_subsystem_find_asset_class_info,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundBuilderBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNodeInputDefault"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_set_node_input_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphOutputName"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_set_graph_output_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphOutputDataType"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_set_graph_output_data_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphOutputAccessType"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_set_graph_output_access_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphInputName"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_set_graph_input_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphInputDefault"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_set_graph_input_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphInputDataType"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_set_graph_input_data_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGraphInputAccessType"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_set_graph_input_access_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveUnusedDependencies"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_remove_unused_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNodeInputDefault"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_remove_node_input_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveNode"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_remove_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveInterface"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_remove_interface,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGraphVariable"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_remove_graph_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGraphOutput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_remove_graph_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveGraphInput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_remove_graph_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NodesAreConnected"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_nodes_are_connected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NodeOutputIsConnected"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_node_output_is_connected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NodeInputIsConnected"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_node_input_is_connected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPreset"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_is_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InterfaceIsDeclared"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_interface_is_declared,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootGraphClassName"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_root_graph_class_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReferencedPresetAsset"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_get_referenced_preset_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeOutputIsConstructorPin"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_get_node_output_is_constructor_pin,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeOutputData"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_node_output_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeInputIsConstructorPin"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_get_node_input_is_constructor_pin,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeInputDefault"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_node_input_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeInputData"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_node_input_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodeInputClassDefault"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_get_node_input_class_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphVariableDefault"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_graph_variable_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphOutputNames"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_graph_output_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphInputNames"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_graph_input_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGraphInputDefault"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_get_graph_input_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeOutputsByDataType"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_find_node_outputs_by_data_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeOutputs"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_node_outputs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeOutputParent"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_node_output_parent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeOutputByName"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_node_output_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeInputsByDataType"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_find_node_inputs_by_data_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeInputs"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_node_inputs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeInputParent"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_node_input_parent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeInputByName"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_node_input_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNodeClassVersion"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_node_class_version,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindInterfaceOutputNodes"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_find_interface_output_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindInterfaceInputNodes"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_interface_input_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindGraphOutputNode"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_graph_output_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindGraphInputNode"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_find_graph_input_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectNodesByInterfaceBindings"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_disconnect_nodes_by_interface_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectNodes"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_disconnect_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectNodeOutput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_disconnect_node_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectNodeInput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_disconnect_node_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertToPreset"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_convert_to_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertFromPreset"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_convert_from_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ContainsNodeOutput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_contains_node_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ContainsNodeInput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_contains_node_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ContainsNode"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_contains_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectNodesByInterfaceBindings"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_connect_nodes_by_interface_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectNodes"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_connect_nodes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectNodeOutputToGraphOutput"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_connect_node_output_to_graph_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "ConnectNodeOutputsToMatchingGraphInterfaceOutputs",
            ),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_connect_node_outputs_to_matching_graph_interface_outputs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectNodeInputToGraphInput"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_connect_node_input_to_graph_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectNodeInputsToMatchingGraphInterfaceInputs"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_connect_node_inputs_to_matching_graph_interface_inputs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectNamedNodeOutputToNamedGraphOutput"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_connect_named_node_output_to_named_graph_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectNamedNodeOutputToGraphOutput"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_connect_named_node_output_to_graph_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BuildNewMetaSound"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_build_new_meta_sound,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BuildAndOverwriteMetaSound"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_build_and_overwrite_meta_sound,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Build"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_build,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNodeByClassName"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_add_node_by_class_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNode"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_add_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInterface"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_add_interface,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGraphVariableSetNode"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_add_graph_variable_set_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGraphVariableGetNode"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_add_graph_variable_get_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGraphVariableGetDelayedNode"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_base_add_graph_variable_get_delayed_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGraphVariable"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_add_graph_variable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGraphOutputNode"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_add_graph_output_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddGraphInputNode"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_base_add_graph_input_node,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundSourceBuilder::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSampleRateOverride"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_source_builder_set_sample_rate_override,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuality"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_source_builder_set_quality,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFormat"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_source_builder_set_format,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBlockRateOverride"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_source_builder_set_block_rate_override,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLiveUpdatesEnabled"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_source_builder_get_live_updates_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Audition"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_source_builder_audition,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundBuilderSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterSourceBuilder"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_unregister_source_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterPatchBuilder"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_unregister_patch_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnregisterBuilder"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_unregister_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTargetPage"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_set_target_page,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterSourceBuilder"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_register_source_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterPatchBuilder"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_register_patch_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RegisterBuilder"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_register_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInterfaceRegistered"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_is_interface_registered,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindSourceBuilder"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_find_source_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPatchBuilder"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_find_patch_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindParentBuilderOfPreset"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_find_parent_builder_of_preset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindBuilderOfDocument"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_find_builder_of_document,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindBuilder"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_find_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateStringMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_string_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateStringArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_string_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateSourcePresetBuilder"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_source_preset_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateSourceBuilder"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_create_source_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePatchPresetBuilder"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_patch_preset_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreatePatchBuilder"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_builder_subsystem_create_patch_builder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateObjectMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_object_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateObjectArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_object_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromParam"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_meta_sound_literal_from_param,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateIntMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_int_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateIntArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_int_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFloatMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_float_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFloatArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_float_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateBoolMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_bool_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateBoolArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_builder_subsystem_create_bool_array_meta_sound_literal,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetasoundFrontendLiteralBlueprintAccess::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetType"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStringValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_string_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStringArrayValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_string_array_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_object_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectArrayValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_object_array_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsStringArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsString"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsObjectArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsObject"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsIntegerArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsInteger"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsFloatArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsFloat"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsBoolArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaSoundLiteralAsBool"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_int_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntArrayValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_int_array_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloatValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_float_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloatArrayValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_float_array_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoolValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_bool_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoolArrayValueFromLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_get_bool_array_value_from_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EqualEqual_MetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_equal_equal_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateStringMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_string_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateStringArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_string_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateObjectMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_object_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateObjectArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_object_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromStringArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromString"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromParam"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_param,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromObjectArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromObject"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromIntegerArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromInteger"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromFloatArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromFloat"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromBooleanArray"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean_array,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromBoolean"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundLiteralFromAudioParameter"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_audio_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateIntMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_int_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateIntArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_int_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFloatMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_float_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateFloatArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_float_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateBoolMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_bool_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateBoolArrayMetaSoundLiteral"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_create_bool_array_meta_sound_literal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_MetaSoundLiteralToString"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_frontend_literal_blueprint_access_conv_meta_sound_literal_to_string,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetasoundGeneratorHandle::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WatchOutput"),
            &raw mut __FUNCTION_PTRS.u_metasound_generator_handle_watch_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateWatchers"),
            &raw mut __FUNCTION_PTRS.u_metasound_generator_handle_update_watchers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCPUCoreUtilization"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_generator_handle_get_cpu_core_utilization,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EnableRuntimeRenderTiming"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_generator_handle_enable_runtime_render_timing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateMetaSoundGeneratorHandle"),
            &raw mut __FUNCTION_PTRS
                .u_metasound_generator_handle_create_meta_sound_generator_handle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyParameterPack"),
            &raw mut __FUNCTION_PTRS.u_metasound_generator_handle_apply_parameter_pack,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundCacheSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TouchOrPrecacheMetaSound"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_cache_subsystem_touch_or_precache_meta_sound,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCachedOperatorsForMetaSound"),
            &raw mut __FUNCTION_PTRS
                .u_meta_sound_cache_subsystem_remove_cached_operators_for_meta_sound,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PrecacheMetaSound"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_cache_subsystem_precache_meta_sound,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetasoundOutputBlueprintAccess::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsTime"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_is_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsString"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_is_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsInt32"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_is_int32,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsFloat"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_is_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsBool"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_is_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimeSeconds"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_get_time_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetString"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_get_string,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInt32"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_get_int32,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloat"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_get_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBool"),
            &raw mut __FUNCTION_PTRS.u_metasound_output_blueprint_access_get_bool,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundOutputSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WatchOutput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_output_subsystem_watch_output,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnwatchOutput"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_output_subsystem_unwatch_output,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundQualityHelper::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetQualityNames"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_quality_helper_get_quality_names,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMetaSoundSettings::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetQualityNames"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_settings_get_quality_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPageNames"),
            &raw mut __FUNCTION_PTRS.u_meta_sound_settings_get_page_names,
        );
    }
}
#[repr(C, align(8))]
pub struct FMetaSoundOutput {
    pub(crate) __padding_end: [u8; 24],
}
impl FMetaSoundOutput {}
#[repr(C, align(8))]
pub struct FMetaSoundAssetDirectory {
    pub directory: crate::bindings::core_u_object::FDirectoryPath,
}
impl FMetaSoundAssetDirectory {}
#[repr(C, align(4))]
pub struct FMetaSoundBuilderNodeInputHandle {
    pub(crate) __padding_end: [u8; 32],
}
impl FMetaSoundBuilderNodeInputHandle {}
#[repr(C, align(4))]
pub struct FMetaSoundBuilderNodeOutputHandle {
    pub(crate) __padding_end: [u8; 32],
}
impl FMetaSoundBuilderNodeOutputHandle {}
#[repr(C, align(4))]
pub struct FMetaSoundNodeHandle {
    pub(crate) __padding_end: [u8; 16],
}
impl FMetaSoundNodeHandle {}
#[repr(C, align(8))]
pub struct FMetaSoundBuilderOptions {
    pub name: FName,
    pub b_force_unique_class_name: bool,
    pub(crate) __padding_end: [u8; 19],
}
impl FMetaSoundBuilderOptions {}
#[repr(C, align(8))]
pub struct UMetasoundEditorGraphBase {
    __padding_end: [u8; 192],
}
impl UMetasoundEditorGraphBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundEditorGraphBase")
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
pub struct UMetaSoundPatch {
    __padding_end: [u8; 1824],
}
impl UMetaSoundPatch {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundPatch")
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
pub struct UMetaSoundAssetSubsystem {
    __padding_end: [u8; 56],
}
impl UMetaSoundAssetSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundAssetSubsystem")
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
    pub fn unregister_asset_classes_in_directories(
        &mut self,
        directories: &TArray<FMetaSoundAssetDirectory>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_unregister_asset_classes_in_directories,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                directories,
                __buffer.add(0).cast::<TArray<FMetaSoundAssetDirectory>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_unregister_asset_classes_in_directories,
                __buffer,
            )
        };
    }
    pub fn replace_references_in_directory(
        &mut self,
        in_directories: &TArray<FMetaSoundAssetDirectory>,
        old_class_name: &crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
        new_class_name: &crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
        old_version: crate::bindings::metasound_frontend::FMetasoundFrontendVersionNumber,
        new_version: crate::bindings::metasound_frontend::FMetasoundFrontendVersionNumber,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_replace_references_in_directory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_directories,
                __buffer.add(0).cast::<TArray<FMetaSoundAssetDirectory>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                old_class_name,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_class_name,
                __buffer
                    .add(52)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_version,
                __buffer
                    .add(88)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendVersionNumber,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_version,
                __buffer
                    .add(96)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendVersionNumber,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_replace_references_in_directory,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn register_asset_classes_in_directories(
        &mut self,
        directories: &TArray<FMetaSoundAssetDirectory>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_register_asset_classes_in_directories,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                directories,
                __buffer.add(0).cast::<TArray<FMetaSoundAssetDirectory>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_register_asset_classes_in_directories,
                __buffer,
            )
        };
    }
    pub fn reassign_class_name(
        &mut self,
        doc_interface: TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_reassign_class_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &doc_interface,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_reassign_class_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn find_referencing_asset_class_info(
        &self,
        meta_sound: TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        out_paths: &mut TArray<crate::bindings::core_u_object::FTopLevelAssetPath>,
        out_doc_info: &mut TArray<
            crate::bindings::metasound_frontend::FMetaSoundDocumentInfo,
        >,
        out_interface_info: &mut TArray<
            crate::bindings::metasound_frontend::FMetaSoundClassInterfaceInfo,
        >,
        b_only_presets: bool,
        b_force_load: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<67>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_find_referencing_asset_class_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_sound,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_paths,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<crate::bindings::core_u_object::FTopLevelAssetPath>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_doc_info,
                __buffer
                    .add(32)
                    .cast::<
                        TArray<
                            crate::bindings::metasound_frontend::FMetaSoundDocumentInfo,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_interface_info,
                __buffer
                    .add(48)
                    .cast::<
                        TArray<
                            crate::bindings::metasound_frontend::FMetaSoundClassInterfaceInfo,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_presets,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_load,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_find_referencing_asset_class_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FTopLevelAssetPath>>()
                .swap(out_paths);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<
                    TArray<crate::bindings::metasound_frontend::FMetaSoundDocumentInfo>,
                >()
                .swap(out_doc_info);
        }
        unsafe {
            __buffer
                .add(48)
                .cast::<
                    TArray<
                        crate::bindings::metasound_frontend::FMetaSoundClassInterfaceInfo,
                    >,
                >()
                .swap(out_interface_info);
        }
        unsafe { __buffer.add(66).cast::<bool>().read() }
    }
    pub fn find_asset_class_info(
        &self,
        in_path: &crate::bindings::core_u_object::FTopLevelAssetPath,
        out_doc_info: &mut crate::bindings::metasound_frontend::FMetaSoundDocumentInfo,
        out_interface_info: &mut crate::bindings::metasound_frontend::FMetaSoundClassInterfaceInfo,
        b_force_load: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<186>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_find_asset_class_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_path,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FTopLevelAssetPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_doc_info,
                __buffer
                    .add(24)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetaSoundDocumentInfo,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_interface_info,
                __buffer
                    .add(56)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetaSoundClassInterfaceInfo,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_load,
                __buffer.add(184).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_asset_subsystem_find_asset_class_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::metasound_frontend::FMetaSoundDocumentInfo>()
                .swap(out_doc_info);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<
                    crate::bindings::metasound_frontend::FMetaSoundClassInterfaceInfo,
                >()
                .swap(out_interface_info);
        }
        unsafe { __buffer.add(185).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMetaSoundBuilderBase {
    __padding_end: [u8; 224],
}
impl UMetaSoundBuilderBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundBuilderBase")
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
    pub fn set_node_input_default(
        &mut self,
        node_input_handle: &FMetaSoundBuilderNodeInputHandle,
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<121>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_node_input_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(32)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(120).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_node_input_default,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(120).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_graph_output_name(
        &mut self,
        output_name: FName,
        new_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_output_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(24).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_output_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_graph_output_data_type(
        &mut self,
        output_name: FName,
        data_type: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_output_data_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(24).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_output_data_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_graph_output_access_type(
        &mut self,
        output_name: FName,
        access_type: crate::bindings::metasound_frontend::EMetasoundFrontendVertexAccessType,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_output_access_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &access_type,
                __buffer
                    .add(12)
                    .cast::<
                        crate::bindings::metasound_frontend::EMetasoundFrontendVertexAccessType,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_output_access_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_graph_input_name(
        &mut self,
        input_name: FName,
        new_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(24).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_graph_input_default(
        &mut self,
        input_name: FName,
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(104).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_default,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(104).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_graph_input_data_type(
        &mut self,
        input_name: FName,
        data_type: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_data_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(24).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_data_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_graph_input_access_type(
        &mut self,
        input_name: FName,
        access_type: crate::bindings::metasound_frontend::EMetasoundFrontendVertexAccessType,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_access_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &access_type,
                __buffer
                    .add(12)
                    .cast::<
                        crate::bindings::metasound_frontend::EMetasoundFrontendVertexAccessType,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_set_graph_input_access_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn remove_unused_dependencies(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_unused_dependencies,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_unused_dependencies,
                __buffer,
            )
        };
    }
    pub fn remove_node_input_default(
        &mut self,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_node_input_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_node_input_default,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn remove_node(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
        b_remove_unused_dependencies: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_unused_dependencies,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn remove_interface(
        &mut self,
        interface_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_interface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interface_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_interface,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn remove_graph_variable(
        &mut self,
        name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_graph_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_graph_variable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn remove_graph_output(
        &mut self,
        name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_graph_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_graph_output,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn remove_graph_input(
        &mut self,
        name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_graph_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_remove_graph_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn nodes_are_connected(
        &self,
        output_handle: &FMetaSoundBuilderNodeOutputHandle,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_nodes_are_connected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(32).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_nodes_are_connected,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn node_output_is_connected(
        &self,
        output_handle: &FMetaSoundBuilderNodeOutputHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_node_output_is_connected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_node_output_is_connected,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn node_input_is_connected(
        &self,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_node_input_is_connected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_node_input_is_connected,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn is_preset(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_is_preset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_is_preset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn interface_is_declared(&self, interface_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_interface_is_declared,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interface_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_interface_is_declared,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_root_graph_class_name(
        &self,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendClassName {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_root_graph_class_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_root_graph_class_name,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
                >()
                .read()
        }
    }
    pub fn get_referenced_preset_asset(
        &self,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_referenced_preset_asset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_referenced_preset_asset,
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
    pub fn get_node_output_is_constructor_pin(
        &self,
        output_handle: &FMetaSoundBuilderNodeOutputHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_output_is_constructor_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_output_is_constructor_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_node_output_data(
        &mut self,
        output_handle: &FMetaSoundBuilderNodeOutputHandle,
        name: &mut FName,
        data_type: &mut FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_output_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(name, __buffer.add(32).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_type,
                __buffer.add(44).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(56).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_output_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FName>().swap(name);
        }
        unsafe {
            __buffer.add(44).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer.add(56).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn get_node_input_is_constructor_pin(
        &self,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_is_constructor_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_is_constructor_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_node_input_default(
        &mut self,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_default,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn get_node_input_data(
        &mut self,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
        name: &mut FName,
        data_type: &mut FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(name, __buffer.add(32).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_type,
                __buffer.add(44).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(56).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FName>().swap(name);
        }
        unsafe {
            __buffer.add(44).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer.add(56).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn get_node_input_class_default(
        &mut self,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_class_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_node_input_class_default,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn get_graph_variable_default(
        &self,
        variable_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_variable_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_variable_default,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn get_graph_output_names(
        &self,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_output_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(0).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_output_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(8).cast::<TArray<FName>>().read() }
    }
    pub fn get_graph_input_names(
        &self,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_input_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(0).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_input_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(8).cast::<TArray<FName>>().read() }
    }
    pub fn get_graph_input_default(
        &self,
        input_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_input_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_get_graph_input_default,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn find_node_outputs_by_data_type(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
        data_type: FName,
    ) -> TArray<FMetaSoundBuilderNodeOutputHandle> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_outputs_by_data_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_outputs_by_data_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer.add(32).cast::<TArray<FMetaSoundBuilderNodeOutputHandle>>().read()
        }
    }
    pub fn find_node_outputs(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FMetaSoundBuilderNodeOutputHandle> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_outputs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_outputs,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer.add(24).cast::<TArray<FMetaSoundBuilderNodeOutputHandle>>().read()
        }
    }
    pub fn find_node_output_parent(
        &mut self,
        output_handle: &FMetaSoundBuilderNodeOutputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_output_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_output_parent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(36).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn find_node_output_by_name(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        output_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundBuilderNodeOutputHandle {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_output_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(28).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_output_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(28).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(32).cast::<FMetaSoundBuilderNodeOutputHandle>().read() }
    }
    pub fn find_node_inputs_by_data_type(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
        data_type: FName,
    ) -> TArray<FMetaSoundBuilderNodeInputHandle> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_inputs_by_data_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_inputs_by_data_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer.add(32).cast::<TArray<FMetaSoundBuilderNodeInputHandle>>().read()
        }
    }
    pub fn find_node_inputs(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FMetaSoundBuilderNodeInputHandle> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_inputs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_inputs,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer.add(24).cast::<TArray<FMetaSoundBuilderNodeInputHandle>>().read()
        }
    }
    pub fn find_node_input_parent(
        &mut self,
        input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_input_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_input_parent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(36).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn find_node_input_by_name(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        input_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundBuilderNodeInputHandle {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_input_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(28).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_input_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(28).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(32).cast::<FMetaSoundBuilderNodeInputHandle>().read() }
    }
    pub fn find_node_class_version(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendVersion {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_class_version,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_node_class_version,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer
                .add(20)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendVersion>()
                .read()
        }
    }
    pub fn find_interface_output_nodes(
        &mut self,
        interface_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FMetaSoundNodeHandle> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_interface_output_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interface_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_interface_output_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(16).cast::<TArray<FMetaSoundNodeHandle>>().read() }
    }
    pub fn find_interface_input_nodes(
        &mut self,
        interface_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FMetaSoundNodeHandle> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_interface_input_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interface_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_interface_input_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(16).cast::<TArray<FMetaSoundNodeHandle>>().read() }
    }
    pub fn find_graph_output_node(
        &mut self,
        output_name: FName,
        out_data_type: &mut FName,
        node_input_handle: &mut FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_graph_output_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_data_type,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_input_handle,
                __buffer.add(24).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(56).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_graph_output_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<FName>().swap(out_data_type);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<FMetaSoundBuilderNodeInputHandle>()
                .swap(node_input_handle);
        }
        unsafe {
            __buffer.add(56).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(60).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn find_graph_input_node(
        &mut self,
        input_name: FName,
        out_data_type: &mut FName,
        node_output_handle: &mut FMetaSoundBuilderNodeOutputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_graph_input_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_data_type,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_output_handle,
                __buffer.add(24).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(56).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_find_graph_input_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<FName>().swap(out_data_type);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<FMetaSoundBuilderNodeOutputHandle>()
                .swap(node_output_handle);
        }
        unsafe {
            __buffer.add(56).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(60).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn disconnect_nodes_by_interface_bindings(
        &mut self,
        from_node_handle: &FMetaSoundNodeHandle,
        to_node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_nodes_by_interface_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from_node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to_node_handle,
                __buffer.add(16).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_nodes_by_interface_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn disconnect_nodes(
        &mut self,
        node_output_handle: &FMetaSoundBuilderNodeOutputHandle,
        node_input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_input_handle,
                __buffer.add(32).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(64).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn disconnect_node_output(
        &mut self,
        node_output_handle: &FMetaSoundBuilderNodeOutputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_node_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_node_output,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn disconnect_node_input(
        &mut self,
        node_input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_node_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_input_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_disconnect_node_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn convert_to_preset(
        &mut self,
        referenced_node_class: &TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_convert_to_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                referenced_node_class,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_convert_to_preset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn convert_from_preset(&mut self, out_result: &mut EMetaSoundBuilderResult) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_convert_from_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(0).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_convert_from_preset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn contains_node_output(
        &self,
        output: &FMetaSoundBuilderNodeOutputHandle,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_contains_node_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_contains_node_output,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn contains_node_input(&self, input: &FMetaSoundBuilderNodeInputHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_contains_node_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_contains_node_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn contains_node(&self, node: &FMetaSoundNodeHandle) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_contains_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_contains_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn connect_nodes_by_interface_bindings(
        &mut self,
        from_node_handle: &FMetaSoundNodeHandle,
        to_node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_nodes_by_interface_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                from_node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                to_node_handle,
                __buffer.add(16).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_nodes_by_interface_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn connect_nodes(
        &mut self,
        node_output_handle: &FMetaSoundBuilderNodeOutputHandle,
        node_input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_output_handle,
                __buffer.add(0).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_input_handle,
                __buffer.add(32).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(64).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn connect_node_output_to_graph_output(
        &mut self,
        graph_output_name: FName,
        node_output_handle: &FMetaSoundBuilderNodeOutputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_output_to_graph_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &graph_output_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_output_handle,
                __buffer.add(12).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(44).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_output_to_graph_output,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(44).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn connect_node_outputs_to_matching_graph_interface_outputs(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FMetaSoundBuilderNodeInputHandle> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_outputs_to_matching_graph_interface_outputs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_outputs_to_matching_graph_interface_outputs,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer.add(24).cast::<TArray<FMetaSoundBuilderNodeInputHandle>>().read()
        }
    }
    pub fn connect_node_input_to_graph_input(
        &mut self,
        graph_input_name: FName,
        node_input_handle: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_input_to_graph_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &graph_input_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_input_handle,
                __buffer.add(12).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(44).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_input_to_graph_input,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(44).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn connect_node_inputs_to_matching_graph_interface_inputs(
        &mut self,
        node_handle: &FMetaSoundNodeHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FMetaSoundBuilderNodeOutputHandle> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_inputs_to_matching_graph_interface_inputs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_handle,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_node_inputs_to_matching_graph_interface_inputs,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer.add(24).cast::<TArray<FMetaSoundBuilderNodeOutputHandle>>().read()
        }
    }
    pub fn connect_named_node_output_to_named_graph_output(
        &mut self,
        source_node: &FMetaSoundNodeHandle,
        node_output_name: &FName,
        graph_output_name: &FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_named_node_output_to_named_graph_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_node,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_output_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                graph_output_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(40).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_named_node_output_to_named_graph_output,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn connect_named_node_output_to_graph_output(
        &mut self,
        source_node: &FMetaSoundNodeHandle,
        node_output_name: &FName,
        graph_output: &FMetaSoundBuilderNodeInputHandle,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<61>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_named_node_output_to_graph_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_node,
                __buffer.add(0).cast::<FMetaSoundNodeHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_output_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                graph_output,
                __buffer.add(28).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(60).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_connect_named_node_output_to_graph_output,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(60).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn build_new_meta_sound(
        &self,
        name_base: FName,
    ) -> TScriptInterface<
        crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
    > {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_build_new_meta_sound,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &name_base,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_build_new_meta_sound,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TScriptInterface<
                        crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                    >,
                >()
                .read()
        }
    }
    pub fn build_and_overwrite_meta_sound(
        &mut self,
        existing_meta_sound: TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        b_force_unique_class_name: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_build_and_overwrite_meta_sound,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &existing_meta_sound,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_unique_class_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_build_and_overwrite_meta_sound,
                __buffer,
            )
        };
    }
    pub fn build(
        &self,
        parent: UPtr<crate::bindings::core_u_object::UObject>,
        options: &FMetaSoundBuilderOptions,
    ) -> TScriptInterface<
        crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
    > {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_build,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                options,
                __buffer.add(8).cast::<FMetaSoundBuilderOptions>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_build,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<
                    TScriptInterface<
                        crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                    >,
                >()
                .read()
        }
    }
    pub fn add_node_by_class_name(
        &mut self,
        class_name: &crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
        out_result: &mut EMetaSoundBuilderResult,
        major_version: i32,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_node_by_class_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                class_name,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendClassName,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(36).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &major_version,
                __buffer.add(40).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_node_by_class_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(36).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(44).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn add_node(
        &mut self,
        node_class: &TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                node_class,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(16).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(20).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn add_interface(
        &mut self,
        interface_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_interface,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interface_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_interface,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn add_graph_variable_set_node(
        &mut self,
        name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable_set_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable_set_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(16).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn add_graph_variable_get_node(
        &mut self,
        name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable_get_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable_get_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(16).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn add_graph_variable_get_delayed_node(
        &mut self,
        name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FMetaSoundNodeHandle {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable_get_delayed_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable_get_delayed_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(16).cast::<FMetaSoundNodeHandle>().read() }
    }
    pub fn add_graph_variable(
        &mut self,
        name: FName,
        data_type: FName,
        default_value: crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer
                    .add(24)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(112).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_variable,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn add_graph_output_node(
        &mut self,
        name: FName,
        data_type: FName,
        default_value: crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
        b_is_constructor_output: bool,
    ) -> FMetaSoundBuilderNodeInputHandle {
        let mut __stack = crate::core_data::StackAlloc::<148>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_output_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer
                    .add(24)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(112).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_constructor_output,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_output_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(116).cast::<FMetaSoundBuilderNodeInputHandle>().read() }
    }
    pub fn add_graph_input_node(
        &mut self,
        name: FName,
        data_type: FName,
        default_value: crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
        b_is_constructor_input: bool,
    ) -> FMetaSoundBuilderNodeOutputHandle {
        let mut __stack = crate::core_data::StackAlloc::<148>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_input_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_type,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &default_value,
                __buffer
                    .add(24)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(112).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_constructor_input,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_base_add_graph_input_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(116).cast::<FMetaSoundBuilderNodeOutputHandle>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMetaSoundPatchBuilder {
    __padding_end: [u8; 224],
}
impl UMetaSoundPatchBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundPatchBuilder")
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
pub struct UMetaSoundSourceBuilder {
    __padding_end: [u8; 264],
}
impl UMetaSoundSourceBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundSourceBuilder")
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
    pub fn set_sample_rate_override(&mut self, sample_rate: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_sample_rate_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sample_rate,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_sample_rate_override,
                __buffer,
            )
        };
    }
    pub fn set_quality(&mut self, quality: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_quality,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&quality, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_quality,
                __buffer,
            )
        };
    }
    pub fn set_format(
        &mut self,
        output_format: EMetaSoundOutputAudioFormat,
        out_result: &mut EMetaSoundBuilderResult,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_format,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_format,
                __buffer.add(0).cast::<EMetaSoundOutputAudioFormat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(1).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_format,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(1).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
    }
    pub fn set_block_rate_override(&mut self, block_rate: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_block_rate_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&block_rate, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_set_block_rate_override,
                __buffer,
            )
        };
    }
    pub fn get_live_updates_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_get_live_updates_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_get_live_updates_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn audition(
        &mut self,
        parent: UPtr<crate::bindings::core_u_object::UObject>,
        audio_component: UPtr<crate::bindings::engine::UAudioComponent>,
        on_create_generator: FAudition_OnCreateGenerator,
        b_live_updates_enabled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_audition,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_component,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAudioComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_create_generator,
                __buffer.add(16).cast::<FAudition_OnCreateGenerator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_live_updates_enabled,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_source_builder_audition,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMetaSoundBuilderSubsystem {
    __padding_end: [u8; 136],
}
impl UMetaSoundBuilderSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundBuilderSubsystem")
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
    pub fn unregister_source_builder(&mut self, builder_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_unregister_source_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_unregister_source_builder,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn unregister_patch_builder(&mut self, builder_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_unregister_patch_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_unregister_patch_builder,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn unregister_builder(&mut self, builder_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_unregister_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_unregister_builder,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_target_page(&mut self, page_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_set_target_page,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &page_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_set_target_page,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn register_source_builder(
        &mut self,
        builder_name: FName,
        builder: UPtr<UMetaSoundSourceBuilder>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_register_source_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder,
                __buffer.add(16).cast::<UPtr<UMetaSoundSourceBuilder>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_register_source_builder,
                __buffer,
            )
        };
    }
    pub fn register_patch_builder(
        &mut self,
        builder_name: FName,
        builder: UPtr<UMetaSoundPatchBuilder>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_register_patch_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder,
                __buffer.add(16).cast::<UPtr<UMetaSoundPatchBuilder>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_register_patch_builder,
                __buffer,
            )
        };
    }
    pub fn register_builder(
        &mut self,
        builder_name: FName,
        builder: UPtr<UMetaSoundBuilderBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_register_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder,
                __buffer.add(16).cast::<UPtr<UMetaSoundBuilderBase>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_register_builder,
                __buffer,
            )
        };
    }
    pub fn is_interface_registered(&self, in_interface_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_is_interface_registered,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_interface_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_is_interface_registered,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn find_source_builder(
        &mut self,
        builder_name: FName,
    ) -> UPtr<UMetaSoundSourceBuilder> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_source_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_source_builder,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMetaSoundSourceBuilder>>().read() }
    }
    pub fn find_patch_builder(
        &mut self,
        builder_name: FName,
    ) -> UPtr<UMetaSoundPatchBuilder> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_patch_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_patch_builder,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMetaSoundPatchBuilder>>().read() }
    }
    pub fn find_parent_builder_of_preset(
        &mut self,
        in_meta_sound_preset: TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        b_follow_preset_chain: bool,
    ) -> UPtr<UMetaSoundBuilderBase> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_parent_builder_of_preset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_meta_sound_preset,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_follow_preset_chain,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_parent_builder_of_preset,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UMetaSoundBuilderBase>>().read() }
    }
    pub fn find_builder_of_document(
        &self,
        in_meta_sound: TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
    ) -> UPtr<UMetaSoundBuilderBase> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_builder_of_document,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_meta_sound,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_builder_of_document,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMetaSoundBuilderBase>>().read() }
    }
    pub fn find_builder(&mut self, builder_name: FName) -> UPtr<UMetaSoundBuilderBase> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_find_builder,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMetaSoundBuilderBase>>().read() }
    }
    pub fn create_string_meta_sound_literal(
        &mut self,
        value: FString,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_string_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_type,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_string_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_string_array_meta_sound_literal(
        &mut self,
        value: &TArray<FString>,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_string_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_type,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_string_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_source_preset_builder(
        &mut self,
        builder_name: FName,
        referenced_source_class: &TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> UPtr<UMetaSoundSourceBuilder> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_source_preset_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                referenced_source_class,
                __buffer
                    .add(16)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_source_preset_builder,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(40).cast::<UPtr<UMetaSoundSourceBuilder>>().read() }
    }
    pub fn create_source_builder(
        &mut self,
        builder_name: FName,
        on_play_node_output: &mut FMetaSoundBuilderNodeOutputHandle,
        on_finished_node_input: &mut FMetaSoundBuilderNodeInputHandle,
        audio_out_node_inputs: &mut TArray<FMetaSoundBuilderNodeInputHandle>,
        out_result: &mut EMetaSoundBuilderResult,
        output_format: EMetaSoundOutputAudioFormat,
        b_is_one_shot: bool,
    ) -> UPtr<UMetaSoundSourceBuilder> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_source_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_play_node_output,
                __buffer.add(12).cast::<FMetaSoundBuilderNodeOutputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_finished_node_input,
                __buffer.add(44).cast::<FMetaSoundBuilderNodeInputHandle>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                audio_out_node_inputs,
                __buffer.add(80).cast::<TArray<FMetaSoundBuilderNodeInputHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(96).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_format,
                __buffer.add(97).cast::<EMetaSoundOutputAudioFormat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_one_shot,
                __buffer.add(98).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_source_builder,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(12)
                .cast::<FMetaSoundBuilderNodeOutputHandle>()
                .swap(on_play_node_output);
        }
        unsafe {
            __buffer
                .add(44)
                .cast::<FMetaSoundBuilderNodeInputHandle>()
                .swap(on_finished_node_input);
        }
        unsafe {
            __buffer
                .add(80)
                .cast::<TArray<FMetaSoundBuilderNodeInputHandle>>()
                .swap(audio_out_node_inputs);
        }
        unsafe {
            __buffer.add(96).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(104).cast::<UPtr<UMetaSoundSourceBuilder>>().read() }
    }
    pub fn create_patch_preset_builder(
        &mut self,
        builder_name: FName,
        referenced_patch_class: &TScriptInterface<
            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
        >,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> UPtr<UMetaSoundPatchBuilder> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_patch_preset_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                referenced_patch_class,
                __buffer
                    .add(16)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::metasound_frontend::UMetaSoundDocumentInterface,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(32).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_patch_preset_builder,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(40).cast::<UPtr<UMetaSoundPatchBuilder>>().read() }
    }
    pub fn create_patch_builder(
        &mut self,
        builder_name: FName,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> UPtr<UMetaSoundPatchBuilder> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_patch_builder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &builder_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(12).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_patch_builder,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(16).cast::<UPtr<UMetaSoundPatchBuilder>>().read() }
    }
    pub fn create_object_meta_sound_literal(
        &mut self,
        value: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_object_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_object_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_object_array_meta_sound_literal(
        &mut self,
        value: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_object_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_object_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_param(
        &mut self,
        param: &crate::bindings::audio_extensions::FAudioParameter,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_meta_sound_literal_from_param,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                param,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::audio_extensions::FAudioParameter>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_meta_sound_literal_from_param,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(160)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_int_meta_sound_literal(
        &mut self,
        value: i32,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_int_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(data_type, __buffer.add(4).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_int_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_int_array_meta_sound_literal(
        &mut self,
        value: &TArray<i32>,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_int_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_type,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_int_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_float_meta_sound_literal(
        &mut self,
        value: f32,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_float_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(data_type, __buffer.add(4).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_float_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_float_array_meta_sound_literal(
        &mut self,
        value: &TArray<f32>,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_float_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_type,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_float_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_bool_meta_sound_literal(
        &mut self,
        value: bool,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_bool_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(data_type, __buffer.add(4).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_bool_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_bool_array_meta_sound_literal(
        &mut self,
        value: &TArray<bool>,
        data_type: &mut FName,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_bool_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<bool>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                data_type,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_builder_subsystem_create_bool_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FName>().swap(data_type);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMetasoundFrontendLiteralBlueprintAccess {
    __padding_end: [u8; 48],
}
impl UMetasoundFrontendLiteralBlueprintAccess {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundFrontendLiteralBlueprintAccess")
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
    pub fn get_type(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> crate::bindings::metasound_frontend::EMetasoundFrontendLiteralType {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<
                    crate::bindings::metasound_frontend::EMetasoundFrontendLiteralType,
                >()
                .read()
        }
    }
    pub fn get_string_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_string_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_string_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(96).cast::<FString>().read() }
    }
    pub fn get_string_array_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_string_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_string_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(96).cast::<TArray<FString>>().read() }
    }
    pub fn get_object_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_object_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_object_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer
                .add(96)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_object_array_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_object_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_object_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe {
            __buffer
                .add(96)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_meta_sound_literal_as_string_array(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<TArray<FString>>().read() }
    }
    pub fn get_meta_sound_literal_as_string(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FString>().read() }
    }
    pub fn get_meta_sound_literal_as_object_array(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object_array,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn get_meta_sound_literal_as_object(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(88)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_meta_sound_literal_as_integer_array(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<TArray<i32>>().read() }
    }
    pub fn get_meta_sound_literal_as_integer(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_integer,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<i32>().read() }
    }
    pub fn get_meta_sound_literal_as_float_array(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<TArray<f32>>().read() }
    }
    pub fn get_meta_sound_literal_as_float(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<f32>().read() }
    }
    pub fn get_meta_sound_literal_as_bool_array(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> TArray<bool> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<TArray<bool>>().read() }
    }
    pub fn get_meta_sound_literal_as_bool(
        in_literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_meta_sound_literal_as_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn get_int_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_int_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_int_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(92).cast::<i32>().read() }
    }
    pub fn get_int_array_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_int_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_int_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(96).cast::<TArray<i32>>().read() }
    }
    pub fn get_float_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_float_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_float_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(92).cast::<f32>().read() }
    }
    pub fn get_float_array_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_float_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_float_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(96).cast::<TArray<f32>>().read() }
    }
    pub fn get_bool_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<90>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_bool_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_bool_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(89).cast::<bool>().read() }
    }
    pub fn get_bool_array_value_from_literal(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        out_result: &mut EMetaSoundBuilderResult,
    ) -> TArray<bool> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_bool_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_result,
                __buffer.add(88).cast::<EMetaSoundBuilderResult>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_get_bool_array_value_from_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<EMetaSoundBuilderResult>().swap(out_result);
        }
        unsafe { __buffer.add(96).cast::<TArray<bool>>().read() }
    }
    pub fn equal_equal_meta_sound_literal(
        literal_a: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
        literal_b: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<177>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_equal_equal_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal_a,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal_b,
                __buffer
                    .add(88)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_equal_equal_meta_sound_literal,
                __buffer,
            )
        };
        unsafe { __buffer.add(176).cast::<bool>().read() }
    }
    pub fn create_string_meta_sound_literal(
        value: FString,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_string_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_string_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_string_array_meta_sound_literal(
        value: &TArray<FString>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_string_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_string_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_object_meta_sound_literal(
        value: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_object_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_object_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_object_array_meta_sound_literal(
        value: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_object_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_object_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_string_array(
        in_string_array: &TArray<FString>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_string_array,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string_array,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_string(
        in_string: FString,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_string,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_string,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_param(
        param: &crate::bindings::audio_extensions::FAudioParameter,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_param,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                param,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::audio_extensions::FAudioParameter>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_param,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(160)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_object_array(
        in_object_array: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_object_array,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object_array,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_object(
        in_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object,
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
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_integer_array(
        in_integer_array: &TArray<i32>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_integer_array,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer_array,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_integer(
        in_integer: i32,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_integer, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_integer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_float_array(
        in_float_array: &TArray<f32>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_float_array,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float_array,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_float(
        in_float: f32,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_float, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_float,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_boolean_array(
        in_boolean_array: &TArray<bool>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean_array,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_boolean_array,
                __buffer.add(0).cast::<TArray<bool>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean_array,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_boolean(
        in_boolean: bool,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_boolean,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_boolean,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_meta_sound_literal_from_audio_parameter(
        in_audio_parameter: &crate::bindings::audio_extensions::FAudioParameter,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<248>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_audio_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_audio_parameter,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::audio_extensions::FAudioParameter>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_meta_sound_literal_from_audio_parameter,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(160)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_int_meta_sound_literal(
        value: i32,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_int_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_int_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_int_array_meta_sound_literal(
        value: &TArray<i32>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_int_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_int_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_float_meta_sound_literal(
        value: f32,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_float_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_float_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_float_array_meta_sound_literal(
        value: &TArray<f32>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_float_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_float_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_bool_meta_sound_literal(
        value: bool,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_bool_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_bool_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn create_bool_array_meta_sound_literal(
        value: &TArray<bool>,
    ) -> crate::bindings::metasound_frontend::FMetasoundFrontendLiteral {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_bool_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(0).cast::<TArray<bool>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_create_bool_array_meta_sound_literal,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::metasound_frontend::FMetasoundFrontendLiteral>()
                .read()
        }
    }
    pub fn conv_meta_sound_literal_to_string(
        literal: &crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_conv_meta_sound_literal_to_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                literal,
                __buffer
                    .add(0)
                    .cast::<
                        crate::bindings::metasound_frontend::FMetasoundFrontendLiteral,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundFrontendLiteralBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_frontend_literal_blueprint_access_conv_meta_sound_literal_to_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMetasoundGeneratorHandle {
    __padding_end: [u8; 224],
}
impl UMetasoundGeneratorHandle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundGeneratorHandle")
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
    pub fn watch_output(
        &mut self,
        output_name: FName,
        on_output_value_changed: &FWatchOutput_OnOutputValueChanged,
        analyzer_name: FName,
        analyzer_output_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_watch_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_output_value_changed,
                __buffer.add(16).cast::<FWatchOutput_OnOutputValueChanged>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &analyzer_name,
                __buffer.add(48).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &analyzer_output_name,
                __buffer.add(60).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_watch_output,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn update_watchers(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_update_watchers,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_update_watchers,
                __buffer,
            )
        };
    }
    pub fn get_cpu_core_utilization(&self) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_get_cpu_core_utilization,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_get_cpu_core_utilization,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f64>().read() }
    }
    pub fn enable_runtime_render_timing(&self, enable: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_enable_runtime_render_timing,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&enable, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_enable_runtime_render_timing,
                __buffer,
            )
        };
    }
    pub fn create_meta_sound_generator_handle(
        on_component: UPtr<crate::bindings::engine::UAudioComponent>,
    ) -> UPtr<UMetasoundGeneratorHandle> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_create_meta_sound_generator_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAudioComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundGeneratorHandle::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_create_meta_sound_generator_handle,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UMetasoundGeneratorHandle>>().read() }
    }
    pub fn apply_parameter_pack(
        &mut self,
        pack: UPtr<crate::bindings::metasound_frontend::UMetasoundParameterPack>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_apply_parameter_pack,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pack,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::metasound_frontend::UMetasoundParameterPack,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_generator_handle_apply_parameter_pack,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMetaSoundCacheSubsystem {
    __padding_end: [u8; 128],
}
impl UMetaSoundCacheSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundCacheSubsystem")
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
    pub fn touch_or_precache_meta_sound(
        &mut self,
        in_meta_sound: UPtr<UMetaSoundSource>,
        in_num_instances: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_cache_subsystem_touch_or_precache_meta_sound,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_meta_sound,
                __buffer.add(0).cast::<UPtr<UMetaSoundSource>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_instances,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_cache_subsystem_touch_or_precache_meta_sound,
                __buffer,
            )
        };
    }
    pub fn remove_cached_operators_for_meta_sound(
        &mut self,
        in_meta_sound: UPtr<UMetaSoundSource>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_cache_subsystem_remove_cached_operators_for_meta_sound,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_meta_sound,
                __buffer.add(0).cast::<UPtr<UMetaSoundSource>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_cache_subsystem_remove_cached_operators_for_meta_sound,
                __buffer,
            )
        };
    }
    pub fn precache_meta_sound(
        &mut self,
        in_meta_sound: UPtr<UMetaSoundSource>,
        in_num_instances: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_cache_subsystem_precache_meta_sound,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_meta_sound,
                __buffer.add(0).cast::<UPtr<UMetaSoundSource>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_instances,
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
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_cache_subsystem_precache_meta_sound,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UMetasoundOutputBlueprintAccess {
    __padding_end: [u8; 48],
}
impl UMetasoundOutputBlueprintAccess {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetasoundOutputBlueprintAccess")
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
    pub fn is_time(output: &FMetaSoundOutput) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_string(output: &FMetaSoundOutput) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_int32(output: &FMetaSoundOutput) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_int32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_float(output: &FMetaSoundOutput) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn is_bool(output: &FMetaSoundOutput) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_is_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_time_seconds(output: &FMetaSoundOutput, success: &mut bool) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_time_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(success, __buffer.add(24).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_time_seconds,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<bool>().swap(success);
        }
        unsafe { __buffer.add(32).cast::<f64>().read() }
    }
    pub fn get_string(output: &FMetaSoundOutput, success: &mut bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(success, __buffer.add(24).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_string,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<bool>().swap(success);
        }
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn get_int32(output: &FMetaSoundOutput, success: &mut bool) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_int32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(success, __buffer.add(24).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_int32,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<bool>().swap(success);
        }
        unsafe { __buffer.add(28).cast::<i32>().read() }
    }
    pub fn get_float(output: &FMetaSoundOutput, success: &mut bool) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(success, __buffer.add(24).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_float,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<bool>().swap(success);
        }
        unsafe { __buffer.add(28).cast::<f32>().read() }
    }
    pub fn get_bool(output: &FMetaSoundOutput, success: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                output,
                __buffer.add(0).cast::<FMetaSoundOutput>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(success, __buffer.add(24).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::metasound_engine::UMetasoundOutputBlueprintAccess::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_metasound_output_blueprint_access_get_bool,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<bool>().swap(success);
        }
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMetaSoundOutputSubsystem {
    __padding_end: [u8; 80],
}
impl UMetaSoundOutputSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundOutputSubsystem")
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
    pub fn watch_output(
        &mut self,
        audio_component: UPtr<crate::bindings::engine::UAudioComponent>,
        output_name: FName,
        on_output_value_changed: &FWatchOutput_OnOutputValueChanged,
        analyzer_name: FName,
        analyzer_output_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_output_subsystem_watch_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAudioComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_output_value_changed,
                __buffer.add(24).cast::<FWatchOutput_OnOutputValueChanged>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &analyzer_name,
                __buffer.add(56).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &analyzer_output_name,
                __buffer.add(68).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_output_subsystem_watch_output,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
    pub fn unwatch_output(
        &mut self,
        audio_component: UPtr<crate::bindings::engine::UAudioComponent>,
        output_name: FName,
        on_output_value_changed: &FUnwatchOutput_OnOutputValueChanged,
        analyzer_name: FName,
        analyzer_output_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<81>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_output_subsystem_unwatch_output,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &audio_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAudioComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                on_output_value_changed,
                __buffer.add(24).cast::<FUnwatchOutput_OnOutputValueChanged>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &analyzer_name,
                __buffer.add(56).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &analyzer_output_name,
                __buffer.add(68).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::metasound_engine::__FUNCTION_PTRS
                    .u_meta_sound_output_subsystem_unwatch_output,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMetaSoundQualityHelper {
    __padding_end: [u8; 48],
}
impl UMetaSoundQualityHelper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundQualityHelper")
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
pub struct UMetaSoundSettings {
    __padding_end: [u8; 664],
}
impl UMetaSoundSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundSettings")
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
pub struct UMetaSoundSource {
    #[doc(hidden)]
    pub(crate) __padding_3880: [u8; 3880],
    pub output_format: EMetaSoundOutputAudioFormat,
    pub quality_setting: FName,
    #[doc(hidden)]
    pub(crate) __padding_3912: [u8; 16],
    pub block_rate_override: crate::bindings::core_u_object::FPerPlatformFloat,
    pub sample_rate_override: crate::bindings::core_u_object::FPerPlatformInt,
    __padding_end: [u8; 440],
}
impl UMetaSoundSource {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMetaSoundSource")
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
pub struct FAudition_OnCreateGenerator {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FWatchOutput_OnOutputValueChanged {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FUnwatchOutput_OnOutputValueChanged {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EMetaSoundBuilderResult(pub u8);
impl EMetaSoundBuilderResult {
    pub const SUCCEEDED: EMetaSoundBuilderResult = EMetaSoundBuilderResult(0);
    pub const FAILED: EMetaSoundBuilderResult = EMetaSoundBuilderResult(1);
}
#[repr(transparent)]
pub struct EMetaSoundOutputAudioFormat(pub u8);
impl EMetaSoundOutputAudioFormat {
    pub const MONO: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(0);
    pub const STEREO: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(1);
    pub const QUAD: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(2);
    pub const FIVE_DOT_ONE: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(3);
    pub const SEVEN_DOT_ONE: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(
        4,
    );
    pub const COUNT: EMetaSoundOutputAudioFormat = EMetaSoundOutputAudioFormat(5);
}
