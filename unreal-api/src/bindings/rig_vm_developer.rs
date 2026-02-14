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
    pub u_rig_vm_blueprint_suspend_notifications: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_split_asset_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_set_auto_vm_recompile: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_request_rig_vm_init: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_request_auto_vm_recompilation: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_rename_member_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_remove_model: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_remove_member_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_recompile_vm_if_required: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_recompile_vm: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_join_asset_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_rig_vm_host_class: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_or_create_local_function_library: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_or_create_controller: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_model: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_member_variables: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_matching_variants: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_local_function_library: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_focused_model: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_default_model: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_debugged_rig_vm_host: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_controller_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_controller: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_available_rig_vm_structs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_auto_vm_recompile: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_asset_variant_ref: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_asset_variant_bp: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_get_all_models: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_generate_python_commands: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_create_rig_vm_host: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_change_member_variable_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_bulk_remove_member_variables: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_add_model: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_blueprint_add_member_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_compiler_compile_vm: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_compiler_compile: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_set_is_excluded_by_early_exit: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_set_has_early_exit_marker: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_set_has_breakpoint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_set_execution_is_halted_at_this_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_visible_in_ui: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_trait_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_selected: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_pure: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_pin_category_expanded: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_mutable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_loop_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_linked_to: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_input_aggregate: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_injected: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_highlighted: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_excluded_by_early_exit: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_event: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_defined_as_varying: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_defined_as_constant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_control_flow_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_is_aggregate: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_pin_of_direction: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_output_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_orphaned_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_lazy_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_io_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_input_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_early_exit_marker: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_has_breakpoint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_trait_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_tool_tip_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_supported_workflows: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_sub_pin_categories: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_size: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_second_aggregate_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_root_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_previous_f_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_position: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_pins_for_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_pin_category_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_pin_categories: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_parent_pin_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_parent_pin_categories: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_orphaned_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_opposite_aggregate_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_node_title: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_node_sub_title: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_node_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_node_layout: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_node_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_node_color: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_next_aggregate_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_links: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_linked_target_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_linked_source_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_injection_info: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_graph_depth: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_first_aggregate_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_event_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_all_pins_recursively: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_aggregate_outputs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_get_aggregate_inputs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_find_root_pin_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_find_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_find_function_for_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_find_execute_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_execution_is_halted_at_this_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_can_only_exist_once: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_node_can_be_upgraded: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_template_node_is_singleton: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_template_node_is_resolved: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_template_node_is_fully_unresolved: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_template_node_get_script_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_template_node_get_notation: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_library_node_get_matching_variants: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_library_node_get_library: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_library_node_get_function_variant_ref: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_library_node_get_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_library_node_get_contained_graph: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_rig_vm_array_node_get_op_code: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_rig_vm_array_node_get_cpp_type_object: *mut crate::ffi::UFunctionOpague,
    pub udeprecated_rig_vm_array_node_get_cpp_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_comment_node_get_comment_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_comment_node_get_comment_font_size: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_comment_node_get_comment_color_bubble: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_comment_node_get_comment_bubble_visible: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_enum_node_get_enum: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_enum_node_get_cpp_type_object: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_enum_node_get_cpp_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_function_reference_node_get_referenced_function_header_for_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_invoke_entry_node_get_entry_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_parameter_node_is_input: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_parameter_node_get_parameter_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_parameter_node_get_parameter_description: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_parameter_node_get_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_parameter_node_get_cpp_type_object: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_parameter_node_get_cpp_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_unit_node_get_struct_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_unit_node_get_method_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_is_local_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_is_input_argument: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_is_getter: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_is_external_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_get_variable_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_get_variable_description: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_get_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_get_cpp_type_object: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_variable_node_get_cpp_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_split_variant_from_set: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_join_variant_set: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_get_variant_ref_for_asset: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_get_used_function_identifiers: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_get_function_identifier_for_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_get_asset_data_for_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_get_asset_data_for_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_get_all_function_identifiers: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_get: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_gather_function_variant_refs_for_asset: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_gather_all_function_variant_refs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_gather_all_asset_variant_refs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_find_function_variant_refs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_find_asset_variant_refs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_create_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_build_data_create_asset_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_action_stack_undo: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_action_stack_redo: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_action_stack_open_undo_bracket: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_action_stack_close_undo_bracket: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_action_stack_cancel_undo_bracket: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_set_schema_class: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_set_default_function_library: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_is_top_level_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_is_root_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_is_node_selected: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_is_node_highlighted: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_variable_descriptions: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_select_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_schema_class: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_schema: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_root_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_return_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_parent_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_output_arguments: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_node_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_local_variables: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_links: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_input_arguments: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_graph_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_graph_depth: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_event_names: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_entry_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_default_function_library: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_get_contained_graphs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_find_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_find_node_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_find_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_find_link: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_graph_contains_link: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_function_library_get_references_for_function: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_function_library_get_reference_paths_for_function: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_function_library_get_functions: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_function_library_find_function_for_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_function_library_find_function: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_target_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_target_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_source_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_source_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_pin_path_representation: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_opposite_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_link_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_graph_depth: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_link_get_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_injection_info_get_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_injection_info_get_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_should_only_show_sub_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_should_hide_sub_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_requires_watch: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_wild_card: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_valid_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_u_object: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_trait_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_struct_member: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_string_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_root_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_reference_counted_container: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_linked_to: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_lazy: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_interface: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_fixed_size_array: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_expanded: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_execute_context: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_enum: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_dynamic_array: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_defined_as_constant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_array_element: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_is_array: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_has_user_provided_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_has_original_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_has_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_has_default_value_override: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_tool_tip_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_target_links: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_sub_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_sub_pin_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_source_links: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_segment_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_script_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_root_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_pin_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_pin_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_pin_for_link: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_parent_script_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_parent_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_original_pin_from_injected_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_original_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_links: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_linked_target_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_linked_source_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_index_in_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_enum: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_direction: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_default_value_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_custom_widget_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_cpp_type_object: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_cpp_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_array_size: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_array_element_cpp_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_all_sub_pins_recursively: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_get_absolute_pin_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_find_sub_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_find_link_for_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_contains_wild_card_sub_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_pin_can_provide_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_registry_unregister_provider: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_registry_register_provider: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_registry_get_workflows: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_user_workflow_registry_get: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_upgrade_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_unresolve_template_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_undo: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_unbind_pin_from_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_swap_function_reference_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_swap_function_reference: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_swap_all_function_references: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_split_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_unit_node_defaults: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_schema_class: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_schema: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_remapped_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_is_watched: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_index_in_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_expansion: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_category_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_category_expansion: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_pin_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_title_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_title: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_size_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_size: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_selection: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_position_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_position: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_layout: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_keywords_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_keywords: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_description_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_description: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_color_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_color: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_category_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_node_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_local_variable_type_from_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_local_variable_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_local_variable_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_is_running_unit_test: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_exposed_pin_index: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_comment_text_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_comment_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_array_pin_size: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_set_action_stack: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_select_node_islands: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_select_node_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_select_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_select_linked_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_resolve_wild_card_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_reset_pin_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_reset_default_value_for_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_reset_default_value_for_all_pins_on_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_reset_default_value_for_all_pins_on_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_replace_parameter_node_with_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_rename_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_rename_pin_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_rename_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_rename_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_rename_local_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_rename_function: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_rename_exposed_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_trait: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_tag_from_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_pin_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_nodes_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_node_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_local_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_injected_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_function_from_library: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_exposed_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_array_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_remove_aggregate_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_refresh_variable_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_redo: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_push_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_promote_pin_to_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_promote_function_reference_node_to_collapse_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_promote_collapse_node_to_function_reference_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_pop_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_perform_user_workflow: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_open_undo_bracket: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_mark_function_as_public: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_make_variable_node_from_binding: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_make_options_for_workflow: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_make_bindings_from_variable_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_localize_functions: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_localize_function_from_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_localize_function: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_join_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_is_transacting: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_is_reporting_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_is_function_public: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_insert_array_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_import_nodes_from_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_unit_structs_for_template: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_top_level_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_template_for_unit_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_schema: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_registered_unit_structs: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_registered_templates: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_pin_default_value: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_controller_for_graph: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_get_action_stack: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_generate_python_commands: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_find_variants_of_function: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_find_graph_function_identifier: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_find_graph_function_header_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_find_graph_function_header: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_export_selected_nodes_to_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_export_node_to_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_export_nodes_to_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_expand_library_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_enable_reporting: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_eject_node_from_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_duplicate_array_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_create_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_collapse_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_close_undo_bracket: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_pin_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_override_on_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_override_on_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_override_on_all_pins_on_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_override_on_all_pins_on_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_node_selection: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_node_layout: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_clear_array_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_change_exposed_pin_type: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_can_import_nodes_from_text: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_cancel_undo_bracket: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_break_link: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_break_all_links: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_bind_pin_to_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_variable_node_from_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_variable_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_unit_node_with_defaults: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_unit_node_from_struct_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_unit_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_trait: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_template_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_tag_to_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_select_node_from_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_select_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_reroute_node_on_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_reroute_node_on_link_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_reroute_node_on_link: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_parameter_node_from_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_parameter_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_override_to_pins: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_override_to_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_override_to_all_pins_on_nodes: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_override_to_all_pins_on_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_local_variable_from_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_local_variable: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_link: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_invoke_entry_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_injected_node_from_struct_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_injected_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_if_node_from_struct: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_if_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_function_to_library: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_function_reference_node_from_description: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_function_reference_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_free_reroute_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_external_function_reference_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_exposed_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_enum_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_empty_pin_category: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_default_tag_to_function_variant: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_comment_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_branch_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_array_pin: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_array_node_from_object_path: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_array_node: *mut crate::ffi::UFunctionOpague,
    pub u_rig_vm_controller_add_aggregate_pin: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_rig_vm_blueprint_suspend_notifications: std::ptr::null_mut(),
            u_rig_vm_blueprint_split_asset_variant: std::ptr::null_mut(),
            u_rig_vm_blueprint_set_auto_vm_recompile: std::ptr::null_mut(),
            u_rig_vm_blueprint_request_rig_vm_init: std::ptr::null_mut(),
            u_rig_vm_blueprint_request_auto_vm_recompilation: std::ptr::null_mut(),
            u_rig_vm_blueprint_rename_member_variable: std::ptr::null_mut(),
            u_rig_vm_blueprint_remove_model: std::ptr::null_mut(),
            u_rig_vm_blueprint_remove_member_variable: std::ptr::null_mut(),
            u_rig_vm_blueprint_recompile_vm_if_required: std::ptr::null_mut(),
            u_rig_vm_blueprint_recompile_vm: std::ptr::null_mut(),
            u_rig_vm_blueprint_join_asset_variant: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_rig_vm_host_class: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_or_create_local_function_library: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_or_create_controller: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_model: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_member_variables: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_matching_variants: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_local_function_library: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_focused_model: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_default_model: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_debugged_rig_vm_host: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_controller_by_name: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_controller: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_available_rig_vm_structs: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_auto_vm_recompile: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_asset_variant_ref: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_asset_variant_bp: std::ptr::null_mut(),
            u_rig_vm_blueprint_get_all_models: std::ptr::null_mut(),
            u_rig_vm_blueprint_generate_python_commands: std::ptr::null_mut(),
            u_rig_vm_blueprint_create_rig_vm_host: std::ptr::null_mut(),
            u_rig_vm_blueprint_change_member_variable_type: std::ptr::null_mut(),
            u_rig_vm_blueprint_bulk_remove_member_variables: std::ptr::null_mut(),
            u_rig_vm_blueprint_add_model: std::ptr::null_mut(),
            u_rig_vm_blueprint_add_member_variable: std::ptr::null_mut(),
            u_rig_vm_compiler_compile_vm: std::ptr::null_mut(),
            u_rig_vm_compiler_compile: std::ptr::null_mut(),
            u_rig_vm_node_set_is_excluded_by_early_exit: std::ptr::null_mut(),
            u_rig_vm_node_set_has_early_exit_marker: std::ptr::null_mut(),
            u_rig_vm_node_set_has_breakpoint: std::ptr::null_mut(),
            u_rig_vm_node_set_execution_is_halted_at_this_node: std::ptr::null_mut(),
            u_rig_vm_node_is_visible_in_ui: std::ptr::null_mut(),
            u_rig_vm_node_is_trait_pin: std::ptr::null_mut(),
            u_rig_vm_node_is_selected: std::ptr::null_mut(),
            u_rig_vm_node_is_pure: std::ptr::null_mut(),
            u_rig_vm_node_is_pin_category_expanded: std::ptr::null_mut(),
            u_rig_vm_node_is_mutable: std::ptr::null_mut(),
            u_rig_vm_node_is_loop_node: std::ptr::null_mut(),
            u_rig_vm_node_is_linked_to: std::ptr::null_mut(),
            u_rig_vm_node_is_input_aggregate: std::ptr::null_mut(),
            u_rig_vm_node_is_injected: std::ptr::null_mut(),
            u_rig_vm_node_is_highlighted: std::ptr::null_mut(),
            u_rig_vm_node_is_excluded_by_early_exit: std::ptr::null_mut(),
            u_rig_vm_node_is_event: std::ptr::null_mut(),
            u_rig_vm_node_is_defined_as_varying: std::ptr::null_mut(),
            u_rig_vm_node_is_defined_as_constant: std::ptr::null_mut(),
            u_rig_vm_node_is_control_flow_node: std::ptr::null_mut(),
            u_rig_vm_node_is_aggregate: std::ptr::null_mut(),
            u_rig_vm_node_has_pin_of_direction: std::ptr::null_mut(),
            u_rig_vm_node_has_output_pin: std::ptr::null_mut(),
            u_rig_vm_node_has_orphaned_pins: std::ptr::null_mut(),
            u_rig_vm_node_has_lazy_pin: std::ptr::null_mut(),
            u_rig_vm_node_has_io_pin: std::ptr::null_mut(),
            u_rig_vm_node_has_input_pin: std::ptr::null_mut(),
            u_rig_vm_node_has_early_exit_marker: std::ptr::null_mut(),
            u_rig_vm_node_has_breakpoint: std::ptr::null_mut(),
            u_rig_vm_node_get_trait_pins: std::ptr::null_mut(),
            u_rig_vm_node_get_tool_tip_text: std::ptr::null_mut(),
            u_rig_vm_node_get_supported_workflows: std::ptr::null_mut(),
            u_rig_vm_node_get_sub_pin_categories: std::ptr::null_mut(),
            u_rig_vm_node_get_size: std::ptr::null_mut(),
            u_rig_vm_node_get_second_aggregate_pin: std::ptr::null_mut(),
            u_rig_vm_node_get_root_graph: std::ptr::null_mut(),
            u_rig_vm_node_get_previous_f_name: std::ptr::null_mut(),
            u_rig_vm_node_get_position: std::ptr::null_mut(),
            u_rig_vm_node_get_pins_for_category: std::ptr::null_mut(),
            u_rig_vm_node_get_pins: std::ptr::null_mut(),
            u_rig_vm_node_get_pin_category_name: std::ptr::null_mut(),
            u_rig_vm_node_get_pin_categories: std::ptr::null_mut(),
            u_rig_vm_node_get_parent_pin_category: std::ptr::null_mut(),
            u_rig_vm_node_get_parent_pin_categories: std::ptr::null_mut(),
            u_rig_vm_node_get_orphaned_pins: std::ptr::null_mut(),
            u_rig_vm_node_get_opposite_aggregate_pin: std::ptr::null_mut(),
            u_rig_vm_node_get_node_title: std::ptr::null_mut(),
            u_rig_vm_node_get_node_sub_title: std::ptr::null_mut(),
            u_rig_vm_node_get_node_path: std::ptr::null_mut(),
            u_rig_vm_node_get_node_layout: std::ptr::null_mut(),
            u_rig_vm_node_get_node_index: std::ptr::null_mut(),
            u_rig_vm_node_get_node_color: std::ptr::null_mut(),
            u_rig_vm_node_get_next_aggregate_name: std::ptr::null_mut(),
            u_rig_vm_node_get_links: std::ptr::null_mut(),
            u_rig_vm_node_get_linked_target_nodes: std::ptr::null_mut(),
            u_rig_vm_node_get_linked_source_nodes: std::ptr::null_mut(),
            u_rig_vm_node_get_injection_info: std::ptr::null_mut(),
            u_rig_vm_node_get_graph_depth: std::ptr::null_mut(),
            u_rig_vm_node_get_graph: std::ptr::null_mut(),
            u_rig_vm_node_get_first_aggregate_pin: std::ptr::null_mut(),
            u_rig_vm_node_get_event_name: std::ptr::null_mut(),
            u_rig_vm_node_get_all_pins_recursively: std::ptr::null_mut(),
            u_rig_vm_node_get_aggregate_outputs: std::ptr::null_mut(),
            u_rig_vm_node_get_aggregate_inputs: std::ptr::null_mut(),
            u_rig_vm_node_find_root_pin_by_name: std::ptr::null_mut(),
            u_rig_vm_node_find_pin: std::ptr::null_mut(),
            u_rig_vm_node_find_function_for_node: std::ptr::null_mut(),
            u_rig_vm_node_find_execute_pin: std::ptr::null_mut(),
            u_rig_vm_node_execution_is_halted_at_this_node: std::ptr::null_mut(),
            u_rig_vm_node_can_only_exist_once: std::ptr::null_mut(),
            u_rig_vm_node_can_be_upgraded: std::ptr::null_mut(),
            u_rig_vm_template_node_is_singleton: std::ptr::null_mut(),
            u_rig_vm_template_node_is_resolved: std::ptr::null_mut(),
            u_rig_vm_template_node_is_fully_unresolved: std::ptr::null_mut(),
            u_rig_vm_template_node_get_script_struct: std::ptr::null_mut(),
            u_rig_vm_template_node_get_notation: std::ptr::null_mut(),
            u_rig_vm_library_node_get_matching_variants: std::ptr::null_mut(),
            u_rig_vm_library_node_get_library: std::ptr::null_mut(),
            u_rig_vm_library_node_get_function_variant_ref: std::ptr::null_mut(),
            u_rig_vm_library_node_get_function_variant: std::ptr::null_mut(),
            u_rig_vm_library_node_get_contained_graph: std::ptr::null_mut(),
            udeprecated_rig_vm_array_node_get_op_code: std::ptr::null_mut(),
            udeprecated_rig_vm_array_node_get_cpp_type_object: std::ptr::null_mut(),
            udeprecated_rig_vm_array_node_get_cpp_type: std::ptr::null_mut(),
            u_rig_vm_comment_node_get_comment_text: std::ptr::null_mut(),
            u_rig_vm_comment_node_get_comment_font_size: std::ptr::null_mut(),
            u_rig_vm_comment_node_get_comment_color_bubble: std::ptr::null_mut(),
            u_rig_vm_comment_node_get_comment_bubble_visible: std::ptr::null_mut(),
            u_rig_vm_enum_node_get_enum: std::ptr::null_mut(),
            u_rig_vm_enum_node_get_cpp_type_object: std::ptr::null_mut(),
            u_rig_vm_enum_node_get_cpp_type: std::ptr::null_mut(),
            u_rig_vm_function_reference_node_get_referenced_function_header_for_blueprint: std::ptr::null_mut(),
            u_rig_vm_invoke_entry_node_get_entry_name: std::ptr::null_mut(),
            u_rig_vm_parameter_node_is_input: std::ptr::null_mut(),
            u_rig_vm_parameter_node_get_parameter_name: std::ptr::null_mut(),
            u_rig_vm_parameter_node_get_parameter_description: std::ptr::null_mut(),
            u_rig_vm_parameter_node_get_default_value: std::ptr::null_mut(),
            u_rig_vm_parameter_node_get_cpp_type_object: std::ptr::null_mut(),
            u_rig_vm_parameter_node_get_cpp_type: std::ptr::null_mut(),
            u_rig_vm_unit_node_get_struct_default_value: std::ptr::null_mut(),
            u_rig_vm_unit_node_get_method_name: std::ptr::null_mut(),
            u_rig_vm_variable_node_is_local_variable: std::ptr::null_mut(),
            u_rig_vm_variable_node_is_input_argument: std::ptr::null_mut(),
            u_rig_vm_variable_node_is_getter: std::ptr::null_mut(),
            u_rig_vm_variable_node_is_external_variable: std::ptr::null_mut(),
            u_rig_vm_variable_node_get_variable_name: std::ptr::null_mut(),
            u_rig_vm_variable_node_get_variable_description: std::ptr::null_mut(),
            u_rig_vm_variable_node_get_default_value: std::ptr::null_mut(),
            u_rig_vm_variable_node_get_cpp_type_object: std::ptr::null_mut(),
            u_rig_vm_variable_node_get_cpp_type: std::ptr::null_mut(),
            u_rig_vm_build_data_split_variant_from_set: std::ptr::null_mut(),
            u_rig_vm_build_data_join_variant_set: std::ptr::null_mut(),
            u_rig_vm_build_data_get_variant_ref_for_asset: std::ptr::null_mut(),
            u_rig_vm_build_data_get_used_function_identifiers: std::ptr::null_mut(),
            u_rig_vm_build_data_get_function_identifier_for_variant: std::ptr::null_mut(),
            u_rig_vm_build_data_get_asset_data_for_variant: std::ptr::null_mut(),
            u_rig_vm_build_data_get_asset_data_for_path: std::ptr::null_mut(),
            u_rig_vm_build_data_get_all_function_identifiers: std::ptr::null_mut(),
            u_rig_vm_build_data_get: std::ptr::null_mut(),
            u_rig_vm_build_data_gather_function_variant_refs_for_asset: std::ptr::null_mut(),
            u_rig_vm_build_data_gather_all_function_variant_refs: std::ptr::null_mut(),
            u_rig_vm_build_data_gather_all_asset_variant_refs: std::ptr::null_mut(),
            u_rig_vm_build_data_find_function_variant_refs: std::ptr::null_mut(),
            u_rig_vm_build_data_find_asset_variant_refs: std::ptr::null_mut(),
            u_rig_vm_build_data_create_function_variant: std::ptr::null_mut(),
            u_rig_vm_build_data_create_asset_variant: std::ptr::null_mut(),
            u_rig_vm_action_stack_undo: std::ptr::null_mut(),
            u_rig_vm_action_stack_redo: std::ptr::null_mut(),
            u_rig_vm_action_stack_open_undo_bracket: std::ptr::null_mut(),
            u_rig_vm_action_stack_close_undo_bracket: std::ptr::null_mut(),
            u_rig_vm_action_stack_cancel_undo_bracket: std::ptr::null_mut(),
            u_rig_vm_graph_set_schema_class: std::ptr::null_mut(),
            u_rig_vm_graph_set_default_function_library: std::ptr::null_mut(),
            u_rig_vm_graph_is_top_level_graph: std::ptr::null_mut(),
            u_rig_vm_graph_is_root_graph: std::ptr::null_mut(),
            u_rig_vm_graph_is_node_selected: std::ptr::null_mut(),
            u_rig_vm_graph_is_node_highlighted: std::ptr::null_mut(),
            u_rig_vm_graph_get_variable_descriptions: std::ptr::null_mut(),
            u_rig_vm_graph_get_select_nodes: std::ptr::null_mut(),
            u_rig_vm_graph_get_schema_class: std::ptr::null_mut(),
            u_rig_vm_graph_get_schema: std::ptr::null_mut(),
            u_rig_vm_graph_get_root_graph: std::ptr::null_mut(),
            u_rig_vm_graph_get_return_node: std::ptr::null_mut(),
            u_rig_vm_graph_get_parent_graph: std::ptr::null_mut(),
            u_rig_vm_graph_get_output_arguments: std::ptr::null_mut(),
            u_rig_vm_graph_get_nodes: std::ptr::null_mut(),
            u_rig_vm_graph_get_node_path: std::ptr::null_mut(),
            u_rig_vm_graph_get_local_variables: std::ptr::null_mut(),
            u_rig_vm_graph_get_links: std::ptr::null_mut(),
            u_rig_vm_graph_get_input_arguments: std::ptr::null_mut(),
            u_rig_vm_graph_get_graph_name: std::ptr::null_mut(),
            u_rig_vm_graph_get_graph_depth: std::ptr::null_mut(),
            u_rig_vm_graph_get_event_names: std::ptr::null_mut(),
            u_rig_vm_graph_get_entry_node: std::ptr::null_mut(),
            u_rig_vm_graph_get_default_function_library: std::ptr::null_mut(),
            u_rig_vm_graph_get_contained_graphs: std::ptr::null_mut(),
            u_rig_vm_graph_find_pin: std::ptr::null_mut(),
            u_rig_vm_graph_find_node_by_name: std::ptr::null_mut(),
            u_rig_vm_graph_find_node: std::ptr::null_mut(),
            u_rig_vm_graph_find_link: std::ptr::null_mut(),
            u_rig_vm_graph_contains_link: std::ptr::null_mut(),
            u_rig_vm_function_library_get_references_for_function: std::ptr::null_mut(),
            u_rig_vm_function_library_get_reference_paths_for_function: std::ptr::null_mut(),
            u_rig_vm_function_library_get_functions: std::ptr::null_mut(),
            u_rig_vm_function_library_find_function_for_node: std::ptr::null_mut(),
            u_rig_vm_function_library_find_function: std::ptr::null_mut(),
            u_rig_vm_link_get_target_pin: std::ptr::null_mut(),
            u_rig_vm_link_get_target_node: std::ptr::null_mut(),
            u_rig_vm_link_get_source_pin: std::ptr::null_mut(),
            u_rig_vm_link_get_source_node: std::ptr::null_mut(),
            u_rig_vm_link_get_pin_path_representation: std::ptr::null_mut(),
            u_rig_vm_link_get_opposite_pin: std::ptr::null_mut(),
            u_rig_vm_link_get_link_index: std::ptr::null_mut(),
            u_rig_vm_link_get_graph_depth: std::ptr::null_mut(),
            u_rig_vm_link_get_graph: std::ptr::null_mut(),
            u_rig_vm_injection_info_get_pin: std::ptr::null_mut(),
            u_rig_vm_injection_info_get_graph: std::ptr::null_mut(),
            u_rig_vm_pin_should_only_show_sub_pins: std::ptr::null_mut(),
            u_rig_vm_pin_should_hide_sub_pins: std::ptr::null_mut(),
            u_rig_vm_pin_requires_watch: std::ptr::null_mut(),
            u_rig_vm_pin_is_wild_card: std::ptr::null_mut(),
            u_rig_vm_pin_is_valid_default_value: std::ptr::null_mut(),
            u_rig_vm_pin_is_u_object: std::ptr::null_mut(),
            u_rig_vm_pin_is_trait_pin: std::ptr::null_mut(),
            u_rig_vm_pin_is_struct_member: std::ptr::null_mut(),
            u_rig_vm_pin_is_struct: std::ptr::null_mut(),
            u_rig_vm_pin_is_string_type: std::ptr::null_mut(),
            u_rig_vm_pin_is_root_pin: std::ptr::null_mut(),
            u_rig_vm_pin_is_reference_counted_container: std::ptr::null_mut(),
            u_rig_vm_pin_is_linked_to: std::ptr::null_mut(),
            u_rig_vm_pin_is_lazy: std::ptr::null_mut(),
            u_rig_vm_pin_is_interface: std::ptr::null_mut(),
            u_rig_vm_pin_is_fixed_size_array: std::ptr::null_mut(),
            u_rig_vm_pin_is_expanded: std::ptr::null_mut(),
            u_rig_vm_pin_is_execute_context: std::ptr::null_mut(),
            u_rig_vm_pin_is_enum: std::ptr::null_mut(),
            u_rig_vm_pin_is_dynamic_array: std::ptr::null_mut(),
            u_rig_vm_pin_is_defined_as_constant: std::ptr::null_mut(),
            u_rig_vm_pin_is_array_element: std::ptr::null_mut(),
            u_rig_vm_pin_is_array: std::ptr::null_mut(),
            u_rig_vm_pin_has_user_provided_default_value: std::ptr::null_mut(),
            u_rig_vm_pin_has_original_default_value: std::ptr::null_mut(),
            u_rig_vm_pin_has_meta_data: std::ptr::null_mut(),
            u_rig_vm_pin_has_default_value_override: std::ptr::null_mut(),
            u_rig_vm_pin_get_tool_tip_text: std::ptr::null_mut(),
            u_rig_vm_pin_get_target_links: std::ptr::null_mut(),
            u_rig_vm_pin_get_sub_pins: std::ptr::null_mut(),
            u_rig_vm_pin_get_sub_pin_path: std::ptr::null_mut(),
            u_rig_vm_pin_get_source_links: std::ptr::null_mut(),
            u_rig_vm_pin_get_segment_path: std::ptr::null_mut(),
            u_rig_vm_pin_get_script_struct: std::ptr::null_mut(),
            u_rig_vm_pin_get_root_pin: std::ptr::null_mut(),
            u_rig_vm_pin_get_pin_path: std::ptr::null_mut(),
            u_rig_vm_pin_get_pin_index: std::ptr::null_mut(),
            u_rig_vm_pin_get_pin_for_link: std::ptr::null_mut(),
            u_rig_vm_pin_get_parent_script_struct: std::ptr::null_mut(),
            u_rig_vm_pin_get_parent_pin: std::ptr::null_mut(),
            u_rig_vm_pin_get_original_pin_from_injected_node: std::ptr::null_mut(),
            u_rig_vm_pin_get_original_default_value: std::ptr::null_mut(),
            u_rig_vm_pin_get_node: std::ptr::null_mut(),
            u_rig_vm_pin_get_meta_data: std::ptr::null_mut(),
            u_rig_vm_pin_get_links: std::ptr::null_mut(),
            u_rig_vm_pin_get_linked_target_pins: std::ptr::null_mut(),
            u_rig_vm_pin_get_linked_source_pins: std::ptr::null_mut(),
            u_rig_vm_pin_get_index_in_category: std::ptr::null_mut(),
            u_rig_vm_pin_get_graph: std::ptr::null_mut(),
            u_rig_vm_pin_get_enum: std::ptr::null_mut(),
            u_rig_vm_pin_get_display_name: std::ptr::null_mut(),
            u_rig_vm_pin_get_direction: std::ptr::null_mut(),
            u_rig_vm_pin_get_default_value_type: std::ptr::null_mut(),
            u_rig_vm_pin_get_default_value: std::ptr::null_mut(),
            u_rig_vm_pin_get_custom_widget_name: std::ptr::null_mut(),
            u_rig_vm_pin_get_cpp_type_object: std::ptr::null_mut(),
            u_rig_vm_pin_get_cpp_type: std::ptr::null_mut(),
            u_rig_vm_pin_get_category: std::ptr::null_mut(),
            u_rig_vm_pin_get_array_size: std::ptr::null_mut(),
            u_rig_vm_pin_get_array_element_cpp_type: std::ptr::null_mut(),
            u_rig_vm_pin_get_all_sub_pins_recursively: std::ptr::null_mut(),
            u_rig_vm_pin_get_absolute_pin_index: std::ptr::null_mut(),
            u_rig_vm_pin_find_sub_pin: std::ptr::null_mut(),
            u_rig_vm_pin_find_link_for_pin: std::ptr::null_mut(),
            u_rig_vm_pin_contains_wild_card_sub_pin: std::ptr::null_mut(),
            u_rig_vm_pin_can_provide_default_value: std::ptr::null_mut(),
            u_rig_vm_user_workflow_registry_unregister_provider: std::ptr::null_mut(),
            u_rig_vm_user_workflow_registry_register_provider: std::ptr::null_mut(),
            u_rig_vm_user_workflow_registry_get_workflows: std::ptr::null_mut(),
            u_rig_vm_user_workflow_registry_get: std::ptr::null_mut(),
            u_rig_vm_controller_upgrade_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_unresolve_template_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_undo: std::ptr::null_mut(),
            u_rig_vm_controller_unbind_pin_from_variable: std::ptr::null_mut(),
            u_rig_vm_controller_swap_function_reference_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_swap_function_reference: std::ptr::null_mut(),
            u_rig_vm_controller_swap_all_function_references: std::ptr::null_mut(),
            u_rig_vm_controller_split_function_variant: std::ptr::null_mut(),
            u_rig_vm_controller_set_unit_node_defaults: std::ptr::null_mut(),
            u_rig_vm_controller_set_schema_class: std::ptr::null_mut(),
            u_rig_vm_controller_set_schema: std::ptr::null_mut(),
            u_rig_vm_controller_set_remapped_variable: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_is_watched: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_index_in_category: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_expansion: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_display_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_default_value: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_category_index: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_category_expansion: std::ptr::null_mut(),
            u_rig_vm_controller_set_pin_category: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_title_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_title: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_size_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_size: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_selection: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_position_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_position: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_layout: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_keywords_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_keywords: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_description_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_description: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_color_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_color: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_category_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_node_category: std::ptr::null_mut(),
            u_rig_vm_controller_set_local_variable_type_from_object_path: std::ptr::null_mut(),
            u_rig_vm_controller_set_local_variable_type: std::ptr::null_mut(),
            u_rig_vm_controller_set_local_variable_default_value: std::ptr::null_mut(),
            u_rig_vm_controller_set_is_running_unit_test: std::ptr::null_mut(),
            u_rig_vm_controller_set_graph: std::ptr::null_mut(),
            u_rig_vm_controller_set_exposed_pin_index: std::ptr::null_mut(),
            u_rig_vm_controller_set_comment_text_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_set_comment_text: std::ptr::null_mut(),
            u_rig_vm_controller_set_array_pin_size: std::ptr::null_mut(),
            u_rig_vm_controller_set_action_stack: std::ptr::null_mut(),
            u_rig_vm_controller_select_node_islands: std::ptr::null_mut(),
            u_rig_vm_controller_select_node_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_select_node: std::ptr::null_mut(),
            u_rig_vm_controller_select_linked_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_resolve_wild_card_pin: std::ptr::null_mut(),
            u_rig_vm_controller_reset_pin_default_value: std::ptr::null_mut(),
            u_rig_vm_controller_reset_default_value_for_pins: std::ptr::null_mut(),
            u_rig_vm_controller_reset_default_value_for_all_pins_on_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_reset_default_value_for_all_pins_on_node: std::ptr::null_mut(),
            u_rig_vm_controller_replace_parameter_node_with_variable: std::ptr::null_mut(),
            u_rig_vm_controller_rename_variable: std::ptr::null_mut(),
            u_rig_vm_controller_rename_pin_category: std::ptr::null_mut(),
            u_rig_vm_controller_rename_parameter: std::ptr::null_mut(),
            u_rig_vm_controller_rename_node: std::ptr::null_mut(),
            u_rig_vm_controller_rename_local_variable: std::ptr::null_mut(),
            u_rig_vm_controller_rename_function: std::ptr::null_mut(),
            u_rig_vm_controller_rename_exposed_pin: std::ptr::null_mut(),
            u_rig_vm_controller_remove_trait: std::ptr::null_mut(),
            u_rig_vm_controller_remove_tag_from_function_variant: std::ptr::null_mut(),
            u_rig_vm_controller_remove_pin_category: std::ptr::null_mut(),
            u_rig_vm_controller_remove_nodes_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_remove_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_remove_node_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_remove_node: std::ptr::null_mut(),
            u_rig_vm_controller_remove_local_variable: std::ptr::null_mut(),
            u_rig_vm_controller_remove_injected_node: std::ptr::null_mut(),
            u_rig_vm_controller_remove_function_from_library: std::ptr::null_mut(),
            u_rig_vm_controller_remove_exposed_pin: std::ptr::null_mut(),
            u_rig_vm_controller_remove_array_pin: std::ptr::null_mut(),
            u_rig_vm_controller_remove_aggregate_pin: std::ptr::null_mut(),
            u_rig_vm_controller_refresh_variable_node: std::ptr::null_mut(),
            u_rig_vm_controller_redo: std::ptr::null_mut(),
            u_rig_vm_controller_push_graph: std::ptr::null_mut(),
            u_rig_vm_controller_promote_pin_to_variable: std::ptr::null_mut(),
            u_rig_vm_controller_promote_function_reference_node_to_collapse_node: std::ptr::null_mut(),
            u_rig_vm_controller_promote_collapse_node_to_function_reference_node: std::ptr::null_mut(),
            u_rig_vm_controller_pop_graph: std::ptr::null_mut(),
            u_rig_vm_controller_perform_user_workflow: std::ptr::null_mut(),
            u_rig_vm_controller_open_undo_bracket: std::ptr::null_mut(),
            u_rig_vm_controller_mark_function_as_public: std::ptr::null_mut(),
            u_rig_vm_controller_make_variable_node_from_binding: std::ptr::null_mut(),
            u_rig_vm_controller_make_options_for_workflow: std::ptr::null_mut(),
            u_rig_vm_controller_make_bindings_from_variable_node: std::ptr::null_mut(),
            u_rig_vm_controller_localize_functions: std::ptr::null_mut(),
            u_rig_vm_controller_localize_function_from_path: std::ptr::null_mut(),
            u_rig_vm_controller_localize_function: std::ptr::null_mut(),
            u_rig_vm_controller_join_function_variant: std::ptr::null_mut(),
            u_rig_vm_controller_is_transacting: std::ptr::null_mut(),
            u_rig_vm_controller_is_reporting_enabled: std::ptr::null_mut(),
            u_rig_vm_controller_is_function_public: std::ptr::null_mut(),
            u_rig_vm_controller_insert_array_pin: std::ptr::null_mut(),
            u_rig_vm_controller_import_nodes_from_text: std::ptr::null_mut(),
            u_rig_vm_controller_get_unit_structs_for_template: std::ptr::null_mut(),
            u_rig_vm_controller_get_top_level_graph: std::ptr::null_mut(),
            u_rig_vm_controller_get_template_for_unit_struct: std::ptr::null_mut(),
            u_rig_vm_controller_get_schema: std::ptr::null_mut(),
            u_rig_vm_controller_get_registered_unit_structs: std::ptr::null_mut(),
            u_rig_vm_controller_get_registered_templates: std::ptr::null_mut(),
            u_rig_vm_controller_get_pin_default_value: std::ptr::null_mut(),
            u_rig_vm_controller_get_graph: std::ptr::null_mut(),
            u_rig_vm_controller_get_controller_for_graph: std::ptr::null_mut(),
            u_rig_vm_controller_get_action_stack: std::ptr::null_mut(),
            u_rig_vm_controller_generate_python_commands: std::ptr::null_mut(),
            u_rig_vm_controller_find_variants_of_function: std::ptr::null_mut(),
            u_rig_vm_controller_find_graph_function_identifier: std::ptr::null_mut(),
            u_rig_vm_controller_find_graph_function_header_by_name: std::ptr::null_mut(),
            u_rig_vm_controller_find_graph_function_header: std::ptr::null_mut(),
            u_rig_vm_controller_export_selected_nodes_to_text: std::ptr::null_mut(),
            u_rig_vm_controller_export_node_to_text: std::ptr::null_mut(),
            u_rig_vm_controller_export_nodes_to_text: std::ptr::null_mut(),
            u_rig_vm_controller_expand_library_node: std::ptr::null_mut(),
            u_rig_vm_controller_enable_reporting: std::ptr::null_mut(),
            u_rig_vm_controller_eject_node_from_pin: std::ptr::null_mut(),
            u_rig_vm_controller_duplicate_array_pin: std::ptr::null_mut(),
            u_rig_vm_controller_create_function_variant: std::ptr::null_mut(),
            u_rig_vm_controller_collapse_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_close_undo_bracket: std::ptr::null_mut(),
            u_rig_vm_controller_clear_pin_category: std::ptr::null_mut(),
            u_rig_vm_controller_clear_override_on_pins: std::ptr::null_mut(),
            u_rig_vm_controller_clear_override_on_pin: std::ptr::null_mut(),
            u_rig_vm_controller_clear_override_on_all_pins_on_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_clear_override_on_all_pins_on_node: std::ptr::null_mut(),
            u_rig_vm_controller_clear_node_selection: std::ptr::null_mut(),
            u_rig_vm_controller_clear_node_layout: std::ptr::null_mut(),
            u_rig_vm_controller_clear_array_pin: std::ptr::null_mut(),
            u_rig_vm_controller_change_exposed_pin_type: std::ptr::null_mut(),
            u_rig_vm_controller_can_import_nodes_from_text: std::ptr::null_mut(),
            u_rig_vm_controller_cancel_undo_bracket: std::ptr::null_mut(),
            u_rig_vm_controller_break_link: std::ptr::null_mut(),
            u_rig_vm_controller_break_all_links: std::ptr::null_mut(),
            u_rig_vm_controller_bind_pin_to_variable: std::ptr::null_mut(),
            u_rig_vm_controller_add_variable_node_from_object_path: std::ptr::null_mut(),
            u_rig_vm_controller_add_variable_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_unit_node_with_defaults: std::ptr::null_mut(),
            u_rig_vm_controller_add_unit_node_from_struct_path: std::ptr::null_mut(),
            u_rig_vm_controller_add_unit_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_trait: std::ptr::null_mut(),
            u_rig_vm_controller_add_template_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_tag_to_function_variant: std::ptr::null_mut(),
            u_rig_vm_controller_add_select_node_from_struct: std::ptr::null_mut(),
            u_rig_vm_controller_add_select_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_reroute_node_on_pin: std::ptr::null_mut(),
            u_rig_vm_controller_add_reroute_node_on_link_path: std::ptr::null_mut(),
            u_rig_vm_controller_add_reroute_node_on_link: std::ptr::null_mut(),
            u_rig_vm_controller_add_parameter_node_from_object_path: std::ptr::null_mut(),
            u_rig_vm_controller_add_parameter_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_override_to_pins: std::ptr::null_mut(),
            u_rig_vm_controller_add_override_to_pin: std::ptr::null_mut(),
            u_rig_vm_controller_add_override_to_all_pins_on_nodes: std::ptr::null_mut(),
            u_rig_vm_controller_add_override_to_all_pins_on_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_local_variable_from_object_path: std::ptr::null_mut(),
            u_rig_vm_controller_add_local_variable: std::ptr::null_mut(),
            u_rig_vm_controller_add_link: std::ptr::null_mut(),
            u_rig_vm_controller_add_invoke_entry_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_injected_node_from_struct_path: std::ptr::null_mut(),
            u_rig_vm_controller_add_injected_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_if_node_from_struct: std::ptr::null_mut(),
            u_rig_vm_controller_add_if_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_function_to_library: std::ptr::null_mut(),
            u_rig_vm_controller_add_function_reference_node_from_description: std::ptr::null_mut(),
            u_rig_vm_controller_add_function_reference_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_free_reroute_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_external_function_reference_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_exposed_pin: std::ptr::null_mut(),
            u_rig_vm_controller_add_enum_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_empty_pin_category: std::ptr::null_mut(),
            u_rig_vm_controller_add_default_tag_to_function_variant: std::ptr::null_mut(),
            u_rig_vm_controller_add_comment_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_branch_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_array_pin: std::ptr::null_mut(),
            u_rig_vm_controller_add_array_node_from_object_path: std::ptr::null_mut(),
            u_rig_vm_controller_add_array_node: std::ptr::null_mut(),
            u_rig_vm_controller_add_aggregate_pin: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMBlueprint::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SuspendNotifications"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_suspend_notifications,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SplitAssetVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_split_asset_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetAutoVMRecompile"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_set_auto_vm_recompile,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestRigVMInit"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_request_rig_vm_init,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestAutoVMRecompilation"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_request_auto_vm_recompilation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameMemberVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_rename_member_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveModel"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_remove_model,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveMemberVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_remove_member_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RecompileVMIfRequired"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_recompile_vm_if_required,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RecompileVM"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_recompile_vm,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("JoinAssetVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_join_asset_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRigVMHostClass"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_rig_vm_host_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOrCreateLocalFunctionLibrary"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_or_create_local_function_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOrCreateController"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_or_create_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetModel"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_model,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMemberVariables"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_member_variables,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMatchingVariants"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_matching_variants,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalFunctionLibrary"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_local_function_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFocusedModel"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_focused_model,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultModel"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_default_model,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDebuggedRigVMHost"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_debugged_rig_vm_host,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControllerByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_controller_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetController"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAvailableRigVMStructs"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_available_rig_vm_structs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAutoVMRecompile"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_auto_vm_recompile,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssetVariantRef"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_asset_variant_ref,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssetVariantBP"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_asset_variant_bp,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllModels"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_get_all_models,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GeneratePythonCommands"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_generate_python_commands,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateRigVMHost"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_create_rig_vm_host,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ChangeMemberVariableType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_change_member_variable_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BulkRemoveMemberVariables"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_bulk_remove_member_variables,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddModel"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_add_model,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddMemberVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_blueprint_add_member_variable,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMCompiler::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CompileVM"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_compiler_compile_vm,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Compile"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_compiler_compile,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetIsExcludedByEarlyExit"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_set_is_excluded_by_early_exit,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHasEarlyExitMarker"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_set_has_early_exit_marker,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetHasBreakpoint"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_set_has_breakpoint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetExecutionIsHaltedAtThisNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_node_set_execution_is_halted_at_this_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsVisibleInUI"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_visible_in_ui,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTraitPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_trait_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSelected"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPure"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_pure,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsPinCategoryExpanded"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_pin_category_expanded,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsMutable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_mutable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLoopNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_loop_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLinkedTo"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_linked_to,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInputAggregate"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_input_aggregate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInjected"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_injected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsHighlighted"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_highlighted,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsExcludedByEarlyExit"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_excluded_by_early_exit,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEvent"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_event,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDefinedAsVarying"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_defined_as_varying,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDefinedAsConstant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_defined_as_constant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsControlFlowNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_control_flow_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAggregate"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_is_aggregate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasPinOfDirection"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_pin_of_direction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasOutputPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_output_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasOrphanedPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_orphaned_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasLazyPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_lazy_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasIOPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_io_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasInputPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_input_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasEarlyExitMarker"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_early_exit_marker,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasBreakpoint"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_has_breakpoint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTraitPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_trait_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetToolTipText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_tool_tip_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSupportedWorkflows"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_supported_workflows,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSubPinCategories"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_sub_pin_categories,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSize"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSecondAggregatePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_second_aggregate_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRootGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_root_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviousFName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_previous_f_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPosition"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinsForCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_pins_for_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinCategoryName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_pin_category_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinCategories"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_pin_categories,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentPinCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_parent_pin_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentPinCategories"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_parent_pin_categories,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOrphanedPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_orphaned_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOppositeAggregatePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_opposite_aggregate_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeTitle"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_node_title,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeSubTitle"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_node_sub_title,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodePath"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_node_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeLayout"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_node_layout,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_node_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodeColor"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_node_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNextAggregateName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_next_aggregate_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinks"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_links,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinkedTargetNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_linked_target_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinkedSourceNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_linked_source_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInjectionInfo"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_injection_info,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraphDepth"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_graph_depth,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFirstAggregatePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_first_aggregate_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEventName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_event_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllPinsRecursively"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_all_pins_recursively,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAggregateOutputs"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_aggregate_outputs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAggregateInputs"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_get_aggregate_inputs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindRootPinByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_find_root_pin_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_find_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindFunctionForNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_find_function_for_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindExecutePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_find_execute_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExecutionIsHaltedAtThisNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_execution_is_halted_at_this_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanOnlyExistOnce"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_can_only_exist_once,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanBeUpgraded"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_node_can_be_upgraded,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMTemplateNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsSingleton"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_template_node_is_singleton,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsResolved"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_template_node_is_resolved,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsFullyUnresolved"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_template_node_is_fully_unresolved,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetScriptStruct"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_template_node_get_script_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNotation"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_template_node_get_notation,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMLibraryNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMatchingVariants"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_library_node_get_matching_variants,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLibrary"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_library_node_get_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFunctionVariantRef"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_library_node_get_function_variant_ref,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFunctionVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_library_node_get_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetContainedGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_library_node_get_contained_graph,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UDEPRECATED_RigVMArrayNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOpCode"),
                &raw mut __FUNCTION_PTRS.udeprecated_rig_vm_array_node_get_op_code,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
                &raw mut __FUNCTION_PTRS
                    .udeprecated_rig_vm_array_node_get_cpp_type_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPType"),
                &raw mut __FUNCTION_PTRS.udeprecated_rig_vm_array_node_get_cpp_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMCommentNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCommentText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_comment_node_get_comment_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCommentFontSize"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_comment_node_get_comment_font_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCommentColorBubble"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_comment_node_get_comment_color_bubble,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCommentBubbleVisible"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_comment_node_get_comment_bubble_visible,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMEnumNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnum"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_enum_node_get_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_enum_node_get_cpp_type_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_enum_node_get_cpp_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMFunctionReferenceNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReferencedFunctionHeader_ForBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_function_reference_node_get_referenced_function_header_for_blueprint,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMInvokeEntryNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEntryName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_invoke_entry_node_get_entry_name,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMParameterNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInput"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_parameter_node_is_input,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParameterName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_parameter_node_get_parameter_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParameterDescription"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_parameter_description,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_parameter_node_get_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_parameter_node_get_cpp_type_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_parameter_node_get_cpp_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMUnitNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetStructDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_unit_node_get_struct_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMethodName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_unit_node_get_method_name,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMVariableNode::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLocalVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_is_local_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInputArgument"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_is_input_argument,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsGetter"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_is_getter,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsExternalVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_is_external_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariableName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_get_variable_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariableDescription"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_get_variable_description,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_get_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_get_cpp_type_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_variable_node_get_cpp_type,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMBuildData::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SplitVariantFromSet"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_split_variant_from_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("JoinVariantSet"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_join_variant_set,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariantRefForAsset"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_get_variant_ref_for_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUsedFunctionIdentifiers"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_build_data_get_used_function_identifiers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFunctionIdentifierForVariant"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_build_data_get_function_identifier_for_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssetDataForVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_get_asset_data_for_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAssetDataForPath"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_get_asset_data_for_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllFunctionIdentifiers"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_get_all_function_identifiers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Get"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_get,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GatherFunctionVariantRefsForAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_function_variant_refs_for_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GatherAllFunctionVariantRefs"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_all_function_variant_refs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GatherAllAssetVariantRefs"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_all_asset_variant_refs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindFunctionVariantRefs"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_find_function_variant_refs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindAssetVariantRefs"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_find_asset_variant_refs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateFunctionVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_create_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateAssetVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_build_data_create_asset_variant,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMActionStack::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Undo"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_action_stack_undo,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Redo"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_action_stack_redo,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenUndoBracket"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_action_stack_open_undo_bracket,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CloseUndoBracket"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_action_stack_close_undo_bracket,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelUndoBracket"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_action_stack_cancel_undo_bracket,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMGraph::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSchemaClass"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_set_schema_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetDefaultFunctionLibrary"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_set_default_function_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTopLevelGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_is_top_level_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRootGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_is_root_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsNodeSelected"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_is_node_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsNodeHighlighted"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_is_node_highlighted,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVariableDescriptions"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_variable_descriptions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_select_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSchemaClass"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_schema_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSchema"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_schema,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRootGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_root_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReturnNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_return_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_parent_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOutputArguments"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_output_arguments,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNodePath"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_node_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalVariables"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_local_variables,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinks"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_links,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetInputArguments"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_input_arguments,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraphName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_graph_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraphDepth"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_graph_depth,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEventNames"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_event_names,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEntryNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_entry_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultFunctionLibrary"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_default_function_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetContainedGraphs"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_get_contained_graphs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_find_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindNodeByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_find_node_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_find_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindLink"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_find_link,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ContainsLink"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_graph_contains_link,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMFunctionLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReferencesForFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_function_library_get_references_for_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetReferencePathsForFunction"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_function_library_get_reference_paths_for_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFunctions"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_function_library_get_functions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindFunctionForNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_function_library_find_function_for_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindFunction"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_function_library_find_function,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMLink::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_target_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_target_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourcePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_source_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_source_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinPathRepresentation"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_pin_path_representation,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOppositePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_opposite_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinkIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_link_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraphDepth"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_graph_depth,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_link_get_graph,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMInjectionInfo::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_injection_info_get_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_injection_info_get_graph,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMPin::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldOnlyShowSubPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_should_only_show_sub_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShouldHideSubPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_should_hide_sub_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequiresWatch"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_requires_watch,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsWildCard"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_wild_card,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsValidDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_valid_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsUObject"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_u_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTraitPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_trait_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsStructMember"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_struct_member,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsStruct"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsStringType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_string_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsRootPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_root_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReferenceCountedContainer"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_reference_counted_container,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLinkedTo"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_linked_to,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLazy"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_lazy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsInterface"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_interface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsFixedSizeArray"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_fixed_size_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsExpanded"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_expanded,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsExecuteContext"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_execute_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsEnum"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDynamicArray"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_dynamic_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsDefinedAsConstant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_defined_as_constant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsArrayElement"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_array_element,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsArray"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_is_array,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasUserProvidedDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_has_user_provided_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasOriginalDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_has_original_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasMetaData"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_has_meta_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HasDefaultValueOverride"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_has_default_value_override,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetToolTipText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_tool_tip_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTargetLinks"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_target_links,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSubPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_sub_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSubPinPath"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_sub_pin_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSourceLinks"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_source_links,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSegmentPath"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_segment_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetScriptStruct"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_script_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRootPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_root_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinPath"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_pin_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_pin_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinForLink"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_pin_for_link,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentScriptStruct"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_parent_script_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetParentPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_parent_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOriginalPinFromInjectedNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_original_pin_from_injected_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetOriginalDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_original_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetMetaData"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_meta_data,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinks"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_links,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinkedTargetPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_linked_target_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLinkedSourcePins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_linked_source_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetIndexInCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_index_in_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetEnum"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_enum,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDisplayName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDirection"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_direction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultValueType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_default_value_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCustomWidgetName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_custom_widget_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPTypeObject"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_cpp_type_object,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCPPType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_cpp_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetArraySize"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_array_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetArrayElementCppType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_array_element_cpp_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllSubPinsRecursively"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_all_sub_pins_recursively,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAbsolutePinIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_get_absolute_pin_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindSubPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_find_sub_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindLinkForPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_find_link_for_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ContainsWildCardSubPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_contains_wild_card_sub_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanProvideDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_pin_can_provide_default_value,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMUserWorkflowRegistry::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnregisterProvider"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_unregister_provider,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RegisterProvider"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_register_provider,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWorkflows"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_user_workflow_registry_get_workflows,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Get"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_user_workflow_registry_get,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = URigVMController::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UpgradeNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_upgrade_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnresolveTemplateNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_unresolve_template_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Undo"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_undo,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("UnbindPinFromVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_unbind_pin_from_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwapFunctionReferenceByName"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_swap_function_reference_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwapFunctionReference"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_swap_function_reference,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SwapAllFunctionReferences"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_swap_all_function_references,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SplitFunctionVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_split_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetUnitNodeDefaults"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_unit_node_defaults,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSchemaClass"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_schema_class,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSchema"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_schema,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetRemappedVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_remapped_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinIsWatched"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_is_watched,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinIndexInCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_index_in_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinExpansion"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_expansion,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinDisplayName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_display_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinCategoryIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_category_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinCategoryExpansion"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_category_expansion,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPinCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_pin_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeTitleByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_title_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeTitle"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_title,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeSizeByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_size_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeSize"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeSelection"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodePositionByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_position_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodePosition"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeLayout"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_layout,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeKeywordsByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_keywords_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeKeywords"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_keywords,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeDescriptionByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_description_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeDescription"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_description,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeColorByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_color_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeColor"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeCategoryByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_category_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetNodeCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_node_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalVariableTypeFromObjectPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_type_from_object_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalVariableType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_local_variable_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalVariableDefaultValue"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetIsRunningUnitTest"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_is_running_unit_test,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetExposedPinIndex"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_exposed_pin_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCommentTextByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_comment_text_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetCommentText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_comment_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetArrayPinSize"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_array_pin_size,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActionStack"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_set_action_stack,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectNodeIslands"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_select_node_islands,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectNodeByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_select_node_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_select_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectLinkedNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_select_linked_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResolveWildCardPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_resolve_wild_card_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetPinDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_reset_pin_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetDefaultValueForPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_reset_default_value_for_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetDefaultValueForAllPinsOnNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_all_pins_on_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ResetDefaultValueForAllPinsOnNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_all_pins_on_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ReplaceParameterNodeWithVariable"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_replace_parameter_node_with_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_rename_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenamePinCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_rename_pin_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameParameter"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_rename_parameter,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_rename_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameLocalVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_rename_local_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameFunction"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_rename_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameExposedPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_rename_exposed_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveTrait"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_trait,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveTagFromFunctionVariant"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_remove_tag_from_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemovePinCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_pin_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveNodesByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_nodes_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveNodeByName"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_node_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveLocalVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_local_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveInjectedNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_injected_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveFunctionFromLibrary"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_function_from_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveExposedPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_exposed_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveArrayPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_array_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveAggregatePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_remove_aggregate_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RefreshVariableNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_refresh_variable_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Redo"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_redo,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PushGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_push_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PromotePinToVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_promote_pin_to_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PromoteFunctionReferenceNodeToCollapseNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_promote_function_reference_node_to_collapse_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PromoteCollapseNodeToFunctionReferenceNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_promote_collapse_node_to_function_reference_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PopGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_pop_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PerformUserWorkflow"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_perform_user_workflow,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OpenUndoBracket"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_open_undo_bracket,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MarkFunctionAsPublic"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_mark_function_as_public,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeVariableNodeFromBinding"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_make_variable_node_from_binding,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeOptionsForWorkflow"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_make_options_for_workflow,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MakeBindingsFromVariableNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_make_bindings_from_variable_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LocalizeFunctions"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_localize_functions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LocalizeFunctionFromPath"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_localize_function_from_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LocalizeFunction"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_localize_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("JoinFunctionVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_join_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsTransacting"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_is_transacting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsReportingEnabled"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_is_reporting_enabled,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsFunctionPublic"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_is_function_public,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("InsertArrayPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_insert_array_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportNodesFromText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_import_nodes_from_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetUnitStructsForTemplate"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_get_unit_structs_for_template,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTopLevelGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_top_level_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetTemplateForUnitStruct"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_template_for_unit_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSchema"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_schema,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRegisteredUnitStructs"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_registered_unit_structs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRegisteredTemplates"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_registered_templates,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPinDefaultValue"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_pin_default_value,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControllerForGraph"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_controller_for_graph,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActionStack"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_get_action_stack,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GeneratePythonCommands"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_generate_python_commands,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindVariantsOfFunction"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_find_variants_of_function,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindGraphFunctionIdentifier"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_identifier,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindGraphFunctionHeaderByName"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_header_by_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindGraphFunctionHeader"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_find_graph_function_header,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportSelectedNodesToText"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_export_selected_nodes_to_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportNodeToText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_export_node_to_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportNodesToText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_export_nodes_to_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExpandLibraryNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_expand_library_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EnableReporting"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_enable_reporting,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("EjectNodeFromPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_eject_node_from_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DuplicateArrayPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_duplicate_array_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateFunctionVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_create_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CollapseNodes"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_collapse_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CloseUndoBracket"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_close_undo_bracket,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearPinCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_clear_pin_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearOverrideOnPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_clear_override_on_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearOverrideOnPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_clear_override_on_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearOverrideOnAllPinsOnNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_all_pins_on_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearOverrideOnAllPinsOnNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_all_pins_on_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearNodeSelection"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_clear_node_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearNodeLayout"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_clear_node_layout,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ClearArrayPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_clear_array_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ChangeExposedPinType"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_change_exposed_pin_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CanImportNodesFromText"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_can_import_nodes_from_text,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CancelUndoBracket"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_cancel_undo_bracket,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BreakLink"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_break_link,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BreakAllLinks"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_break_all_links,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BindPinToVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_bind_pin_to_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddVariableNodeFromObjectPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_variable_node_from_object_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddVariableNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_variable_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddUnitNodeWithDefaults"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_unit_node_with_defaults,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddUnitNodeFromStructPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_unit_node_from_struct_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddUnitNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_unit_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTrait"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_trait,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTemplateNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_template_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddTagToFunctionVariant"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_tag_to_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSelectNodeFromStruct"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_select_node_from_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSelectNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_select_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddRerouteNodeOnPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_reroute_node_on_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddRerouteNodeOnLinkPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_reroute_node_on_link_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddRerouteNodeOnLink"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_reroute_node_on_link,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddParameterNodeFromObjectPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_parameter_node_from_object_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddParameterNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_parameter_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOverrideToPins"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_override_to_pins,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOverrideToPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_override_to_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOverrideToAllPinsOnNodes"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_all_pins_on_nodes,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddOverrideToAllPinsOnNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_all_pins_on_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLocalVariableFromObjectPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_local_variable_from_object_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLocalVariable"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_local_variable,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddLink"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_link,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddInvokeEntryNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_invoke_entry_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddInjectedNodeFromStructPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_injected_node_from_struct_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddInjectedNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_injected_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddIfNodeFromStruct"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_if_node_from_struct,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddIfNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_if_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddFunctionToLibrary"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_function_to_library,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddFunctionReferenceNodeFromDescription"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_function_reference_node_from_description,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddFunctionReferenceNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_function_reference_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddFreeRerouteNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_free_reroute_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddExternalFunctionReferenceNode"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_external_function_reference_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddExposedPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_exposed_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddEnumNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_enum_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddEmptyPinCategory"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_empty_pin_category,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddDefaultTagToFunctionVariant"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_default_tag_to_function_variant,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddCommentNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_comment_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddBranchNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_branch_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddArrayPin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_array_pin,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddArrayNodeFromObjectPath"),
                &raw mut __FUNCTION_PTRS
                    .u_rig_vm_controller_add_array_node_from_object_path,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddArrayNode"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_array_node,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddAggregatePin"),
                &raw mut __FUNCTION_PTRS.u_rig_vm_controller_add_aggregate_pin,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FRigVMGraphVariableDescription {
    pub name: FName,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 16],
    pub default_value: FString,
    pub category: FText,
    pub tooltip: FText,
    pub b_exposed_on_spawn: bool,
    pub b_expose_to_cinematics: bool,
    pub b_public: bool,
    pub b_private: bool,
}
impl FRigVMGraphVariableDescription {}
#[repr(C, align(8))]
pub struct FRigVMParserASTSettings {
    #[doc(hidden)]
    pub(crate) __padding_3: [u8; 3],
    pub b_setup_traits: bool,
    pub(crate) __padding_end: [u8; 52],
}
impl FRigVMParserASTSettings {}
#[repr(C, align(8))]
pub struct FRigVMCompileSettings {
    pub surpress_info_messages: bool,
    pub surpress_warnings: bool,
    pub surpress_errors: bool,
    pub enable_pin_watches: bool,
    pub ast_settings: FRigVMParserASTSettings,
    #[doc(hidden)]
    pub(crate) __padding_66: [u8; 2],
    pub b_warn_about_deprecated_nodes: bool,
    pub b_warn_about_duplicate_events: bool,
}
impl FRigVMCompileSettings {}
#[repr(C, align(8))]
pub struct FRigVMGraphParameterDescription {
    pub name: FName,
    pub b_is_input: bool,
    pub cpp_type: FString,
    pub cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
    pub default_value: FString,
}
impl FRigVMGraphParameterDescription {}
#[repr(C, align(8))]
pub struct FRigVMFunctionReferenceArray {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigVMFunctionReferenceArray {}
#[repr(C, align(8))]
pub struct FRigStructScope {
    pub(crate) __padding_end: [u8; 16],
}
impl FRigStructScope {}
#[repr(C, align(8))]
pub struct URigVMEdGraph {
    __padding_end: [u8; 456],
}
impl URigVMEdGraph {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraph")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraph")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMEdGraphNode {
    __padding_end: [u8; 1152],
}
impl URigVMEdGraphNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMEdGraphSchema {
    __padding_end: [u8; 168],
}
impl URigVMEdGraphSchema {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphSchema")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEdGraphSchema")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct IRigVMAssetInterface {}
#[repr(C, align(8))]
pub struct URigVMAssetInterface {
    __padding_end: [u8; 48],
}
impl URigVMAssetInterface {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMAssetInterface")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMAssetInterface")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMBlueprint {
    #[doc(hidden)]
    pub(crate) __padding_2992: [u8; 2992],
    pub vm_compile_settings: FRigVMCompileSettings,
    __padding_end: [u8; 744],
}
impl URigVMBlueprint {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBlueprint")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBlueprint")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn suspend_notifications(&mut self, b_suspend_notifs: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_suspend_notifications,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_suspend_notifs,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_suspend_notifications,
                __buffer,
            )
        };
    }
    pub fn split_asset_variant(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_split_asset_variant,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_split_asset_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn set_auto_vm_recompile(&mut self, b_auto_recompile: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_set_auto_vm_recompile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_recompile,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_set_auto_vm_recompile,
                __buffer,
            )
        };
    }
    pub fn request_rig_vm_init(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_request_rig_vm_init,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_request_rig_vm_init,
                __buffer,
            )
        };
    }
    pub fn request_auto_vm_recompilation(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_request_auto_vm_recompilation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_request_auto_vm_recompilation,
                __buffer,
            )
        };
    }
    pub fn rename_member_variable(
        &mut self,
        in_old_name: &FName,
        in_new_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_rename_member_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_name,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_rename_member_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_model(
        &mut self,
        in_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_remove_model,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_remove_model,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_member_variable(&mut self, in_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_remove_member_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_remove_member_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn recompile_vm_if_required(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_recompile_vm_if_required,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_recompile_vm_if_required,
                __buffer,
            )
        };
    }
    pub fn recompile_vm(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_recompile_vm,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_recompile_vm,
                __buffer,
            )
        };
    }
    pub fn join_asset_variant(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_join_asset_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_join_asset_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_rig_vm_host_class(
        &self,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_rig_vm_host_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_rig_vm_host_class,
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
    pub fn get_or_create_local_function_library(
        &mut self,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_or_create_local_function_library,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_or_create_local_function_library,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_or_create_controller(
        &mut self,
        in_graph: UPtr<URigVMGraph>,
    ) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_or_create_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_or_create_controller,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_model(
        &self,
        in_ed_graph: UPtr<crate::bindings::engine::UEdGraph>,
    ) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_model,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_ed_graph,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UEdGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_model,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_member_variables(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_member_variables,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_member_variables,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_matching_variants(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_matching_variants,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_matching_variants,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn get_local_function_library(&self) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_local_function_library,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_local_function_library,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_focused_model(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_focused_model,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_focused_model,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_default_model(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_default_model,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_default_model,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_debugged_rig_vm_host(
        &mut self,
    ) -> UPtr<crate::bindings::rig_vm::URigVMHost> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_debugged_rig_vm_host,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_debugged_rig_vm_host,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::rig_vm::URigVMHost>>().read()
        }
    }
    pub fn get_controller_by_name(
        &self,
        in_graph_name: FString,
    ) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_controller_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_controller_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_controller(&self, in_graph: UPtr<URigVMGraph>) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_controller,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_available_rig_vm_structs(
        &self,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UStruct>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_available_rig_vm_structs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_available_rig_vm_structs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UStruct>>>()
                .read()
        }
    }
    pub fn get_auto_vm_recompile(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_auto_vm_recompile,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_auto_vm_recompile,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_asset_variant_ref(&self) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_asset_variant_ref,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_asset_variant_ref,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_asset_variant(&self) -> crate::bindings::rig_vm::FRigVMVariant {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_asset_variant_bp,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_asset_variant_bp,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariant>().read()
        }
    }
    pub fn get_all_models(&self) -> TArray<UPtr<URigVMGraph>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_all_models,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_get_all_models,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMGraph>>>().read() }
    }
    pub fn generate_python_commands(
        &mut self,
        in_new_blueprint_name: FString,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_generate_python_commands,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_blueprint_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_generate_python_commands,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FString>>().read() }
    }
    pub fn create_rig_vm_host(&mut self) -> UPtr<crate::bindings::rig_vm::URigVMHost> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_create_rig_vm_host,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_create_rig_vm_host,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::rig_vm::URigVMHost>>().read()
        }
    }
    pub fn change_member_variable_type(
        &mut self,
        in_name: &FName,
        in_cpp_type: FString,
        b_is_public: bool,
        b_is_read_only: bool,
        in_default_value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_change_member_variable_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_public,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_change_member_variable_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn bulk_remove_member_variables(&mut self, in_names: &TArray<FName>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_bulk_remove_member_variables,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_names,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_bulk_remove_member_variables,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_model(
        &mut self,
        in_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_add_model,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_add_model,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn add_member_variable(
        &mut self,
        in_name: &FName,
        in_cpp_type: FString,
        b_is_public: bool,
        b_is_read_only: bool,
        in_default_value: FString,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<68>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_add_member_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_public,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_read_only,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_blueprint_add_member_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMCompiler {
    __padding_end: [u8; 312],
}
impl URigVMCompiler {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCompiler")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCompiler")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn compile_vm(
        &mut self,
        in_graphs: TArray<UPtr<URigVMGraph>>,
        in_controller: UPtr<URigVMController>,
        out_vm: UPtr<crate::bindings::rig_vm::URigVM>,
        context: &mut crate::bindings::rig_vm::FRigVMExtendedExecuteContext,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<801>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_compiler_compile_vm,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graphs,
                __buffer.add(0).cast::<TArray<UPtr<URigVMGraph>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller,
                __buffer.add(16).cast::<UPtr<URigVMController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_vm,
                __buffer.add(24).cast::<UPtr<crate::bindings::rig_vm::URigVM>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                context,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::rig_vm::FRigVMExtendedExecuteContext>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_compiler_compile_vm,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::rig_vm::FRigVMExtendedExecuteContext>()
                .swap(context);
        }
        unsafe { __buffer.add(800).cast::<bool>().read() }
    }
    pub fn compile(
        &mut self,
        in_graphs: TArray<UPtr<URigVMGraph>>,
        in_controller: UPtr<URigVMController>,
        out_vm: UPtr<crate::bindings::rig_vm::URigVM>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_compiler_compile,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graphs,
                __buffer.add(0).cast::<TArray<UPtr<URigVMGraph>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_controller,
                __buffer.add(16).cast::<UPtr<URigVMController>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &out_vm,
                __buffer.add(24).cast::<UPtr<crate::bindings::rig_vm::URigVM>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_compiler_compile,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMNode {
    __padding_end: [u8; 544],
}
impl URigVMNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_is_excluded_by_early_exit(&mut self, b_is_excluded: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_is_excluded_by_early_exit,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_excluded,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_is_excluded_by_early_exit,
                __buffer,
            )
        };
    }
    pub fn set_has_early_exit_marker(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_has_early_exit_marker,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_has_early_exit_marker,
                __buffer,
            )
        };
    }
    pub fn set_has_breakpoint(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_has_breakpoint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_has_breakpoint,
                __buffer,
            )
        };
    }
    pub fn set_execution_is_halted_at_this_node(&mut self, b_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_execution_is_halted_at_this_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_value, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_set_execution_is_halted_at_this_node,
                __buffer,
            )
        };
    }
    pub fn is_visible_in_ui(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_visible_in_ui,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_visible_in_ui,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_trait_pin(&self, in_name: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_trait_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_trait_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_selected(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_selected,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_pure(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_node_is_pure,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_node_is_pure,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_pin_category_expanded(&self, in_category: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_pin_category_expanded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_pin_category_expanded,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_mutable(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_mutable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_mutable,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_loop_node(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_loop_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_loop_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_linked_to(&self, in_node: UPtr<URigVMNode>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_linked_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_linked_to,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_input_aggregate(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_input_aggregate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_input_aggregate,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_injected(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_injected,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_injected,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_highlighted(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_highlighted,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_highlighted,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_excluded_by_early_exit(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_excluded_by_early_exit,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_excluded_by_early_exit,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_event(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_event,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_event,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_defined_as_varying(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_defined_as_varying,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_defined_as_varying,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_defined_as_constant(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_defined_as_constant,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_defined_as_constant,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_control_flow_node(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_control_flow_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_control_flow_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_aggregate(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_aggregate,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_is_aggregate,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_pin_of_direction(
        &self,
        in_direction: crate::bindings::rig_vm::ERigVMPinDirection,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_pin_of_direction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_direction,
                __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMPinDirection>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_pin_of_direction,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_output_pin(&self, b_include_io: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_output_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_io,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_output_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_orphaned_pins(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_orphaned_pins,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_orphaned_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_lazy_pin(&self, b_only_consider_pins_with_links: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_lazy_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_consider_pins_with_links,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_lazy_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_io_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_io_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_io_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_input_pin(&self, b_include_io: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_input_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_io,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_input_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn has_early_exit_marker(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_early_exit_marker,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_early_exit_marker,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_breakpoint(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_breakpoint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_has_breakpoint,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_trait_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_trait_pins,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_trait_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_tool_tip_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_tool_tip_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_tool_tip_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_supported_workflows(
        &self,
        in_type: crate::bindings::rig_vm::ERigVMUserWorkflowType,
        in_subject: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<crate::bindings::rig_vm::FRigVMUserWorkflow> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_supported_workflows,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::ERigVMUserWorkflowType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_subject,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_supported_workflows,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMUserWorkflow>>()
                .read()
        }
    }
    pub fn get_sub_pin_categories(
        &self,
        in_category: FString,
        b_only_existing: bool,
        b_recursive: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_sub_pin_categories,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_existing,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_sub_pin_categories,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn get_size(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_size,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_second_aggregate_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_second_aggregate_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_second_aggregate_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_root_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_root_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_root_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_previous_f_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_previous_f_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_previous_f_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_position(&self) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_position,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_pins_for_category(
        &self,
        in_category: FString,
    ) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pins_for_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pins_for_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pins,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_pin_category_name(&self, in_category: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pin_category_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pin_category_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_pin_categories(&self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pin_categories,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_pin_categories,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_parent_pin_category(
        &self,
        in_category: FString,
        b_only_existing: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_parent_pin_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_existing,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_parent_pin_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_parent_pin_categories(
        &self,
        in_category: FString,
        b_only_existing: bool,
        b_include_self: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_parent_pin_categories,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_existing,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_self,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_parent_pin_categories,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn get_orphaned_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_orphaned_pins,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_orphaned_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_opposite_aggregate_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_opposite_aggregate_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_opposite_aggregate_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_node_title(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_title,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_title,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node_sub_title(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_sub_title,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_sub_title,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node_path(&self, b_recursive: bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_node_layout(
        &self,
        b_include_empty_categories: bool,
    ) -> crate::bindings::rig_vm::FRigVMNodeLayout {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_layout,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_empty_categories,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_layout,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::rig_vm::FRigVMNodeLayout>().read()
        }
    }
    pub fn get_node_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_node_color(&self) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_color,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_node_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>().read()
        }
    }
    pub fn get_next_aggregate_name(&self, in_last_aggregate_pin_name: &FName) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_next_aggregate_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_last_aggregate_pin_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_next_aggregate_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<FName>().read() }
    }
    pub fn get_links(&self) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_links,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_links,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_linked_target_nodes(&self) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_linked_target_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_linked_target_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn get_linked_source_nodes(&self) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_linked_source_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_linked_source_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn get_injection_info(&self) -> UPtr<URigVMInjectionInfo> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_injection_info,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_injection_info,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMInjectionInfo>>().read() }
    }
    pub fn get_graph_depth(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_graph_depth,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_graph_depth,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_first_aggregate_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_first_aggregate_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_first_aggregate_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_event_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_event_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_event_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_all_pins_recursively(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_all_pins_recursively,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_all_pins_recursively,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_aggregate_outputs(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_aggregate_outputs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_aggregate_outputs,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_aggregate_inputs(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_aggregate_inputs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_get_aggregate_inputs,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn find_root_pin_by_name(&self, in_pin_name: &FName) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_root_pin_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_root_pin_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_pin(&self, in_pin_path: FString) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_function_for_node(&self) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_function_for_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_function_for_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn find_execute_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_execute_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_find_execute_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn execution_is_halted_at_this_node(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_execution_is_halted_at_this_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_execution_is_halted_at_this_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_only_exist_once(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_can_only_exist_once,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_can_only_exist_once,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_be_upgraded(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_can_be_upgraded,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_node_can_be_upgraded,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMTemplateNode {
    __padding_end: [u8; 632],
}
impl URigVMTemplateNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMTemplateNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMTemplateNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn is_singleton(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_is_singleton,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_is_singleton,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_resolved(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_is_resolved,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_is_resolved,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_fully_unresolved(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_is_fully_unresolved,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_is_fully_unresolved,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_script_struct(
        &self,
    ) -> UPtr<crate::bindings::core_u_object::UScriptStruct> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_get_script_struct,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_get_script_struct,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>()
                .read()
        }
    }
    pub fn get_notation(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_get_notation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_template_node_get_notation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMLibraryNode {
    __padding_end: [u8; 632],
}
impl URigVMLibraryNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMLibraryNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMLibraryNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_matching_variants(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_matching_variants,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_matching_variants,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn get_library(&self) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_library,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_library,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_function_variant_ref(&self) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_function_variant_ref,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_function_variant_ref,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_function_variant(&self) -> crate::bindings::rig_vm::FRigVMVariant {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_function_variant,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_function_variant,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariant>().read()
        }
    }
    pub fn get_contained_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_contained_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_library_node_get_contained_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMCollapseNode {
    __padding_end: [u8; 688],
}
impl URigVMCollapseNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCollapseNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCollapseNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMAggregateNode {
    __padding_end: [u8; 704],
}
impl URigVMAggregateNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMAggregateNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMAggregateNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UDEPRECATED_RigVMArrayNode {
    __padding_end: [u8; 640],
}
impl UDEPRECATED_RigVMArrayNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMArrayNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMArrayNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_op_code(&self) -> crate::bindings::rig_vm::ERigVMOpCode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .udeprecated_rig_vm_array_node_get_op_code,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .udeprecated_rig_vm_array_node_get_op_code,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMOpCode>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .udeprecated_rig_vm_array_node_get_cpp_type_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .udeprecated_rig_vm_array_node_get_cpp_type_object,
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
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .udeprecated_rig_vm_array_node_get_cpp_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .udeprecated_rig_vm_array_node_get_cpp_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_RigVMBranchNode {
    __padding_end: [u8; 544],
}
impl UDEPRECATED_RigVMBranchNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMBranchNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMBranchNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMCommentNode {
    __padding_end: [u8; 568],
}
impl URigVMCommentNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCommentNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMCommentNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_comment_text(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_comment_font_size(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_font_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_font_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_comment_color_bubble(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_color_bubble,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_color_bubble,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_comment_bubble_visible(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_bubble_visible,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_comment_node_get_comment_bubble_visible,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMDispatchNode {
    __padding_end: [u8; 720],
}
impl URigVMDispatchNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMDispatchNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMDispatchNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMEnumNode {
    __padding_end: [u8; 544],
}
impl URigVMEnumNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEnumNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEnumNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_enum(&self) -> UPtr<crate::bindings::core_u_object::UEnum> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_enum_node_get_enum,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_enum_node_get_enum,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UEnum>>().read()
        }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_enum_node_get_cpp_type_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_enum_node_get_cpp_type_object,
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
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_enum_node_get_cpp_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_enum_node_get_cpp_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMFunctionInterfaceNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionInterfaceNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionInterfaceNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionInterfaceNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMFunctionEntryNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionEntryNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionEntryNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionEntryNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMFunctionReferenceNode {
    __padding_end: [u8; 1288],
}
impl URigVMFunctionReferenceNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionReferenceNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionReferenceNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_referenced_function_header(
        &self,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionHeader {
        let mut __stack = crate::core_data::StackAlloc::<528>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_reference_node_get_referenced_function_header_for_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_reference_node_get_referenced_function_header_for_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct URigVMFunctionReturnNode {
    __padding_end: [u8; 632],
}
impl URigVMFunctionReturnNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionReturnNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionReturnNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UDEPRECATED_RigVMIfNode {
    __padding_end: [u8; 632],
}
impl UDEPRECATED_RigVMIfNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMIfNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMIfNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMInvokeEntryNode {
    __padding_end: [u8; 544],
}
impl URigVMInvokeEntryNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMInvokeEntryNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMInvokeEntryNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_entry_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_invoke_entry_node_get_entry_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_invoke_entry_node_get_entry_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMParameterNode {
    __padding_end: [u8; 544],
}
impl URigVMParameterNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMParameterNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMParameterNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn is_input(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_is_input,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_is_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_parameter_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_parameter_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_parameter_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_parameter_description(&self) -> FRigVMGraphParameterDescription {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_parameter_description,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_parameter_description,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FRigVMGraphParameterDescription>().read() }
    }
    pub fn get_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_cpp_type_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_cpp_type_object,
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
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_cpp_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_parameter_node_get_cpp_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMRerouteNode {
    __padding_end: [u8; 544],
}
impl URigVMRerouteNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMRerouteNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMRerouteNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct UDEPRECATED_RigVMSelectNode {
    __padding_end: [u8; 632],
}
impl UDEPRECATED_RigVMSelectNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMSelectNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_RigVMSelectNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMUnitNode {
    __padding_end: [u8; 680],
}
impl URigVMUnitNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMUnitNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMUnitNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_struct_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_unit_node_get_struct_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_unit_node_get_struct_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_method_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_unit_node_get_method_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_unit_node_get_method_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMVariableNode {
    __padding_end: [u8; 544],
}
impl URigVMVariableNode {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMVariableNode")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMVariableNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn is_local_variable(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_local_variable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_local_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_input_argument(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_input_argument,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_input_argument,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_getter(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_getter,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_getter,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_external_variable(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_external_variable,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_is_external_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_variable_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_variable_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_variable_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_variable_description(&self) -> FRigVMGraphVariableDescription {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_variable_description,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_variable_description,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FRigVMGraphVariableDescription>().read() }
    }
    pub fn get_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_cpp_type_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_cpp_type_object,
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
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_cpp_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_variable_node_get_cpp_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMBuildData {
    __padding_end: [u8; 216],
}
impl URigVMBuildData {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBuildData")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMBuildData")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn split_variant_from_set(
        &mut self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_split_variant_from_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_split_variant_from_set,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn join_variant_set(
        &mut self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_join_variant_set,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_join_variant_set,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_variant_ref_for_asset(
        &self,
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_variant_ref_for_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_variant_ref_for_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(152).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn get_used_function_identifiers(
        &self,
        b_only_public: bool,
    ) -> TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_used_function_identifiers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_public,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_used_function_identifiers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>>()
                .read()
        }
    }
    pub fn get_function_identifier_for_variant(
        &self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_function_identifier_for_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_function_identifier_for_variant,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(72)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>()
                .read()
        }
    }
    pub fn get_asset_data_for_variant(
        &self,
        in_variant_ref: &crate::bindings::rig_vm::FRigVMVariantRef,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<224>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_asset_data_for_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_ref,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMVariantRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_asset_data_for_variant,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn get_asset_data_for_path(
        &self,
        in_object_path: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<192>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_asset_data_for_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_object_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_asset_data_for_path,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn get_all_function_identifiers(
        &self,
        b_only_public: bool,
    ) -> TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_all_function_identifiers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_public,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get_all_function_identifiers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>>()
                .read()
        }
    }
    pub fn get() -> UPtr<URigVMBuildData> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMBuildData::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_get,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMBuildData>>().read() }
    }
    pub fn gather_function_variant_refs_for_asset(
        &self,
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_function_variant_refs_for_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_function_variant_refs_for_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(152)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn gather_all_function_variant_refs(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_all_function_variant_refs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_all_function_variant_refs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn gather_all_asset_variant_refs(
        &self,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_all_asset_variant_refs,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_gather_all_asset_variant_refs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn find_function_variant_refs(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_find_function_variant_refs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_find_function_variant_refs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn find_asset_variant_refs(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_find_asset_variant_refs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_find_asset_variant_refs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn create_function_variant(
        &mut self,
        in_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        in_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_create_function_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_identifier,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_name, __buffer.add(96).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_create_function_variant,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(112).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
    pub fn create_asset_variant(
        &mut self,
        in_asset_data: &crate::bindings::core_u_object::FAssetData,
        in_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMVariantRef {
        let mut __stack = crate::core_data::StackAlloc::<240>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_create_asset_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_asset_data,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name,
                __buffer.add(152).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_build_data_create_asset_variant,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(168).cast::<crate::bindings::rig_vm::FRigVMVariantRef>().read()
        }
    }
}
pub struct IRigVMClientHost {}
#[repr(C, align(8))]
pub struct URigVMClientHost {
    __padding_end: [u8; 48],
}
impl URigVMClientHost {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMClientHost")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMClientHost")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct IRigVMEditorSideObject {}
#[repr(C, align(8))]
pub struct URigVMEditorSideObject {
    __padding_end: [u8; 48],
}
impl URigVMEditorSideObject {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorSideObject")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMEditorSideObject")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct IRigVMClientExternalModelHost {}
#[repr(C, align(8))]
pub struct URigVMClientExternalModelHost {
    __padding_end: [u8; 48],
}
impl URigVMClientExternalModelHost {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMClientExternalModelHost")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMClientExternalModelHost")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMActionStack {
    __padding_end: [u8; 152],
}
impl URigVMActionStack {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMActionStack")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMActionStack")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct IRigVMExternalDependencyManager {}
#[repr(C, align(8))]
pub struct URigVMExternalDependencyManager {
    __padding_end: [u8; 48],
}
impl URigVMExternalDependencyManager {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMExternalDependencyManager")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMExternalDependencyManager")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMGraph {
    __padding_end: [u8; 296],
}
impl URigVMGraph {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMGraph")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMGraph")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn set_schema_class(&mut self, in_schema_class: TSubclassOf<URigVMSchema>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_set_schema_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_schema_class,
                __buffer.add(0).cast::<TSubclassOf<URigVMSchema>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_set_schema_class,
                __buffer,
            )
        };
    }
    pub fn set_default_function_library(
        &mut self,
        in_function_library: UPtr<URigVMFunctionLibrary>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_set_default_function_library,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_library,
                __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_set_default_function_library,
                __buffer,
            )
        };
    }
    pub fn is_top_level_graph(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_top_level_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_top_level_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_root_graph(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_root_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_root_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_node_selected(&self, in_node_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_node_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_node_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_node_highlighted(&self, in_node_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_node_highlighted,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_is_node_highlighted,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_variable_descriptions(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_variable_descriptions,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_variable_descriptions,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_select_nodes(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_select_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_select_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_schema_class(&self) -> TSubclassOf<URigVMSchema> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_schema_class,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_schema_class,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TSubclassOf<URigVMSchema>>().read() }
    }
    pub fn get_schema(&self) -> UPtr<URigVMSchema> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_schema,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_schema,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMSchema>>().read() }
    }
    pub fn get_root_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_root_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_root_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_return_node(&self) -> UPtr<URigVMFunctionReturnNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_return_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_return_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionReturnNode>>().read() }
    }
    pub fn get_parent_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_parent_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_parent_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_output_arguments(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_output_arguments,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_output_arguments,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_nodes(&self) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_nodes,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn get_node_path(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_node_path,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_node_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_local_variables(
        &self,
        b_include_input_arguments: bool,
    ) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_local_variables,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_input_arguments,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_local_variables,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_links(&self) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_links,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_links,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_input_arguments(&self) -> TArray<FRigVMGraphVariableDescription> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_input_arguments,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_input_arguments,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FRigVMGraphVariableDescription>>().read()
        }
    }
    pub fn get_graph_name(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_graph_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_graph_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_graph_depth(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_graph_depth,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_graph_depth,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_event_names(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_event_names,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_event_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_entry_node(&self) -> UPtr<URigVMFunctionEntryNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_entry_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_entry_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionEntryNode>>().read() }
    }
    pub fn get_default_function_library(&self) -> UPtr<URigVMFunctionLibrary> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_default_function_library,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_default_function_library,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMFunctionLibrary>>().read() }
    }
    pub fn get_contained_graphs(&self, b_recursive: bool) -> TArray<UPtr<URigVMGraph>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_contained_graphs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_get_contained_graphs,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMGraph>>>().read() }
    }
    pub fn find_pin(&self, in_pin_path: FString) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_node_by_name(&self, in_node_name: &FName) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_node_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_node_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn find_node(&self, in_node_path: FString) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn find_link(
        &self,
        in_link_pin_path_representation: FString,
    ) -> UPtr<URigVMLink> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_link_pin_path_representation,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_find_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMLink>>().read() }
    }
    pub fn contains_link(&self, in_pin_path_representation: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_contains_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path_representation,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_graph_contains_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMFunctionLibrary {
    __padding_end: [u8; 576],
}
impl URigVMFunctionLibrary {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionLibrary")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMFunctionLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_references_for_function(
        &mut self,
        in_function_name: &FName,
    ) -> TArray<TSoftObjectPtr<URigVMFunctionReferenceNode>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_get_references_for_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_get_references_for_function,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<TSoftObjectPtr<URigVMFunctionReferenceNode>>>()
                .read()
        }
    }
    pub fn get_reference_paths_for_function(
        &mut self,
        in_function_name: &FName,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_get_reference_paths_for_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_get_reference_paths_for_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<FString>>().read() }
    }
    pub fn get_functions(&self) -> TArray<UPtr<URigVMLibraryNode>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_get_functions,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_get_functions,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLibraryNode>>>().read() }
    }
    pub fn find_function_for_node(
        &self,
        in_node: UPtr<URigVMNode>,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_find_function_for_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_find_function_for_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn find_function(&self, in_function_name: &FName) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_find_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_function_library_find_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMLink {
    __padding_end: [u8; 96],
}
impl URigVMLink {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMLink")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMLink")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_target_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_target_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_target_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_target_node(&self) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_target_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_target_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn get_source_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_source_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_source_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_source_node(&self) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_source_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_source_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn get_pin_path_representation(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_pin_path_representation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_pin_path_representation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_opposite_pin(&self, in_pin: UPtr<URigVMPin>) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_opposite_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_opposite_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_link_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_link_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_link_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph_depth(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_graph_depth,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_graph_depth,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_link_get_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMInjectionInfo {
    __padding_end: [u8; 88],
}
impl URigVMInjectionInfo {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMInjectionInfo")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMInjectionInfo")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn get_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_injection_info_get_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_injection_info_get_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_injection_info_get_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_injection_info_get_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMPin {
    __padding_end: [u8; 544],
}
impl URigVMPin {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMPin")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("URigVMPin").copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn should_only_show_sub_pins(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_should_only_show_sub_pins,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_should_only_show_sub_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn should_hide_sub_pins(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_should_hide_sub_pins,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_should_hide_sub_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn requires_watch(&self, b_check_exposed_pin_chain: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_requires_watch,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_check_exposed_pin_chain,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_requires_watch,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn is_wild_card(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_wild_card,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_wild_card,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_valid_default_value(&self, in_default_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_valid_default_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_valid_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_u_object(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_u_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_u_object,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_trait_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_trait_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_trait_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_struct_member(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_struct_member,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_struct_member,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_struct(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_struct,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_struct,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_string_type(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_string_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_string_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_root_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_root_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_root_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_reference_counted_container(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_reference_counted_container,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_reference_counted_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_linked_to(&self, in_pin: UPtr<URigVMPin>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_linked_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_linked_to,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_lazy(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_is_lazy,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_is_lazy,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_interface(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_interface,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_interface,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_fixed_size_array(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_fixed_size_array,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_fixed_size_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_expanded(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_expanded,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_expanded,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_execute_context(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_execute_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_execute_context,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_enum(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_is_enum,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_is_enum,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_dynamic_array(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_dynamic_array,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_dynamic_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_defined_as_constant(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_defined_as_constant,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_defined_as_constant,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_array_element(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_array_element,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_is_array_element,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_array(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_is_array,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_is_array,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_user_provided_default_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_user_provided_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_user_provided_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_original_default_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_original_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_original_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn has_meta_data(&self, in_key: FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_meta_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn has_default_value_override(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_default_value_override,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_has_default_value_override,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_tool_tip_text(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_tool_tip_text,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_tool_tip_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_target_links(&self, b_recursive: bool) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_target_links,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_target_links,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_sub_pins(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_sub_pins,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_sub_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_sub_pin_path(
        &self,
        in_parent_pin: UPtr<URigVMPin>,
        b_include_parent_pin_name: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_sub_pin_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_parent_pin_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_sub_pin_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_source_links(&self, b_recursive: bool) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_source_links,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_source_links,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_segment_path(&self, b_include_root_pin: bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_segment_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_root_pin,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_segment_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_script_struct(
        &self,
    ) -> UPtr<crate::bindings::core_u_object::UScriptStruct> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_script_struct,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_script_struct,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>()
                .read()
        }
    }
    pub fn get_root_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_root_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_root_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_pin_path(&self, b_use_node_path: bool) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_pin_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_node_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_pin_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_pin_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_pin_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_pin_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_pin_for_link(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_pin_for_link,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_pin_for_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_parent_script_struct(
        &self,
        fallback_node: UPtr<URigVMUnitNode>,
    ) -> UPtr<crate::bindings::core_u_object::UScriptStruct> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_parent_script_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fallback_node,
                __buffer.add(0).cast::<UPtr<URigVMUnitNode>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_parent_script_struct,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>()
                .read()
        }
    }
    pub fn get_parent_pin(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_parent_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_parent_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_original_pin_from_injected_node(&self) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_original_pin_from_injected_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_original_pin_from_injected_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn get_original_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_original_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_original_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_node(&self) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_get_node,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_get_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn get_meta_data(&self, in_key: FName) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_key, __buffer.add(0).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_meta_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_links(&self) -> TArray<UPtr<URigVMLink>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_links,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_links,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMLink>>>().read() }
    }
    pub fn get_linked_target_pins(&self, b_recursive: bool) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_linked_target_pins,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_linked_target_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_linked_source_pins(&self, b_recursive: bool) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_linked_source_pins,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_linked_source_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_index_in_category(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_index_in_category,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_index_in_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_enum(&self) -> UPtr<crate::bindings::core_u_object::UEnum> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_get_enum,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS.u_rig_vm_pin_get_enum,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UEnum>>().read()
        }
    }
    pub fn get_display_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_display_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_direction(&self) -> crate::bindings::rig_vm::ERigVMPinDirection {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_direction,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_direction,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMPinDirection>().read()
        }
    }
    pub fn get_default_value_type(&self) -> ERigVMPinDefaultValueType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_default_value_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_default_value_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ERigVMPinDefaultValueType>().read() }
    }
    pub fn get_default_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_custom_widget_name(&self) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_custom_widget_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_custom_widget_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FName>().read() }
    }
    pub fn get_cpp_type_object(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_cpp_type_object,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_cpp_type_object,
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
    pub fn get_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_cpp_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_cpp_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_category(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_category,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_array_size(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_array_size,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_array_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_array_element_cpp_type(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_array_element_cpp_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_array_element_cpp_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_all_sub_pins_recursively(&self) -> TArray<UPtr<URigVMPin>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_all_sub_pins_recursively,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_all_sub_pins_recursively,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<URigVMPin>>>().read() }
    }
    pub fn get_absolute_pin_index(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_absolute_pin_index,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_get_absolute_pin_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn find_sub_pin(&self, in_pin_path: FString) -> UPtr<URigVMPin> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_find_sub_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_find_sub_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<URigVMPin>>().read() }
    }
    pub fn find_link_for_pin(&self, in_other_pin: UPtr<URigVMPin>) -> UPtr<URigVMLink> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_find_link_for_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_other_pin,
                __buffer.add(0).cast::<UPtr<URigVMPin>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_find_link_for_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMLink>>().read() }
    }
    pub fn contains_wild_card_sub_pin(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_contains_wild_card_sub_pin,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_contains_wild_card_sub_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_provide_default_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_can_provide_default_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_pin_can_provide_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMSchema {
    __padding_end: [u8; 80],
}
impl URigVMSchema {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMSchema")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMSchema")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct URigVMUserWorkflowRegistry {
    __padding_end: [u8; 72],
}
impl URigVMUserWorkflowRegistry {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMUserWorkflowRegistry")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMUserWorkflowRegistry")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn unregister_provider(&mut self, in_handle: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_unregister_provider,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&in_handle, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_unregister_provider,
                __buffer,
            )
        };
    }
    pub fn register_provider(
        &mut self,
        in_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_provider: FRegisterProvider_InProvider,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_register_provider,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_provider,
                __buffer.add(8).cast::<FRegisterProvider_InProvider>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_register_provider,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<i32>().read() }
    }
    pub fn get_workflows(
        &self,
        in_type: crate::bindings::rig_vm::ERigVMUserWorkflowType,
        in_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_subject: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<crate::bindings::rig_vm::FRigVMUserWorkflow> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_get_workflows,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::ERigVMUserWorkflowType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_struct,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_subject,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_get_workflows,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMUserWorkflow>>()
                .read()
        }
    }
    pub fn get() -> UPtr<URigVMUserWorkflowRegistry> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_get,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMUserWorkflowRegistry::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_user_workflow_registry_get,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMUserWorkflowRegistry>>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMController {
    #[doc(hidden)]
    pub(crate) __padding_296: [u8; 296],
    pub modified_event_dynamic: FRigVMController_ModifiedEventDynamic,
    __padding_end: [u8; 408],
}
impl URigVMController {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMController")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMController")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings()
                .core_fns
                .get_cdo_from_class)(class.ptr.cast(), &raw mut cdo);
            cdo
        }
    }
    pub fn upgrade_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_recursive: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_upgrade_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_upgrade_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn unresolve_template_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_unresolve_template_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_unresolve_template_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn undo(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_undo,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_undo,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn unbind_pin_from_variable(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_unbind_pin_from_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_unbind_pin_from_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn swap_function_reference_by_name(
        &mut self,
        in_function_reference_node_name: &FName,
        in_new_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_setup_orphan_pins: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<116>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_swap_function_reference_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_reference_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_identifier,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(113).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(114).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_swap_function_reference_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(115).cast::<bool>().read() }
    }
    pub fn swap_function_reference(
        &mut self,
        in_function_reference_node: UPtr<URigVMFunctionReferenceNode>,
        in_new_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_setup_orphan_pins: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<108>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_swap_function_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_reference_node,
                __buffer.add(0).cast::<UPtr<URigVMFunctionReferenceNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_identifier,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(106).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_swap_function_reference,
                __buffer,
            )
        };
        unsafe { __buffer.add(107).cast::<bool>().read() }
    }
    pub fn swap_all_function_references(
        &mut self,
        in_old_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        in_new_function_identifier: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_setup_orphan_pins: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<196>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_swap_all_function_references,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_function_identifier,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_identifier,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(192).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(193).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(194).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_swap_all_function_references,
                __buffer,
            )
        };
        unsafe { __buffer.add(195).cast::<bool>().read() }
    }
    pub fn split_function_variant(
        &mut self,
        in_function_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_split_function_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_split_function_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn set_unit_node_defaults(
        &mut self,
        in_node: UPtr<URigVMUnitNode>,
        in_defaults: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_unit_node_defaults,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMUnitNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_defaults,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_unit_node_defaults,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn set_schema_class(&mut self, in_schema_class: TSubclassOf<URigVMSchema>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_schema_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_schema_class,
                __buffer.add(0).cast::<TSubclassOf<URigVMSchema>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_schema_class,
                __buffer,
            )
        };
    }
    pub fn set_schema(&mut self, in_schema: UPtr<URigVMSchema>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_schema,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_schema,
                __buffer.add(0).cast::<UPtr<URigVMSchema>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_schema,
                __buffer,
            )
        };
    }
    pub fn set_remapped_variable(
        &mut self,
        in_function_ref_node: UPtr<URigVMFunctionReferenceNode>,
        in_inner_variable_name: &FName,
        in_outer_variable_name: &FName,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_remapped_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_ref_node,
                __buffer.add(0).cast::<UPtr<URigVMFunctionReferenceNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_inner_variable_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_outer_variable_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_remapped_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn set_pin_is_watched(
        &mut self,
        in_pin_path: FString,
        b_is_watched: bool,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_is_watched,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_watched,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_is_watched,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_pin_index_in_category(
        &mut self,
        in_pin_path: FString,
        in_index_in_category: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<23>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_index_in_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_index_in_category,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_index_in_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(22).cast::<bool>().read() }
    }
    pub fn set_pin_expansion(
        &mut self,
        in_pin_path: FString,
        b_is_expanded: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_expansion,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_expanded,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_expansion,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn set_pin_display_name(
        &mut self,
        in_pin_path: FString,
        in_display_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_display_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_pin_default_value(
        &mut self,
        in_pin_path: FString,
        in_default_value: FString,
        b_resize_arrays: bool,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
        b_set_value_on_linked_pins: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_default_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_resize_arrays,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(33).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(35).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_value_on_linked_pins,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(37).cast::<bool>().read() }
    }
    pub fn set_pin_category_index(
        &mut self,
        in_node_name: &FName,
        in_pin_category: FString,
        in_new_index: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<39>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_category_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_index,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_category_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(38).cast::<bool>().read() }
    }
    pub fn set_pin_category_expansion(
        &mut self,
        in_node_name: &FName,
        in_pin_category: FString,
        b_is_expanded: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_category_expansion,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_expanded,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_category_expansion,
                __buffer,
            )
        };
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn set_pin_category(
        &mut self,
        in_pin_path: FString,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_pin_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_title_by_name(
        &mut self,
        in_node_name: &FName,
        in_node_title: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_title_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_title,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_title_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_title(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_node_title: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_title,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_title,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_title,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_size_by_name(
        &mut self,
        in_node_name: &FName,
        in_size: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_size_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_size,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_size_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn set_node_size(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_size: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_size,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_selection(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_node_position_by_name(
        &mut self,
        in_node_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_position_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_position_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(35).cast::<bool>().read() }
    }
    pub fn set_node_position(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_position,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_layout(
        &mut self,
        in_node_name: &FName,
        in_layout: crate::bindings::rig_vm::FRigVMNodeLayout,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<195>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_layout,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_layout,
                __buffer.add(16).cast::<crate::bindings::rig_vm::FRigVMNodeLayout>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(192).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(193).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_layout,
                __buffer,
            )
        };
        unsafe { __buffer.add(194).cast::<bool>().read() }
    }
    pub fn set_node_keywords_by_name(
        &mut self,
        in_node_name: &FName,
        in_keywords: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_keywords_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keywords,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_keywords_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_keywords(
        &mut self,
        in_node: UPtr<URigVMCollapseNode>,
        in_keywords: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_keywords,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMCollapseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_keywords,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_keywords,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_description_by_name(
        &mut self,
        in_node_name: &FName,
        in_description: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_description_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_description_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_description(
        &mut self,
        in_node: UPtr<URigVMCollapseNode>,
        in_description: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_description,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMCollapseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_description,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_description,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_color_by_name(
        &mut self,
        in_node_name: &FName,
        in_color: &crate::bindings::core_u_object::FLinearColor,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<31>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_color_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_color_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(30).cast::<bool>().read() }
    }
    pub fn set_node_color(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_color: &crate::bindings::core_u_object::FLinearColor,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_color,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_node_category_by_name(
        &mut self,
        in_node_name: &FName,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_category_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_category_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_node_category(
        &mut self,
        in_node: UPtr<URigVMCollapseNode>,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_merge_undo_action: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMCollapseNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_merge_undo_action,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(26).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_node_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(27).cast::<bool>().read() }
    }
    pub fn set_local_variable_type_from_object_path(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<51>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_type_from_object_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_type_from_object_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(50).cast::<bool>().read() }
    }
    pub fn set_local_variable_type(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(42).cast::<bool>().read() }
    }
    pub fn set_local_variable_default_value(
        &mut self,
        in_variable_name: &FName,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_default_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_local_variable_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn set_is_running_unit_test(&mut self, b_is_running: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_is_running_unit_test,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_running,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_is_running_unit_test,
                __buffer,
            )
        };
    }
    pub fn set_graph(&mut self, in_graph: UPtr<URigVMGraph>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_graph,
                __buffer,
            )
        };
    }
    pub fn set_exposed_pin_index(
        &mut self,
        in_pin_name: &FName,
        in_new_index: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_exposed_pin_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_index,
                __buffer.add(12).cast::<i32>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_exposed_pin_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn set_comment_text_by_name(
        &mut self,
        in_node_name: &FName,
        in_comment_text: FString,
        in_comment_font_size: &i32,
        b_in_comment_bubble_visible: &bool,
        b_in_comment_color_bubble: &bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_comment_text_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comment_text,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_comment_font_size,
                __buffer.add(32).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_bubble_visible,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_color_bubble,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(38).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(39).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_comment_text_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn set_comment_text(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_comment_text: FString,
        in_comment_font_size: &i32,
        b_in_comment_bubble_visible: &bool,
        b_in_comment_color_bubble: &bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_comment_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comment_text,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_comment_font_size,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_bubble_visible,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_in_comment_color_bubble,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(30).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_comment_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_array_pin_size(
        &mut self,
        in_array_pin_path: FString,
        in_size: i32,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_array_pin_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_size, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_array_pin_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(42).cast::<bool>().read() }
    }
    pub fn set_action_stack(&mut self, in_action_stack: UPtr<URigVMActionStack>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_action_stack,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_action_stack,
                __buffer.add(0).cast::<UPtr<URigVMActionStack>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_set_action_stack,
                __buffer,
            )
        };
    }
    pub fn select_node_islands(
        &mut self,
        in_node_names: &TArray<FName>,
        b_clear_selection: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_node_islands,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_node_islands,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn select_node_by_name(
        &mut self,
        in_node_name: &FName,
        b_select: bool,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_node_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(12).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_node_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn select_node(
        &mut self,
        in_node: UPtr<URigVMNode>,
        b_select: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(11).cast::<bool>().read() }
    }
    pub fn select_linked_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        b_select_source_nodes: bool,
        b_clear_selection: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_linked_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_source_nodes,
                __buffer.add(16).cast::<bool>(),
                1,
            );
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
                &b_setup_undo_redo,
                __buffer.add(18).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_select_linked_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn resolve_wild_card_pin(
        &mut self,
        in_pin_path: FString,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<47>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_resolve_wild_card_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_resolve_wild_card_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(46).cast::<bool>().read() }
    }
    pub fn reset_pin_default_value(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_pin_default_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_pin_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn reset_default_value_for_pins(
        &mut self,
        in_pin_paths: &TArray<FString>,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_pins,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn reset_default_value_for_all_pins_on_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_all_pins_on_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_all_pins_on_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn reset_default_value_for_all_pins_on_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_all_pins_on_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_reset_default_value_for_all_pins_on_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn replace_parameter_node_with_variable(
        &mut self,
        in_node_name: &FName,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMVariableNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_replace_parameter_node_with_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                &in_cpp_type,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_replace_parameter_node_with_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMVariableNode>>().read() }
    }
    pub fn rename_variable(
        &mut self,
        in_old_name: &FName,
        in_new_name: &FName,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_name,
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
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn rename_pin_category(
        &mut self,
        in_node_name: &FName,
        in_old_pin_category: FString,
        in_new_pin_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<51>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_pin_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_old_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_pin_category,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_pin_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(50).cast::<bool>().read() }
    }
    pub fn rename_parameter(
        &mut self,
        in_old_name: &FName,
        in_new_name: &FName,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_name,
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
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_parameter,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn rename_node(
        &mut self,
        in_node: UPtr<URigVMNode>,
        in_new_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<23>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(22).cast::<bool>().read() }
    }
    pub fn rename_local_variable(
        &mut self,
        in_variable_name: &FName,
        in_new_variable_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_local_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_variable_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_local_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn rename_function(
        &mut self,
        in_old_function_name: &FName,
        in_new_function_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_function_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn rename_exposed_pin(
        &mut self,
        in_old_pin_name: &FName,
        in_new_pin_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_exposed_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_pin_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_rename_exposed_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn remove_trait(
        &mut self,
        in_node_name: &FName,
        in_trait_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_trait,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trait_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_trait,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn remove_tag_from_function_variant(
        &mut self,
        in_function_name: &FName,
        in_tag_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_tag_from_function_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_tag_from_function_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn remove_pin_category(
        &mut self,
        in_node_name: &FName,
        in_pin_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_pin_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_pin_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn remove_nodes_by_name(
        &mut self,
        in_node_names: &TArray<FName>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_nodes_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_nodes_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_nodes(
        &mut self,
        in_nodes: TArray<UPtr<URigVMNode>>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_nodes,
                __buffer.add(0).cast::<TArray<UPtr<URigVMNode>>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_node_by_name(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_node_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_node_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_node(
        &mut self,
        in_node: UPtr<URigVMNode>,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(10).cast::<bool>().read() }
    }
    pub fn remove_local_variable(
        &mut self,
        in_variable_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_local_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_local_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_injected_node(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_injected_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_injected_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn remove_function_from_library(
        &mut self,
        in_function_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_function_from_library,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_function_from_library,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_exposed_pin(
        &mut self,
        in_pin_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_exposed_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_exposed_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn remove_array_pin(
        &mut self,
        in_array_element_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_array_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_element_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_array_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn remove_aggregate_pin(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_aggregate_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_remove_aggregate_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn refresh_variable_node(
        &mut self,
        in_node_name: &FName,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_setup_undo_redo: bool,
        b_setup_orphan_pins: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_refresh_variable_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
                &in_cpp_type,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(40).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_refresh_variable_node,
                __buffer,
            )
        };
    }
    pub fn redo(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_redo,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_redo,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn push_graph(
        &mut self,
        in_graph: UPtr<URigVMGraph>,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_push_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_push_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn promote_pin_to_variable(
        &mut self,
        in_pin_path: FString,
        b_create_variable_node: bool,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_promote_pin_to_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_variable_node,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_promote_pin_to_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(42).cast::<bool>().read() }
    }
    pub fn promote_function_reference_node_to_collapse_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_remove_function_definition: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_promote_function_reference_node_to_collapse_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_function_definition,
                __buffer.add(14).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_promote_function_reference_node_to_collapse_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FName>().read() }
    }
    pub fn promote_collapse_node_to_function_reference_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        in_existing_function_definition_path: FString,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_promote_collapse_node_to_function_reference_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_existing_function_definition_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_promote_collapse_node_to_function_reference_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FName>().read() }
    }
    pub fn pop_graph(&mut self, b_setup_undo_redo: bool) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_pop_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_pop_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn perform_user_workflow(
        &mut self,
        in_workflow: &crate::bindings::rig_vm::FRigVMUserWorkflow,
        in_options: UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions>,
        b_setup_undo_redo: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<122>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_perform_user_workflow,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_workflow,
                __buffer.add(0).cast::<crate::bindings::rig_vm::FRigVMUserWorkflow>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_options,
                __buffer
                    .add(112)
                    .cast::<UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_perform_user_workflow,
                __buffer,
            )
        };
        unsafe { __buffer.add(121).cast::<bool>().read() }
    }
    pub fn open_undo_bracket(&mut self, in_title: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_open_undo_bracket,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_title,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_open_undo_bracket,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn mark_function_as_public(
        &mut self,
        in_function_name: &FName,
        b_in_is_public: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_mark_function_as_public,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_public,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(13).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(14).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_mark_function_as_public,
                __buffer,
            )
        };
        unsafe { __buffer.add(15).cast::<bool>().read() }
    }
    pub fn make_variable_node_from_binding(
        &mut self,
        in_pin_path: FString,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_make_variable_node_from_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_make_variable_node_from_binding,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn make_options_for_workflow(
        &mut self,
        in_subject: UPtr<crate::bindings::core_u_object::UObject>,
        in_workflow: &crate::bindings::rig_vm::FRigVMUserWorkflow,
    ) -> UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_make_options_for_workflow,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_subject,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_workflow,
                __buffer.add(8).cast::<crate::bindings::rig_vm::FRigVMUserWorkflow>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_make_options_for_workflow,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(120)
                .cast::<UPtr<crate::bindings::rig_vm::URigVMUserWorkflowOptions>>()
                .read()
        }
    }
    pub fn make_bindings_from_variable_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_make_bindings_from_variable_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_make_bindings_from_variable_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn localize_functions(
        &mut self,
        in_function_definitions: TArray<
            crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        >,
        b_localize_dependent_private_functions: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> TMap<
        crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        UPtr<URigVMLibraryNode>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_localize_functions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_definitions,
                __buffer
                    .add(0)
                    .cast::<
                        TArray<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_localize_dependent_private_functions,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_localize_functions,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    TMap<
                        crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
                        UPtr<URigVMLibraryNode>,
                    >,
                >()
                .read()
        }
    }
    pub fn localize_function_from_path(
        &mut self,
        in_host_path: FString,
        in_function_name: &FName,
        b_localize_dependent_private_functions: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_localize_function_from_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_localize_dependent_private_functions,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_localize_function_from_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn localize_function(
        &mut self,
        in_function_definition: &crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
        b_localize_dependent_private_functions: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_localize_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_definition,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_localize_dependent_private_functions,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_localize_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn join_function_variant(
        &mut self,
        in_function_name: &FName,
        in_guid: &crate::bindings::core_u_object::FGuid,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<31>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_join_function_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_guid,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_join_function_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(30).cast::<bool>().read() }
    }
    pub fn is_transacting(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_is_transacting,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_is_transacting,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_reporting_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_is_reporting_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_is_reporting_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_function_public(&mut self, in_function_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_is_function_public,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_is_function_public,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn insert_array_pin(
        &mut self,
        in_array_pin_path: FString,
        in_index: i32,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_insert_array_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_index, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_insert_array_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<FString>().read() }
    }
    pub fn import_nodes_from_text(
        &mut self,
        in_text: FString,
        b_setup_undo_redo: bool,
        b_print_python_commands: bool,
    ) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_import_nodes_from_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_text,
                __buffer.add(0).cast::<FString>(),
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_commands,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_import_nodes_from_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FName>>().read() }
    }
    pub fn get_unit_structs_for_template(
        in_notation: &FName,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UScriptStruct>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_unit_structs_for_template,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_notation,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_unit_structs_for_template,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UScriptStruct>>>()
                .read()
        }
    }
    pub fn get_top_level_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_top_level_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_top_level_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_template_for_unit_struct(
        in_function: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_method_name: FString,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_template_for_unit_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_method_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_template_for_unit_struct,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_schema(&self) -> UPtr<URigVMSchema> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_schema,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_schema,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMSchema>>().read() }
    }
    pub fn get_registered_unit_structs() -> TArray<
        UPtr<crate::bindings::core_u_object::UScriptStruct>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_registered_unit_structs,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_registered_unit_structs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UScriptStruct>>>()
                .read()
        }
    }
    pub fn get_registered_templates() -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_registered_templates,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::rig_vm_developer::URigVMController::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_registered_templates,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn get_pin_default_value(&mut self, in_pin_path: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_pin_default_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_pin_default_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn get_graph(&self) -> UPtr<URigVMGraph> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_graph,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMGraph>>().read() }
    }
    pub fn get_controller_for_graph(
        &self,
        in_graph: UPtr<URigVMGraph>,
    ) -> UPtr<URigVMController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_controller_for_graph,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_graph,
                __buffer.add(0).cast::<UPtr<URigVMGraph>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_controller_for_graph,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<URigVMController>>().read() }
    }
    pub fn get_action_stack(&self) -> UPtr<URigVMActionStack> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_action_stack,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_get_action_stack,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<URigVMActionStack>>().read() }
    }
    pub fn generate_python_commands(&mut self) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_generate_python_commands,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_generate_python_commands,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FString>>().read() }
    }
    pub fn find_variants_of_function(
        &mut self,
        in_function_name: &FName,
    ) -> TArray<crate::bindings::rig_vm::FRigVMVariantRef> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_variants_of_function,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_variants_of_function,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::rig_vm::FRigVMVariantRef>>()
                .read()
        }
    }
    pub fn find_graph_function_identifier(
        &self,
        in_host_path: FString,
        in_function_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_identifier,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_identifier,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>()
                .read()
        }
    }
    pub fn find_graph_function_header_by_name(
        &self,
        in_host_path: FString,
        in_function_name: FName,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionHeader {
        let mut __stack = crate::core_data::StackAlloc::<560>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_header_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_name,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_header_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>()
                .read()
        }
    }
    pub fn find_graph_function_header(
        &self,
        in_function_identifier: crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier,
    ) -> crate::bindings::rig_vm::FRigVMGraphFunctionHeader {
        let mut __stack = crate::core_data::StackAlloc::<624>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_header,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_identifier,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionIdentifier>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_find_graph_function_header,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(96)
                .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>()
                .read()
        }
    }
    pub fn export_selected_nodes_to_text(
        &mut self,
        b_include_exterior_links: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_export_selected_nodes_to_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_exterior_links,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_export_selected_nodes_to_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn export_node_to_text(
        &mut self,
        in_node: UPtr<URigVMNode>,
        b_include_exterior_links: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_export_node_to_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node,
                __buffer.add(0).cast::<UPtr<URigVMNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_exterior_links,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_export_node_to_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn export_nodes_to_text(
        &mut self,
        in_node_names: &TArray<FName>,
        b_include_exterior_links: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_export_nodes_to_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_exterior_links,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_export_nodes_to_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn expand_library_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> TArray<UPtr<URigVMNode>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_expand_library_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_expand_library_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TArray<UPtr<URigVMNode>>>().read() }
    }
    pub fn enable_reporting(&mut self, b_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_enable_reporting,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_enable_reporting,
                __buffer,
            )
        };
    }
    pub fn eject_node_from_pin(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_eject_node_from_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_eject_node_from_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn duplicate_array_pin(
        &mut self,
        in_array_element_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_duplicate_array_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_element_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_duplicate_array_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn create_function_variant(
        &mut self,
        in_function_name: &FName,
        in_variant_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_create_function_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variant_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_create_function_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn collapse_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
        in_collapse_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_is_aggregate: bool,
    ) -> UPtr<URigVMCollapseNode> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_collapse_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_collapse_node_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_aggregate,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_collapse_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<URigVMCollapseNode>>().read() }
    }
    pub fn close_undo_bracket(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_close_undo_bracket,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_close_undo_bracket,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn clear_pin_category(
        &mut self,
        in_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_pin_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_pin_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_pins(
        &mut self,
        in_pin_paths: &TArray<FString>,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_pins,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_pin(
        &mut self,
        in_pin_path: FString,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_all_pins_on_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_all_pins_on_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_all_pins_on_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn clear_override_on_all_pins_on_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_all_pins_on_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_override_on_all_pins_on_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn clear_node_selection(
        &mut self,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_node_selection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_node_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn clear_node_layout(
        &mut self,
        in_node_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_node_layout,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_node_layout,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn clear_array_pin(
        &mut self,
        in_array_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_array_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_clear_array_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn change_exposed_pin_type(
        &mut self,
        in_pin_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        b_setup_undo_redo: &mut bool,
        b_setup_orphan_pins: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_change_exposed_pin_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_setup_undo_redo,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_orphan_pins,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_change_exposed_pin_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(44).cast::<bool>().swap(b_setup_undo_redo);
        }
        unsafe { __buffer.add(47).cast::<bool>().read() }
    }
    pub fn can_import_nodes_from_text(&mut self, in_text: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_can_import_nodes_from_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_text,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_can_import_nodes_from_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn cancel_undo_bracket(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_cancel_undo_bracket,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_cancel_undo_bracket,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn break_link(
        &mut self,
        in_output_pin_path: FString,
        in_input_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_break_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_output_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_pin_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_break_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn break_all_links(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_break_all_links,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_break_all_links,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn bind_pin_to_variable(
        &mut self,
        in_pin_path: FString,
        in_new_bound_variable_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_bind_pin_to_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_bound_variable_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_bind_pin_to_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn add_variable_node_from_object_path(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        b_is_getter: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMVariableNode> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_variable_node_from_object_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_getter,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(88).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_variable_node_from_object_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<UPtr<URigVMVariableNode>>().read() }
    }
    pub fn add_variable_node(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_getter: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMVariableNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_variable_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_getter,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_variable_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMVariableNode>>().read() }
    }
    pub fn add_unit_node_with_defaults(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_defaults: FString,
        in_method_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMUnitNode> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_unit_node_with_defaults,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_defaults,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(73).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_unit_node_with_defaults,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<UPtr<URigVMUnitNode>>().read() }
    }
    pub fn add_unit_node_from_struct_path(
        &mut self,
        in_script_struct_path: FString,
        in_method_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMUnitNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_unit_node_from_struct_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_unit_node_from_struct_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMUnitNode>>().read() }
    }
    pub fn add_unit_node(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_method_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMUnitNode> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_unit_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_unit_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<URigVMUnitNode>>().read() }
    }
    pub fn add_trait(
        &mut self,
        in_node_name: &FName,
        in_trait_type_object_path: &FName,
        in_trait_name: &FName,
        in_default_value: FString,
        in_pin_index: i32,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_trait,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trait_type_object_path,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_trait_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_index,
                __buffer.add(56).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(60).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(61).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_trait,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<FName>().read() }
    }
    pub fn add_template_node(
        &mut self,
        in_notation: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMTemplateNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_template_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_notation,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_template_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMTemplateNode>>().read() }
    }
    pub fn add_tag_to_function_variant(
        &mut self,
        in_function_name: &FName,
        in_tag: &crate::bindings::rig_vm::FRigVMTag,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<91>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_tag_to_function_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag,
                __buffer.add(16).cast::<crate::bindings::rig_vm::FRigVMTag>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(89).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_tag_to_function_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(90).cast::<bool>().read() }
    }
    pub fn add_select_node_from_struct(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_select_node_from_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_select_node_from_struct,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_select_node(
        &mut self,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_select_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_select_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_reroute_node_on_pin(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_reroute_node_on_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_reroute_node_on_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_reroute_node_on_link_path(
        &mut self,
        in_link_pin_path_representation: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_reroute_node_on_link_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_link_pin_path_representation,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_reroute_node_on_link_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_reroute_node_on_link(
        &mut self,
        in_link: UPtr<URigVMLink>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_reroute_node_on_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_link,
                __buffer.add(0).cast::<UPtr<URigVMLink>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_reroute_node_on_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_parameter_node_from_object_path(
        &mut self,
        in_parameter_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        b_is_input: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMParameterNode> {
        let mut __stack = crate::core_data::StackAlloc::<120>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_parameter_node_from_object_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_input,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(72).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(88).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(104).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_parameter_node_from_object_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<UPtr<URigVMParameterNode>>().read() }
    }
    pub fn add_parameter_node(
        &mut self,
        in_parameter_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_is_input: bool,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMParameterNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_parameter_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parameter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_input,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_parameter_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMParameterNode>>().read() }
    }
    pub fn add_override_to_pins(
        &mut self,
        in_pin_paths: &TArray<FString>,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_pins,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_paths,
                __buffer.add(0).cast::<TArray<FString>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_pins,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn add_override_to_pin(
        &mut self,
        in_pin_path: FString,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn add_override_to_all_pins_on_nodes(
        &mut self,
        in_node_names: &TArray<FName>,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_all_pins_on_nodes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_names,
                __buffer.add(0).cast::<TArray<FName>>(),
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_all_pins_on_nodes,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn add_override_to_all_pins_on_node(
        &mut self,
        in_node_name: &FName,
        b_setup_undo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<15>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_all_pins_on_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_override_to_all_pins_on_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(14).cast::<bool>().read() }
    }
    pub fn add_local_variable_from_object_path(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        in_default_value: FString,
        b_setup_undo_redo: bool,
    ) -> FRigVMGraphVariableDescription {
        let mut __stack = crate::core_data::StackAlloc::<184>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_local_variable_from_object_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_local_variable_from_object_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<FRigVMGraphVariableDescription>().read() }
    }
    pub fn add_local_variable(
        &mut self,
        in_variable_name: &FName,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FRigVMGraphVariableDescription {
        let mut __stack = crate::core_data::StackAlloc::<176>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_local_variable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_local_variable,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<FRigVMGraphVariableDescription>().read() }
    }
    pub fn add_link(
        &mut self,
        in_output_pin_path: FString,
        in_input_pin_path: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        in_user_direction: crate::bindings::rig_vm::ERigVMPinDirection,
        b_create_cast_node: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<37>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_link,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_output_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_input_pin_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_user_direction,
                __buffer.add(34).cast::<crate::bindings::rig_vm::ERigVMPinDirection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_cast_node,
                __buffer.add(35).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_link,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<bool>().read() }
    }
    pub fn add_invoke_entry_node(
        &mut self,
        in_entry_name: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMInvokeEntryNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_invoke_entry_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_entry_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_invoke_entry_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMInvokeEntryNode>>().read() }
    }
    pub fn add_injected_node_from_struct_path(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        in_script_struct_path: FString,
        in_method_name: &FName,
        in_input_pin_name: &FName,
        in_output_pin_name: &FName,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMInjectionInfo> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_injected_node_from_struct_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(40).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_input_pin_name,
                __buffer.add(52).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_output_pin_name,
                __buffer.add(64).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_injected_node_from_struct_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMInjectionInfo>>().read() }
    }
    pub fn add_injected_node(
        &mut self,
        in_pin_path: FString,
        b_as_input: bool,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_method_name: &FName,
        in_input_pin_name: &FName,
        in_output_pin_name: &FName,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMInjectionInfo> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_injected_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_as_input,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_method_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_input_pin_name,
                __buffer.add(44).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_output_pin_name,
                __buffer.add(56).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(72).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(89).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_injected_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<UPtr<URigVMInjectionInfo>>().read() }
    }
    pub fn add_if_node_from_struct(
        &mut self,
        in_script_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_if_node_from_struct,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_script_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_if_node_from_struct,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_if_node(
        &mut self,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_if_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_if_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_function_to_library(
        &mut self,
        in_function_name: &FName,
        b_mutable: bool,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMLibraryNode> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_function_to_library,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mutable,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_function_to_library,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<URigVMLibraryNode>>().read() }
    }
    pub fn add_function_reference_node_from_description(
        &mut self,
        in_function_definition: &crate::bindings::rig_vm::FRigVMGraphFunctionHeader,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMFunctionReferenceNode> {
        let mut __stack = crate::core_data::StackAlloc::<576>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_function_reference_node_from_description,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_definition,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::rig_vm::FRigVMGraphFunctionHeader>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(528).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(544).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(560).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(561).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_function_reference_node_from_description,
                __buffer,
            )
        };
        unsafe { __buffer.add(568).cast::<UPtr<URigVMFunctionReferenceNode>>().read() }
    }
    pub fn add_function_reference_node(
        &mut self,
        in_function_definition: UPtr<URigVMLibraryNode>,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMFunctionReferenceNode> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_function_reference_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_function_definition,
                __buffer.add(0).cast::<UPtr<URigVMLibraryNode>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_function_reference_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<UPtr<URigVMFunctionReferenceNode>>().read() }
    }
    pub fn add_free_reroute_node(
        &mut self,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        b_is_constant: bool,
        in_custom_widget_name: &FName,
        in_default_value: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
    ) -> UPtr<URigVMRerouteNode> {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_free_reroute_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_constant,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_custom_widget_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(80).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(96).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_free_reroute_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<UPtr<URigVMRerouteNode>>().read() }
    }
    pub fn add_external_function_reference_node(
        &mut self,
        in_host_path: FString,
        in_function_name: &FName,
        in_node_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMFunctionReferenceNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_external_function_reference_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_host_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_external_function_reference_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMFunctionReferenceNode>>().read() }
    }
    pub fn add_exposed_pin(
        &mut self,
        in_pin_name: &FName,
        in_direction: crate::bindings::rig_vm::ERigVMPinDirection,
        in_cpp_type: FString,
        in_cpp_type_object_path: &FName,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_exposed_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_pin_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_direction,
                __buffer.add(12).cast::<crate::bindings::rig_vm::ERigVMPinDirection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_exposed_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(68).cast::<FName>().read() }
    }
    pub fn add_enum_node(
        &mut self,
        in_cpp_type_object_path: &FName,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMEnumNode> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_enum_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_cpp_type_object_path,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_enum_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<UPtr<URigVMEnumNode>>().read() }
    }
    pub fn add_empty_pin_category(
        &mut self,
        in_node_name: &FName,
        in_category: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_empty_pin_category,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_node_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_category,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_empty_pin_category,
                __buffer,
            )
        };
        unsafe { __buffer.add(34).cast::<bool>().read() }
    }
    pub fn add_default_tag_to_function_variant(
        &mut self,
        in_function_name: &FName,
        in_tag_name: &FName,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<27>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_default_tag_to_function_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_function_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_tag_name,
                __buffer.add(12).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_default_tag_to_function_variant,
                __buffer,
            )
        };
        unsafe { __buffer.add(26).cast::<bool>().read() }
    }
    pub fn add_comment_node(
        &mut self,
        in_comment_text: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_size: &crate::bindings::core_u_object::FVector2D,
        in_color: &crate::bindings::core_u_object::FLinearColor,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMCommentNode> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_comment_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_comment_text,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_size,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
                __buffer.add(48).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(64).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(80).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(81).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_comment_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<UPtr<URigVMCommentNode>>().read() }
    }
    pub fn add_branch_node(
        &mut self,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_branch_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_branch_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_array_pin(
        &mut self,
        in_array_pin_path: FString,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_array_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_array_pin_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_array_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<FString>().read() }
    }
    pub fn add_array_node_from_object_path(
        &mut self,
        in_op_code: crate::bindings::rig_vm::ERigVMOpCode,
        in_cpp_type: FString,
        in_cpp_type_object_path: FString,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_is_patching: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_array_node_from_object_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_code,
                __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMOpCode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(72).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(73).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_patching,
                __buffer.add(74).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_array_node_from_object_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_array_node(
        &mut self,
        in_op_code: crate::bindings::rig_vm::ERigVMOpCode,
        in_cpp_type: FString,
        in_cpp_type_object: UPtr<crate::bindings::core_u_object::UObject>,
        in_position: &crate::bindings::core_u_object::FVector2D,
        in_node_name: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
        b_is_patching: bool,
    ) -> UPtr<URigVMNode> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_array_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_op_code,
                __buffer.add(0).cast::<crate::bindings::rig_vm::ERigVMOpCode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_cpp_type_object,
                __buffer.add(24).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_position,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(48).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_python_command,
                __buffer.add(65).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_patching,
                __buffer.add(66).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_array_node,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<UPtr<URigVMNode>>().read() }
    }
    pub fn add_aggregate_pin(
        &mut self,
        in_node_name: FString,
        in_pin_name: FString,
        in_default_value: FString,
        b_setup_undo_redo: bool,
        b_print_python_command: bool,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_aggregate_pin,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_node_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_pin_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(32).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_setup_undo_redo,
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
                crate::bindings::rig_vm_developer::__FUNCTION_PTRS
                    .u_rig_vm_controller_add_aggregate_pin,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct URigVMControllerSettings {
    __padding_end: [u8; 136],
}
impl URigVMControllerSettings {
    pub fn static_class() -> crate::core_data::UPtr<
        crate::bindings::core_u_object::UClass,
    > {
        let ptr = *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMControllerSettings")
            .unwrap();
        crate::core_data::UPtr {
            ptr: ptr.cast(),
        }
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigVMControllerSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
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
pub struct FRegisterProvider_InProvider {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRigVMController_ModifiedEventDynamic {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ERigVMTagDisplayMode(pub u8);
impl ERigVMTagDisplayMode {
    pub const NONE: ERigVMTagDisplayMode = ERigVMTagDisplayMode(0);
    pub const ALL: ERigVMTagDisplayMode = ERigVMTagDisplayMode(1);
    pub const DEPRECATION_ONLY: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
    pub const LAST: ERigVMTagDisplayMode = ERigVMTagDisplayMode(2);
}
#[repr(transparent)]
pub struct ERigVMPinDefaultValueType(pub u8);
impl ERigVMPinDefaultValueType {
    pub const AUTO_DETECT: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(0);
    pub const UNSET: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(1);
    pub const OVERRIDE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(2);
    pub const KEEP_VALUE_TYPE: ERigVMPinDefaultValueType = ERigVMPinDefaultValueType(3);
}
#[repr(transparent)]
pub struct ERigVMNodeColorType(pub u8);
impl ERigVMNodeColorType {
    pub const FROM_METADATA: ERigVMNodeColorType = ERigVMNodeColorType(0);
    pub const USER_DEFINED: ERigVMNodeColorType = ERigVMNodeColorType(1);
}
#[repr(transparent)]
pub struct ERigVMGraphNotifType(pub u8);
impl ERigVMGraphNotifType {
    pub const GRAPH_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(0);
    pub const NODE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(1);
    pub const NODE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(2);
    pub const NODE_SELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(3);
    pub const NODE_DESELECTED: ERigVMGraphNotifType = ERigVMGraphNotifType(4);
    pub const NODE_SELECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(5);
    pub const NODE_POSITION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(6);
    pub const NODE_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(7);
    pub const NODE_TITLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(8);
    pub const NODE_COLOR_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(9);
    pub const PIN_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(10);
    pub const PIN_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(11);
    pub const PIN_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(12);
    pub const PIN_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(13);
    pub const PIN_WATCHED_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(14);
    pub const PIN_ARRAY_SIZE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(15);
    pub const PIN_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(16);
    pub const PIN_DIRECTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(17);
    pub const PIN_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(18);
    pub const PIN_INDEX_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(19);
    pub const LINK_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(20);
    pub const LINK_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(21);
    pub const COMMENT_TEXT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(22);
    pub const VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(23);
    pub const VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(24);
    pub const VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(25);
    pub const INTERACTION_BRACKET_OPENED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        26,
    );
    pub const INTERACTION_BRACKET_CLOSED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        27,
    );
    pub const INTERACTION_BRACKET_CANCELED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        28,
    );
    pub const PIN_BOUND_VARIABLE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        29,
    );
    pub const NODE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(30);
    pub const FUNCTION_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(31);
    pub const NODE_REFERENCE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(32);
    pub const NODE_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(33);
    pub const NODE_KEYWORDS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(34);
    pub const NODE_DESCRIPTION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(35);
    pub const VARIABLE_REMAPPING_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        36,
    );
    pub const LIBRARY_TEMPLATE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(37);
    pub const FUNCTION_ACCESS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(38);
    pub const VARIANT_TAGS_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(39);
    pub const PIN_DISPLAY_NAME_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(40);
    pub const PIN_CATEGORY_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(41);
    pub const PIN_CATEGORIES_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(42);
    pub const PIN_CATEGORY_EXPANSION_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        43,
    );
    pub const FUNCTION_VARIANT_GUID_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        44,
    );
    pub const NODE_EARLY_EXIT_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(45);
    pub const LOCAL_VARIABLE_DEFAULT_VALUE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        46,
    );
    pub const LOCAL_VARIABLE_ADDED: ERigVMGraphNotifType = ERigVMGraphNotifType(47);
    pub const LOCAL_VARIABLE_REMOVED: ERigVMGraphNotifType = ERigVMGraphNotifType(48);
    pub const LOCAL_VARIABLE_RENAMED: ERigVMGraphNotifType = ERigVMGraphNotifType(49);
    pub const LOCAL_VARIABLE_TYPE_CHANGED: ERigVMGraphNotifType = ERigVMGraphNotifType(
        50,
    );
    pub const INVALID: ERigVMGraphNotifType = ERigVMGraphNotifType(51);
}
