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
    pub uaie_selection_sets_show_or_hide_controls: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_show_all_controls: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_set_show_and_set_selected_only: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_set_item_row: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_set_item_color: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_set_actor_as_active: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_select_item: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_rename_set_item: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_remove_selection_from_set_item: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_load_from_json_string: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_load_from_json_file: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_key_all: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_isolate_controls: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_is_multi_asset: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_show_and_set_selected_only: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_item_row: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_item_name: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_item_guids: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_item_color: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_all_actors: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_active_selection_sets: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_get_active_actors: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_export_as_json_string: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_export_as_json_file: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_delete_set_item: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_create_set_item_from_selection: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_create_mirror: *mut crate::ffi::UFunctionOpague,
    pub uaie_selection_sets_add_selection_to_set_item: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_setup_all_editor_menus: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_set_preview_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_request_control_rig_init: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_get_preview_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_get_hierarchy_controller: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_get_hierarchy: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_get_currently_open_rig_blueprints: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_get_available_rig_units: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_get_available_rig_modules: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_editor_library_cast_to_control_rig_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_factory_create_new_control_rig_asset: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_blueprint_factory_create_control_rig_from_skeletal_mesh_or_skeleton: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_tween_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_space_compensate: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_snap_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_smart_reduce: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_show_all_controls: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_ds: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scales: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scale: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_scales: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_scale: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_rotators: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_positions: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_position: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_ints: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_int: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_floats: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_float: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_bools: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_local_control_rig_bool: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_interaction: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_controls_mask: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_control_rig_world_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_control_rig_world_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_control_rig_space: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_control_rig_priority_order: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_control_rig_layered_mode: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_control_rig_apply_mode: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_set_constraint_active_key: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_rename_control_rig_control_channels: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_move_control_rig_space: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_move_constraint_key: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_merge_anim_layers_with_settings: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_merge_anim_layers: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section_with_range: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_is_layered_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_is_fk_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_import_fbx_to_control_rig_track: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_hide_all_controls: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_world_space_reference_key: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_visible_control_rigs: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_selection_sets: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_ds: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scales: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scale: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_scales: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_scale: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_rotators: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_rotator: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_positions: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_position: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_ints: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_int: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_floats: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_float: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_bools: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_local_control_rig_bool: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_fk_control_rig_apply_mode: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_default_parent_key: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_controls_mask: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_control_rig_world_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_control_rig_world_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_control_rigs: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_control_rig_priority_order: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_constraints_for_handle: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_constraint_keys: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_anim_layers: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_anim_layer_index: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_actor_world_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_get_actor_world_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_find_or_create_control_rig_track: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_find_or_create_control_rig_component_track: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_export_fbx_from_control_rig_section: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_export_anim_sequence_from_sequencer: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_duplicate_anim_layer: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_delete_control_rig_space: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_delete_constraint_key: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_delete_anim_layer: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_control_rig_copy_vector_parameter_curves_to_transform: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_compensate_all: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_compensate: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers_with_settings: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_blend_values_on_selected: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_bake_to_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_bake_control_rig_space: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_bake_constraints: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_bake_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_add_constraint: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_sequencer_editor_library_add_anim_layer_from_selection: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_edit_mode_delegate_helper_post_pose_update: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_edit_mode_delegate_helper_on_pose_initialized: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_context_menu_context_is_alt_down: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_context_menu_context_get_rig_hierarchy_to_graph_drag_and_drop_context: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_context_menu_context_get_rig_hierarchy_drag_and_drop_context: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_context_menu_context_get_graph_node_context_menu_context: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_context_menu_context_get_control_rig_blueprint: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_context_menu_context_get_control_rig_asset_interface: *mut crate::ffi::UFunctionOpague,
    pub u_control_rig_context_menu_context_get_control_rig: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_set_weight: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_set_type: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_set_selected: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_set_name: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_set_lock: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_set_keyed: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_set_active: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_remove_selected_in_sequencer: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_get_weight: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_get_type: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_get_selected: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_get_name: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_get_lock: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_get_keyed: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_get_active: *mut crate::ffi::UFunctionOpague,
    pub u_anim_layer_add_selected_in_sequencer: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            uaie_selection_sets_show_or_hide_controls: std::ptr::null_mut(),
            uaie_selection_sets_show_all_controls: std::ptr::null_mut(),
            uaie_selection_sets_set_show_and_set_selected_only: std::ptr::null_mut(),
            uaie_selection_sets_set_item_row: std::ptr::null_mut(),
            uaie_selection_sets_set_item_color: std::ptr::null_mut(),
            uaie_selection_sets_set_actor_as_active: std::ptr::null_mut(),
            uaie_selection_sets_select_item: std::ptr::null_mut(),
            uaie_selection_sets_rename_set_item: std::ptr::null_mut(),
            uaie_selection_sets_remove_selection_from_set_item: std::ptr::null_mut(),
            uaie_selection_sets_load_from_json_string: std::ptr::null_mut(),
            uaie_selection_sets_load_from_json_file: std::ptr::null_mut(),
            uaie_selection_sets_key_all: std::ptr::null_mut(),
            uaie_selection_sets_isolate_controls: std::ptr::null_mut(),
            uaie_selection_sets_is_multi_asset: std::ptr::null_mut(),
            uaie_selection_sets_get_show_and_set_selected_only: std::ptr::null_mut(),
            uaie_selection_sets_get_item_row: std::ptr::null_mut(),
            uaie_selection_sets_get_item_name: std::ptr::null_mut(),
            uaie_selection_sets_get_item_guids: std::ptr::null_mut(),
            uaie_selection_sets_get_item_color: std::ptr::null_mut(),
            uaie_selection_sets_get_all_actors: std::ptr::null_mut(),
            uaie_selection_sets_get_active_selection_sets: std::ptr::null_mut(),
            uaie_selection_sets_get_active_actors: std::ptr::null_mut(),
            uaie_selection_sets_export_as_json_string: std::ptr::null_mut(),
            uaie_selection_sets_export_as_json_file: std::ptr::null_mut(),
            uaie_selection_sets_delete_set_item: std::ptr::null_mut(),
            uaie_selection_sets_create_set_item_from_selection: std::ptr::null_mut(),
            uaie_selection_sets_create_mirror: std::ptr::null_mut(),
            uaie_selection_sets_add_selection_to_set_item: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_setup_all_editor_menus: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_set_preview_mesh: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_request_control_rig_init: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_get_preview_mesh: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_get_hierarchy_controller: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_get_hierarchy: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_get_currently_open_rig_blueprints: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_get_available_rig_units: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_get_available_rig_modules: std::ptr::null_mut(),
            u_control_rig_blueprint_editor_library_cast_to_control_rig_blueprint: std::ptr::null_mut(),
            u_control_rig_blueprint_factory_create_new_control_rig_asset: std::ptr::null_mut(),
            u_control_rig_blueprint_factory_create_control_rig_from_skeletal_mesh_or_skeleton: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_tween_control_rig: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_space_compensate: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_snap_control_rig: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_smart_reduce: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_show_all_controls: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_ds: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_d: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scales: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scale: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_scales: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_scale: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_rotators: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_rotator: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_positions: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_position: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_ints: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_int: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_floats: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_float: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_bools: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_local_control_rig_bool: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_interaction: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_controls_mask: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_control_rig_world_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_control_rig_world_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_control_rig_space: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_control_rig_priority_order: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_control_rig_layered_mode: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_control_rig_apply_mode: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_set_constraint_active_key: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_rename_control_rig_control_channels: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_move_control_rig_space: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_move_constraint_key: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_merge_anim_layers_with_settings: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_merge_anim_layers: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section_with_range: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_is_layered_control_rig: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_is_fk_control_rig: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_import_fbx_to_control_rig_track: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_hide_all_controls: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_world_space_reference_key: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_visible_control_rigs: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_selection_sets: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_ds: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_d: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scales: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scale: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_scales: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_scale: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_rotators: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_rotator: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_positions: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_position: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_ints: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_int: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_floats: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_float: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_bools: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_local_control_rig_bool: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_fk_control_rig_apply_mode: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_default_parent_key: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_controls_mask: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_control_rig_world_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_control_rig_world_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_control_rigs: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_control_rig_priority_order: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_constraints_for_handle: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_constraint_keys: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_anim_layers: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_anim_layer_index: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_actor_world_transforms: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_get_actor_world_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_find_or_create_control_rig_track: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_find_or_create_control_rig_component_track: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_export_fbx_from_control_rig_section: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_export_anim_sequence_from_sequencer: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_duplicate_anim_layer: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_delete_control_rig_space: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_delete_constraint_key: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_delete_anim_layer: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_control_rig_copy_vector_parameter_curves_to_transform: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_compensate_all: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_compensate: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers_with_settings: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_blend_values_on_selected: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_bake_to_control_rig: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_bake_control_rig_space: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_bake_constraints: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_bake_constraint: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_add_constraint: std::ptr::null_mut(),
            u_control_rig_sequencer_editor_library_add_anim_layer_from_selection: std::ptr::null_mut(),
            u_control_rig_edit_mode_delegate_helper_post_pose_update: std::ptr::null_mut(),
            u_control_rig_edit_mode_delegate_helper_on_pose_initialized: std::ptr::null_mut(),
            u_control_rig_context_menu_context_is_alt_down: std::ptr::null_mut(),
            u_control_rig_context_menu_context_get_rig_hierarchy_to_graph_drag_and_drop_context: std::ptr::null_mut(),
            u_control_rig_context_menu_context_get_rig_hierarchy_drag_and_drop_context: std::ptr::null_mut(),
            u_control_rig_context_menu_context_get_graph_node_context_menu_context: std::ptr::null_mut(),
            u_control_rig_context_menu_context_get_control_rig_blueprint: std::ptr::null_mut(),
            u_control_rig_context_menu_context_get_control_rig_asset_interface: std::ptr::null_mut(),
            u_control_rig_context_menu_context_get_control_rig: std::ptr::null_mut(),
            u_anim_layer_set_weight: std::ptr::null_mut(),
            u_anim_layer_set_type: std::ptr::null_mut(),
            u_anim_layer_set_selected: std::ptr::null_mut(),
            u_anim_layer_set_name: std::ptr::null_mut(),
            u_anim_layer_set_lock: std::ptr::null_mut(),
            u_anim_layer_set_keyed: std::ptr::null_mut(),
            u_anim_layer_set_active: std::ptr::null_mut(),
            u_anim_layer_remove_selected_in_sequencer: std::ptr::null_mut(),
            u_anim_layer_get_weight: std::ptr::null_mut(),
            u_anim_layer_get_type: std::ptr::null_mut(),
            u_anim_layer_get_selected: std::ptr::null_mut(),
            u_anim_layer_get_name: std::ptr::null_mut(),
            u_anim_layer_get_lock: std::ptr::null_mut(),
            u_anim_layer_get_keyed: std::ptr::null_mut(),
            u_anim_layer_get_active: std::ptr::null_mut(),
            u_anim_layer_add_selected_in_sequencer: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAIESelectionSets::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowOrHideControls"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_show_or_hide_controls,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowAllControls"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_show_all_controls,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetShowAndSetSelectedOnly"),
                &raw mut __FUNCTION_PTRS
                    .uaie_selection_sets_set_show_and_set_selected_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetItemRow"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_set_item_row,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetItemColor"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_set_item_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActorAsActive"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_set_actor_as_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SelectItem"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_select_item,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameSetItem"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_rename_set_item,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveSelectionFromSetItem"),
                &raw mut __FUNCTION_PTRS
                    .uaie_selection_sets_remove_selection_from_set_item,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LoadFromJsonString"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_load_from_json_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LoadFromJsonFile"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_load_from_json_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("KeyAll"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_key_all,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsolateControls"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_isolate_controls,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsMultiAsset"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_is_multi_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetShowAndSetSelectedOnly"),
                &raw mut __FUNCTION_PTRS
                    .uaie_selection_sets_get_show_and_set_selected_only,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetItemRow"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_get_item_row,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetItemName"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_get_item_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetItemGuids"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_get_item_guids,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetItemColor"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_get_item_color,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAllActors"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_get_all_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveSelectionSets"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_get_active_selection_sets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActiveActors"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_get_active_actors,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportAsJsonString"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_export_as_json_string,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportAsJsonFile"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_export_as_json_file,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteSetItem"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_delete_set_item,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateSetItemFromSelection"),
                &raw mut __FUNCTION_PTRS
                    .uaie_selection_sets_create_set_item_from_selection,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateMirror"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_create_mirror,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSelectionToSetItem"),
                &raw mut __FUNCTION_PTRS.uaie_selection_sets_add_selection_to_set_item,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigBlueprintEditorLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetupAllEditorMenus"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_setup_all_editor_menus,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetPreviewMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_set_preview_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RequestControlRigInit"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_request_control_rig_init,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetPreviewMesh"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_preview_mesh,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHierarchyController"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_hierarchy_controller,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetHierarchy"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_hierarchy,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetCurrentlyOpenRigBlueprints"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_currently_open_rig_blueprints,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAvailableRigUnits"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_available_rig_units,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAvailableRigModules"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_available_rig_modules,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CastToControlRigBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_cast_to_control_rig_blueprint,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigBlueprintFactory::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateNewControlRigAsset"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_factory_create_new_control_rig_asset,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CreateControlRigFromSkeletalMeshOrSkeleton"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_blueprint_factory_create_control_rig_from_skeletal_mesh_or_skeleton,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigSequencerEditorLibrary::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("TweenControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_tween_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SpaceCompensate"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_space_compensate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SnapControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_snap_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SmartReduce"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_smart_reduce,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ShowAllControls"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_show_all_controls,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigVector2Ds"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_ds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigVector2D"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_d,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigTransformNoScales"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scales,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigTransformNoScale"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigScales"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_scales,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigScale"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigRotators"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_rotators,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigRotator"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigPositions"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_positions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigInts"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_ints,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigInt"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigFloats"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_floats,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigFloat"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigEulerTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigEulerTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigBools"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_bools,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLocalControlRigBool"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetInteraction"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_interaction,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlsMask"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_controls_mask,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigWorldTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_world_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigWorldTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_world_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigSpace"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigPriorityOrder"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_priority_order,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigLayeredMode"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_layered_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetControlRigApplyMode"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_apply_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetConstraintActiveKey"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_constraint_active_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RenameControlRigControlChannels"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_rename_control_rig_control_channels,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveControlRigSpace"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_move_control_rig_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MoveConstraintKey"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_move_constraint_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MergeAnimLayersWithSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_merge_anim_layers_with_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("MergeAnimLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_merge_anim_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "LoadAnimSequenceIntoControlRigSectionWithRange",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section_with_range,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("LoadAnimSequenceIntoControlRigSection"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsLayeredControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_is_layered_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsFKControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_is_fk_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ImportFBXToControlRigTrack"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_import_fbx_to_control_rig_track,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("HideAllControls"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_hide_all_controls,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWorldSpaceReferenceKey"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_world_space_reference_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetVisibleControlRigs"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_visible_control_rigs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSkeletalMeshComponentWorldTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSkeletalMeshComponentWorldTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelectionSets"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_selection_sets,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigVector2Ds"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_ds,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigVector2D"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_d,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigTransformNoScales"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scales,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigTransformNoScale"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigScales"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_scales,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigScale"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_scale,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigRotators"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_rotators,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigRotator"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_rotator,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigPositions"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_positions,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigPosition"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_position,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigInts"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_ints,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigInt"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_int,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigFloats"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_floats,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigFloat"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_float,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigEulerTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigEulerTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigBools"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_bools,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLocalControlRigBool"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_bool,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetFKControlRigApplyMode"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_fk_control_rig_apply_mode,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetDefaultParentKey"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_default_parent_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlsMask"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_controls_mask,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRigWorldTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_world_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRigWorldTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_world_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRigs"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rigs,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRigPriorityOrder"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_priority_order,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConstraintsForHandle"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_constraints_for_handle,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetConstraintKeys"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_constraint_keys,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_anim_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetAnimLayerIndex"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_anim_layer_index,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorWorldTransforms"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_actor_world_transforms,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActorWorldTransform"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_actor_world_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindOrCreateControlRigTrack"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_find_or_create_control_rig_track,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("FindOrCreateControlRigComponentTrack"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_find_or_create_control_rig_component_track,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportFBXFromControlRigSection"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_export_fbx_from_control_rig_section,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("ExportAnimSequenceFromSequencer"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_export_anim_sequence_from_sequencer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DuplicateAnimLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_duplicate_anim_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteControlRigSpace"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_control_rig_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteConstraintKey"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_constraint_key,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("DeleteAnimLayer"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_anim_layer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from(
                    "ControlRigCopyVectorParameterCurvesToTransform",
                ),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_control_rig_copy_vector_parameter_curves_to_transform,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CompensateAll"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_compensate_all,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("Compensate"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_compensate,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CollapseControlRigAnimLayersWithSettings"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers_with_settings,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("CollapseControlRigAnimLayers"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BlendValuesOnSelected"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_blend_values_on_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BakeToControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_to_control_rig,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BakeControlRigSpace"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_control_rig_space,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BakeConstraints"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_constraints,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("BakeConstraint"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_constraint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddConstraint"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_add_constraint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddAnimLayerFromSelection"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_add_anim_layer_from_selection,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigEditModeDelegateHelper::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("PostPoseUpdate"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_edit_mode_delegate_helper_post_pose_update,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("OnPoseInitialized"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_edit_mode_delegate_helper_on_pose_initialized,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UControlRigContextMenuContext::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("IsAltDown"),
                &raw mut __FUNCTION_PTRS.u_control_rig_context_menu_context_is_alt_down,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRigHierarchyToGraphDragAndDropContext"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_rig_hierarchy_to_graph_drag_and_drop_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetRigHierarchyDragAndDropContext"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_rig_hierarchy_drag_and_drop_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetGraphNodeContextMenuContext"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_graph_node_context_menu_context,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRigBlueprint"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig_blueprint,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRigAssetInterface"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig_asset_interface,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetControlRig"),
                &raw mut __FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig,
            );
        }
    }
    unsafe {
        let bindings = crate::module::bindings();
        if let Some(class_ptr) = UAnimLayer::try_static_class() {
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetWeight"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_set_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetType"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_set_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetSelected"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_set_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetName"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_set_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetLock"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_set_lock,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetKeyed"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_set_keyed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("SetActive"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_set_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("RemoveSelectedInSequencer"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_remove_selected_in_sequencer,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetWeight"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_get_weight,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetType"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_get_type,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetSelected"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_get_selected,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetName"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_get_name,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetLock"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_get_lock,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetKeyed"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_get_keyed,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("GetActive"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_get_active,
            );
            (bindings
                .core_fns
                .find_function_by_name)(
                class_ptr,
                unreal_ffi::Utf8Str::from("AddSelectedInSequencer"),
                &raw mut __FUNCTION_PTRS.u_anim_layer_add_selected_in_sequencer,
            );
        }
    }
}
#[repr(C, align(8))]
pub struct FMultiControlRigElementSelection {
    pub(crate) __padding_end: [u8; 32],
}
impl FMultiControlRigElementSelection {}
#[repr(C, align(8))]
pub struct FControlRigInteractionTransformContext {
    #[doc(hidden)]
    pub(crate) __padding_8: [u8; 8],
    pub drag: crate::bindings::core_u_object::FVector,
    #[doc(hidden)]
    pub(crate) __padding_40: [u8; 8],
    pub rot: crate::bindings::core_u_object::FRotator,
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 8],
    pub scale: crate::bindings::core_u_object::FVector,
    pub space: EControlRigInteractionTransformSpace,
}
impl FControlRigInteractionTransformContext {}
#[repr(C, align(4))]
pub struct FRigSpacePickerBakeSettings {
    pub target_space: crate::bindings::control_rig::FRigElementKey,
    pub settings: crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    pub(crate) __padding_end: [u8; 16],
}
impl FRigSpacePickerBakeSettings {}
#[repr(C, align(8))]
pub struct FAIESelectionSetItemName {
    pub name: FString,
    pub mirror_name: FString,
    pub ty: i32,
    pub owner_actor_name: FString,
}
impl FAIESelectionSetItemName {}
#[repr(C, align(4))]
pub struct FAIESelectionSetItemViewData {
    pub color: crate::bindings::core_u_object::FLinearColor,
    pub row: i32,
    pub(crate) __padding_end: [u8; 4],
}
impl FAIESelectionSetItemViewData {}
#[repr(C, align(8))]
pub struct FAIESelectionSetItem {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub item_name: FText,
    pub names: TArray<FAIESelectionSetItemName>,
    pub view_data: FAIESelectionSetItemViewData,
    pub(crate) __padding_end: [u8; 32],
}
impl FAIESelectionSetItem {}
#[repr(C, align(1))]
pub struct FAnimDetailsBool {
    pub bool: bool,
}
impl FAnimDetailsBool {}
#[repr(C, align(8))]
pub struct FAnimDetailsEnum {
    pub(crate) __padding_end: [u8; 16],
}
impl FAnimDetailsEnum {}
#[repr(C, align(8))]
pub struct FAnimDetailsFloat {
    pub float: f64,
}
impl FAnimDetailsFloat {}
#[repr(C, align(8))]
pub struct FAnimDetailsInteger {
    pub integer: i64,
}
impl FAnimDetailsInteger {}
#[repr(C, align(8))]
pub struct FAnimDetailsLocation {
    pub lx: f64,
    pub ly: f64,
    pub lz: f64,
}
impl FAnimDetailsLocation {}
#[repr(C, align(8))]
pub struct FAnimDetailsRotation {
    pub rx: f64,
    pub ry: f64,
    pub rz: f64,
}
impl FAnimDetailsRotation {}
#[repr(C, align(8))]
pub struct FAnimDetailsScale {
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}
impl FAnimDetailsScale {}
#[repr(C, align(8))]
pub struct FAnimDetailsVector2D {
    pub x: f64,
    pub y: f64,
}
impl FAnimDetailsVector2D {}
#[repr(C, align(8))]
pub struct FControlRigSequencerBindingProxy {
    pub proxy: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    pub control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    pub track: UPtr<crate::bindings::control_rig::UMovieSceneControlRigParameterTrack>,
}
impl FControlRigSequencerBindingProxy {}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
    pub target_hierarchy_key: crate::bindings::control_rig::FRigHierarchyKey,
}
impl FControlRigRigHierarchyDragAndDropContext {}
#[repr(C, align(8))]
pub struct FControlRigGraphNodeContextMenuContext {
    pub graph: UPtr<crate::bindings::rig_vm_developer::URigVMGraph>,
    pub node: UPtr<crate::bindings::rig_vm_developer::URigVMNode>,
    pub pin: UPtr<crate::bindings::rig_vm_developer::URigVMPin>,
}
impl FControlRigGraphNodeContextMenuContext {}
#[repr(C, align(8))]
pub struct FControlRigRigHierarchyToGraphDragAndDropContext {
    pub dragged_hierarchy_keys: TArray<crate::bindings::control_rig::FRigHierarchyKey>,
    pub(crate) __padding_end: [u8; 24],
}
impl FControlRigRigHierarchyToGraphDragAndDropContext {}
#[repr(C, align(4))]
pub struct FMergeAnimLayerSettings {
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance_percentage: f32,
}
impl FMergeAnimLayerSettings {}
#[repr(C, align(4))]
pub struct FAnimLayerPropertyAndChannels {
    pub name: FName,
    pub(crate) __padding_end: [u8; 4],
}
impl FAnimLayerPropertyAndChannels {}
#[repr(C, align(8))]
pub struct FAnimLayerSelectionSet {
    pub bound_object: TWeakObjectPtr<crate::bindings::core_u_object::UObject>,
    pub names: TMap<FName, FAnimLayerPropertyAndChannels>,
}
impl FAnimLayerSelectionSet {}
#[repr(C, align(8))]
pub struct FAnimLayerSectionItem {
    pub anim_layer_set: FAnimLayerSelectionSet,
    pub section: TWeakObjectPtr<crate::bindings::movie_scene::UMovieSceneSection>,
}
impl FAnimLayerSectionItem {}
#[repr(C, align(8))]
pub struct FAnimLayerItem {
    pub section_items: TArray<FAnimLayerSectionItem>,
    pub(crate) __padding_end: [u8; 16],
}
impl FAnimLayerItem {}
#[repr(C, align(8))]
pub struct FAnimLayerState {
    pub b_keyed: crate::bindings::slate_core::ECheckBoxState,
    pub b_selected: crate::bindings::slate_core::ECheckBoxState,
    pub b_active: bool,
    pub b_lock: bool,
    pub name: FText,
    pub weight: f64,
    pub ty: i32,
}
impl FAnimLayerState {}
#[repr(C, align(8))]
pub struct FAnimLayerControlRigObject {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
impl FAnimLayerControlRigObject {}
#[repr(C, align(4))]
pub struct FAnimLayerSceneObject {
    pub scene_object_or_component: TWeakObjectPtr<
        crate::bindings::core_u_object::UObject,
    >,
}
impl FAnimLayerSceneObject {}
#[repr(C, align(8))]
pub struct FAnimLayerObjects {
    pub control_rig_objects: TArray<FAnimLayerControlRigObject>,
    pub scene_objects: TArray<FAnimLayerSceneObject>,
}
impl FAnimLayerObjects {}
#[repr(C, align(8))]
pub struct FControlRigForWorldTransforms {
    pub control_rig: TWeakObjectPtr<crate::bindings::control_rig::UControlRig>,
    pub control_names: TArray<FName>,
}
impl FControlRigForWorldTransforms {}
#[repr(C, align(8))]
pub struct FControlRigSnapperSelection {
    pub actors: TArray<crate::bindings::movie_scene::FActorForWorldTransforms>,
    pub control_rigs: TArray<FControlRigForWorldTransforms>,
}
impl FControlRigSnapperSelection {}
#[repr(C, align(8))]
pub struct UAnimDetailsOptionsMenuContext {
    __padding_end: [u8; 64],
}
impl UAnimDetailsOptionsMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsOptionsMenuContext")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsOptionsMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimDetailsSettings {
    __padding_end: [u8; 56],
}
impl UAnimDetailsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimSequenceConverterFactory {
    __padding_end: [u8; 168],
}
impl UAnimSequenceConverterFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequenceConverterFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequenceConverterFactory")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UConstraintCreationOptions {
    __padding_end: [u8; 56],
}
impl UConstraintCreationOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintCreationOptions")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConstraintCreationOptions")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAIESelectionSets {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub selection_sets: TMap<
        crate::bindings::core_u_object::FGuid,
        FAIESelectionSetItem,
    >,
    __padding_end: [u8; 200],
}
impl UAIESelectionSets {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIESelectionSets")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAIESelectionSets")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn show_or_hide_controls(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        b_show: bool,
        b_do_mirror: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_show_or_hide_controls,
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
        unsafe {
            std::ptr::copy_nonoverlapping(&b_show, __buffer.add(16).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_do_mirror,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_show_or_hide_controls,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn show_all_controls(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_show_all_controls,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_show_all_controls,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_show_and_set_selected_only(&mut self, b_in_show_selected_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_show_and_set_selected_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_show_selected_only,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_show_and_set_selected_only,
                __buffer,
            )
        };
    }
    pub fn set_item_row(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        in_row: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_item_row,
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
        unsafe {
            std::ptr::copy_nonoverlapping(&in_row, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_item_row,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_item_color(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        in_color: &crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_item_color,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_color,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_item_color,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_actor_as_active(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        b_set_active: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_actor_as_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_actor,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_active,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_set_actor_as_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn select_item(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        b_do_mirror: bool,
        b_add: bool,
        b_toggle: bool,
        b_select: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_select_item,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_do_mirror,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_add, __buffer.add(17).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_toggle, __buffer.add(18).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(19).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_select_item,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn rename_set_item(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        new_name: &FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_rename_set_item,
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
        unsafe {
            std::ptr::copy_nonoverlapping(new_name, __buffer.add(16).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_rename_set_item,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_selection_from_set_item(
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_remove_selection_from_set_item,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_remove_selection_from_set_item,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_from_json_string(&mut self, json_string: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_load_from_json_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &json_string,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_load_from_json_string,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_from_json_file(
        &mut self,
        json_file_path: &crate::bindings::core_u_object::FFilePath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_load_from_json_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                json_file_path,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFilePath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_load_from_json_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn key_all(&self, in_guid: &crate::bindings::core_u_object::FGuid) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_key_all,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_key_all,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn isolate_controls(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_isolate_controls,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_isolate_controls,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_multi_asset(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_is_multi_asset,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_is_multi_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_show_and_set_selected_only(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_show_and_set_selected_only,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_show_and_set_selected_only,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_item_row(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        out_row: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_row,
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
        unsafe {
            std::ptr::copy_nonoverlapping(out_row, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_row,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(out_row);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_item_name(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        out_name: &mut FText,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_name,
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
        unsafe {
            std::ptr::copy_nonoverlapping(out_name, __buffer.add(16).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FText>().swap(out_name);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_item_guids(
        &self,
        item_name: &FText,
        out_guids: &mut TArray<crate::bindings::core_u_object::FGuid>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_guids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(item_name, __buffer.add(0).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_guids,
                __buffer.add(16).cast::<TArray<crate::bindings::core_u_object::FGuid>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_guids,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::core_u_object::FGuid>>()
                .swap(out_guids);
        }
    }
    pub fn get_item_color(
        &self,
        in_guid: &crate::bindings::core_u_object::FGuid,
        out_color: &mut crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_color,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_color,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_item_color,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .swap(out_color);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_all_actors(&self) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_all_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_all_actors,
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
    pub fn get_active_selection_sets(
        &mut self,
    ) -> TArray<crate::bindings::core_u_object::FGuid> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_active_selection_sets,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_active_selection_sets,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FGuid>>()
                .read()
        }
    }
    pub fn get_active_actors(&self) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_active_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_get_active_actors,
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
    pub fn export_as_json_string(&self, out_json_string: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_export_as_json_string,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_json_string,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_export_as_json_string,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(out_json_string);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn export_as_json_file(
        &self,
        json_file_path: &crate::bindings::core_u_object::FFilePath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_export_as_json_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                json_file_path,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFilePath>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_export_as_json_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_set_item(
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_delete_set_item,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_delete_set_item,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn create_set_item_from_selection(
        &mut self,
    ) -> crate::bindings::core_u_object::FGuid {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_create_set_item_from_selection,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_create_set_item_from_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::core_u_object::FGuid>().read() }
    }
    pub fn create_mirror(
        &mut self,
        in_guid: &crate::bindings::core_u_object::FGuid,
    ) -> crate::bindings::core_u_object::FGuid {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_create_mirror,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_create_mirror,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FGuid>().read()
        }
    }
    pub fn add_selection_to_set_item(
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_add_selection_to_set_item,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .uaie_selection_sets_add_selection_to_set_item,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAnimDetailsProxyManager {
    __padding_end: [u8; 336],
}
impl UAnimDetailsProxyManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyManager")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyManager")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimDetailsSelection {
    __padding_end: [u8; 160],
}
impl UAnimDetailsSelection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsSelection")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsSelection")
            .copied()
    }
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
pub struct UAnimDetailsProxyBase {
    __padding_end: [u8; 368],
}
impl UAnimDetailsProxyBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyBase")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyBase")
            .copied()
    }
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
pub struct UAnimDetailsProxyBool {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub bool: FAnimDetailsBool,
}
impl UAnimDetailsProxyBool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyBool")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyBool")
            .copied()
    }
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
pub struct UAnimDetailsProxyEnum {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub enum_: FAnimDetailsEnum,
}
impl UAnimDetailsProxyEnum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyEnum")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyEnum")
            .copied()
    }
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
pub struct UAnimDetailsProxyFloat {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub float: FAnimDetailsFloat,
}
impl UAnimDetailsProxyFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyFloat")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyFloat")
            .copied()
    }
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
pub struct UAnimDetailsProxyInteger {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub integer: FAnimDetailsInteger,
}
impl UAnimDetailsProxyInteger {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyInteger")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyInteger")
            .copied()
    }
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
pub struct UAnimDetailsProxyLocation {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub location: FAnimDetailsLocation,
}
impl UAnimDetailsProxyLocation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyLocation")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyLocation")
            .copied()
    }
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
pub struct UAnimDetailsProxyRotation {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub rotation: FAnimDetailsRotation,
}
impl UAnimDetailsProxyRotation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyRotation")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyRotation")
            .copied()
    }
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
pub struct UAnimDetailsProxyScale {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub scale: FAnimDetailsScale,
}
impl UAnimDetailsProxyScale {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyScale")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyScale")
            .copied()
    }
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
pub struct UAnimDetailsProxyTransform {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub location: FAnimDetailsLocation,
    pub rotation: FAnimDetailsRotation,
    pub scale: FAnimDetailsScale,
}
impl UAnimDetailsProxyTransform {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyTransform")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyTransform")
            .copied()
    }
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
pub struct UAnimDetailsProxyVector2D {
    #[doc(hidden)]
    pub(crate) __padding_360: [u8; 360],
    pub vector2_d: FAnimDetailsVector2D,
}
impl UAnimDetailsProxyVector2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyVector2D")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimDetailsProxyVector2D")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBakeToControlRigSettings {
    __padding_end: [u8; 72],
}
impl UBakeToControlRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeToControlRigSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBakeToControlRigSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigBlueprintEditorLibrary {
    __padding_end: [u8; 48],
}
impl UControlRigBlueprintEditorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintEditorLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintEditorLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn setup_all_editor_menus() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_setup_all_editor_menus,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_setup_all_editor_menus,
                __buffer,
            )
        };
    }
    pub fn set_preview_mesh(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
        preview_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        b_mark_as_dirty: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_set_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_mesh,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_mark_as_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_set_preview_mesh,
                __buffer,
            )
        };
    }
    pub fn request_control_rig_init(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_request_control_rig_init,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_request_control_rig_init,
                __buffer,
            )
        };
    }
    pub fn get_preview_mesh(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) -> UPtr<crate::bindings::engine::USkeletalMesh> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_preview_mesh,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>().read()
        }
    }
    pub fn get_hierarchy_controller(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) -> UPtr<crate::bindings::control_rig::URigHierarchyController> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_hierarchy_controller,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_hierarchy_controller,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::control_rig::URigHierarchyController>>()
                .read()
        }
    }
    pub fn get_hierarchy(
        in_rig_blueprint: UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) -> UPtr<crate::bindings::control_rig::URigHierarchy> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_hierarchy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_rig_blueprint,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_hierarchy,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::control_rig::URigHierarchy>>()
                .read()
        }
    }
    pub fn get_currently_open_rig_blueprints() -> TArray<
        UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_currently_open_rig_blueprints,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_currently_open_rig_blueprints,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TArray<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >,
                >()
                .read()
        }
    }
    pub fn get_available_rig_units() -> TArray<
        UPtr<crate::bindings::core_u_object::UStruct>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_available_rig_units,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_available_rig_units,
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
    pub fn get_available_rig_modules() -> TArray<
        crate::bindings::control_rig::FRigModuleDescription,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_available_rig_modules,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_get_available_rig_modules,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::control_rig::FRigModuleDescription>>()
                .read()
        }
    }
    pub fn cast_to_control_rig_blueprint(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        branches: &mut ECastToControlRigBlueprintCases,
        as_control_rig_blueprint: &mut UPtr<
            crate::bindings::control_rig_developer::UControlRigBlueprint,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_cast_to_control_rig_blueprint,
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
                branches,
                __buffer.add(8).cast::<ECastToControlRigBlueprintCases>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                as_control_rig_blueprint,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig_developer::UControlRigBlueprint,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_editor_library_cast_to_control_rig_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<ECastToControlRigBlueprintCases>().swap(branches);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .swap(as_control_rig_blueprint);
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigBlueprintFactory {
    __padding_end: [u8; 144],
}
impl UControlRigBlueprintFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigBlueprintFactory")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn create_new_control_rig_asset(
        in_desired_package_path: FString,
        b_modular_rig: bool,
    ) -> UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_factory_create_new_control_rig_asset,
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
                &b_modular_rig,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintFactory::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_factory_create_new_control_rig_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .read()
        }
    }
    pub fn create_control_rig_from_skeletal_mesh_or_skeleton(
        in_selected_object: UPtr<crate::bindings::core_u_object::UObject>,
        b_modular_rig: bool,
    ) -> UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_factory_create_control_rig_from_skeletal_mesh_or_skeleton,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_selected_object,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_modular_rig,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigBlueprintFactory::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_blueprint_factory_create_control_rig_from_skeletal_mesh_or_skeleton,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigShapeLibraryFactory {
    __padding_end: [u8; 136],
}
impl UControlRigShapeLibraryFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigShapeLibraryFactory")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigShapeLibraryFactory")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigSequencerEditorLibrary {
    __padding_end: [u8; 48],
}
impl UControlRigSequencerEditorLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSequencerEditorLibrary")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSequencerEditorLibrary")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn tween_control_rig(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        tween_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_tween_control_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tween_value,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_tween_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn space_compensate(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_space_compensate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_space_compensate,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn snap_control_rig(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        start_frame: crate::bindings::core_u_object::FFrameNumber,
        end_frame: crate::bindings::core_u_object::FFrameNumber,
        children_to_snap: &FControlRigSnapperSelection,
        parent_to_snap: &FControlRigSnapperSelection,
        snap_settings: UPtr<UControlRigSnapSettings>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<90>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_snap_control_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_frame,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_frame,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                children_to_snap,
                __buffer.add(16).cast::<FControlRigSnapperSelection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                parent_to_snap,
                __buffer.add(48).cast::<FControlRigSnapperSelection>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &snap_settings,
                __buffer.add(80).cast::<UPtr<UControlRigSnapSettings>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(88)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_snap_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(89).cast::<bool>().read() }
    }
    pub fn smart_reduce(
        reduce_params: &mut crate::bindings::curve_editor::FSmartReduceParams,
        movie_scene_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_smart_reduce,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                reduce_params,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::curve_editor::FSmartReduceParams>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movie_scene_section,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_smart_reduce,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::curve_editor::FSmartReduceParams>()
                .swap(reduce_params);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn show_all_controls(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_show_all_controls,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_show_all_controls,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_vector2_ds(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FVector2D>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_ds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_ds,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_vector2_d(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FVector2D,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_vector2_d,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FTransform>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transforms,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transform_no_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::animation_core::FTransformNoScale>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scales,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<
                        TArray<crate::bindings::animation_core::FTransformNoScale>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scales,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transform_no_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::animation_core::FTransformNoScale,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<98>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::animation_core::FTransformNoScale>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(96)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(97).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform_no_scale,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FTransform,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<130>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(128)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(129).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_transform,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FVector>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_scales,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_scales,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FVector,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(56)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_scale,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_rotators(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FRotator>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_rotators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FRotator>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_rotators,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_rotator(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FRotator,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(56)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_rotator,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_positions(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::core_u_object::FVector>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_positions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_positions,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_position(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::core_u_object::FVector,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<58>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(56)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(57).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_position,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_ints(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<i32>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_ints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer.add(48).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_ints,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_int(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: i32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_int,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_floats(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<f32>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_floats,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer.add(48).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_floats,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_float(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_float,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_euler_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<crate::bindings::animation_core::FEulerTransform>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::animation_core::FEulerTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transforms,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_euler_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: crate::bindings::animation_core::FEulerTransform,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<106>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &value,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::animation_core::FEulerTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(104)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(105).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_euler_transform,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_bools(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        values: TArray<bool>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_bools,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &values,
                __buffer.add(48).cast::<TArray<bool>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_bools,
                __buffer,
            )
        };
    }
    pub fn set_local_control_rig_bool(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        value: bool,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<35>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(32).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(33)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(34).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_local_control_rig_bool,
                __buffer,
            )
        };
    }
    pub fn set_interaction(b_is_interacting: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_interaction,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_interacting,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_interaction,
                __buffer,
            )
        };
    }
    pub fn set_controls_mask(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        control_names: &TArray<FName>,
        b_visible: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_controls_mask,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                control_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_controls_mask,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        world_transforms: &TArray<crate::bindings::core_u_object::FTransform>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_world_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                world_transforms,
                __buffer
                    .add(48)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(64)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_world_transforms,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        world_transform: &crate::bindings::core_u_object::FTransform,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_set_key: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<130>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_world_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                world_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(128)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_key,
                __buffer.add(129).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_world_transform,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_name: FName,
        in_space_key: &crate::bindings::control_rig::FRigElementKey,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_space_key,
                __buffer.add(28).cast::<crate::bindings::control_rig::FRigElementKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(44).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_space,
                __buffer,
            )
        };
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn set_control_rig_priority_order(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        priority_order: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_priority_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &priority_order,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_priority_order,
                __buffer,
            )
        };
    }
    pub fn set_control_rig_layered_mode(
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        b_set_is_layered: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_layered_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_set_is_layered,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_layered_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_control_rig_apply_mode(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_apply_mode: crate::bindings::control_rig::EControlRigFKRigExecuteMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_apply_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_apply_mode,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::control_rig::EControlRigFKRigExecuteMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_control_rig_apply_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn set_constraint_active_key(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        b_active: bool,
        in_frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_constraint_active_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_active, __buffer.add(8).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_set_constraint_active_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn rename_control_rig_control_channels(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_old_control_names: &TArray<FName>,
        in_new_control_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_rename_control_rig_control_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_old_control_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_control_names,
                __buffer.add(32).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_rename_control_rig_control_channels,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn move_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_name: FName,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        in_new_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<38>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_move_control_rig_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_time,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_move_control_rig_space,
                __buffer,
            )
        };
        unsafe { __buffer.add(37).cast::<bool>().read() }
    }
    pub fn move_constraint_key(
        constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        constraint_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        in_new_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_move_constraint_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint_section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_time,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_move_constraint_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn merge_anim_layers_with_settings(
        indices: &TArray<i32>,
        merge_anim_layer_settings: &FMergeAnimLayerSettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_merge_anim_layers_with_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                indices,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                merge_anim_layer_settings,
                __buffer.add(16).cast::<FMergeAnimLayerSettings>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_merge_anim_layers_with_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn merge_anim_layers(indices: &TArray<i32>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_merge_anim_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                indices,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_merge_anim_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_anim_sequence_into_control_rig_section_with_range(
        movie_scene_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        skel_mesh_comp: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        in_start_frame: crate::bindings::core_u_object::FFrameNumber,
        b_use_custom_anim_range: bool,
        anim_start_range: crate::bindings::core_u_object::FFrameNumber,
        anim_end_range: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_key_reduce: bool,
        tolerance: f32,
        interpolation: crate::bindings::movie_scene::EMovieSceneKeyInterpolation,
        b_reset_controls: bool,
        b_onto_selected_controls: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section_with_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movie_scene_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skel_mesh_comp,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_frame,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_use_custom_anim_range,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_start_range,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_end_range,
                __buffer.add(36).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_key_reduce,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(44).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneKeyInterpolation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_controls,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_onto_selected_controls,
                __buffer.add(50).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section_with_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(51).cast::<bool>().read() }
    }
    pub fn load_anim_sequence_into_control_rig_section(
        movie_scene_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        skel_mesh_comp: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        in_start_frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        b_key_reduce: bool,
        tolerance: f32,
        interpolation: crate::bindings::movie_scene::EMovieSceneKeyInterpolation,
        b_reset_controls: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<39>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &movie_scene_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skel_mesh_comp,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_start_frame,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(28)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_key_reduce,
                __buffer.add(29).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(32).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation,
                __buffer
                    .add(36)
                    .cast::<crate::bindings::movie_scene::EMovieSceneKeyInterpolation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_controls,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_load_anim_sequence_into_control_rig_section,
                __buffer,
            )
        };
        unsafe { __buffer.add(38).cast::<bool>().read() }
    }
    pub fn is_layered_control_rig(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_is_layered_control_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_is_layered_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_fk_control_rig(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_is_fk_control_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_is_fk_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn import_fbx_to_control_rig_track(
        world: UPtr<crate::bindings::engine::UWorld>,
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        in_section: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
        >,
        selected_control_rig_names: &TArray<FString>,
        import_fbx_control_rig_settings: UPtr<
            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXControlRigSettings,
        >,
        import_filename: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_import_fbx_to_control_rig_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(24)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                selected_control_rig_names,
                __buffer.add(32).cast::<TArray<FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &import_fbx_control_rig_settings,
                __buffer
                    .add(48)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tools::UMovieSceneUserImportFBXControlRigSettings,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &import_filename,
                __buffer.add(56).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_import_fbx_to_control_rig_track,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn hide_all_controls(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_hide_all_controls,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_hide_all_controls,
                __buffer,
            )
        };
    }
    pub fn get_world_space_reference_key() -> crate::bindings::control_rig::FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_world_space_reference_key,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_world_space_reference_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::control_rig::FRigElementKey>().read()
        }
    }
    pub fn get_visible_control_rigs() -> TArray<
        UPtr<crate::bindings::control_rig::UControlRig>,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_visible_control_rigs,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_visible_control_rigs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::control_rig::UControlRig>>>()
                .read()
        }
    }
    pub fn get_skeletal_mesh_component_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        reference_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reference_name,
                __buffer.add(36).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_skeletal_mesh_component_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        reference_name: FName,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &reference_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_skeletal_mesh_component_world_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_selection_sets(b_add_if_does_not_exist: bool) -> UPtr<UAIESelectionSets> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_selection_sets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_if_does_not_exist,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_selection_sets,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UAIESelectionSets>>().read() }
    }
    pub fn get_local_control_rig_vector2_ds(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FVector2D> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_ds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_ds,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .read()
        }
    }
    pub fn get_local_control_rig_vector2_d(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_vector2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_local_control_rig_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_local_control_rig_transform_no_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::animation_core::FTransformNoScale> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scales,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scales,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::animation_core::FTransformNoScale>>()
                .read()
        }
    }
    pub fn get_local_control_rig_transform_no_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::animation_core::FTransformNoScale {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform_no_scale,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<crate::bindings::animation_core::FTransformNoScale>()
                .read()
        }
    }
    pub fn get_local_control_rig_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_local_control_rig_scales(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_scales,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_scales,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_local_control_rig_scale(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_scale,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_local_control_rig_rotators(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FRotator> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_rotators,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_rotators,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FRotator>>()
                .read()
        }
    }
    pub fn get_local_control_rig_rotator(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FRotator {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_rotator,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_rotator,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>().read()
        }
    }
    pub fn get_local_control_rig_positions(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_positions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_positions,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_local_control_rig_position(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_local_control_rig_ints(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_ints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_ints,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<i32>>().read() }
    }
    pub fn get_local_control_rig_int(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_int,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<i32>().read() }
    }
    pub fn get_local_control_rig_floats(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_floats,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_floats,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<f32>>().read() }
    }
    pub fn get_local_control_rig_float(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(36).cast::<f32>().read() }
    }
    pub fn get_local_control_rig_euler_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::animation_core::FEulerTransform> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::animation_core::FEulerTransform>>()
                .read()
        }
    }
    pub fn get_local_control_rig_euler_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::animation_core::FEulerTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_euler_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::animation_core::FEulerTransform>()
                .read()
        }
    }
    pub fn get_local_control_rig_bools(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<bool> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_bools,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_bools,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<TArray<bool>>().read() }
    }
    pub fn get_local_control_rig_bool(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_local_control_rig_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn get_fk_control_rig_apply_mode(
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
    ) -> crate::bindings::control_rig::EControlRigFKRigExecuteMode {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_fk_control_rig_apply_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_fk_control_rig_apply_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::control_rig::EControlRigFKRigExecuteMode>()
                .read()
        }
    }
    pub fn get_default_parent_key() -> crate::bindings::control_rig::FRigElementKey {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_default_parent_key,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_default_parent_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::control_rig::FRigElementKey>().read()
        }
    }
    pub fn get_controls_mask(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        control_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_controls_mask,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_controls_mask,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_control_rig_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_world_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_world_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_control_rig_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        control_name: FName,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_world_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_world_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_control_rigs(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
    ) -> TArray<FControlRigSequencerBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rigs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rigs,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FControlRigSequencerBindingProxy>>().read()
        }
    }
    pub fn get_control_rig_priority_order(
        in_section: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_priority_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_control_rig_priority_order,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_constraints_for_handle(
        in_world: UPtr<crate::bindings::engine::UWorld>,
        in_child: UPtr<crate::bindings::constraints::UTransformableHandle>,
    ) -> TArray<UPtr<crate::bindings::constraints::UTickableConstraint>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_constraints_for_handle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::constraints::UTransformableHandle>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_constraints_for_handle,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<
                    TArray<UPtr<crate::bindings::constraints::UTickableConstraint>>,
                >()
                .read()
        }
    }
    pub fn get_constraint_keys(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        constraint_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        out_bools: &mut TArray<bool>,
        out_frames: &mut TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<50>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_constraint_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint_section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_bools,
                __buffer.add(16).cast::<TArray<bool>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_frames,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(48)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_constraint_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<bool>>().swap(out_bools);
        }
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>()
                .swap(out_frames);
        }
        unsafe { __buffer.add(49).cast::<bool>().read() }
    }
    pub fn get_anim_layers(b_add_if_does_not_exist: bool) -> TArray<UPtr<UAnimLayer>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_anim_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_if_does_not_exist,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_anim_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TArray<UPtr<UAnimLayer>>>().read() }
    }
    pub fn get_anim_layer_index(anim_layer: UPtr<UAnimLayer>) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_anim_layer_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_layer,
                __buffer.add(0).cast::<UPtr<UAnimLayer>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_anim_layer_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_actor_world_transforms(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        actor: UPtr<crate::bindings::engine::AActor>,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::core_u_object::FTransform> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_actor_world_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_actor_world_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .read()
        }
    }
    pub fn get_actor_world_transform(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        actor: UPtr<crate::bindings::engine::AActor>,
        frame: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_actor_world_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_get_actor_world_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn find_or_create_control_rig_track(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_is_layered_control_rig: bool,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneTrack> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_find_or_create_control_rig_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_layered_control_rig,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_find_or_create_control_rig_track,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>()
                .read()
        }
    }
    pub fn find_or_create_control_rig_component_track(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_find_or_create_control_rig_component_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_find_or_create_control_rig_component_track,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn export_fbx_from_control_rig_section(
        sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        section: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
        >,
        export_fbx_control_rig_settings: UPtr<
            crate::bindings::movie_scene_tools::UMovieSceneUserExportFBXControlRigSettings,
        >,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_export_fbx_from_control_rig_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterSection,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_fbx_control_rig_settings,
                __buffer
                    .add(16)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tools::UMovieSceneUserExportFBXControlRigSettings,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_export_fbx_from_control_rig_section,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn export_anim_sequence_from_sequencer(
        anim_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        export_option: UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>,
        binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_create_link: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_export_anim_sequence_from_sequencer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_option,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                binding,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_create_link,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_export_anim_sequence_from_sequencer,
                __buffer,
            )
        };
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn duplicate_anim_layer(index: i32) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_duplicate_anim_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_duplicate_anim_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<i32>().read() }
    }
    pub fn delete_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_name: FName,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_control_rig_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(28).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_control_rig_space,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn delete_constraint_key(
        constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        constraint_section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_constraint_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint_section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(20)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_constraint_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn delete_anim_layer(index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_anim_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_delete_anim_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn control_rig_copy_vector_parameter_curves_to_transform(
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        in_control_name: &FName,
        in_type: crate::bindings::control_rig::ERigControlType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_control_rig_copy_vector_parameter_curves_to_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_control_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer.add(20).cast::<crate::bindings::control_rig::ERigControlType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_control_rig_copy_vector_parameter_curves_to_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn compensate_all(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_compensate_all,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_compensate_all,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn compensate(
        in_constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_compensate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_constraint,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_compensate,
                __buffer,
            )
        };
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn collapse_control_rig_anim_layers_with_settings(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        in_settings: &crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers_with_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers_with_settings,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn collapse_control_rig_anim_layers(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_track: UPtr<
            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
        >,
        b_key_reduce: bool,
        tolerance: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<
                            crate::bindings::control_rig::UMovieSceneControlRigParameterTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_key_reduce,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_collapse_control_rig_anim_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn blend_values_on_selected(
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        blend_operation: EAnimToolBlendOperation,
        blend_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_blend_values_on_selected,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_operation,
                __buffer.add(8).cast::<EAnimToolBlendOperation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &blend_value,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_blend_values_on_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn bake_to_control_rig(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        control_rig_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        export_options: UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>,
        b_reduce_keys: bool,
        tolerance: f32,
        binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b_reset_controls: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<66>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_to_control_rig,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &control_rig_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &export_options,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::unreal_ed::UAnimSeqExportOption>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reduce_keys,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&tolerance, __buffer.add(36).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                binding,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_controls,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_to_control_rig,
                __buffer,
            )
        };
        unsafe { __buffer.add(65).cast::<bool>().read() }
    }
    pub fn bake_control_rig_space(
        in_sequence: UPtr<crate::bindings::level_sequence::ULevelSequence>,
        in_control_rig: UPtr<crate::bindings::control_rig::UControlRig>,
        in_control_names: &TArray<FName>,
        in_settings: FRigSpacePickerBakeSettings,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<94>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_control_rig_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::level_sequence::ULevelSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_control_rig,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::control_rig::UControlRig>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_control_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_settings,
                __buffer.add(32).cast::<FRigSpacePickerBakeSettings>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(92)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_control_rig_space,
                __buffer,
            )
        };
        unsafe { __buffer.add(93).cast::<bool>().read() }
    }
    pub fn bake_constraints(
        world: UPtr<crate::bindings::engine::UWorld>,
        in_constraints: &mut TArray<
            UPtr<crate::bindings::constraints::UTickableConstraint>,
        >,
        in_settings: &crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<53>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_constraints,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_constraints,
                __buffer
                    .add(8)
                    .cast::<
                        TArray<UPtr<crate::bindings::constraints::UTickableConstraint>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_settings,
                __buffer
                    .add(24)
                    .cast::<
                        crate::bindings::movie_scene_tools::FBakingAnimationKeySettings,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_constraints,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    TArray<UPtr<crate::bindings::constraints::UTickableConstraint>>,
                >()
                .swap(in_constraints);
        }
        unsafe { __buffer.add(52).cast::<bool>().read() }
    }
    pub fn bake_constraint(
        world: UPtr<crate::bindings::engine::UWorld>,
        constraint: UPtr<crate::bindings::constraints::UTickableConstraint>,
        frames: &TArray<crate::bindings::core_u_object::FFrameNumber>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_constraint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constraint,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                frames,
                __buffer
                    .add(16)
                    .cast::<TArray<crate::bindings::core_u_object::FFrameNumber>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_bake_constraint,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn add_constraint(
        world: UPtr<crate::bindings::engine::UWorld>,
        in_type: crate::bindings::animation_core::ETransformConstraintType,
        in_child: UPtr<crate::bindings::constraints::UTransformableHandle>,
        in_parent: UPtr<crate::bindings::constraints::UTransformableHandle>,
        b_maintain_offset: bool,
    ) -> UPtr<crate::bindings::constraints::UTickableConstraint> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_add_constraint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &world,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::animation_core::ETransformConstraintType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_child,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::constraints::UTransformableHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_parent,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::constraints::UTransformableHandle>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_maintain_offset,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_add_constraint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<UPtr<crate::bindings::constraints::UTickableConstraint>>()
                .read()
        }
    }
    pub fn add_anim_layer_from_selection() -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_add_anim_layer_from_selection,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::control_rig_editor::UControlRigSequencerEditorLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_sequencer_editor_library_add_anim_layer_from_selection,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UControlRigThumbnailRenderer {
    __padding_end: [u8; 232],
}
impl UControlRigThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigThumbnailRenderer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigThumbnailRenderer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigEditModeDelegateHelper {
    __padding_end: [u8; 72],
}
impl UControlRigEditModeDelegateHelper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditModeDelegateHelper")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditModeDelegateHelper")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigEditModeSettings {
    __padding_end: [u8; 240],
}
impl UControlRigEditModeSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditModeSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigEditModeSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigContextMenuContext {
    __padding_end: [u8; 232],
}
impl UControlRigContextMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigContextMenuContext")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigContextMenuContext")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn is_alt_down(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_is_alt_down,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_is_alt_down,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_rig_hierarchy_to_graph_drag_and_drop_context(
        &mut self,
    ) -> FControlRigRigHierarchyToGraphDragAndDropContext {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_rig_hierarchy_to_graph_drag_and_drop_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_rig_hierarchy_to_graph_drag_and_drop_context,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<FControlRigRigHierarchyToGraphDragAndDropContext>()
                .read()
        }
    }
    pub fn get_rig_hierarchy_drag_and_drop_context(
        &mut self,
    ) -> FControlRigRigHierarchyDragAndDropContext {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_rig_hierarchy_drag_and_drop_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_rig_hierarchy_drag_and_drop_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FControlRigRigHierarchyDragAndDropContext>().read()
        }
    }
    pub fn get_graph_node_context_menu_context(
        &mut self,
    ) -> FControlRigGraphNodeContextMenuContext {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_graph_node_context_menu_context,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_graph_node_context_menu_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FControlRigGraphNodeContextMenuContext>().read()
        }
    }
    pub fn get_control_rig_blueprint(
        &self,
    ) -> UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig_blueprint,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig_blueprint,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    UPtr<crate::bindings::control_rig_developer::UControlRigBlueprint>,
                >()
                .read()
        }
    }
    pub fn get_control_rig_asset_interface(
        &self,
    ) -> TScriptInterface<
        crate::bindings::control_rig_developer::UControlRigAssetInterface,
    > {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig_asset_interface,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig_asset_interface,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<
                    TScriptInterface<
                        crate::bindings::control_rig_developer::UControlRigAssetInterface,
                    >,
                >()
                .read()
        }
    }
    pub fn get_control_rig(&self) -> UPtr<crate::bindings::control_rig::UControlRig> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_control_rig_context_menu_context_get_control_rig,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<UPtr<crate::bindings::control_rig::UControlRig>>()
                .read()
        }
    }
}
#[repr(C, align(16))]
pub struct UControlRigSkeletalMeshComponent {
    __padding_end: [u8; 5504],
}
impl UControlRigSkeletalMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSkeletalMeshComponent")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSkeletalMeshComponent")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigWrapperObject {
    __padding_end: [u8; 152],
}
impl UControlRigWrapperObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigWrapperObject")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigWrapperObject")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigConnectorTargetsDetailWrapper {
    __padding_end: [u8; 88],
}
impl URigConnectorTargetsDetailWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigConnectorTargetsDetailWrapper")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigConnectorTargetsDetailWrapper")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigDependencyGraph {
    __padding_end: [u8; 848],
}
impl URigDependencyGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraph")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraph")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigDependencyGraphNode {
    __padding_end: [u8; 456],
}
impl URigDependencyGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraphNode")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraphNode")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct URigDependencyGraphSchema {
    __padding_end: [u8; 48],
}
impl URigDependencyGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraphSchema")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URigDependencyGraphSchema")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimationAuthoringSettings {
    __padding_end: [u8; 112],
}
impl UAnimationAuthoringSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationAuthoringSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationAuthoringSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayerSequencerFilter {
    __padding_end: [u8; 48],
}
impl UAnimLayerSequencerFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayerSequencerFilter")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayerSequencerFilter")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayerWeightProxy {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub weight: f64,
}
impl UAnimLayerWeightProxy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayerWeightProxy")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayerWeightProxy")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayer {
    __padding_end: [u8; 184],
}
impl UAnimLayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_weight(&mut self, in_weight: f64) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_weight,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_weight,
                __buffer,
            )
        };
    }
    pub fn set_type(&mut self, layer_type: EAnimLayerType) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &layer_type,
                __buffer.add(0).cast::<EAnimLayerType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_type,
                __buffer,
            )
        };
    }
    pub fn set_selected(&mut self, b_in_selected: bool, b_clear_selection: bool) {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_selected,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_selected,
                __buffer,
            )
        };
    }
    pub fn set_name(&mut self, in_name: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(0).cast::<FText>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_name,
                __buffer,
            )
        };
    }
    pub fn set_lock(&mut self, b_in_lock: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_lock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_in_lock, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_lock,
                __buffer,
            )
        };
    }
    pub fn set_keyed(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_keyed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_keyed,
                __buffer,
            )
        };
    }
    pub fn set_active(&mut self, b_in_active: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_active,
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
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_set_active,
                __buffer,
            )
        };
    }
    pub fn remove_selected_in_sequencer(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_remove_selected_in_sequencer,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_remove_selected_in_sequencer,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_weight(&self) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_weight,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f64>().read() }
    }
    pub fn get_type(&self) -> EAnimLayerType {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_type,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<EAnimLayerType>().read() }
    }
    pub fn get_selected(&self) -> crate::bindings::slate_core::ECheckBoxState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_selected,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_selected,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::slate_core::ECheckBoxState>().read()
        }
    }
    pub fn get_name(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_name,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_lock(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_lock,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_lock,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_keyed(&self) -> crate::bindings::slate_core::ECheckBoxState {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_keyed,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_keyed,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<crate::bindings::slate_core::ECheckBoxState>().read()
        }
    }
    pub fn get_active(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_active,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_get_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn add_selected_in_sequencer(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_add_selected_in_sequencer,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::control_rig_editor::__FUNCTION_PTRS
                    .u_anim_layer_add_selected_in_sequencer,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAnimLayers {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub anim_layers: TArray<UPtr<UAnimLayer>>,
    __padding_end: [u8; 72],
}
impl UAnimLayers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayers")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayers")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigTrackFilter {
    __padding_end: [u8; 48],
}
impl UControlRigTrackFilter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigTrackFilter")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigTrackFilter")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULoadAnimToControlRigSettings {
    __padding_end: [u8; 80],
}
impl ULoadAnimToControlRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoadAnimToControlRigSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoadAnimToControlRigSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USelectionSetsSettings {
    __padding_end: [u8; 120],
}
impl USelectionSetsSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelectionSetsSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelectionSetsSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetDefinition_ControlRigPose {
    __padding_end: [u8; 72],
}
impl UAssetDefinition_ControlRigPose {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_ControlRigPose")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinition_ControlRigPose")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigPoseThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UControlRigPoseThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseThumbnailRenderer")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigPoseThumbnailRenderer")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UControlRigSnapSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub b_keep_offset: bool,
    pub b_snap_position: bool,
    pub b_snap_rotation: bool,
    pub b_snap_scale: bool,
    pub baking_key_settings: crate::bindings::movie_scene_tools::EBakingKeySettings,
    pub frame_increment: i32,
    pub b_reduce_keys: bool,
    pub tolerance: f32,
}
impl UControlRigSnapSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSnapSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UControlRigSnapSettings")
            .copied()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UCreateControlPoseAssetRigSettings {
    __padding_end: [u8; 64],
}
impl UCreateControlPoseAssetRigSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCreateControlPoseAssetRigSettings")
            .unwrap()
    }
    pub fn try_static_class() -> Option<*mut crate::ffi::UObjectOpague> {
        crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCreateControlPoseAssetRigSettings")
            .copied()
    }
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
pub struct EControlRigInteractionTransformSpace(pub i32);
impl EControlRigInteractionTransformSpace {
    pub const WORLD: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        0,
    );
    pub const LOCAL: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        1,
    );
    pub const PARENT: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        2,
    );
    pub const EXPLICIT: EControlRigInteractionTransformSpace = EControlRigInteractionTransformSpace(
        3,
    );
}
#[repr(transparent)]
pub struct EControlRigConstrainTab(pub u8);
impl EControlRigConstrainTab {
    pub const SPACES: EControlRigConstrainTab = EControlRigConstrainTab(0);
    pub const CONSTRAINTS: EControlRigConstrainTab = EControlRigConstrainTab(1);
    pub const SNAPPER: EControlRigConstrainTab = EControlRigConstrainTab(2);
}
#[repr(transparent)]
pub struct ECastToControlRigBlueprintCases(pub u8);
impl ECastToControlRigBlueprintCases {
    pub const CAST_SUCCEEDED: ECastToControlRigBlueprintCases = ECastToControlRigBlueprintCases(
        0,
    );
    pub const CAST_FAILED: ECastToControlRigBlueprintCases = ECastToControlRigBlueprintCases(
        1,
    );
}
#[repr(transparent)]
pub struct EAnimToolBlendOperation(pub u8);
impl EAnimToolBlendOperation {
    pub const TWEEN: EAnimToolBlendOperation = EAnimToolBlendOperation(0);
    pub const BLEND_TO_NEIGHBOR: EAnimToolBlendOperation = EAnimToolBlendOperation(1);
    pub const PUSH_PULL: EAnimToolBlendOperation = EAnimToolBlendOperation(2);
    pub const BLEND_RELATIVE: EAnimToolBlendOperation = EAnimToolBlendOperation(3);
    pub const BLEND_TO_EASE: EAnimToolBlendOperation = EAnimToolBlendOperation(4);
    pub const SMOOTH_ROUGH: EAnimToolBlendOperation = EAnimToolBlendOperation(5);
}
#[repr(transparent)]
pub struct EAnimLayerType(pub u8);
impl EAnimLayerType {
    pub const BASE: EAnimLayerType = EAnimLayerType(0);
    pub const ADDITIVE: EAnimLayerType = EAnimLayerType(1);
    pub const OVERRIDE: EAnimLayerType = EAnimLayerType(2);
}
