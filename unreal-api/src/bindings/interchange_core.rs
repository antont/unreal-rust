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
    pub u_interchange_factory_base_set_source_filename: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_get_source_filenames: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_get_factory_class: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_get_factory_asset_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_support_reimport: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_scripted_set_reimport_source_index: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_scripted_get_pipeline_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_scripted_execute_post_import_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_scripted_execute_post_factory_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_scripted_execute_post_broadcast_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_scripted_execute_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_scripted_execute_export_pipeline: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_is_reimport_context: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_get_support_asset_classes: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_find_or_add_property_states: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_pipeline_base_does_property_states_exist: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_data_set_filename: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_data_set_context_object_by_tag: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_data_remove_all_context_objects: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_data_get_filename: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_data_get_context_object_by_tag: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_data_get_all_context_object_tags: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_settings_save_settings: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_settings_load_settings: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_base_set_settings: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_base_get_translator_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_base_get_supported_formats: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_base_get_supported_asset_types: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_base_get_source_data: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_base_get_settings: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_translator_base_can_import_source_data: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_set_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_set_display_label: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_set_asset_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_remove_target_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_remove_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_is_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_initialize_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_vector2_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_unique_id: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_type_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_target_node_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_target_node_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_string_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_parent_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_node_container_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_namespace: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_linear_color_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_int32_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_icon_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_guid_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_float_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_double_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_display_label: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_desired_child_index: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_boolean_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_get_asset_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_vector2_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_target_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_string_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_linear_color_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_int32_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_guid_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_float_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_double_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_add_boolean_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_set_node_parent_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_set_node_desired_child_index: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_set_namespace: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_save_to_file: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_reset_children_cache: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_reset: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_replace_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_remove_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_load_from_file: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_is_node_uid_valid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_roots: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_node_children_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_node_children_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_node_children: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_is_ancestor: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_get_factory_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_compute_children_cache: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_clear_node_parent_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_node_container_add_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_unset_skip_node_import: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_unset_force_node_reimport: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_should_skip_node_import: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_should_force_node_reimport: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_set_skip_node_import: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_set_reimport_strategy_flags: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_set_force_node_reimport: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_set_custom_sub_path: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_set_custom_reference_object: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_set_custom_level_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_remove_factory_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_is_runtime_import_allowed: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_reimport_strategy_flags: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_object_class: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_factory_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_factory_dependencies_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_factory_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_custom_sub_path: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_custom_reference_object: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_get_custom_level_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_factory_base_node_add_factory_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_extra_information: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_use_legacy_skeletal_mesh_bake_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_use_asset_type_sub_path_suffix: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_sub_path_prefix: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_source_timeline_start: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_source_timeline_end: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_source_frame_rate_numerator: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_source_frame_rate_denominator: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_skeletal_mesh_front_axis: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_reimport_strategy_flags: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_nanite_triangle_threshold: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_import_unused_material: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_axis_conversion_inverse_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_animated_time_start: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_animated_time_end: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_set_custom_allow_scene_root_as_joint: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_remove_extra_information: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_initialize_source_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_unique_instance: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_extra_information: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_use_legacy_skeletal_mesh_bake_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_use_asset_type_sub_path_suffix: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_sub_path_prefix: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_source_timeline_start: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_source_timeline_end: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_source_frame_rate_numerator: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_source_frame_rate_denominator: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_skeletal_mesh_front_axis: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_reimport_strategy_flags: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_nanite_triangle_threshold: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_import_unused_material: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_axis_conversion_inverse_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_animated_time_start: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_animated_time_end: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_source_node_get_custom_allow_scene_root_as_joint: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_remove_user_defined_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_get_user_defined_attribute_infos: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_get_user_defined_attribute_int32: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_get_user_defined_attribute_f_string: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_get_user_defined_attribute_float: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_get_user_defined_attribute_double: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_get_user_defined_attribute_boolean: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_duplicate_all_user_defined_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_create_user_defined_attribute_int32: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_create_user_defined_attribute_f_string: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_create_user_defined_attribute_float: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_create_user_defined_attribute_double: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_user_defined_attributes_api_create_user_defined_attribute_boolean: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_interchange_factory_base_set_source_filename: std::ptr::null_mut(),
            u_interchange_factory_base_get_source_filenames: std::ptr::null_mut(),
            u_interchange_factory_base_get_factory_class: std::ptr::null_mut(),
            u_interchange_factory_base_get_factory_asset_type: std::ptr::null_mut(),
            u_interchange_pipeline_base_support_reimport: std::ptr::null_mut(),
            u_interchange_pipeline_base_scripted_set_reimport_source_index: std::ptr::null_mut(),
            u_interchange_pipeline_base_scripted_get_pipeline_display_name: std::ptr::null_mut(),
            u_interchange_pipeline_base_scripted_execute_post_import_pipeline: std::ptr::null_mut(),
            u_interchange_pipeline_base_scripted_execute_post_factory_pipeline: std::ptr::null_mut(),
            u_interchange_pipeline_base_scripted_execute_post_broadcast_pipeline: std::ptr::null_mut(),
            u_interchange_pipeline_base_scripted_execute_pipeline: std::ptr::null_mut(),
            u_interchange_pipeline_base_scripted_execute_export_pipeline: std::ptr::null_mut(),
            u_interchange_pipeline_base_is_reimport_context: std::ptr::null_mut(),
            u_interchange_pipeline_base_get_support_asset_classes: std::ptr::null_mut(),
            u_interchange_pipeline_base_find_or_add_property_states: std::ptr::null_mut(),
            u_interchange_pipeline_base_does_property_states_exist: std::ptr::null_mut(),
            u_interchange_source_data_set_filename: std::ptr::null_mut(),
            u_interchange_source_data_set_context_object_by_tag: std::ptr::null_mut(),
            u_interchange_source_data_remove_all_context_objects: std::ptr::null_mut(),
            u_interchange_source_data_get_filename: std::ptr::null_mut(),
            u_interchange_source_data_get_context_object_by_tag: std::ptr::null_mut(),
            u_interchange_source_data_get_all_context_object_tags: std::ptr::null_mut(),
            u_interchange_translator_settings_save_settings: std::ptr::null_mut(),
            u_interchange_translator_settings_load_settings: std::ptr::null_mut(),
            u_interchange_translator_base_set_settings: std::ptr::null_mut(),
            u_interchange_translator_base_get_translator_type: std::ptr::null_mut(),
            u_interchange_translator_base_get_supported_formats: std::ptr::null_mut(),
            u_interchange_translator_base_get_supported_asset_types: std::ptr::null_mut(),
            u_interchange_translator_base_get_source_data: std::ptr::null_mut(),
            u_interchange_translator_base_get_settings: std::ptr::null_mut(),
            u_interchange_translator_base_can_import_source_data: std::ptr::null_mut(),
            u_interchange_base_node_set_enabled: std::ptr::null_mut(),
            u_interchange_base_node_set_display_label: std::ptr::null_mut(),
            u_interchange_base_node_set_asset_name: std::ptr::null_mut(),
            u_interchange_base_node_remove_target_node_uid: std::ptr::null_mut(),
            u_interchange_base_node_remove_attribute: std::ptr::null_mut(),
            u_interchange_base_node_is_enabled: std::ptr::null_mut(),
            u_interchange_base_node_initialize_node: std::ptr::null_mut(),
            u_interchange_base_node_get_vector2_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_unique_id: std::ptr::null_mut(),
            u_interchange_base_node_get_type_name: std::ptr::null_mut(),
            u_interchange_base_node_get_target_node_uids: std::ptr::null_mut(),
            u_interchange_base_node_get_target_node_count: std::ptr::null_mut(),
            u_interchange_base_node_get_string_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_parent_uid: std::ptr::null_mut(),
            u_interchange_base_node_get_node_container_type: std::ptr::null_mut(),
            u_interchange_base_node_get_namespace: std::ptr::null_mut(),
            u_interchange_base_node_get_linear_color_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_int32_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_icon_name: std::ptr::null_mut(),
            u_interchange_base_node_get_guid_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_float_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_double_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_display_label: std::ptr::null_mut(),
            u_interchange_base_node_get_desired_child_index: std::ptr::null_mut(),
            u_interchange_base_node_get_boolean_attribute: std::ptr::null_mut(),
            u_interchange_base_node_get_asset_name: std::ptr::null_mut(),
            u_interchange_base_node_add_vector2_attribute: std::ptr::null_mut(),
            u_interchange_base_node_add_target_node_uid: std::ptr::null_mut(),
            u_interchange_base_node_add_string_attribute: std::ptr::null_mut(),
            u_interchange_base_node_add_linear_color_attribute: std::ptr::null_mut(),
            u_interchange_base_node_add_int32_attribute: std::ptr::null_mut(),
            u_interchange_base_node_add_guid_attribute: std::ptr::null_mut(),
            u_interchange_base_node_add_float_attribute: std::ptr::null_mut(),
            u_interchange_base_node_add_double_attribute: std::ptr::null_mut(),
            u_interchange_base_node_add_boolean_attribute: std::ptr::null_mut(),
            u_interchange_base_node_container_set_node_parent_uid: std::ptr::null_mut(),
            u_interchange_base_node_container_set_node_desired_child_index: std::ptr::null_mut(),
            u_interchange_base_node_container_set_namespace: std::ptr::null_mut(),
            u_interchange_base_node_container_save_to_file: std::ptr::null_mut(),
            u_interchange_base_node_container_reset_children_cache: std::ptr::null_mut(),
            u_interchange_base_node_container_reset: std::ptr::null_mut(),
            u_interchange_base_node_container_replace_node: std::ptr::null_mut(),
            u_interchange_base_node_container_remove_node: std::ptr::null_mut(),
            u_interchange_base_node_container_load_from_file: std::ptr::null_mut(),
            u_interchange_base_node_container_is_node_uid_valid: std::ptr::null_mut(),
            u_interchange_base_node_container_get_roots: std::ptr::null_mut(),
            u_interchange_base_node_container_get_nodes: std::ptr::null_mut(),
            u_interchange_base_node_container_get_node_children_uids: std::ptr::null_mut(),
            u_interchange_base_node_container_get_node_children_count: std::ptr::null_mut(),
            u_interchange_base_node_container_get_node_children: std::ptr::null_mut(),
            u_interchange_base_node_container_get_node: std::ptr::null_mut(),
            u_interchange_base_node_container_get_is_ancestor: std::ptr::null_mut(),
            u_interchange_base_node_container_get_factory_node: std::ptr::null_mut(),
            u_interchange_base_node_container_compute_children_cache: std::ptr::null_mut(),
            u_interchange_base_node_container_clear_node_parent_uid: std::ptr::null_mut(),
            u_interchange_base_node_container_add_node: std::ptr::null_mut(),
            u_interchange_factory_base_node_unset_skip_node_import: std::ptr::null_mut(),
            u_interchange_factory_base_node_unset_force_node_reimport: std::ptr::null_mut(),
            u_interchange_factory_base_node_should_skip_node_import: std::ptr::null_mut(),
            u_interchange_factory_base_node_should_force_node_reimport: std::ptr::null_mut(),
            u_interchange_factory_base_node_set_skip_node_import: std::ptr::null_mut(),
            u_interchange_factory_base_node_set_reimport_strategy_flags: std::ptr::null_mut(),
            u_interchange_factory_base_node_set_force_node_reimport: std::ptr::null_mut(),
            u_interchange_factory_base_node_set_custom_sub_path: std::ptr::null_mut(),
            u_interchange_factory_base_node_set_custom_reference_object: std::ptr::null_mut(),
            u_interchange_factory_base_node_set_custom_level_uid: std::ptr::null_mut(),
            u_interchange_factory_base_node_remove_factory_dependency_uid: std::ptr::null_mut(),
            u_interchange_factory_base_node_is_runtime_import_allowed: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_reimport_strategy_flags: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_object_class: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_factory_dependency: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_factory_dependencies_count: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_factory_dependencies: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_custom_sub_path: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_custom_reference_object: std::ptr::null_mut(),
            u_interchange_factory_base_node_get_custom_level_uid: std::ptr::null_mut(),
            u_interchange_factory_base_node_add_factory_dependency_uid: std::ptr::null_mut(),
            u_interchange_source_node_set_extra_information: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_use_legacy_skeletal_mesh_bake_transform: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_use_asset_type_sub_path_suffix: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_sub_path_prefix: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_source_timeline_start: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_source_timeline_end: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_source_frame_rate_numerator: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_source_frame_rate_denominator: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_skeletal_mesh_front_axis: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_reimport_strategy_flags: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_nanite_triangle_threshold: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_import_unused_material: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_axis_conversion_inverse_transform: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_animated_time_start: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_animated_time_end: std::ptr::null_mut(),
            u_interchange_source_node_set_custom_allow_scene_root_as_joint: std::ptr::null_mut(),
            u_interchange_source_node_remove_extra_information: std::ptr::null_mut(),
            u_interchange_source_node_initialize_source_node: std::ptr::null_mut(),
            u_interchange_source_node_get_unique_instance: std::ptr::null_mut(),
            u_interchange_source_node_get_extra_information: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_use_legacy_skeletal_mesh_bake_transform: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_use_asset_type_sub_path_suffix: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_sub_path_prefix: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_source_timeline_start: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_source_timeline_end: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_source_frame_rate_numerator: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_source_frame_rate_denominator: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_skeletal_mesh_front_axis: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_reimport_strategy_flags: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_nanite_triangle_threshold: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_import_unused_material: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_axis_conversion_inverse_transform: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_animated_time_start: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_animated_time_end: std::ptr::null_mut(),
            u_interchange_source_node_get_custom_allow_scene_root_as_joint: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_remove_user_defined_attribute: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_get_user_defined_attribute_infos: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_get_user_defined_attribute_int32: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_get_user_defined_attribute_f_string: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_get_user_defined_attribute_float: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_get_user_defined_attribute_double: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_get_user_defined_attribute_boolean: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_duplicate_all_user_defined_attribute: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_create_user_defined_attribute_int32: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_create_user_defined_attribute_f_string: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_create_user_defined_attribute_float: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_create_user_defined_attribute_double: std::ptr::null_mut(),
            u_interchange_user_defined_attributes_api_create_user_defined_attribute_boolean: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeFactoryBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSourceFilename"),
                &raw mut __FUNCTION_PTRS.u_interchange_factory_base_set_source_filename,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceFilenames"),
                &raw mut __FUNCTION_PTRS.u_interchange_factory_base_get_source_filenames,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFactoryClass"),
                &raw mut __FUNCTION_PTRS.u_interchange_factory_base_get_factory_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFactoryAssetType"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_get_factory_asset_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangePipelineBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SupportReimport"),
                &raw mut __FUNCTION_PTRS.u_interchange_pipeline_base_support_reimport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScriptedSetReimportSourceIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_set_reimport_source_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScriptedGetPipelineDisplayName"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_get_pipeline_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScriptedExecutePostImportPipeline"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_import_pipeline,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScriptedExecutePostFactoryPipeline"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_factory_pipeline,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScriptedExecutePostBroadcastPipeline"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_broadcast_pipeline,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScriptedExecutePipeline"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_pipeline,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ScriptedExecuteExportPipeline"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_export_pipeline,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReimportContext"),
                &raw mut __FUNCTION_PTRS.u_interchange_pipeline_base_is_reimport_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportAssetClasses"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_get_support_asset_classes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindOrAddPropertyStates"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_find_or_add_property_states,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DoesPropertyStatesExist"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_pipeline_base_does_property_states_exist,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeSourceData::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetFilename"),
                &raw mut __FUNCTION_PTRS.u_interchange_source_data_set_filename,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetContextObjectByTag"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_data_set_context_object_by_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAllContextObjects"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_data_remove_all_context_objects,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFilename"),
                &raw mut __FUNCTION_PTRS.u_interchange_source_data_get_filename,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetContextObjectByTag"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_data_get_context_object_by_tag,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllContextObjectTags"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_data_get_all_context_object_tags,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeTranslatorSettings::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SaveSettings"),
                &raw mut __FUNCTION_PTRS.u_interchange_translator_settings_save_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LoadSettings"),
                &raw mut __FUNCTION_PTRS.u_interchange_translator_settings_load_settings,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeTranslatorBase::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSettings"),
                &raw mut __FUNCTION_PTRS.u_interchange_translator_base_set_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTranslatorType"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_translator_base_get_translator_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedFormats"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_translator_base_get_supported_formats,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedAssetTypes"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_translator_base_get_supported_asset_types,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceData"),
                &raw mut __FUNCTION_PTRS.u_interchange_translator_base_get_source_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSettings"),
                &raw mut __FUNCTION_PTRS.u_interchange_translator_base_get_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanImportSourceData"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_translator_base_can_import_source_data,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeBaseNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetEnabled"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_set_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDisplayLabel"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_set_display_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAssetName"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_set_asset_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveTargetNodeUid"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_remove_target_node_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_remove_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnabled"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_is_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InitializeNode"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_initialize_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVector2Attribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_vector2_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUniqueID"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_unique_id,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTypeName"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_type_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetNodeUids"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_target_node_uids,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetNodeCount"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_target_node_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStringAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_string_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentUid"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_parent_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeContainerType"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_node_container_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNamespace"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_namespace,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinearColorAttribute"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_get_linear_color_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInt32Attribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_int32_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIconName"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_icon_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGuidAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_guid_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFloatAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_float_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDoubleAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_double_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayLabel"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_display_label,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDesiredChildIndex"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_desired_child_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetBooleanAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_boolean_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssetName"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_get_asset_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddVector2Attribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_vector2_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTargetNodeUid"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_target_node_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddStringAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_string_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLinearColorAttribute"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_add_linear_color_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddInt32Attribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_int32_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddGuidAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_guid_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddFloatAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_float_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddDoubleAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_double_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBooleanAttribute"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_add_boolean_attribute,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeBaseNodeContainer::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeParentUid"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_set_node_parent_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeDesiredChildIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_set_node_desired_child_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNamespace"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_set_namespace,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SaveToFile"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_save_to_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetChildrenCache"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_reset_children_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Reset"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_reset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceNode"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_replace_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveNode"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_remove_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LoadFromFile"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_load_from_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsNodeUidValid"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_is_node_uid_valid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRoots"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_get_roots,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodes"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_get_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeChildrenUids"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children_uids,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeChildrenCount"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeChildren"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNode"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_get_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIsAncestor"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_get_is_ancestor,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFactoryNode"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_get_factory_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ComputeChildrenCache"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_compute_children_cache,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearNodeParentUid"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_base_node_container_clear_node_parent_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddNode"),
                &raw mut __FUNCTION_PTRS.u_interchange_base_node_container_add_node,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeFactoryBaseNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnsetSkipNodeImport"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_unset_skip_node_import,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnsetForceNodeReimport"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_unset_force_node_reimport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldSkipNodeImport"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_should_skip_node_import,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldForceNodeReimport"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_should_force_node_reimport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSkipNodeImport"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_skip_node_import,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetReimportStrategyFlags"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_reimport_strategy_flags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetForceNodeReimport"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_force_node_reimport,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomSubPath"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_sub_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomReferenceObject"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_reference_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomLevelUid"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_level_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveFactoryDependencyUid"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_remove_factory_dependency_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRuntimeImportAllowed"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_is_runtime_import_allowed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReimportStrategyFlags"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_reimport_strategy_flags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetObjectClass"),
                &raw mut __FUNCTION_PTRS.u_interchange_factory_base_node_get_object_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFactoryDependency"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependency,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFactoryDependenciesCount"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependencies_count,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFactoryDependencies"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependencies,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomSubPath"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_sub_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomReferenceObject"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_reference_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomLevelUid"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_level_uid,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddFactoryDependencyUid"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_factory_base_node_add_factory_dependency_uid,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeSourceNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetExtraInformation"),
                &raw mut __FUNCTION_PTRS.u_interchange_source_node_set_extra_information,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomUseLegacySkeletalMeshBakeTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_use_legacy_skeletal_mesh_bake_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomUseAssetTypeSubPathSuffix"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_use_asset_type_sub_path_suffix,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomSubPathPrefix"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_sub_path_prefix,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomSourceTimelineStart"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_timeline_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomSourceTimelineEnd"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_timeline_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomSourceFrameRateNumerator"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_frame_rate_numerator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomSourceFrameRateDenominator"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_frame_rate_denominator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomSkeletalMeshFrontAxis"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_skeletal_mesh_front_axis,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomReimportStrategyFlags"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_reimport_strategy_flags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomNaniteTriangleThreshold"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_nanite_triangle_threshold,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomImportUnusedMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_import_unused_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomAxisConversionInverseTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_axis_conversion_inverse_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomAnimatedTimeStart"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_animated_time_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomAnimatedTimeEnd"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_animated_time_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCustomAllowSceneRootAsJoint"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_allow_scene_root_as_joint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveExtraInformation"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_remove_extra_information,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InitializeSourceNode"),
                &raw mut __FUNCTION_PTRS.u_interchange_source_node_initialize_source_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUniqueInstance"),
                &raw mut __FUNCTION_PTRS.u_interchange_source_node_get_unique_instance,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetExtraInformation"),
                &raw mut __FUNCTION_PTRS.u_interchange_source_node_get_extra_information,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomUseLegacySkeletalMeshBakeTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_use_legacy_skeletal_mesh_bake_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomUseAssetTypeSubPathSuffix"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_use_asset_type_sub_path_suffix,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomSubPathPrefix"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_sub_path_prefix,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomSourceTimelineStart"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_timeline_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomSourceTimelineEnd"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_timeline_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomSourceFrameRateNumerator"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_frame_rate_numerator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomSourceFrameRateDenominator"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_frame_rate_denominator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomSkeletalMeshFrontAxis"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_skeletal_mesh_front_axis,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomReimportStrategyFlags"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_reimport_strategy_flags,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomNaniteTriangleThreshold"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_nanite_triangle_threshold,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomImportUnusedMaterial"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_import_unused_material,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomAxisConversionInverseTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_axis_conversion_inverse_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomAnimatedTimeStart"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_animated_time_start,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomAnimatedTimeEnd"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_animated_time_end,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomAllowSceneRootAsJoint"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_allow_scene_root_as_joint,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UInterchangeUserDefinedAttributesAPI::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveUserDefinedAttribute"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_remove_user_defined_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserDefinedAttributeInfos"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_infos,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Int32"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_int32,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_FString"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_f_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Float"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Double"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_double,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUserDefinedAttribute_Boolean"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_boolean,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DuplicateAllUserDefinedAttribute"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_duplicate_all_user_defined_attribute,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Int32"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_int32,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_FString"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_f_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Float"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Double"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_double,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateUserDefinedAttribute_Boolean"),
                &raw mut __FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_boolean,
            );
        }
    }
}
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStatePerContext {
    pub b_visible: bool,
}
impl FInterchangePipelinePropertyStatePerContext {}
#[repr(C, align(1))]
pub struct FInterchangePipelinePropertyStates {
    pub b_locked: bool,
    pub b_pre_dialog_reset: bool,
    pub basic_layout_states: FInterchangePipelinePropertyStatePerContext,
    pub import_states: FInterchangePipelinePropertyStatePerContext,
    pub reimport_states: FInterchangePipelinePropertyStatePerContext,
}
impl FInterchangePipelinePropertyStates {}
#[repr(C, align(8))]
pub struct FInterchangeUserDefinedAttributeInfo {
    pub name: FString,
    pub(crate) __padding_end: [u8; 32],
}
impl FInterchangeUserDefinedAttributeInfo {}
#[repr(C, align(8))]
pub struct UInterchangeFactoryBase {
    __padding_end: [u8; 56],
}
impl UInterchangeFactoryBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFactoryBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFactoryBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_source_filename(
        &self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        source_filename: FString,
        source_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_set_source_filename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_filename,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_index,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_set_source_filename,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn get_source_filenames(
        &self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        out_source_filenames: &mut TArray<FString>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_get_source_filenames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_source_filenames,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_get_source_filenames,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FString>>().swap(out_source_filenames);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_factory_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_get_factory_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_get_factory_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_factory_asset_type(&mut self) -> EInterchangeFactoryAssetType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_get_factory_asset_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_get_factory_asset_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeFactoryAssetType>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeWriterBase {
    __padding_end: [u8; 48],
}
impl UInterchangeWriterBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeWriterBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeWriterBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangePipelineBase {
    __padding_end: [u8; 344],
}
impl UInterchangePipelineBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePipelineBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePipelineBase")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn support_reimport(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_support_reimport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_support_reimport,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn scripted_set_reimport_source_index(
        &mut self,
        reimport_object_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        source_file_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_set_reimport_source_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reimport_object_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_file_index,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_set_reimport_source_index,
                __buffer,
            )
        };
    }
    pub fn scripted_get_pipeline_display_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_get_pipeline_display_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_get_pipeline_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn scripted_execute_post_import_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        factory_node_key: FString,
        created_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_a_reimport: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_import_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory_node_key,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &created_asset,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_reimport,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_import_pipeline,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_post_factory_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        factory_node_key: FString,
        created_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_a_reimport: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_factory_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory_node_key,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &created_asset,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_reimport,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_factory_pipeline,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_post_broadcast_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        factory_node_key: FString,
        created_asset: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_a_reimport: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_broadcast_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &factory_node_key,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &created_asset,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_reimport,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_post_broadcast_pipeline,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
        source_datas: &TArray<UPtr<UInterchangeSourceData>>,
        content_base_path: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_datas,
                __buffer.add(8).cast::<TArray<UPtr<UInterchangeSourceData>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &content_base_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_pipeline,
                __buffer,
            )
        };
    }
    pub fn scripted_execute_export_pipeline(
        &mut self,
        base_node_container: UPtr<UInterchangeBaseNodeContainer>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_export_pipeline,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_scripted_execute_export_pipeline,
                __buffer,
            )
        };
    }
    pub fn is_reimport_context(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_is_reimport_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_is_reimport_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_support_asset_classes(
        &self,
        pipeline_support_asset_classes: &mut TArray<
            TSubclassOf<crate::bindings::core_u_object::UObject>,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_get_support_asset_classes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                pipeline_support_asset_classes,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_get_support_asset_classes,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<TSubclassOf<crate::bindings::core_u_object::UObject>>>()
                .swap(pipeline_support_asset_classes);
        }
    }
    pub fn find_or_add_property_states(
        &mut self,
        property_path: FName,
    ) -> FInterchangePipelinePropertyStates {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_find_or_add_property_states,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_path,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_find_or_add_property_states,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FInterchangePipelinePropertyStates>().read() }
    }
    pub fn does_property_states_exist(&self, property_path: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_does_property_states_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_path,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_pipeline_base_does_property_states_exist,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResult {
    __padding_end: [u8; 120],
}
impl UInterchangeResult {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResult")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResult")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultSuccess {
    __padding_end: [u8; 120],
}
impl UInterchangeResultSuccess {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultSuccess")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultSuccess")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultWarning {
    __padding_end: [u8; 120],
}
impl UInterchangeResultWarning {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultWarning")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultWarning")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultError {
    __padding_end: [u8; 120],
}
impl UInterchangeResultError {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultWarning_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultWarning_Generic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultWarning_Generic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultWarning_Generic")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultError_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultError_Generic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError_Generic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError_Generic")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultError_ReimportFail {
    __padding_end: [u8; 120],
}
impl UInterchangeResultError_ReimportFail {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError_ReimportFail")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultError_ReimportFail")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultDisplay_Generic {
    __padding_end: [u8; 136],
}
impl UInterchangeResultDisplay_Generic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultDisplay_Generic")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultDisplay_Generic")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultsContainer {
    __padding_end: [u8; 104],
}
impl UInterchangeResultsContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultsContainer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultsContainer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSourceData {
    __padding_end: [u8; 168],
}
impl UInterchangeSourceData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSourceData")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSourceData")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_filename(&mut self, in_filename: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_set_filename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_filename,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_set_filename,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_context_object_by_tag(
        &self,
        tag: FString,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_set_context_object_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_set_context_object_by_tag,
                __buffer,
            )
        };
    }
    pub fn remove_all_context_objects(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_remove_all_context_objects,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_remove_all_context_objects,
                __buffer,
            )
        };
    }
    pub fn get_filename(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_get_filename,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_get_filename,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_context_object_by_tag(
        &self,
        tag: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_get_context_object_by_tag,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_get_context_object_by_tag,
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
    pub fn get_all_context_object_tags(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_get_all_context_object_tags,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_data_get_all_context_object_tags,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTranslatorSettings {
    __padding_end: [u8; 48],
}
impl UInterchangeTranslatorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTranslatorSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTranslatorSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn save_settings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_settings_save_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_settings_save_settings,
                __buffer,
            )
        };
    }
    pub fn load_settings(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_settings_load_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_settings_load_settings,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTranslatorBase {
    __padding_end: [u8; 80],
}
impl UInterchangeTranslatorBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTranslatorBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTranslatorBase")
            .copied()
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
        interchange_translator_settings: UPtr<UInterchangeTranslatorSettings>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_set_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_translator_settings,
                __buffer.add(0).cast::<UPtr<UInterchangeTranslatorSettings>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_set_settings,
                __buffer,
            )
        };
    }
    pub fn get_translator_type(&self) -> EInterchangeTranslatorType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_translator_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_translator_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeTranslatorType>().read() }
    }
    pub fn get_supported_formats(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_supported_formats,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_supported_formats,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_supported_asset_types(&self) -> EInterchangeTranslatorAssetType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_supported_asset_types,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_supported_asset_types,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeTranslatorAssetType>().read() }
    }
    pub fn get_source_data(&self) -> UPtr<UInterchangeSourceData> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_source_data,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_source_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UInterchangeSourceData>>().read() }
    }
    pub fn get_settings(&self) -> UPtr<UInterchangeTranslatorSettings> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_settings,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_get_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UInterchangeTranslatorSettings>>().read() }
    }
    pub fn can_import_source_data(
        &self,
        in_source_data: UPtr<UInterchangeSourceData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_can_import_source_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_source_data,
                __buffer.add(0).cast::<UPtr<UInterchangeSourceData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_translator_base_can_import_source_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeBaseNode {
    __padding_end: [u8; 112],
}
impl UInterchangeBaseNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBaseNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBaseNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_enabled(&mut self, b_is_enabled: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_set_enabled,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_set_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_display_label(&mut self, in_display_label: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_set_display_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_label,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_set_display_label,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_asset_name(&mut self, asset_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_set_asset_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_name,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_set_asset_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_target_node_uid(&self, asset_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_remove_target_node_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_uid,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_remove_target_node_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_attribute(&mut self, node_attribute_key: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_remove_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_remove_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_is_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_is_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn initialize_node(
        &mut self,
        unique_id: FString,
        display_label: FString,
        node_container_type: EInterchangeNodeContainerType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_initialize_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &display_label,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_container_type,
                __buffer.add(32).cast::<EInterchangeNodeContainerType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_initialize_node,
                __buffer,
            )
        };
    }
    pub fn get_vector2_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_vector2_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_vector2_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FVector2f>()
                .swap(out_value);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_unique_id(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_unique_id,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_unique_id,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_type_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_type_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_type_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_target_node_uids(&self, out_target_assets: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_target_node_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_target_assets,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_target_node_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_target_assets);
        }
    }
    pub fn get_target_node_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_target_node_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_target_node_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_string_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_string_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_string_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(out_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_parent_uid(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_parent_uid,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_parent_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node_container_type(&self) -> EInterchangeNodeContainerType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_node_container_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_node_container_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EInterchangeNodeContainerType>().read() }
    }
    pub fn get_namespace(&self, namespace: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_namespace,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                namespace,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_namespace,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(namespace);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_linear_color_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_linear_color_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_linear_color_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .swap(out_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_int32_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_int32_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_int32_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(out_value);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_icon_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_icon_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_icon_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_guid_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_guid_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_guid_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FGuid>()
                .swap(out_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_float_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_float_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_float_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<f32>().swap(out_value);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_double_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_double_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_double_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<f64>().swap(out_value);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_display_label(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_display_label,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_display_label,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_desired_child_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_desired_child_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_desired_child_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_boolean_attribute(
        &self,
        node_attribute_key: FString,
        out_value: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_boolean_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_boolean_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(out_value);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_asset_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_asset_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_get_asset_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn add_vector2_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &crate::bindings::core_u_object::FVector2f,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_vector2_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2f>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_vector2_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_target_node_uid(&self, asset_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_target_node_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_uid,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_target_node_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_string_attribute(
        &mut self,
        node_attribute_key: FString,
        value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_string_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_string_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_linear_color_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_linear_color_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_linear_color_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_int32_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_int32_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_int32_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn add_guid_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_guid_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_guid_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_float_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_float_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_float_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn add_double_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_double_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_double_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_boolean_attribute(
        &mut self,
        node_attribute_key: FString,
        value: &bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_boolean_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_add_boolean_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeBaseNodeContainer {
    __padding_end: [u8; 208],
}
impl UInterchangeBaseNodeContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBaseNodeContainer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBaseNodeContainer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_node_parent_uid(
        &mut self,
        node_unique_id: FString,
        new_parent_node_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_set_node_parent_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_parent_node_uid,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_set_node_parent_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_node_desired_child_index(
        &mut self,
        node_unique_id: FString,
        new_node_desired_child_index: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_set_node_desired_child_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_node_desired_child_index,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_set_node_desired_child_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_namespace(
        &mut self,
        namespace: FString,
        target_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_set_namespace,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &namespace,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_class,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_set_namespace,
                __buffer,
            )
        };
    }
    pub fn save_to_file(&mut self, filename: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_save_to_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filename,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_save_to_file,
                __buffer,
            )
        };
    }
    pub fn reset_children_cache(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_reset_children_cache,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_reset_children_cache,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_reset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_reset,
                __buffer,
            )
        };
    }
    pub fn replace_node(
        &mut self,
        node_unique_id: FString,
        new_node: UPtr<UInterchangeFactoryBaseNode>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_replace_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_node,
                __buffer.add(16).cast::<UPtr<UInterchangeFactoryBaseNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_replace_node,
                __buffer,
            )
        };
    }
    pub fn remove_node(&mut self, node_unique_id: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_remove_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_remove_node,
                __buffer,
            )
        };
    }
    pub fn load_from_file(&mut self, filename: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_load_from_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &filename,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_load_from_file,
                __buffer,
            )
        };
    }
    pub fn is_node_uid_valid(&self, node_unique_id: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_is_node_uid_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_is_node_uid_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_roots(&self, root_nodes: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_roots,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                root_nodes,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_roots,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(root_nodes);
        }
    }
    pub fn get_nodes(
        &self,
        class_node: TSubclassOf<crate::bindings::core_u_object::UObject>,
        out_nodes: &mut TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_node,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_nodes,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_nodes,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FString>>().swap(out_nodes);
        }
    }
    pub fn get_node_children_uids(&self, node_unique_id: FString) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children_uids,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FString>>().read() }
    }
    pub fn get_node_children_count(&self, node_unique_id: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children_count,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_node_children(
        &mut self,
        node_unique_id: FString,
        child_index: i32,
    ) -> UPtr<UInterchangeBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &child_index,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node_children,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UInterchangeBaseNode>>().read() }
    }
    pub fn get_node(&self, node_unique_id: FString) -> UPtr<UInterchangeBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UInterchangeBaseNode>>().read() }
    }
    pub fn get_is_ancestor(
        &self,
        node_unique_id: FString,
        ancestor_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_is_ancestor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &ancestor_uid,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_is_ancestor,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_factory_node(
        &self,
        node_unique_id: FString,
    ) -> UPtr<UInterchangeFactoryBaseNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_factory_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_get_factory_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UInterchangeFactoryBaseNode>>().read() }
    }
    pub fn compute_children_cache(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_compute_children_cache,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_compute_children_cache,
                __buffer,
            )
        };
    }
    pub fn clear_node_parent_uid(&mut self, node_unique_id: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_clear_node_parent_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_unique_id,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_clear_node_parent_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_node(&mut self, node: UPtr<UInterchangeBaseNode>) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_add_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_base_node_container_add_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeFactoryBaseNode {
    __padding_end: [u8; 464],
}
impl UInterchangeFactoryBaseNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFactoryBaseNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFactoryBaseNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn unset_skip_node_import(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_unset_skip_node_import,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_unset_skip_node_import,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn unset_force_node_reimport(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_unset_force_node_reimport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_unset_force_node_reimport,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn should_skip_node_import(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_should_skip_node_import,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_should_skip_node_import,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn should_force_node_reimport(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_should_force_node_reimport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_should_force_node_reimport,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_skip_node_import(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_skip_node_import,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_skip_node_import,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_reimport_strategy_flags(
        &mut self,
        reimport_strategy_flags: &EReimportStrategyFlags,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_reimport_strategy_flags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                reimport_strategy_flags,
                __buffer.add(0).cast::<EReimportStrategyFlags>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_reimport_strategy_flags,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_force_node_reimport(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_force_node_reimport,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_force_node_reimport,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_custom_sub_path(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_sub_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_sub_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_reference_object(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_reference_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_reference_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_custom_level_uid(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_level_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_set_custom_level_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_factory_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_remove_factory_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dependency_uid,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_remove_factory_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_runtime_import_allowed(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_is_runtime_import_allowed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_is_runtime_import_allowed,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_reimport_strategy_flags(&self) -> EReimportStrategyFlags {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_reimport_strategy_flags,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_reimport_strategy_flags,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EReimportStrategyFlags>().read() }
    }
    pub fn get_object_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_object_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_object_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_factory_dependency(&self, index: i32, out_dependency: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dependency,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency);
        }
    }
    pub fn get_factory_dependencies_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependencies_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependencies_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_factory_dependencies(&self, out_dependencies: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dependencies,
                __buffer.add(0).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_factory_dependencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependencies);
        }
    }
    pub fn get_custom_sub_path(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_sub_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_sub_path,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_reference_object(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_reference_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_reference_object,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FSoftObjectPath>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn get_custom_level_uid(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_level_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_get_custom_level_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_factory_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_add_factory_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dependency_uid,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_factory_base_node_add_factory_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSourceNode {
    __padding_end: [u8; 472],
}
impl UInterchangeSourceNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSourceNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSourceNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_extra_information(&mut self, name: FString, value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_extra_information,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(16).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_extra_information,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_custom_use_legacy_skeletal_mesh_bake_transform(
        &mut self,
        attribute_value: &bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_use_legacy_skeletal_mesh_bake_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_use_legacy_skeletal_mesh_bake_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_use_asset_type_sub_path_suffix(&mut self, suffix: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_use_asset_type_sub_path_suffix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(suffix, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_use_asset_type_sub_path_suffix,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_sub_path_prefix(&mut self, prefix: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_sub_path_prefix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&prefix, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_sub_path_prefix,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_source_timeline_start(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_timeline_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_timeline_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_source_timeline_end(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_timeline_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_timeline_end,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_source_frame_rate_numerator(
        &mut self,
        attribute_value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_frame_rate_numerator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_frame_rate_numerator,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_source_frame_rate_denominator(
        &mut self,
        attribute_value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_frame_rate_denominator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_source_frame_rate_denominator,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_skeletal_mesh_front_axis(&mut self, attribute_value: u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_skeletal_mesh_front_axis,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_skeletal_mesh_front_axis,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_reimport_strategy_flags(&mut self, strategy_flag: u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_reimport_strategy_flags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &strategy_flag,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_reimport_strategy_flags,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_nanite_triangle_threshold(
        &mut self,
        min_num_triangles: &i64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_nanite_triangle_threshold,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                min_num_triangles,
                __buffer.add(0).cast::<i64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_nanite_triangle_threshold,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_import_unused_material(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_import_unused_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_import_unused_material,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_axis_conversion_inverse_transform(
        &mut self,
        axis_conversion_inverse_transform: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_axis_conversion_inverse_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                axis_conversion_inverse_transform,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_axis_conversion_inverse_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn set_custom_animated_time_start(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_animated_time_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_animated_time_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_animated_time_end(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_animated_time_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_animated_time_end,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_allow_scene_root_as_joint(
        &mut self,
        b_allow_scene_root_as_joint: &bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_allow_scene_root_as_joint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_allow_scene_root_as_joint,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_set_custom_allow_scene_root_as_joint,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn remove_extra_information(&mut self, name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_remove_extra_information,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_remove_extra_information,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn initialize_source_node(
        &mut self,
        unique_id: FString,
        display_label: FString,
        node_container: UPtr<UInterchangeBaseNodeContainer>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_initialize_source_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &display_label,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_container,
                __buffer.add(32).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_initialize_source_node,
                __buffer,
            )
        };
    }
    pub fn get_unique_instance(
        node_container: UPtr<UInterchangeBaseNodeContainer>,
    ) -> UPtr<UInterchangeSourceNode> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_unique_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_container,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNodeContainer>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeSourceNode::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_unique_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UInterchangeSourceNode>>().read() }
    }
    pub fn get_extra_information(
        &self,
        out_extra_information: &mut TMap<FString, FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_extra_information,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_extra_information,
                __buffer.add(0).cast::<TMap<FString, FString>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_extra_information,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TMap<FString, FString>>().swap(out_extra_information);
        }
    }
    pub fn get_custom_use_legacy_skeletal_mesh_bake_transform(
        &self,
        attribute_value: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_use_legacy_skeletal_mesh_bake_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_use_legacy_skeletal_mesh_bake_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_use_asset_type_sub_path_suffix(&self, suffix: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_use_asset_type_sub_path_suffix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(suffix, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_use_asset_type_sub_path_suffix,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(suffix);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_sub_path_prefix(&self, prefix: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_sub_path_prefix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(prefix, __buffer.add(0).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_sub_path_prefix,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(prefix);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_source_timeline_start(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_timeline_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_timeline_start,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_source_timeline_end(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_timeline_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_timeline_end,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_source_frame_rate_numerator(
        &self,
        attribute_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_frame_rate_numerator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_frame_rate_numerator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_source_frame_rate_denominator(
        &self,
        attribute_value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_frame_rate_denominator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_source_frame_rate_denominator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_skeletal_mesh_front_axis(&self, attribute_value: &mut u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_skeletal_mesh_front_axis,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_skeletal_mesh_front_axis,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<u8>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_reimport_strategy_flags(&self, strategy_flag: &mut u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_reimport_strategy_flags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                strategy_flag,
                __buffer.add(0).cast::<u8>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_reimport_strategy_flags,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<u8>().swap(strategy_flag);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_nanite_triangle_threshold(
        &self,
        min_num_triangles: &mut i64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_nanite_triangle_threshold,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                min_num_triangles,
                __buffer.add(0).cast::<i64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_nanite_triangle_threshold,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i64>().swap(min_num_triangles);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_import_unused_material(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_import_unused_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_import_unused_material,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_axis_conversion_inverse_transform(
        &self,
        axis_conversion_inverse_transform: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_axis_conversion_inverse_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                axis_conversion_inverse_transform,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_axis_conversion_inverse_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(axis_conversion_inverse_transform);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_animated_time_start(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_animated_time_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_animated_time_start,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_animated_time_end(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_animated_time_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_animated_time_end,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_allow_scene_root_as_joint(
        &self,
        b_allow_scene_root_as_joint: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_allow_scene_root_as_joint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_allow_scene_root_as_joint,
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
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_source_node_get_custom_allow_scene_root_as_joint,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(b_allow_scene_root_as_joint);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeUserDefinedAttributesAPI {
    __padding_end: [u8; 48],
}
impl UInterchangeUserDefinedAttributesAPI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeUserDefinedAttributesAPI")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeUserDefinedAttributesAPI")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn remove_user_defined_attribute(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_remove_user_defined_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_remove_user_defined_attribute,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_infos(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_infos: &mut TArray<FInterchangeUserDefinedAttributeInfo>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_infos,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                user_defined_attribute_infos,
                __buffer.add(8).cast::<TArray<FInterchangeUserDefinedAttributeInfo>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_infos,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<FInterchangeUserDefinedAttributeInfo>>()
                .swap(user_defined_attribute_infos);
        }
    }
    pub fn get_user_defined_attribute_int32(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut i32,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_int32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_int32,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<i32>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_f_string(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut FString,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_f_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_value,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_f_string,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<FString>().swap(out_value);
        }
        unsafe {
            __buffer.add(40).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_float(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut f32,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_float,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<f32>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_double(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut f64,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_double,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_double,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<f64>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_user_defined_attribute_boolean(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        out_value: &mut bool,
        out_payload_key: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_boolean,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_value, __buffer.add(24).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_get_user_defined_attribute_boolean,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<bool>().swap(out_value);
        }
        unsafe {
            __buffer.add(32).cast::<FString>().swap(out_payload_key);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn duplicate_all_user_defined_attribute(
        interchange_source_node: UPtr<UInterchangeBaseNode>,
        interchange_destination_node: UPtr<UInterchangeBaseNode>,
        b_add_source_node_name: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_duplicate_all_user_defined_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_source_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_destination_node,
                __buffer.add(8).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_source_node_name,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_duplicate_all_user_defined_attribute,
                __buffer,
            )
        };
    }
    pub fn create_user_defined_attribute_int32(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &i32,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_int32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_f_string(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: FString,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_f_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<FString>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_f_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(57).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_float(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &f32,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_double(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &f64,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_double,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_double,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn create_user_defined_attribute_boolean(
        interchange_node: UPtr<UInterchangeBaseNode>,
        user_defined_attribute_name: FString,
        value: &bool,
        payload_key: FString,
        requires_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_boolean,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer.add(0).cast::<UPtr<UInterchangeBaseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_defined_attribute_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requires_delegate,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_core::UInterchangeUserDefinedAttributesAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_core::__FUNCTION_PTRS
                    .u_interchange_user_defined_attributes_api_create_user_defined_attribute_boolean,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
}
#[repr(transparent)]
pub struct EInterchangePipelineContext(pub u8);
impl EInterchangePipelineContext {
    pub const NONE: EInterchangePipelineContext = EInterchangePipelineContext(0);
    pub const ASSET_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(1);
    pub const ASSET_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        2,
    );
    pub const SCENE_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(3);
    pub const SCENE_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        4,
    );
    pub const ASSET_CUSTOM_LOD_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        5,
    );
    pub const ASSET_CUSTOM_LOD_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        6,
    );
    pub const ASSET_ALTERNATE_SKINNING_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        7,
    );
    pub const ASSET_ALTERNATE_SKINNING_REIMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        8,
    );
    pub const ASSET_CUSTOM_MORPH_TARGET_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        9,
    );
    pub const ASSET_CUSTOM_MORPH_TARGET_RE_IMPORT: EInterchangePipelineContext = EInterchangePipelineContext(
        10,
    );
}
#[repr(transparent)]
pub struct EInterchangeTranslatorAssetType(pub u8);
impl EInterchangeTranslatorAssetType {
    pub const NONE: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(0);
    pub const TEXTURES: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        1,
    );
    pub const MATERIALS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        2,
    );
    pub const MESHES: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        4,
    );
    pub const ANIMATIONS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        8,
    );
    pub const SOUNDS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        16,
    );
    pub const GROOMS: EInterchangeTranslatorAssetType = EInterchangeTranslatorAssetType(
        32,
    );
}
#[repr(transparent)]
pub struct EInterchangeFactoryAssetType(pub u8);
impl EInterchangeFactoryAssetType {
    pub const NONE: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(0);
    pub const TEXTURES: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(1);
    pub const MATERIALS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(2);
    pub const MESHES: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(3);
    pub const ANIMATIONS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(4);
    pub const PHYSICS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(5);
    pub const GROOMS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(6);
    pub const SOUNDS: EInterchangeFactoryAssetType = EInterchangeFactoryAssetType(7);
}
#[repr(transparent)]
pub struct EInterchangeTranslatorType(pub u8);
impl EInterchangeTranslatorType {
    pub const INVALID: EInterchangeTranslatorType = EInterchangeTranslatorType(0);
    pub const ASSETS: EInterchangeTranslatorType = EInterchangeTranslatorType(2);
    pub const ACTORS: EInterchangeTranslatorType = EInterchangeTranslatorType(4);
    pub const SCENES: EInterchangeTranslatorType = EInterchangeTranslatorType(6);
}
#[repr(transparent)]
pub struct EInterchangeNodeContainerType(pub u8);
impl EInterchangeNodeContainerType {
    pub const NONE: EInterchangeNodeContainerType = EInterchangeNodeContainerType(0);
    pub const TRANSLATED_SCENE: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        1,
    );
    pub const TRANSLATED_ASSET: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        2,
    );
    pub const FACTORY_DATA: EInterchangeNodeContainerType = EInterchangeNodeContainerType(
        3,
    );
}
#[repr(transparent)]
pub struct EReimportStrategyFlags(pub u8);
impl EReimportStrategyFlags {
    pub const APPLY_NO_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(0);
    pub const APPLY_PIPELINE_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(
        1,
    );
    pub const APPLY_EDITOR_CHANGED_PROPERTIES: EReimportStrategyFlags = EReimportStrategyFlags(
        2,
    );
}
#[repr(transparent)]
pub struct EInterchangeNodeUserInterfaceContext(pub u8);
impl EInterchangeNodeUserInterfaceContext {
    pub const NONE: EInterchangeNodeUserInterfaceContext = EInterchangeNodeUserInterfaceContext(
        0,
    );
    pub const PREVIEW: EInterchangeNodeUserInterfaceContext = EInterchangeNodeUserInterfaceContext(
        1,
    );
}
