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
    pub u_factory_script_factory_create_file: *mut crate::ffi::UFunctionOpague,
    pub u_factory_script_factory_can_import: *mut crate::ffi::UFunctionOpague,
    pub u_asset_editor_toolkit_menu_context_get_editing_objects: *mut crate::ffi::UFunctionOpague,
    pub u_fbx_import_ui_reset_to_default: *mut crate::ffi::UFunctionOpague,
    pub uudim_texture_function_library_make_udim_virtual_texture_from_texture2_ds: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_unlock_selected_groups: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_ungroup_selected: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_ungroup_actors: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_set_grouping_active: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_remove_selected_from_group: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_lock_selected_groups: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_is_grouping_active: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_group_selected: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_group_actors: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_get: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_can_group_selected_actors: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_can_group_actors: *mut crate::ffi::UFunctionOpague,
    pub u_actor_grouping_utils_add_selected_to_group: *mut crate::ffi::UFunctionOpague,
    pub u_asset_import_task_is_async_import_complete: *mut crate::ffi::UFunctionOpague,
    pub u_asset_import_task_get_objects: *mut crate::ffi::UFunctionOpague,
    pub u_cook_function_library_cook_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_set_level_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_set_levels_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_move_selected_actors_to_level: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_move_actors_to_level: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_make_level_current: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_k2_remove_level_from_world: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_k2_add_level_to_world_with_transform: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_k2_add_level_to_world: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_get_levels: *mut crate::ffi::UFunctionOpague,
    pub u_editor_level_utils_create_new_streaming_level: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_helpers_find_existing_material_from_search_location: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_helpers_find_existing_material: *mut crate::ffi::UFunctionOpague,
    pub u_reimport_fbx_scene_factory_script_reimport_helper: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_unload_packages: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_save_packages_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_save_packages: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_save_map: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_save_dirty_packages_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_save_dirty_packages: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_save_current_level: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_reload_packages: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_new_map_from_template: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_new_blank_map: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_load_map_with_dialog: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_load_map: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_import_scene: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_get_dirty_map_packages: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_get_dirty_content_packages: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_fully_load_packages: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_fully_load_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_loading_and_saving_utils_export_scene: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_update_all_view_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_update_all_actors_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_update_actor_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_update_actor_all_views_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_try_get_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_toggle_layer_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_toggle_layers_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_set_layer_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_set_layers_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_select_actors_in_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_select_actors_in_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_rename_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_remove_selected_actors_from_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_remove_selected_actors_from_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_remove_level_layer_information: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_remove_actors_from_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_remove_actors_from_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_remove_actor_from_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_remove_actor_from_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_make_all_layers_visible: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_is_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_is_actor_valid_for_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_initialize_new_actor_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_get_world: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_get_selected_actors: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_get_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_get_actors_from_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_get_actors_from_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_editor_refresh_layer_browser: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_editor_map_change: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_disassociate_actors_from_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_disassociate_actor_from_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_delete_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_delete_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_create_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_append_actors_from_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_append_actors_from_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_selected_actors_to_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_selected_actors_to_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_level_layer_information: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_all_layers_to: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_all_layer_names_to: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_actor_to_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_actor_to_layer: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_actors_to_layers: *mut crate::ffi::UFunctionOpague,
    pub u_layers_subsystem_add_actors_to_layer: *mut crate::ffi::UFunctionOpague,
    pub u_package_tools_sanitize_package_name: *mut crate::ffi::UFunctionOpague,
    pub u_package_tools_package_name_to_filename: *mut crate::ffi::UFunctionOpague,
    pub u_package_tools_filename_to_package_name: *mut crate::ffi::UFunctionOpague,
    pub a_property_editor_test_actor_get_options_func: *mut crate::ffi::UFunctionOpague,
    pub u_asset_editor_subsystem_open_editor_for_assets: *mut crate::ffi::UFunctionOpague,
    pub u_asset_editor_subsystem_close_all_editors_for_asset: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_reparent_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_rename_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_remove_assets_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_remove_asset_ptrs_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_remove_asset_ptr_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_remove_asset_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_remove_asset_datas_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_remove_asset_data_from_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_collections_containing_asset_ptr: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_collections_containing_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_collections_containing_asset: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_collections_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_collections: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_collection_containers: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_base_game_collection_container: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_get_assets_in_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_empty_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_destroy_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_create_or_empty_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_create_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_collection_exists: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_add_asset_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_add_assets_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_add_asset_ptr_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_add_asset_ptrs_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_add_asset_data_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_collection_manager_scripting_subsystem_add_asset_datas_to_collection: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_spawn_actor_from_object: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_spawn_actor_from_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_set_selected_level_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_set_component_transform: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_set_actor_transform: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_set_actor_selection_state: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_select_nothing: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_select_all_children: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_select_all: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_invert_selection: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_get_selected_level_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_get_all_level_actors_components: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_get_all_level_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_get_actor_reference: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_duplicate_selected_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_duplicate_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_duplicate_actor: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_destroy_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_destroy_actor: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_delete_selected_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_convert_actors: *mut crate::ffi::UFunctionOpague,
    pub u_editor_actor_subsystem_clear_actor_selection_set: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_sort_by_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_set_metadata_tag: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_set_dirty_flag: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_save_loaded_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_save_loaded_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_save_directory: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_save_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_rename_loaded_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_rename_directory: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_rename_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_remove_on_extract_asset_from_file: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_remove_metadata_tag: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_on_extract_asset_from_file_dynamic_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_make_directory: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_load_blueprint_class: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_load_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_list_assets_by_tag_value: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_list_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_get_tag_values: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_get_path_name_for_loaded_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_get_metadata_tag_values: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_get_metadata_tag: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_get_loaded_asset_filename_length_for_cooking: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_get_asset_filename_length_for_cooking: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_get_all_assets_by_meta_data_tags: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_find_package_referencers_for_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_find_asset_data: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_duplicate_loaded_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_duplicate_directory: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_duplicate_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_does_directory_exist: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_does_directory_contain_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_does_asset_exist: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_do_assets_exist: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_delete_loaded_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_delete_loaded_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_delete_directory: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_delete_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_consolidate_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_checkout_loaded_assets: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_checkout_loaded_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_checkout_directory: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_checkout_asset: *mut crate::ffi::UFunctionOpague,
    pub u_editor_asset_subsystem_add_on_extract_asset_from_file: *mut crate::ffi::UFunctionOpague,
    pub u_editor_subsystem_blueprint_library_toggle_preview_platform: *mut crate::ffi::UFunctionOpague,
    pub u_editor_subsystem_blueprint_library_set_preview_platform: *mut crate::ffi::UFunctionOpague,
    pub u_editor_subsystem_blueprint_library_get_preview_platform_options: *mut crate::ffi::UFunctionOpague,
    pub u_editor_subsystem_blueprint_library_get_editor_subsystem: *mut crate::ffi::UFunctionOpague,
    pub u_editor_subsystem_blueprint_library_disable_preview_platform: *mut crate::ffi::UFunctionOpague,
    pub u_import_subsystem_on_asset_reimport_dyn_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_import_subsystem_on_asset_pre_import_dyn_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_import_subsystem_on_asset_post_lod_import_dyn_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_import_subsystem_on_asset_post_import_dyn_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_unreal_editor_subsystem_set_level_viewport_camera_info: *mut crate::ffi::UFunctionOpague,
    pub u_unreal_editor_subsystem_get_level_viewport_camera_info: *mut crate::ffi::UFunctionOpague,
    pub u_unreal_editor_subsystem_get_game_world: *mut crate::ffi::UFunctionOpague,
    pub u_unreal_editor_subsystem_get_editor_world: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_factory_script_factory_create_file: std::ptr::null_mut(),
            u_factory_script_factory_can_import: std::ptr::null_mut(),
            u_asset_editor_toolkit_menu_context_get_editing_objects: std::ptr::null_mut(),
            u_fbx_import_ui_reset_to_default: std::ptr::null_mut(),
            uudim_texture_function_library_make_udim_virtual_texture_from_texture2_ds: std::ptr::null_mut(),
            u_actor_grouping_utils_unlock_selected_groups: std::ptr::null_mut(),
            u_actor_grouping_utils_ungroup_selected: std::ptr::null_mut(),
            u_actor_grouping_utils_ungroup_actors: std::ptr::null_mut(),
            u_actor_grouping_utils_set_grouping_active: std::ptr::null_mut(),
            u_actor_grouping_utils_remove_selected_from_group: std::ptr::null_mut(),
            u_actor_grouping_utils_lock_selected_groups: std::ptr::null_mut(),
            u_actor_grouping_utils_is_grouping_active: std::ptr::null_mut(),
            u_actor_grouping_utils_group_selected: std::ptr::null_mut(),
            u_actor_grouping_utils_group_actors: std::ptr::null_mut(),
            u_actor_grouping_utils_get: std::ptr::null_mut(),
            u_actor_grouping_utils_can_group_selected_actors: std::ptr::null_mut(),
            u_actor_grouping_utils_can_group_actors: std::ptr::null_mut(),
            u_actor_grouping_utils_add_selected_to_group: std::ptr::null_mut(),
            u_asset_import_task_is_async_import_complete: std::ptr::null_mut(),
            u_asset_import_task_get_objects: std::ptr::null_mut(),
            u_cook_function_library_cook_asset: std::ptr::null_mut(),
            u_editor_level_utils_set_level_visibility: std::ptr::null_mut(),
            u_editor_level_utils_set_levels_visibility: std::ptr::null_mut(),
            u_editor_level_utils_move_selected_actors_to_level: std::ptr::null_mut(),
            u_editor_level_utils_move_actors_to_level: std::ptr::null_mut(),
            u_editor_level_utils_make_level_current: std::ptr::null_mut(),
            u_editor_level_utils_k2_remove_level_from_world: std::ptr::null_mut(),
            u_editor_level_utils_k2_add_level_to_world_with_transform: std::ptr::null_mut(),
            u_editor_level_utils_k2_add_level_to_world: std::ptr::null_mut(),
            u_editor_level_utils_get_levels: std::ptr::null_mut(),
            u_editor_level_utils_create_new_streaming_level: std::ptr::null_mut(),
            u_material_import_helpers_find_existing_material_from_search_location: std::ptr::null_mut(),
            u_material_import_helpers_find_existing_material: std::ptr::null_mut(),
            u_reimport_fbx_scene_factory_script_reimport_helper: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_unload_packages: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_save_packages_with_dialog: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_save_packages: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_save_map: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_save_dirty_packages_with_dialog: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_save_dirty_packages: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_save_current_level: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_reload_packages: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_new_map_from_template: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_new_blank_map: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_load_map_with_dialog: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_load_map: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_import_scene: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_get_dirty_map_packages: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_get_dirty_content_packages: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_fully_load_packages: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_fully_load_assets: std::ptr::null_mut(),
            u_editor_loading_and_saving_utils_export_scene: std::ptr::null_mut(),
            u_layers_subsystem_update_all_view_visibility: std::ptr::null_mut(),
            u_layers_subsystem_update_all_actors_visibility: std::ptr::null_mut(),
            u_layers_subsystem_update_actor_visibility: std::ptr::null_mut(),
            u_layers_subsystem_update_actor_all_views_visibility: std::ptr::null_mut(),
            u_layers_subsystem_try_get_layer: std::ptr::null_mut(),
            u_layers_subsystem_toggle_layer_visibility: std::ptr::null_mut(),
            u_layers_subsystem_toggle_layers_visibility: std::ptr::null_mut(),
            u_layers_subsystem_set_layer_visibility: std::ptr::null_mut(),
            u_layers_subsystem_set_layers_visibility: std::ptr::null_mut(),
            u_layers_subsystem_select_actors_in_layers: std::ptr::null_mut(),
            u_layers_subsystem_select_actors_in_layer: std::ptr::null_mut(),
            u_layers_subsystem_rename_layer: std::ptr::null_mut(),
            u_layers_subsystem_remove_selected_actors_from_layers: std::ptr::null_mut(),
            u_layers_subsystem_remove_selected_actors_from_layer: std::ptr::null_mut(),
            u_layers_subsystem_remove_level_layer_information: std::ptr::null_mut(),
            u_layers_subsystem_remove_actors_from_layers: std::ptr::null_mut(),
            u_layers_subsystem_remove_actors_from_layer: std::ptr::null_mut(),
            u_layers_subsystem_remove_actor_from_layers: std::ptr::null_mut(),
            u_layers_subsystem_remove_actor_from_layer: std::ptr::null_mut(),
            u_layers_subsystem_make_all_layers_visible: std::ptr::null_mut(),
            u_layers_subsystem_is_layer: std::ptr::null_mut(),
            u_layers_subsystem_is_actor_valid_for_layer: std::ptr::null_mut(),
            u_layers_subsystem_initialize_new_actor_layers: std::ptr::null_mut(),
            u_layers_subsystem_get_world: std::ptr::null_mut(),
            u_layers_subsystem_get_selected_actors: std::ptr::null_mut(),
            u_layers_subsystem_get_layer: std::ptr::null_mut(),
            u_layers_subsystem_get_actors_from_layers: std::ptr::null_mut(),
            u_layers_subsystem_get_actors_from_layer: std::ptr::null_mut(),
            u_layers_subsystem_editor_refresh_layer_browser: std::ptr::null_mut(),
            u_layers_subsystem_editor_map_change: std::ptr::null_mut(),
            u_layers_subsystem_disassociate_actors_from_layers: std::ptr::null_mut(),
            u_layers_subsystem_disassociate_actor_from_layers: std::ptr::null_mut(),
            u_layers_subsystem_delete_layers: std::ptr::null_mut(),
            u_layers_subsystem_delete_layer: std::ptr::null_mut(),
            u_layers_subsystem_create_layer: std::ptr::null_mut(),
            u_layers_subsystem_append_actors_from_layers: std::ptr::null_mut(),
            u_layers_subsystem_append_actors_from_layer: std::ptr::null_mut(),
            u_layers_subsystem_add_selected_actors_to_layers: std::ptr::null_mut(),
            u_layers_subsystem_add_selected_actors_to_layer: std::ptr::null_mut(),
            u_layers_subsystem_add_level_layer_information: std::ptr::null_mut(),
            u_layers_subsystem_add_all_layers_to: std::ptr::null_mut(),
            u_layers_subsystem_add_all_layer_names_to: std::ptr::null_mut(),
            u_layers_subsystem_add_actor_to_layers: std::ptr::null_mut(),
            u_layers_subsystem_add_actor_to_layer: std::ptr::null_mut(),
            u_layers_subsystem_add_actors_to_layers: std::ptr::null_mut(),
            u_layers_subsystem_add_actors_to_layer: std::ptr::null_mut(),
            u_package_tools_sanitize_package_name: std::ptr::null_mut(),
            u_package_tools_package_name_to_filename: std::ptr::null_mut(),
            u_package_tools_filename_to_package_name: std::ptr::null_mut(),
            a_property_editor_test_actor_get_options_func: std::ptr::null_mut(),
            u_asset_editor_subsystem_open_editor_for_assets: std::ptr::null_mut(),
            u_asset_editor_subsystem_close_all_editors_for_asset: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_reparent_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_rename_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_remove_assets_from_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_remove_asset_ptrs_from_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_remove_asset_ptr_from_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_remove_asset_from_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_remove_asset_datas_from_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_remove_asset_data_from_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_collections_containing_asset_ptr: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_collections_containing_asset_data: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_collections_containing_asset: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_collections_by_name: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_collections: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_collection_containers: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_base_game_collection_container: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_get_assets_in_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_empty_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_destroy_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_create_or_empty_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_create_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_collection_exists: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_add_asset_to_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_add_assets_to_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_add_asset_ptr_to_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_add_asset_ptrs_to_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_add_asset_data_to_collection: std::ptr::null_mut(),
            u_collection_manager_scripting_subsystem_add_asset_datas_to_collection: std::ptr::null_mut(),
            u_editor_actor_subsystem_spawn_actor_from_object: std::ptr::null_mut(),
            u_editor_actor_subsystem_spawn_actor_from_class: std::ptr::null_mut(),
            u_editor_actor_subsystem_set_selected_level_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_set_component_transform: std::ptr::null_mut(),
            u_editor_actor_subsystem_set_actor_transform: std::ptr::null_mut(),
            u_editor_actor_subsystem_set_actor_selection_state: std::ptr::null_mut(),
            u_editor_actor_subsystem_select_nothing: std::ptr::null_mut(),
            u_editor_actor_subsystem_select_all_children: std::ptr::null_mut(),
            u_editor_actor_subsystem_select_all: std::ptr::null_mut(),
            u_editor_actor_subsystem_invert_selection: std::ptr::null_mut(),
            u_editor_actor_subsystem_get_selected_level_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_get_all_level_actors_components: std::ptr::null_mut(),
            u_editor_actor_subsystem_get_all_level_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_get_actor_reference: std::ptr::null_mut(),
            u_editor_actor_subsystem_duplicate_selected_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_duplicate_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_duplicate_actor: std::ptr::null_mut(),
            u_editor_actor_subsystem_destroy_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_destroy_actor: std::ptr::null_mut(),
            u_editor_actor_subsystem_delete_selected_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_convert_actors: std::ptr::null_mut(),
            u_editor_actor_subsystem_clear_actor_selection_set: std::ptr::null_mut(),
            u_editor_asset_subsystem_sort_by_meta_data: std::ptr::null_mut(),
            u_editor_asset_subsystem_set_metadata_tag: std::ptr::null_mut(),
            u_editor_asset_subsystem_set_dirty_flag: std::ptr::null_mut(),
            u_editor_asset_subsystem_save_loaded_assets: std::ptr::null_mut(),
            u_editor_asset_subsystem_save_loaded_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_save_directory: std::ptr::null_mut(),
            u_editor_asset_subsystem_save_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_rename_loaded_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_rename_directory: std::ptr::null_mut(),
            u_editor_asset_subsystem_rename_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_remove_on_extract_asset_from_file: std::ptr::null_mut(),
            u_editor_asset_subsystem_remove_metadata_tag: std::ptr::null_mut(),
            u_editor_asset_subsystem_on_extract_asset_from_file_dynamic_delegate_signature: std::ptr::null_mut(),
            u_editor_asset_subsystem_make_directory: std::ptr::null_mut(),
            u_editor_asset_subsystem_load_blueprint_class: std::ptr::null_mut(),
            u_editor_asset_subsystem_load_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_list_assets_by_tag_value: std::ptr::null_mut(),
            u_editor_asset_subsystem_list_assets: std::ptr::null_mut(),
            u_editor_asset_subsystem_get_tag_values: std::ptr::null_mut(),
            u_editor_asset_subsystem_get_path_name_for_loaded_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_get_metadata_tag_values: std::ptr::null_mut(),
            u_editor_asset_subsystem_get_metadata_tag: std::ptr::null_mut(),
            u_editor_asset_subsystem_get_loaded_asset_filename_length_for_cooking: std::ptr::null_mut(),
            u_editor_asset_subsystem_get_asset_filename_length_for_cooking: std::ptr::null_mut(),
            u_editor_asset_subsystem_get_all_assets_by_meta_data_tags: std::ptr::null_mut(),
            u_editor_asset_subsystem_find_package_referencers_for_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_find_asset_data: std::ptr::null_mut(),
            u_editor_asset_subsystem_duplicate_loaded_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_duplicate_directory: std::ptr::null_mut(),
            u_editor_asset_subsystem_duplicate_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_does_directory_exist: std::ptr::null_mut(),
            u_editor_asset_subsystem_does_directory_contain_assets: std::ptr::null_mut(),
            u_editor_asset_subsystem_does_asset_exist: std::ptr::null_mut(),
            u_editor_asset_subsystem_do_assets_exist: std::ptr::null_mut(),
            u_editor_asset_subsystem_delete_loaded_assets: std::ptr::null_mut(),
            u_editor_asset_subsystem_delete_loaded_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_delete_directory: std::ptr::null_mut(),
            u_editor_asset_subsystem_delete_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_consolidate_assets: std::ptr::null_mut(),
            u_editor_asset_subsystem_checkout_loaded_assets: std::ptr::null_mut(),
            u_editor_asset_subsystem_checkout_loaded_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_checkout_directory: std::ptr::null_mut(),
            u_editor_asset_subsystem_checkout_asset: std::ptr::null_mut(),
            u_editor_asset_subsystem_add_on_extract_asset_from_file: std::ptr::null_mut(),
            u_editor_subsystem_blueprint_library_toggle_preview_platform: std::ptr::null_mut(),
            u_editor_subsystem_blueprint_library_set_preview_platform: std::ptr::null_mut(),
            u_editor_subsystem_blueprint_library_get_preview_platform_options: std::ptr::null_mut(),
            u_editor_subsystem_blueprint_library_get_editor_subsystem: std::ptr::null_mut(),
            u_editor_subsystem_blueprint_library_disable_preview_platform: std::ptr::null_mut(),
            u_import_subsystem_on_asset_reimport_dyn_delegate_signature: std::ptr::null_mut(),
            u_import_subsystem_on_asset_pre_import_dyn_delegate_signature: std::ptr::null_mut(),
            u_import_subsystem_on_asset_post_lod_import_dyn_delegate_signature: std::ptr::null_mut(),
            u_import_subsystem_on_asset_post_import_dyn_delegate_signature: std::ptr::null_mut(),
            u_unreal_editor_subsystem_set_level_viewport_camera_info: std::ptr::null_mut(),
            u_unreal_editor_subsystem_get_level_viewport_camera_info: std::ptr::null_mut(),
            u_unreal_editor_subsystem_get_game_world: std::ptr::null_mut(),
            u_unreal_editor_subsystem_get_editor_world: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFactory::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptFactoryCreateFile"),
            &raw mut __FUNCTION_PTRS.u_factory_script_factory_create_file,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptFactoryCanImport"),
            &raw mut __FUNCTION_PTRS.u_factory_script_factory_can_import,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetEditorToolkitMenuContext::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditingObjects"),
            &raw mut __FUNCTION_PTRS
                .u_asset_editor_toolkit_menu_context_get_editing_objects,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UFbxImportUI::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetToDefault"),
            &raw mut __FUNCTION_PTRS.u_fbx_import_ui_reset_to_default,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUDIMTextureFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeUDIMVirtualTextureFromTexture2Ds"),
            &raw mut __FUNCTION_PTRS
                .uudim_texture_function_library_make_udim_virtual_texture_from_texture2_ds,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UActorGroupingUtils::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnlockSelectedGroups"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_unlock_selected_groups,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UngroupSelected"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_ungroup_selected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UngroupActors"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_ungroup_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGroupingActive"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_set_grouping_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSelectedFromGroup"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_remove_selected_from_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LockSelectedGroups"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_lock_selected_groups,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsGroupingActive"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_is_grouping_active,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GroupSelected"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_group_selected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GroupActors"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_group_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Get"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_get,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanGroupSelectedActors"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_can_group_selected_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CanGroupActors"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_can_group_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSelectedToGroup"),
            &raw mut __FUNCTION_PTRS.u_actor_grouping_utils_add_selected_to_group,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetImportTask::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAsyncImportComplete"),
            &raw mut __FUNCTION_PTRS.u_asset_import_task_is_async_import_complete,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjects"),
            &raw mut __FUNCTION_PTRS.u_asset_import_task_get_objects,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCookFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CookAsset"),
            &raw mut __FUNCTION_PTRS.u_cook_function_library_cook_asset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorLevelUtils::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLevelVisibility"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_set_level_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLevelsVisibility"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_set_levels_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveSelectedActorsToLevel"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_move_selected_actors_to_level,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveActorsToLevel"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_move_actors_to_level,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeLevelCurrent"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_make_level_current,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_RemoveLevelFromWorld"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_k2_remove_level_from_world,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_AddLevelToWorldWithTransform"),
            &raw mut __FUNCTION_PTRS
                .u_editor_level_utils_k2_add_level_to_world_with_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("K2_AddLevelToWorld"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_k2_add_level_to_world,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevels"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_get_levels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNewStreamingLevel"),
            &raw mut __FUNCTION_PTRS.u_editor_level_utils_create_new_streaming_level,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMaterialImportHelpers::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindExistingMaterialFromSearchLocation"),
            &raw mut __FUNCTION_PTRS
                .u_material_import_helpers_find_existing_material_from_search_location,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindExistingMaterial"),
            &raw mut __FUNCTION_PTRS.u_material_import_helpers_find_existing_material,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UReimportFbxSceneFactory::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ScriptReimportHelper"),
            &raw mut __FUNCTION_PTRS.u_reimport_fbx_scene_factory_script_reimport_helper,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorLoadingAndSavingUtils::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnloadPackages"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_unload_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SavePackagesWithDialog"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_save_packages_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SavePackages"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_save_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveMap"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_save_map,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveDirtyPackagesWithDialog"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_save_dirty_packages_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveDirtyPackages"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_save_dirty_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveCurrentLevel"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_save_current_level,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReloadPackages"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_reload_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewMapFromTemplate"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_new_map_from_template,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NewBlankMap"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_new_blank_map,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadMapWithDialog"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_load_map_with_dialog,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadMap"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_load_map,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ImportScene"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_import_scene,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDirtyMapPackages"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_get_dirty_map_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDirtyContentPackages"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_get_dirty_content_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FullyLoadPackages"),
            &raw mut __FUNCTION_PTRS
                .u_editor_loading_and_saving_utils_fully_load_packages,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FullyLoadAssets"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_fully_load_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExportScene"),
            &raw mut __FUNCTION_PTRS.u_editor_loading_and_saving_utils_export_scene,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULayersSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateAllViewVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_update_all_view_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateAllActorsVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_update_all_actors_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateActorVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_update_actor_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UpdateActorAllViewsVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_update_actor_all_views_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TryGetLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_try_get_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToggleLayerVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_toggle_layer_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToggleLayersVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_toggle_layers_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLayerVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_set_layer_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLayersVisibility"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_set_layers_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectActorsInLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_select_actors_in_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectActorsInLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_select_actors_in_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_rename_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSelectedActorsFromLayers"),
            &raw mut __FUNCTION_PTRS
                .u_layers_subsystem_remove_selected_actors_from_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSelectedActorsFromLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_remove_selected_actors_from_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveLevelLayerInformation"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_remove_level_layer_information,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorsFromLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_remove_actors_from_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorsFromLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_remove_actors_from_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorFromLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_remove_actor_from_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveActorFromLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_remove_actor_from_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeAllLayersVisible"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_make_all_layers_visible,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_is_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsActorValidForLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_is_actor_valid_for_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitializeNewActorLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_initialize_new_actor_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorld"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_get_world,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedActors"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_get_selected_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_get_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorsFromLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_get_actors_from_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorsFromLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_get_actors_from_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorRefreshLayerBrowser"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_editor_refresh_layer_browser,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EditorMapChange"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_editor_map_change,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisassociateActorsFromLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_disassociate_actors_from_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisassociateActorFromLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_disassociate_actor_from_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_delete_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_delete_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_create_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AppendActorsFromLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_append_actors_from_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AppendActorsFromLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_append_actors_from_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSelectedActorsToLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_selected_actors_to_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSelectedActorsToLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_selected_actors_to_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLevelLayerInformation"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_level_layer_information,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAllLayersTo"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_all_layers_to,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAllLayerNamesTo"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_all_layer_names_to,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorToLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_actor_to_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorToLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_actor_to_layer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorsToLayers"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_actors_to_layers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddActorsToLayer"),
            &raw mut __FUNCTION_PTRS.u_layers_subsystem_add_actors_to_layer,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPackageTools::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SanitizePackageName"),
            &raw mut __FUNCTION_PTRS.u_package_tools_sanitize_package_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PackageNameToFilename"),
            &raw mut __FUNCTION_PTRS.u_package_tools_package_name_to_filename,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FilenameToPackageName"),
            &raw mut __FUNCTION_PTRS.u_package_tools_filename_to_package_name,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = APropertyEditorTestActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOptionsFunc"),
            &raw mut __FUNCTION_PTRS.a_property_editor_test_actor_get_options_func,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetEditorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OpenEditorForAssets"),
            &raw mut __FUNCTION_PTRS.u_asset_editor_subsystem_open_editor_for_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CloseAllEditorsForAsset"),
            &raw mut __FUNCTION_PTRS.u_asset_editor_subsystem_close_all_editors_for_asset,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UCollectionManagerScriptingSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReparentCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_reparent_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_rename_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetsFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_remove_assets_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetPtrsFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_remove_asset_ptrs_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetPtrFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_remove_asset_ptr_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_remove_asset_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetDatasFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_remove_asset_datas_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssetDataFromCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_remove_asset_data_from_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAssetPtr"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_collections_containing_asset_ptr,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAssetData"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_collections_containing_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsContainingAsset"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_collections_containing_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionsByName"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_collections_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollections"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_collections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCollectionContainers"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_collection_containers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBaseGameCollectionContainer"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_base_game_collection_container,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetsInCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_get_assets_in_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EmptyCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_empty_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DestroyCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_destroy_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateOrEmptyCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_create_or_empty_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_create_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CollectionExists"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_collection_exists,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetToCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_add_asset_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetsToCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_add_assets_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetPtrToCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_add_asset_ptr_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetPtrsToCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_add_asset_ptrs_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetDataToCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_add_asset_data_to_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssetDatasToCollection"),
            &raw mut __FUNCTION_PTRS
                .u_collection_manager_scripting_subsystem_add_asset_datas_to_collection,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorActorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnActorFromObject"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_spawn_actor_from_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnActorFromClass"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_spawn_actor_from_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSelectedLevelActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_set_selected_level_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetComponentTransform"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_set_component_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActorTransform"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_set_actor_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActorSelectionState"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_set_actor_selection_state,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectNothing"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_select_nothing,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectAllChildren"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_select_all_children,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SelectAll"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_select_all,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InvertSelection"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_invert_selection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSelectedLevelActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_get_selected_level_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllLevelActorsComponents"),
            &raw mut __FUNCTION_PTRS
                .u_editor_actor_subsystem_get_all_level_actors_components,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllLevelActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_get_all_level_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetActorReference"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_get_actor_reference,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateSelectedActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_duplicate_selected_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_duplicate_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateActor"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_duplicate_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DestroyActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_destroy_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DestroyActor"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_destroy_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteSelectedActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_delete_selected_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConvertActors"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_convert_actors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearActorSelectionSet"),
            &raw mut __FUNCTION_PTRS.u_editor_actor_subsystem_clear_actor_selection_set,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorAssetSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortByMetaData"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_sort_by_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMetadataTag"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_set_metadata_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDirtyFlag"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_set_dirty_flag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveLoadedAssets"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_save_loaded_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveLoadedAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_save_loaded_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveDirectory"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_save_directory,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SaveAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_save_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameLoadedAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_rename_loaded_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameDirectory"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_rename_directory,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_rename_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveOnExtractAssetFromFile"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_remove_on_extract_asset_from_file,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMetadataTag"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_remove_metadata_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from(
                "OnExtractAssetFromFileDynamic__DelegateSignature",
            ),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_on_extract_asset_from_file_dynamic_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeDirectory"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_make_directory,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadBlueprintClass"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_load_blueprint_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_load_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ListAssetsByTagValue"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_list_assets_by_tag_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ListAssets"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_list_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTagValues"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_get_tag_values,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPathNameForLoadedAsset"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_get_path_name_for_loaded_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetadataTagValues"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_get_metadata_tag_values,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetadataTag"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_get_metadata_tag,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLoadedAssetFilenameLengthForCooking"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_get_loaded_asset_filename_length_for_cooking,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssetFilenameLengthForCooking"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_get_asset_filename_length_for_cooking,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllAssetsByMetaDataTags"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_get_all_assets_by_meta_data_tags,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindPackageReferencersForAsset"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_find_package_referencers_for_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindAssetData"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_find_asset_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateLoadedAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_duplicate_loaded_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateDirectory"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_duplicate_directory,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DuplicateAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_duplicate_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesDirectoryExist"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_does_directory_exist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesDirectoryContainAssets"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_does_directory_contain_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesAssetExist"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_does_asset_exist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoAssetsExist"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_do_assets_exist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteLoadedAssets"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_delete_loaded_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteLoadedAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_delete_loaded_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteDirectory"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_delete_directory,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_delete_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConsolidateAssets"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_consolidate_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutLoadedAssets"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_checkout_loaded_assets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutLoadedAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_checkout_loaded_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutDirectory"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_checkout_directory,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckoutAsset"),
            &raw mut __FUNCTION_PTRS.u_editor_asset_subsystem_checkout_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddOnExtractAssetFromFile"),
            &raw mut __FUNCTION_PTRS
                .u_editor_asset_subsystem_add_on_extract_asset_from_file,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UEditorSubsystemBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("TogglePreviewPlatform"),
            &raw mut __FUNCTION_PTRS
                .u_editor_subsystem_blueprint_library_toggle_preview_platform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreviewPlatform"),
            &raw mut __FUNCTION_PTRS
                .u_editor_subsystem_blueprint_library_set_preview_platform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewPlatformOptions"),
            &raw mut __FUNCTION_PTRS
                .u_editor_subsystem_blueprint_library_get_preview_platform_options,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorSubsystem"),
            &raw mut __FUNCTION_PTRS
                .u_editor_subsystem_blueprint_library_get_editor_subsystem,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisablePreviewPlatform"),
            &raw mut __FUNCTION_PTRS
                .u_editor_subsystem_blueprint_library_disable_preview_platform,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UImportSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAssetReimport_Dyn__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_import_subsystem_on_asset_reimport_dyn_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAssetPreImport_Dyn__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_import_subsystem_on_asset_pre_import_dyn_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAssetPostLODImport_Dyn__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_import_subsystem_on_asset_post_lod_import_dyn_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnAssetPostImport_Dyn__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_import_subsystem_on_asset_post_import_dyn_delegate_signature,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UUnrealEditorSubsystem::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLevelViewportCameraInfo"),
            &raw mut __FUNCTION_PTRS
                .u_unreal_editor_subsystem_set_level_viewport_camera_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLevelViewportCameraInfo"),
            &raw mut __FUNCTION_PTRS
                .u_unreal_editor_subsystem_get_level_viewport_camera_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGameWorld"),
            &raw mut __FUNCTION_PTRS.u_unreal_editor_subsystem_get_game_world,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEditorWorld"),
            &raw mut __FUNCTION_PTRS.u_unreal_editor_subsystem_get_editor_world,
        );
    }
}
#[repr(C, align(8))]
pub struct FCSVImportSettings {
    pub import_row_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    pub import_type: ECSVImportType,
    pub import_curve_interp_mode: crate::bindings::engine::ERichCurveInterpMode,
    pub(crate) __padding_end: [u8; 22],
}
impl FCSVImportSettings {}
#[repr(C, align(4))]
pub struct FFbxMaterialBakeSize {
    pub size: crate::bindings::core_u_object::FIntPoint,
    pub b_auto_detect: bool,
}
impl FFbxMaterialBakeSize {}
#[repr(C, align(8))]
pub struct FImportMeshLodSectionsData {
    pub(crate) __padding_end: [u8; 16],
}
impl FImportMeshLodSectionsData {}
#[repr(C, align(8))]
pub struct FCollectionScriptingContainerSource {
    pub name: FName,
    pub title: FText,
}
impl FCollectionScriptingContainerSource {}
#[repr(C, align(4))]
pub struct FCollectionScriptingRef {
    pub container: FName,
    pub name: FName,
    pub share_type: crate::bindings::engine::ECollectionScriptingShareType,
}
impl FCollectionScriptingRef {}
#[repr(C, align(8))]
pub struct UAssetDefinitionDefault {
    __padding_end: [u8; 72],
}
impl UAssetDefinitionDefault {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetDefinitionDefault")
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
pub struct UFactory {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub flags_48: u8,
    pub supported_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub context_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
    pub formats: TArray<FString>,
    pub flags_88: u8,
    pub automated_import_data: UPtr<UAutomatedAssetImportData>,
    pub asset_import_task: UPtr<UAssetImportTask>,
    pub supported_workflows: u8,
    __padding_end: [u8; 23],
}
impl UFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UFactory").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn script_factory_create_file(
        &mut self,
        in_task: UPtr<UAssetImportTask>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_factory_script_factory_create_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_task,
                __buffer.add(0).cast::<UPtr<UAssetImportTask>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_factory_script_factory_create_file,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn script_factory_can_import(&mut self, filename: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_factory_script_factory_can_import,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_factory_script_factory_can_import,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditorWorldExtension {
    __padding_end: [u8; 80],
}
impl UEditorWorldExtension {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorWorldExtension")
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
pub struct UActorFactory {
    __padding_end: [u8; 144],
}
impl UActorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactory")
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
pub struct UActorFactoryVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryVolume")
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
pub struct UActorFactoryBoxVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryBoxVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryBoxVolume")
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
pub struct UThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UThumbnailRenderer")
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
pub struct UDefaultSizedThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl UDefaultSizedThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDefaultSizedThumbnailRenderer")
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
pub struct UBlueprintThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UBlueprintThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintThumbnailRenderer")
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
pub struct UEditorState {
    __padding_end: [u8; 48],
}
impl UEditorState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorState")
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
pub struct UWorldDependantEditorState {
    __padding_end: [u8; 48],
}
impl UWorldDependantEditorState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldDependantEditorState")
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
pub struct UActorEditorContextStateCollection {
    __padding_end: [u8; 128],
}
impl UActorEditorContextStateCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorEditorContextStateCollection")
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
pub struct UActorEditorContextClientState {
    __padding_end: [u8; 48],
}
impl UActorEditorContextClientState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorEditorContextClientState")
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
pub struct UBaseWidgetBlueprint {
    __padding_end: [u8; 1440],
}
impl UBaseWidgetBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseWidgetBlueprint")
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
pub struct UEditorInteractiveToolsContext {
    __padding_end: [u8; 944],
}
impl UEditorInteractiveToolsContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorInteractiveToolsContext")
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
pub struct UModeManagerInteractiveToolsContext {
    __padding_end: [u8; 1296],
}
impl UModeManagerInteractiveToolsContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModeManagerInteractiveToolsContext")
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
pub struct UActorExporterT3D {
    __padding_end: [u8; 128],
}
impl UActorExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorExporterT3D")
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
pub struct UGroupActorExporterT3D {
    __padding_end: [u8; 128],
}
impl UGroupActorExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGroupActorExporterT3D")
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
pub struct UPhysicsVolumeExporterT3D {
    __padding_end: [u8; 128],
}
impl UPhysicsVolumeExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsVolumeExporterT3D")
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
pub struct UActorFactoryAmbientSound {
    __padding_end: [u8; 144],
}
impl UActorFactoryAmbientSound {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryAmbientSound")
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
pub struct UActorFactorySkeletalMesh {
    __padding_end: [u8; 152],
}
impl UActorFactorySkeletalMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactorySkeletalMesh")
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
pub struct UActorFactoryAnimationAsset {
    __padding_end: [u8; 152],
}
impl UActorFactoryAnimationAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryAnimationAsset")
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
pub struct UActorFactoryStaticMesh {
    __padding_end: [u8; 144],
}
impl UActorFactoryStaticMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryStaticMesh")
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
pub struct UActorFactoryBasicShape {
    __padding_end: [u8; 144],
}
impl UActorFactoryBasicShape {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryBasicShape")
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
pub struct UActorFactoryBlueprint {
    __padding_end: [u8; 144],
}
impl UActorFactoryBlueprint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryBlueprint")
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
pub struct UActorFactoryBoxReflectionCapture {
    __padding_end: [u8; 144],
}
impl UActorFactoryBoxReflectionCapture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryBoxReflectionCapture")
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
pub struct UActorFactoryCameraActor {
    __padding_end: [u8; 144],
}
impl UActorFactoryCameraActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryCameraActor")
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
pub struct UActorFactoryCharacter {
    __padding_end: [u8; 144],
}
impl UActorFactoryCharacter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryCharacter")
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
pub struct UActorFactoryClass {
    __padding_end: [u8; 144],
}
impl UActorFactoryClass {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryClass")
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
pub struct UActorFactoryCylinderVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryCylinderVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryCylinderVolume")
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
pub struct UActorFactoryDeferredDecal {
    __padding_end: [u8; 144],
}
impl UActorFactoryDeferredDecal {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryDeferredDecal")
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
pub struct UActorFactoryDirectionalLight {
    __padding_end: [u8; 144],
}
impl UActorFactoryDirectionalLight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryDirectionalLight")
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
pub struct UActorFactoryEmitter {
    __padding_end: [u8; 144],
}
impl UActorFactoryEmitter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryEmitter")
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
pub struct UActorFactoryEmptyActor {
    __padding_end: [u8; 152],
}
impl UActorFactoryEmptyActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryEmptyActor")
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
pub struct UActorFactoryExponentialHeightFog {
    __padding_end: [u8; 144],
}
impl UActorFactoryExponentialHeightFog {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryExponentialHeightFog")
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
pub struct UActorFactoryInteractiveFoliage {
    __padding_end: [u8; 144],
}
impl UActorFactoryInteractiveFoliage {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryInteractiveFoliage")
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
pub struct UActorFactoryLevelSequence {
    __padding_end: [u8; 144],
}
impl UActorFactoryLevelSequence {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryLevelSequence")
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
pub struct UActorFactoryLocalFogVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryLocalFogVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryLocalFogVolume")
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
pub struct UActorFactoryNote {
    __padding_end: [u8; 144],
}
impl UActorFactoryNote {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryNote")
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
pub struct UActorFactoryPawn {
    __padding_end: [u8; 152],
}
impl UActorFactoryPawn {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryPawn")
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
pub struct UActorFactoryPhysicsAsset {
    __padding_end: [u8; 144],
}
impl UActorFactoryPhysicsAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryPhysicsAsset")
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
pub struct UActorFactoryPlanarReflection {
    __padding_end: [u8; 144],
}
impl UActorFactoryPlanarReflection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryPlanarReflection")
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
pub struct UActorFactoryPlaneReflectionCapture {
    __padding_end: [u8; 144],
}
impl UActorFactoryPlaneReflectionCapture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryPlaneReflectionCapture")
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
pub struct UActorFactoryPlayerStart {
    __padding_end: [u8; 144],
}
impl UActorFactoryPlayerStart {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryPlayerStart")
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
pub struct UActorFactoryRuntimeVirtualTextureVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryRuntimeVirtualTextureVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryRuntimeVirtualTextureVolume")
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
pub struct UActorFactorySkyAtmosphere {
    __padding_end: [u8; 144],
}
impl UActorFactorySkyAtmosphere {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactorySkyAtmosphere")
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
pub struct UActorFactorySkyLight {
    __padding_end: [u8; 144],
}
impl UActorFactorySkyLight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactorySkyLight")
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
pub struct UActorFactorySphereReflectionCapture {
    __padding_end: [u8; 144],
}
impl UActorFactorySphereReflectionCapture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactorySphereReflectionCapture")
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
pub struct UActorFactorySphereVolume {
    __padding_end: [u8; 144],
}
impl UActorFactorySphereVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactorySphereVolume")
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
pub struct UActorFactoryTargetPoint {
    __padding_end: [u8; 144],
}
impl UActorFactoryTargetPoint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryTargetPoint")
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
pub struct UActorFactoryTextRender {
    __padding_end: [u8; 144],
}
impl UActorFactoryTextRender {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryTextRender")
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
pub struct UActorFactoryTriggerBox {
    __padding_end: [u8; 144],
}
impl UActorFactoryTriggerBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryTriggerBox")
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
pub struct UActorFactoryTriggerCapsule {
    __padding_end: [u8; 144],
}
impl UActorFactoryTriggerCapsule {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryTriggerCapsule")
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
pub struct UActorFactoryTriggerSphere {
    __padding_end: [u8; 144],
}
impl UActorFactoryTriggerSphere {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryTriggerSphere")
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
pub struct UActorFactoryVectorFieldVolume {
    __padding_end: [u8; 144],
}
impl UActorFactoryVectorFieldVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryVectorFieldVolume")
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
pub struct UActorFactoryVolumetricCloud {
    __padding_end: [u8; 144],
}
impl UActorFactoryVolumetricCloud {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryVolumetricCloud")
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
pub struct UBlendSpaceFactory1D {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UBlendSpaceFactory1D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpaceFactory1D")
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
pub struct UAimOffsetBlendSpaceFactory1D {
    __padding_end: [u8; 168],
}
impl UAimOffsetBlendSpaceFactory1D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAimOffsetBlendSpaceFactory1D")
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
pub struct UBlendSpaceFactoryNew {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UBlendSpaceFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpaceFactoryNew")
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
pub struct UAimOffsetBlendSpaceFactoryNew {
    __padding_end: [u8; 168],
}
impl UAimOffsetBlendSpaceFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAimOffsetBlendSpaceFactoryNew")
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
pub struct UAnimationBlueprintEditorOptions {
    __padding_end: [u8; 56],
}
impl UAnimationBlueprintEditorOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationBlueprintEditorOptions")
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
pub struct UAnimBlueprintSettings {
    __padding_end: [u8; 72],
}
impl UAnimBlueprintSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBlueprintSettings")
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
pub struct UExporterFBX {
    __padding_end: [u8; 128],
}
impl UExporterFBX {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExporterFBX")
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
pub struct UAnimSequenceExporterFBX {
    __padding_end: [u8; 128],
}
impl UAnimSequenceExporterFBX {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequenceExporterFBX")
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
pub struct UAssetEditorToolkitMenuContext {
    __padding_end: [u8; 64],
}
impl UAssetEditorToolkitMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetEditorToolkitMenuContext")
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
    pub fn get_editing_objects(
        &self,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_editor_toolkit_menu_context_get_editing_objects,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_editor_toolkit_menu_context_get_editing_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UBlueprintFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintFactory")
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
pub struct UBlueprintFunctionLibraryFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintFunctionLibraryFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintFunctionLibraryFactory")
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
pub struct UBlueprintInterfaceFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintInterfaceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintInterfaceFactory")
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
pub struct UBlueprintMacroFactory {
    __padding_end: [u8; 176],
}
impl UBlueprintMacroFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintMacroFactory")
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
pub struct UCanvasRenderTarget2DFactoryNew {
    __padding_end: [u8; 152],
}
impl UCanvasRenderTarget2DFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCanvasRenderTarget2DFactoryNew")
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
pub struct UCascadeOptions {
    __padding_end: [u8; 296],
}
impl UCascadeOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCascadeOptions")
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
pub struct UClassViewerSettings {
    __padding_end: [u8; 72],
}
impl UClassViewerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClassViewerSettings")
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
pub struct UCompressAnimationsCommandlet {
    __padding_end: [u8; 136],
}
impl UCompressAnimationsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompressAnimationsCommandlet")
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
pub struct UEditorBrushBuilder {
    __padding_end: [u8; 136],
}
impl UEditorBrushBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorBrushBuilder")
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
pub struct UConeBuilder {
    __padding_end: [u8; 176],
}
impl UConeBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConeBuilder")
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
pub struct UContentBrowserSettings {
    __padding_end: [u8; 80],
}
impl UContentBrowserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UContentBrowserSettings")
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
pub struct UCookGlobalShadersCommandlet {
    __padding_end: [u8; 224],
}
impl UCookGlobalShadersCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookGlobalShadersCommandlet")
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
pub struct UCookGlobalShadersDeviceHelperBase {
    __padding_end: [u8; 48],
}
impl UCookGlobalShadersDeviceHelperBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookGlobalShadersDeviceHelperBase")
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
pub struct UCookGlobalShadersDeviceHelperStaged {
    __padding_end: [u8; 64],
}
impl UCookGlobalShadersDeviceHelperStaged {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookGlobalShadersDeviceHelperStaged")
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
pub struct UCrashReporterSettings {
    __padding_end: [u8; 96],
}
impl UCrashReporterSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrashReporterSettings")
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
pub struct UCubeBuilder {
    __padding_end: [u8; 168],
}
impl UCubeBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCubeBuilder")
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
pub struct UCurvedStairBuilder {
    __padding_end: [u8; 176],
}
impl UCurvedStairBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurvedStairBuilder")
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
pub struct UCurveEdOptions {
    __padding_end: [u8; 168],
}
impl UCurveEdOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveEdOptions")
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
pub struct UCurveFactory {
    __padding_end: [u8; 144],
}
impl UCurveFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveFactory")
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
pub struct UCurveFloatFactory {
    __padding_end: [u8; 144],
}
impl UCurveFloatFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveFloatFactory")
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
pub struct UCurveLinearColorFactory {
    __padding_end: [u8; 144],
}
impl UCurveLinearColorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveLinearColorFactory")
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
pub struct UCurveVectorFactory {
    __padding_end: [u8; 144],
}
impl UCurveVectorFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveVectorFactory")
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
pub struct UCurveImportFactory {
    __padding_end: [u8; 136],
}
impl UCurveImportFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveImportFactory")
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
pub struct UCurveLinearColorAtlasFactory {
    __padding_end: [u8; 144],
}
impl UCurveLinearColorAtlasFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveLinearColorAtlasFactory")
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
pub struct UCylinderBuilder {
    __padding_end: [u8; 168],
}
impl UCylinderBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCylinderBuilder")
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
pub struct UDataAssetFactory {
    __padding_end: [u8; 144],
}
impl UDataAssetFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataAssetFactory")
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
pub struct UDEditorParameterValue {
    __padding_end: [u8; 128],
}
impl UDEditorParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorParameterValue")
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
pub struct UDEditorDoubleVectorParameterValue {
    __padding_end: [u8; 160],
}
impl UDEditorDoubleVectorParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorDoubleVectorParameterValue")
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
pub struct UDEditorFontParameterValue {
    __padding_end: [u8; 144],
}
impl UDEditorFontParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorFontParameterValue")
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
pub struct UDEditorMaterialLayersParameterValue {
    __padding_end: [u8; 408],
}
impl UDEditorMaterialLayersParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorMaterialLayersParameterValue")
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
pub struct UDEditorParameterCollectionParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorParameterCollectionParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorParameterCollectionParameterValue")
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
pub struct UDEditorRuntimeVirtualTextureParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorRuntimeVirtualTextureParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorRuntimeVirtualTextureParameterValue")
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
pub struct UDEditorScalarParameterValue {
    __padding_end: [u8; 304],
}
impl UDEditorScalarParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorScalarParameterValue")
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
pub struct UDEditorSparseVolumeTextureParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorSparseVolumeTextureParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorSparseVolumeTextureParameterValue")
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
pub struct UDEditorStaticComponentMaskParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorStaticComponentMaskParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorStaticComponentMaskParameterValue")
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
pub struct UDEditorStaticSwitchParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorStaticSwitchParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorStaticSwitchParameterValue")
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
pub struct UDEditorTextureCollectionParameterValue {
    __padding_end: [u8; 136],
}
impl UDEditorTextureCollectionParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorTextureCollectionParameterValue")
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
pub struct UDEditorTextureParameterValue {
    __padding_end: [u8; 200],
}
impl UDEditorTextureParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorTextureParameterValue")
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
pub struct UDEditorVectorParameterValue {
    __padding_end: [u8; 216],
}
impl UDEditorVectorParameterValue {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEditorVectorParameterValue")
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
pub struct UEditorExperimentalSettings {
    __padding_end: [u8; 120],
}
impl UEditorExperimentalSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorExperimentalSettings")
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
pub struct UEditorLoadingSavingSettings {
    __padding_end: [u8; 200],
}
impl UEditorLoadingSavingSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorLoadingSavingSettings")
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
pub struct UEditorMiscSettings {
    __padding_end: [u8; 48],
}
impl UEditorMiscSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorMiscSettings")
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
pub struct UEditorStyleSettings {
    __padding_end: [u8; 624],
}
impl UEditorStyleSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorStyleSettings")
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
pub struct UEnumFactory {
    __padding_end: [u8; 136],
}
impl UEnumFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEnumFactory")
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
pub struct UExportTextContainer {
    __padding_end: [u8; 64],
}
impl UExportTextContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExportTextContainer")
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
pub struct UFbxImportUI {
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 56],
    pub b_is_obj_import: bool,
    pub original_import_type: EFBXImportType,
    pub mesh_type_to_import: EFBXImportType,
    #[doc(hidden)]
    pub(crate) __padding_60: [u8; 1],
    pub flags_60: u8,
    #[doc(hidden)]
    pub(crate) __padding_64: [u8; 3],
    pub b_import_as_skeletal: bool,
    pub b_import_mesh: bool,
    pub skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub flags_80: u8,
    pub physics_asset: UPtr<crate::bindings::engine::UPhysicsAsset>,
    pub flags_96: u8,
    pub lod_distance0: f32,
    pub lod_distance1: f32,
    pub lod_distance2: f32,
    pub lod_distance3: f32,
    pub lod_distance4: f32,
    pub lod_distance5: f32,
    pub lod_distance6: f32,
    pub lod_distance7: f32,
    pub minimum_lod_number: i32,
    pub lod_number: i32,
    pub flags_140: u8,
    pub override_animation_name: FString,
    pub flags_160: u8,
    pub static_mesh_import_data: UPtr<UFbxStaticMeshImportData>,
    pub skeletal_mesh_import_data: UPtr<UFbxSkeletalMeshImportData>,
    pub anim_sequence_import_data: UPtr<UFbxAnimSequenceImportData>,
    pub texture_import_data: UPtr<UFbxTextureImportData>,
    pub b_automated_import_should_detect_type: bool,
    __padding_end: [u8; 335],
}
impl UFbxImportUI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxImportUI")
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
    pub fn reset_to_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_fbx_import_ui_reset_to_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_fbx_import_ui_reset_to_default,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UFontFactory {
    __padding_end: [u8; 136],
}
impl UFontFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFontFactory")
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
pub struct UFontFileImportFactory {
    __padding_end: [u8; 176],
}
impl UFontFileImportFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFontFileImportFactory")
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
pub struct UForceFeedbackAttenuationFactory {
    __padding_end: [u8; 136],
}
impl UForceFeedbackAttenuationFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UForceFeedbackAttenuationFactory")
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
pub struct UForceFeedbackEffectFactory {
    __padding_end: [u8; 136],
}
impl UForceFeedbackEffectFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UForceFeedbackEffectFactory")
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
pub struct UHapticFeedbackEffectBufferFactory {
    __padding_end: [u8; 136],
}
impl UHapticFeedbackEffectBufferFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHapticFeedbackEffectBufferFactory")
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
pub struct UHapticFeedbackEffectCurveFactory {
    __padding_end: [u8; 136],
}
impl UHapticFeedbackEffectCurveFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHapticFeedbackEffectCurveFactory")
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
pub struct UHapticFeedbackEffectSoundWaveFactory {
    __padding_end: [u8; 136],
}
impl UHapticFeedbackEffectSoundWaveFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHapticFeedbackEffectSoundWaveFactory")
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
pub struct ULandscapeGrassTypeCommandlet {
    __padding_end: [u8; 136],
}
impl ULandscapeGrassTypeCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULandscapeGrassTypeCommandlet")
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
pub struct ILegacyEdModeSelectInterface {}
#[repr(C, align(8))]
pub struct ULegacyEdModeSelectInterface {
    __padding_end: [u8; 48],
}
impl ULegacyEdModeSelectInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyEdModeSelectInterface")
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
pub struct ILegacyEdModeWidgetInterface {}
#[repr(C, align(8))]
pub struct ULegacyEdModeWidgetInterface {
    __padding_end: [u8; 48],
}
impl ULegacyEdModeWidgetInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyEdModeWidgetInterface")
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
pub struct ILegacyEdModeToolInterface {}
#[repr(C, align(8))]
pub struct ULegacyEdModeToolInterface {
    __padding_end: [u8; 48],
}
impl ULegacyEdModeToolInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyEdModeToolInterface")
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
pub struct ILegacyEdModeDrawHelperInterface {}
#[repr(C, align(8))]
pub struct ULegacyEdModeDrawHelperInterface {
    __padding_end: [u8; 48],
}
impl ULegacyEdModeDrawHelperInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyEdModeDrawHelperInterface")
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
pub struct ILegacyEdModeViewportInterface {}
#[repr(C, align(8))]
pub struct ULegacyEdModeViewportInterface {
    __padding_end: [u8; 48],
}
impl ULegacyEdModeViewportInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyEdModeViewportInterface")
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
pub struct ULevelEditorMiscSettings {
    __padding_end: [u8; 176],
}
impl ULevelEditorMiscSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorMiscSettings")
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
pub struct UCommonResolutionMenuContext {
    __padding_end: [u8; 72],
}
impl UCommonResolutionMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCommonResolutionMenuContext")
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
pub struct ULevelEditorPlaySettings {
    __padding_end: [u8; 576],
}
impl ULevelEditorPlaySettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorPlaySettings")
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
pub struct ULevelEditorViewportSettings {
    __padding_end: [u8; 664],
}
impl ULevelEditorViewportSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorViewportSettings")
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
pub struct ULevelExporterFBX {
    __padding_end: [u8; 128],
}
impl ULevelExporterFBX {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelExporterFBX")
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
pub struct ULevelExporterLOD {
    __padding_end: [u8; 128],
}
impl ULevelExporterLOD {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelExporterLOD")
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
pub struct ULevelExporterOBJ {
    __padding_end: [u8; 128],
}
impl ULevelExporterOBJ {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelExporterOBJ")
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
pub struct ULevelExporterSTL {
    __padding_end: [u8; 128],
}
impl ULevelExporterSTL {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelExporterSTL")
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
pub struct ULevelExporterT3D {
    __padding_end: [u8; 128],
}
impl ULevelExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelExporterT3D")
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
pub struct ULevelFactory {
    __padding_end: [u8; 136],
}
impl ULevelFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelFactory")
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
pub struct ULinearStairBuilder {
    __padding_end: [u8; 168],
}
impl ULinearStairBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULinearStairBuilder")
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
pub struct UListMaterialsUsedWithMeshEmittersCommandlet {
    __padding_end: [u8; 136],
}
impl UListMaterialsUsedWithMeshEmittersCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UListMaterialsUsedWithMeshEmittersCommandlet")
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
pub struct UListStaticMeshesImportedFromSpeedTreesCommandlet {
    __padding_end: [u8; 136],
}
impl UListStaticMeshesImportedFromSpeedTreesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UListStaticMeshesImportedFromSpeedTreesCommandlet")
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
pub struct ULoadPackageCommandlet {
    __padding_end: [u8; 136],
}
impl ULoadPackageCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULoadPackageCommandlet")
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
pub struct UMaterialEditorParameters {
    __padding_end: [u8; 80],
}
impl UMaterialEditorParameters {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditorParameters")
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
pub struct UMaterialEditorInstanceConstant {
    #[doc(hidden)]
    pub(crate) __padding_120: [u8; 120],
    pub subsurface_profile: UPtr<crate::bindings::engine::USubsurfaceProfile>,
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 8],
    pub specular_profile: UPtr<crate::bindings::engine::USpecularProfile>,
    __padding_end: [u8; 200],
}
impl UMaterialEditorInstanceConstant {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditorInstanceConstant")
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
pub struct UMaterialEditorOptions {
    __padding_end: [u8; 72],
}
impl UMaterialEditorOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditorOptions")
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
pub struct UMaterialEditorPreviewParameters {
    __padding_end: [u8; 120],
}
impl UMaterialEditorPreviewParameters {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditorPreviewParameters")
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
pub struct UMaterialFactoryNew {
    __padding_end: [u8; 144],
}
impl UMaterialFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFactoryNew")
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
pub struct UMaterialFunctionFactoryNew {
    __padding_end: [u8; 136],
}
impl UMaterialFunctionFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFunctionFactoryNew")
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
pub struct UMaterialFunctionInstanceFactory {
    __padding_end: [u8; 144],
}
impl UMaterialFunctionInstanceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFunctionInstanceFactory")
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
pub struct UMaterialFunctionMaterialLayerInstanceFactory {
    __padding_end: [u8; 144],
}
impl UMaterialFunctionMaterialLayerInstanceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFunctionMaterialLayerInstanceFactory")
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
pub struct UMaterialFunctionMaterialLayerBlendInstanceFactory {
    __padding_end: [u8; 144],
}
impl UMaterialFunctionMaterialLayerBlendInstanceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFunctionMaterialLayerBlendInstanceFactory")
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
pub struct UMaterialFunctionMaterialLayerBlendFactory {
    __padding_end: [u8; 136],
}
impl UMaterialFunctionMaterialLayerBlendFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFunctionMaterialLayerBlendFactory")
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
pub struct UMaterialFunctionMaterialLayerFactory {
    __padding_end: [u8; 136],
}
impl UMaterialFunctionMaterialLayerFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFunctionMaterialLayerFactory")
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
pub struct UMaterialInstanceConstantFactoryNew {
    __padding_end: [u8; 144],
}
impl UMaterialInstanceConstantFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialInstanceConstantFactoryNew")
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
pub struct UMaterialParameterCollectionFactoryNew {
    __padding_end: [u8; 136],
}
impl UMaterialParameterCollectionFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialParameterCollectionFactoryNew")
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
pub struct UMaterialStatsOptions {
    __padding_end: [u8; 688],
}
impl UMaterialStatsOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialStatsOptions")
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
pub struct UModelExporterT3D {
    __padding_end: [u8; 128],
}
impl UModelExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelExporterT3D")
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
pub struct UModelFactory {
    __padding_end: [u8; 136],
}
impl UModelFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UModelFactory")
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
pub struct UNeuralProfileFactory {
    __padding_end: [u8; 136],
}
impl UNeuralProfileFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNeuralProfileFactory")
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
pub struct UObjectExporterT3D {
    __padding_end: [u8; 128],
}
impl UObjectExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectExporterT3D")
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
pub struct UObjectLibraryFactory {
    __padding_end: [u8; 136],
}
impl UObjectLibraryFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectLibraryFactory")
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
pub struct UPackageFactory {
    __padding_end: [u8; 136],
}
impl UPackageFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPackageFactory")
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
pub struct UParticleSystemFactoryNew {
    __padding_end: [u8; 136],
}
impl UParticleSystemFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParticleSystemFactoryNew")
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
pub struct UPersonaOptions {
    __padding_end: [u8; 632],
}
impl UPersonaOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPersonaOptions")
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
pub struct UPhysicalMaterialFactoryNew {
    __padding_end: [u8; 144],
}
impl UPhysicalMaterialFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicalMaterialFactoryNew")
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
pub struct UPhysicalMaterialMaskFactory {
    __padding_end: [u8; 176],
}
impl UPhysicalMaterialMaskFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicalMaterialMaskFactory")
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
pub struct UPhysicsAssetEditorOptions {
    __padding_end: [u8; 144],
}
impl UPhysicsAssetEditorOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsAssetEditorOptions")
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
pub struct UPhysicsAssetGenerationSettings {
    __padding_end: [u8; 88],
}
impl UPhysicsAssetGenerationSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsAssetGenerationSettings")
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
pub struct UPkgInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UPkgInfoCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPkgInfoCommandlet")
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
pub struct UPolysExporterOBJ {
    __padding_end: [u8; 128],
}
impl UPolysExporterOBJ {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolysExporterOBJ")
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
pub struct UPolysExporterT3D {
    __padding_end: [u8; 128],
}
impl UPolysExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolysExporterT3D")
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
pub struct UPolysFactory {
    __padding_end: [u8; 136],
}
impl UPolysFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPolysFactory")
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
pub struct UPreviewMeshCollectionFactory {
    __padding_end: [u8; 144],
}
impl UPreviewMeshCollectionFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPreviewMeshCollectionFactory")
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
pub struct UPropertyColorSettings {
    __padding_end: [u8; 64],
}
impl UPropertyColorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyColorSettings")
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
pub struct UCSVImportFactory {
    #[doc(hidden)]
    pub(crate) __padding_152: [u8; 152],
    pub automated_import_settings: FCSVImportSettings,
    __padding_end: [u8; 8],
}
impl UCSVImportFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCSVImportFactory")
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
pub struct UReimportCurveFactory {
    __padding_end: [u8; 224],
}
impl UReimportCurveFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportCurveFactory")
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
pub struct UReimportCurveTableFactory {
    __padding_end: [u8; 224],
}
impl UReimportCurveTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportCurveTableFactory")
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
pub struct UReimportDataTableFactory {
    __padding_end: [u8; 224],
}
impl UReimportDataTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportDataTableFactory")
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
pub struct UFbxFactory {
    __padding_end: [u8; 160],
}
impl UFbxFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxFactory")
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
pub struct UReimportFbxAnimSequenceFactory {
    __padding_end: [u8; 192],
}
impl UReimportFbxAnimSequenceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportFbxAnimSequenceFactory")
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
pub struct UReimportFbxSkeletalMeshFactory {
    __padding_end: [u8; 192],
}
impl UReimportFbxSkeletalMeshFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportFbxSkeletalMeshFactory")
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
pub struct UReimportFbxStaticMeshFactory {
    __padding_end: [u8; 192],
}
impl UReimportFbxStaticMeshFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportFbxStaticMeshFactory")
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
pub struct UTextureFactory {
    __padding_end: [u8; 256],
}
impl UTextureFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureFactory")
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
pub struct UReimportTextureFactory {
    __padding_end: [u8; 304],
}
impl UReimportTextureFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportTextureFactory")
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
pub struct UVectorFieldStaticFactory {
    __padding_end: [u8; 136],
}
impl UVectorFieldStaticFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVectorFieldStaticFactory")
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
pub struct UReimportVectorFieldStaticFactory {
    __padding_end: [u8; 168],
}
impl UReimportVectorFieldStaticFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportVectorFieldStaticFactory")
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
pub struct URenderTargetExporterPNG {
    __padding_end: [u8; 128],
}
impl URenderTargetExporterPNG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URenderTargetExporterPNG")
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
pub struct URenderTargetExporterEXR {
    __padding_end: [u8; 128],
}
impl URenderTargetExporterEXR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URenderTargetExporterEXR")
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
pub struct UReplaceActorCommandlet {
    __padding_end: [u8; 136],
}
impl UReplaceActorCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReplaceActorCommandlet")
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
pub struct UResavePackagesCommandlet {
    __padding_end: [u8; 624],
}
impl UResavePackagesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UResavePackagesCommandlet")
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
pub struct USceneImportFactory {
    __padding_end: [u8; 136],
}
impl USceneImportFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USceneImportFactory")
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
pub struct UEditorViewportViewMenuContext {
    __padding_end: [u8; 64],
}
impl UEditorViewportViewMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorViewportViewMenuContext")
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
pub struct USequenceExporterT3D {
    __padding_end: [u8; 128],
}
impl USequenceExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequenceExporterT3D")
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
pub struct USheetBuilder {
    __padding_end: [u8; 168],
}
impl USheetBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USheetBuilder")
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
pub struct USkeletalMeshEditorSettings {
    __padding_end: [u8; 96],
}
impl USkeletalMeshEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshEditorSettings")
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
pub struct USkeletalMeshExporterFBX {
    __padding_end: [u8; 128],
}
impl USkeletalMeshExporterFBX {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshExporterFBX")
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
pub struct USkeletalMeshToolMenuContext {
    __padding_end: [u8; 64],
}
impl USkeletalMeshToolMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshToolMenuContext")
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
pub struct USoundExporterOGG {
    __padding_end: [u8; 128],
}
impl USoundExporterOGG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundExporterOGG")
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
pub struct USoundExporterWAV {
    __padding_end: [u8; 128],
}
impl USoundExporterWAV {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundExporterWAV")
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
pub struct USoundSurroundExporterWAV {
    __padding_end: [u8; 128],
}
impl USoundSurroundExporterWAV {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundSurroundExporterWAV")
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
pub struct USpecularProfileFactory {
    __padding_end: [u8; 136],
}
impl USpecularProfileFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpecularProfileFactory")
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
pub struct USpiralStairBuilder {
    __padding_end: [u8; 176],
}
impl USpiralStairBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpiralStairBuilder")
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
pub struct UStaticMeshExporterFBX {
    __padding_end: [u8; 128],
}
impl UStaticMeshExporterFBX {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshExporterFBX")
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
pub struct UStaticMeshExporterOBJ {
    __padding_end: [u8; 128],
}
impl UStaticMeshExporterOBJ {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshExporterOBJ")
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
pub struct UStaticMeshMinLodCommandlet {
    __padding_end: [u8; 136],
}
impl UStaticMeshMinLodCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshMinLodCommandlet")
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
pub struct UStringTableFactory {
    __padding_end: [u8; 136],
}
impl UStringTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStringTableFactory")
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
pub struct UStructureFactory {
    __padding_end: [u8; 136],
}
impl UStructureFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStructureFactory")
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
pub struct UStructViewerSettings {
    __padding_end: [u8; 56],
}
impl UStructViewerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStructViewerSettings")
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
pub struct USubsurfaceProfileFactory {
    __padding_end: [u8; 136],
}
impl USubsurfaceProfileFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubsurfaceProfileFactory")
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
pub struct USubUVAnimationFactory {
    __padding_end: [u8; 144],
}
impl USubUVAnimationFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubUVAnimationFactory")
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
pub struct UTetrahedronBuilder {
    __padding_end: [u8; 160],
}
impl UTetrahedronBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTetrahedronBuilder")
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
pub struct UTexAligner {
    __padding_end: [u8; 80],
}
impl UTexAligner {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexAligner")
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
pub struct UTexAlignerBox {
    __padding_end: [u8; 80],
}
impl UTexAlignerBox {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexAlignerBox")
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
pub struct UTexAlignerDefault {
    __padding_end: [u8; 80],
}
impl UTexAlignerDefault {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexAlignerDefault")
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
pub struct UTexAlignerFit {
    __padding_end: [u8; 80],
}
impl UTexAlignerFit {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexAlignerFit")
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
pub struct UTexAlignerPlanar {
    __padding_end: [u8; 80],
}
impl UTexAlignerPlanar {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexAlignerPlanar")
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
pub struct UTextBufferExporterTXT {
    __padding_end: [u8; 128],
}
impl UTextBufferExporterTXT {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextBufferExporterTXT")
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
pub struct UTextureThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTextureThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureThumbnailRenderer")
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
pub struct UTexture2DArrayThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTexture2DArrayThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexture2DArrayThumbnailRenderer")
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
pub struct UTexture2DFactoryNew {
    __padding_end: [u8; 144],
}
impl UTexture2DFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexture2DFactoryNew")
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
pub struct UTextureCubeExporterHDR {
    __padding_end: [u8; 128],
}
impl UTextureCubeExporterHDR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureCubeExporterHDR")
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
pub struct URenderTargetCubeExporterHDR {
    __padding_end: [u8; 128],
}
impl URenderTargetCubeExporterHDR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("URenderTargetCubeExporterHDR")
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
pub struct UTextureExporterGeneric {
    __padding_end: [u8; 128],
}
impl UTextureExporterGeneric {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterGeneric")
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
pub struct UTextureExporterBMP {
    __padding_end: [u8; 128],
}
impl UTextureExporterBMP {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterBMP")
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
pub struct UVirtualTextureBuilderExporterBMP {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterBMP {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVirtualTextureBuilderExporterBMP")
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
pub struct UTextureExporterDDS {
    __padding_end: [u8; 128],
}
impl UTextureExporterDDS {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterDDS")
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
pub struct UVirtualTextureBuilderExporterDDS {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterDDS {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVirtualTextureBuilderExporterDDS")
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
pub struct UTextureExporterEXR {
    __padding_end: [u8; 128],
}
impl UTextureExporterEXR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterEXR")
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
pub struct UVirtualTextureBuilderExporterEXR {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterEXR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVirtualTextureBuilderExporterEXR")
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
pub struct UTextureExporterHDR {
    __padding_end: [u8; 128],
}
impl UTextureExporterHDR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterHDR")
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
pub struct UVirtualTextureBuilderExporterHDR {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterHDR {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVirtualTextureBuilderExporterHDR")
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
pub struct UTextureExporterPNG {
    __padding_end: [u8; 128],
}
impl UTextureExporterPNG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterPNG")
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
pub struct UVirtualTextureBuilderExporterPNG {
    __padding_end: [u8; 128],
}
impl UVirtualTextureBuilderExporterPNG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVirtualTextureBuilderExporterPNG")
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
pub struct UTextureExporterJPEG {
    __padding_end: [u8; 128],
}
impl UTextureExporterJPEG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterJPEG")
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
pub struct UTextureExporterUEJPEG {
    __padding_end: [u8; 128],
}
impl UTextureExporterUEJPEG {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterUEJPEG")
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
pub struct UTextureExporterTGA {
    __padding_end: [u8; 128],
}
impl UTextureExporterTGA {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureExporterTGA")
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
pub struct UUDIMTextureFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UUDIMTextureFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUDIMTextureFunctionLibrary")
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
    pub fn make_udim_virtual_texture_from_texture2_ds(
        output_path_name: FString,
        source_textures: &TArray<UPtr<crate::bindings::engine::UTexture2D>>,
        block_coords: &TArray<crate::bindings::core_u_object::FIntPoint>,
        b_keep_existing_settings: bool,
        b_check_out_and_save: bool,
    ) -> UPtr<crate::bindings::engine::UTexture2D> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .uudim_texture_function_library_make_udim_virtual_texture_from_texture2_ds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_path_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_textures,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::engine::UTexture2D>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                block_coords,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FIntPoint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_keep_existing_settings,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_check_out_and_save,
                __buffer.add(49).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UUDIMTextureFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .uudim_texture_function_library_make_udim_virtual_texture_from_texture2_ds,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(56).cast::<UPtr<crate::bindings::engine::UTexture2D>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UTextureRenderTarget2DArrayFactoryNew {
    __padding_end: [u8; 152],
}
impl UTextureRenderTarget2DArrayFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureRenderTarget2DArrayFactoryNew")
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
pub struct UTextureRenderTargetCubeFactoryNew {
    __padding_end: [u8; 144],
}
impl UTextureRenderTargetCubeFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureRenderTargetCubeFactoryNew")
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
pub struct UTextureRenderTargetFactoryNew {
    __padding_end: [u8; 152],
}
impl UTextureRenderTargetFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureRenderTargetFactoryNew")
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
pub struct UTextureRenderTargetVolumeFactoryNew {
    __padding_end: [u8; 152],
}
impl UTextureRenderTargetVolumeFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureRenderTargetVolumeFactoryNew")
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
pub struct UTouchInterfaceFactory {
    __padding_end: [u8; 136],
}
impl UTouchInterfaceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTouchInterfaceFactory")
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
pub struct UTransactor {
    __padding_end: [u8; 48],
}
impl UTransactor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransactor")
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
pub struct UTransBuffer {
    __padding_end: [u8; 336],
}
impl UTransBuffer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransBuffer")
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
pub struct UTrueTypeFontFactory {
    __padding_end: [u8; 304],
}
impl UTrueTypeFontFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTrueTypeFontFactory")
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
pub struct UUnrealEdKeyBindings {
    __padding_end: [u8; 64],
}
impl UUnrealEdKeyBindings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnrealEdKeyBindings")
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
pub struct UUnrealEdOptions {
    __padding_end: [u8; 240],
}
impl UUnrealEdOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnrealEdOptions")
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
pub struct UVectorFieldExporter {
    __padding_end: [u8; 128],
}
impl UVectorFieldExporter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVectorFieldExporter")
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
pub struct UViewportToolBarContext {
    __padding_end: [u8; 80],
}
impl UViewportToolBarContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UViewportToolBarContext")
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
pub struct UVolumetricBuilder {
    __padding_end: [u8; 160],
}
impl UVolumetricBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumetricBuilder")
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
pub struct UWorldPartitionBuildNavigationOptions {
    __padding_end: [u8; 56],
}
impl UWorldPartitionBuildNavigationOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionBuildNavigationOptions")
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
pub struct UDataLayerConversionInfo {
    __padding_end: [u8; 96],
}
impl UDataLayerConversionInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerConversionInfo")
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
pub struct UDataLayerToAssetCommandletContext {
    __padding_end: [u8; 80],
}
impl UDataLayerToAssetCommandletContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerToAssetCommandletContext")
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
pub struct UDataLayerToAssetCommandlet {
    __padding_end: [u8; 192],
}
impl UDataLayerToAssetCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataLayerToAssetCommandlet")
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
pub struct UWorldPartitionBuilder {
    __padding_end: [u8; 464],
}
impl UWorldPartitionBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionBuilder")
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
pub struct UWorldPartitionLandscapeBuilder {
    __padding_end: [u8; 464],
}
impl UWorldPartitionLandscapeBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionLandscapeBuilder")
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
pub struct UWrangleContentCommandlet {
    __padding_end: [u8; 136],
}
impl UWrangleContentCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWrangleContentCommandlet")
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
pub struct UActorGroupingUtils {
    __padding_end: [u8; 48],
}
impl UActorGroupingUtils {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorGroupingUtils")
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
    pub fn unlock_selected_groups(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_unlock_selected_groups,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_unlock_selected_groups,
                __buffer,
            )
        };
    }
    pub fn ungroup_selected(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_ungroup_selected,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_ungroup_selected,
                __buffer,
            )
        };
    }
    pub fn ungroup_actors(
        &mut self,
        actors_to_ungroup: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_ungroup_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_ungroup,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_ungroup_actors,
                __buffer,
            )
        };
    }
    pub fn set_grouping_active(b_in_grouping_active: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_set_grouping_active,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_grouping_active,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UActorGroupingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_set_grouping_active,
                __buffer,
            )
        };
    }
    pub fn remove_selected_from_group(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_remove_selected_from_group,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_remove_selected_from_group,
                __buffer,
            )
        };
    }
    pub fn lock_selected_groups(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_lock_selected_groups,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_lock_selected_groups,
                __buffer,
            )
        };
    }
    pub fn is_grouping_active() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_is_grouping_active,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::unreal_ed::UActorGroupingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_is_grouping_active,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn group_selected(&mut self) -> UPtr<AGroupActor> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_group_selected,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_group_selected,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<AGroupActor>>().read() }
    }
    pub fn group_actors(
        &mut self,
        actors_to_group: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> UPtr<AGroupActor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_group_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_group,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_group_actors,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<AGroupActor>>().read() }
    }
    pub fn get() -> UPtr<UActorGroupingUtils> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_actor_grouping_utils_get,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::unreal_ed::UActorGroupingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_actor_grouping_utils_get,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UActorGroupingUtils>>().read() }
    }
    pub fn can_group_selected_actors(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_can_group_selected_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_can_group_selected_actors,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn can_group_actors(
        &self,
        actors_to_group: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_can_group_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_group,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_can_group_actors,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_selected_to_group(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_add_selected_to_group,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_actor_grouping_utils_add_selected_to_group,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UAnalyticsPrivacySettings {
    __padding_end: [u8; 64],
}
impl UAnalyticsPrivacySettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnalyticsPrivacySettings")
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
pub struct UCrashReportsPrivacySettings {
    __padding_end: [u8; 64],
}
impl UCrashReportsPrivacySettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCrashReportsPrivacySettings")
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
pub struct UAnimSeqExportOption {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub b_export_transforms: bool,
    pub b_export_morph_targets: bool,
    pub b_export_attribute_curves: bool,
    pub b_export_material_curves: bool,
    pub b_record_in_world_space: bool,
    pub b_evaluate_all_skeletal_mesh_components: bool,
    pub interpolation: crate::bindings::engine::EAnimInterpolationType,
    pub curve_interpolation: crate::bindings::engine::ERichCurveInterpMode,
    pub include_animation_names: TArray<FString>,
    pub exclude_animation_names: TArray<FString>,
    pub warm_up_frames: crate::bindings::core_u_object::FFrameNumber,
    pub delay_before_start: crate::bindings::core_u_object::FFrameNumber,
    pub b_transact_recording: bool,
    pub b_bake_timecode: bool,
    pub b_timecode_rate_override: bool,
    pub override_timecode_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_time_range: bool,
    pub custom_start_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_end_frame: crate::bindings::core_u_object::FFrameNumber,
    pub custom_display_rate: crate::bindings::core_u_object::FFrameRate,
    pub b_use_custom_frame_rate: bool,
    pub custom_frame_rate: crate::bindings::core_u_object::FFrameRate,
}
impl UAnimSeqExportOption {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSeqExportOption")
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
pub struct UDebugSkelMeshComponent {
    __padding_end: [u8; 5184],
}
impl UDebugSkelMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDebugSkelMeshComponent")
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
pub struct UEditorAnimBaseObj {
    __padding_end: [u8; 80],
}
impl UEditorAnimBaseObj {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorAnimBaseObj")
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
pub struct UEditorAnimCompositeSegment {
    __padding_end: [u8; 128],
}
impl UEditorAnimCompositeSegment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorAnimCompositeSegment")
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
pub struct UEditorAnimCurveBoneLinks {
    __padding_end: [u8; 136],
}
impl UEditorAnimCurveBoneLinks {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorAnimCurveBoneLinks")
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
pub struct UEditorAnimSegment {
    __padding_end: [u8; 128],
}
impl UEditorAnimSegment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorAnimSegment")
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
pub struct UEditorCompositeSection {
    __padding_end: [u8; 184],
}
impl UEditorCompositeSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorCompositeSection")
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
pub struct UEditorNotifyObject {
    __padding_end: [u8; 320],
}
impl UEditorNotifyObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorNotifyObject")
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
pub struct UEditorParentPlayerListObj {
    __padding_end: [u8; 152],
}
impl UEditorParentPlayerListObj {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorParentPlayerListObj")
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
pub struct UEditorSkeletonNotifyObj {
    __padding_end: [u8; 80],
}
impl UEditorSkeletonNotifyObj {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorSkeletonNotifyObj")
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
pub struct UAssetGuideline {
    __padding_end: [u8; 112],
}
impl UAssetGuideline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetGuideline")
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
pub struct UAssetImportTask {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub filename: FString,
    pub destination_path: FString,
    pub destination_name: FString,
    pub b_replace_existing: bool,
    pub b_replace_existing_settings: bool,
    pub b_automated: bool,
    pub b_save: bool,
    pub b_async: bool,
    pub factory: UPtr<UFactory>,
    pub options: UPtr<crate::bindings::core_u_object::UObject>,
    pub imported_object_paths: TArray<FString>,
    __padding_end: [u8; 32],
}
impl UAssetImportTask {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetImportTask")
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
    pub fn is_async_import_complete(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_import_task_is_async_import_complete,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_import_task_is_async_import_complete,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_objects(&self) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_import_task_get_objects,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_import_task_get_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAutomatedAssetImportData {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub group_name: FString,
    pub filenames: TArray<FString>,
    pub destination_path: FString,
    pub factory_name: FString,
    pub b_replace_existing: bool,
    pub b_skip_read_only: bool,
    pub factory: UPtr<UFactory>,
    pub level_to_load: FString,
    __padding_end: [u8; 16],
}
impl UAutomatedAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutomatedAssetImportData")
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
pub struct UAutoReimportManager {
    __padding_end: [u8; 64],
}
impl UAutoReimportManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAutoReimportManager")
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
pub struct UAssetRegUtilCommandlet {
    __padding_end: [u8; 144],
}
impl UAssetRegUtilCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetRegUtilCommandlet")
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
pub struct UAssetRegistryDumpCommandlet {
    __padding_end: [u8; 136],
}
impl UAssetRegistryDumpCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetRegistryDumpCommandlet")
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
pub struct UAssetSizeQueryCommandlet {
    __padding_end: [u8; 136],
}
impl UAssetSizeQueryCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetSizeQueryCommandlet")
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
pub struct UAudioMixerCommandlet {
    __padding_end: [u8; 136],
}
impl UAudioMixerCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAudioMixerCommandlet")
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
pub struct UBaseIteratePackagesCommandlet {
    __padding_end: [u8; 584],
}
impl UBaseIteratePackagesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseIteratePackagesCommandlet")
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
pub struct UChunkDependencyInfo {
    __padding_end: [u8; 192],
}
impl UChunkDependencyInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UChunkDependencyInfo")
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
pub struct UCompileAllBlueprintsCommandlet {
    __padding_end: [u8; 288],
}
impl UCompileAllBlueprintsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompileAllBlueprintsCommandlet")
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
pub struct UCompileShadersTestBedCommandlet {
    __padding_end: [u8; 136],
}
impl UCompileShadersTestBedCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompileShadersTestBedCommandlet")
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
pub struct UConvertLevelsToExternalActorsCommandlet {
    __padding_end: [u8; 144],
}
impl UConvertLevelsToExternalActorsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UConvertLevelsToExternalActorsCommandlet")
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
pub struct UCookCommandlet {
    __padding_end: [u8; 216],
}
impl UCookCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookCommandlet")
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
pub struct UCookShadersCommandlet {
    __padding_end: [u8; 136],
}
impl UCookShadersCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookShadersCommandlet")
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
pub struct UCopyNaniteFallbackSettingsToRayTracingProxyCommandlet {
    __padding_end: [u8; 136],
}
impl UCopyNaniteFallbackSettingsToRayTracingProxyCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCopyNaniteFallbackSettingsToRayTracingProxyCommandlet")
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
pub struct UDDCCleanupCommandlet {
    __padding_end: [u8; 136],
}
impl UDDCCleanupCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDDCCleanupCommandlet")
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
pub struct UDebugShaderCompileJobCommandlet {
    __padding_end: [u8; 136],
}
impl UDebugShaderCompileJobCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDebugShaderCompileJobCommandlet")
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
pub struct UDerivedDataCacheCommandlet {
    __padding_end: [u8; 432],
}
impl UDerivedDataCacheCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDerivedDataCacheCommandlet")
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
pub struct UDetectOrphanedLocalizedAssetsCommandlet {
    __padding_end: [u8; 136],
}
impl UDetectOrphanedLocalizedAssetsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDetectOrphanedLocalizedAssetsCommandlet")
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
pub struct UDiffAssetRegistriesCommandlet {
    __padding_end: [u8; 2488],
}
impl UDiffAssetRegistriesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDiffAssetRegistriesCommandlet")
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
pub struct UDiffAssetsCommandlet {
    __padding_end: [u8; 136],
}
impl UDiffAssetsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDiffAssetsCommandlet")
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
pub struct UDiffCookCommandlet {
    __padding_end: [u8; 3488],
}
impl UDiffCookCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDiffCookCommandlet")
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
pub struct UDiffFilesCommandlet {
    __padding_end: [u8; 272],
}
impl UDiffFilesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDiffFilesCommandlet")
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
pub struct UDumpAssetRegistryCommandlet {
    __padding_end: [u8; 192],
}
impl UDumpAssetRegistryCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpAssetRegistryCommandlet")
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
pub struct UDumpBlueprintsInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpBlueprintsInfoCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpBlueprintsInfoCommandlet")
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
pub struct UDumpHiddenCategoriesCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpHiddenCategoriesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpHiddenCategoriesCommandlet")
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
pub struct UDumpLightFunctionMaterialInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpLightFunctionMaterialInfoCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpLightFunctionMaterialInfoCommandlet")
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
pub struct UDumpMaterialExpressionInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialExpressionInfoCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpMaterialExpressionInfoCommandlet")
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
pub struct UDumpMaterialExpressionsCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialExpressionsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpMaterialExpressionsCommandlet")
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
pub struct UDumpMaterialInfoCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialInfoCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpMaterialInfoCommandlet")
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
pub struct UDumpMaterialShaderTypesCommandlet {
    __padding_end: [u8; 136],
}
impl UDumpMaterialShaderTypesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDumpMaterialShaderTypesCommandlet")
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
pub struct UGatherTextCommandletBase {
    __padding_end: [u8; 264],
}
impl UGatherTextCommandletBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGatherTextCommandletBase")
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
pub struct UExportDialogueScriptCommandlet {
    __padding_end: [u8; 264],
}
impl UExportDialogueScriptCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExportDialogueScriptCommandlet")
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
pub struct UExportPakDependenciesCommandlet {
    __padding_end: [u8; 136],
}
impl UExportPakDependenciesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExportPakDependenciesCommandlet")
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
pub struct UExternalActorsCommandlet {
    __padding_end: [u8; 176],
}
impl UExternalActorsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExternalActorsCommandlet")
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
pub struct UExtractLocResCommandlet {
    __padding_end: [u8; 136],
}
impl UExtractLocResCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UExtractLocResCommandlet")
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
pub struct UFileServerCommandlet {
    __padding_end: [u8; 152],
}
impl UFileServerCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFileServerCommandlet")
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
pub struct UFixConflictingLocalizationKeysCommandlet {
    __padding_end: [u8; 136],
}
impl UFixConflictingLocalizationKeysCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixConflictingLocalizationKeysCommandlet")
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
pub struct UFixupNeedsLoadForEditorGameCommandlet {
    __padding_end: [u8; 704],
}
impl UFixupNeedsLoadForEditorGameCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFixupNeedsLoadForEditorGameCommandlet")
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
pub struct UGatherTextCommandlet {
    __padding_end: [u8; 1024],
}
impl UGatherTextCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGatherTextCommandlet")
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
pub struct UGatherTextFromAssetsCommandlet {
    __padding_end: [u8; 656],
}
impl UGatherTextFromAssetsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGatherTextFromAssetsCommandlet")
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
pub struct UGatherTextFromAssetsWorkerCommandlet {
    __padding_end: [u8; 232],
}
impl UGatherTextFromAssetsWorkerCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGatherTextFromAssetsWorkerCommandlet")
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
pub struct UGatherTextFromMetaDataCommandlet {
    __padding_end: [u8; 336],
}
impl UGatherTextFromMetaDataCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGatherTextFromMetaDataCommandlet")
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
pub struct UGatherTextFromSourceCommandlet {
    __padding_end: [u8; 264],
}
impl UGatherTextFromSourceCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGatherTextFromSourceCommandlet")
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
pub struct UGenerateAssetManifestCommandlet {
    __padding_end: [u8; 136],
}
impl UGenerateAssetManifestCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateAssetManifestCommandlet")
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
pub struct UGenerateBlueprintAPICommandlet {
    __padding_end: [u8; 136],
}
impl UGenerateBlueprintAPICommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateBlueprintAPICommandlet")
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
pub struct UGenerateDistillFileSetsCommandlet {
    __padding_end: [u8; 136],
}
impl UGenerateDistillFileSetsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateDistillFileSetsCommandlet")
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
pub struct UGenerateGatherArchiveCommandlet {
    __padding_end: [u8; 264],
}
impl UGenerateGatherArchiveCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateGatherArchiveCommandlet")
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
pub struct UGenerateGatherManifestCommandlet {
    __padding_end: [u8; 264],
}
impl UGenerateGatherManifestCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateGatherManifestCommandlet")
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
pub struct UGenerateTextLocalizationReportCommandlet {
    __padding_end: [u8; 312],
}
impl UGenerateTextLocalizationReportCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateTextLocalizationReportCommandlet")
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
pub struct UGenerateTextLocalizationResourceCommandlet {
    __padding_end: [u8; 264],
}
impl UGenerateTextLocalizationResourceCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UGenerateTextLocalizationResourceCommandlet")
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
pub struct UImportAssetsCommandlet {
    __padding_end: [u8; 192],
}
impl UImportAssetsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImportAssetsCommandlet")
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
pub struct UImportDialogueScriptCommandlet {
    __padding_end: [u8; 264],
}
impl UImportDialogueScriptCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImportDialogueScriptCommandlet")
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
pub struct UImportLocalizedDialogueCommandlet {
    __padding_end: [u8; 344],
}
impl UImportLocalizedDialogueCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImportLocalizedDialogueCommandlet")
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
pub struct UInternationalizationConditioningCommandlet {
    __padding_end: [u8; 17232],
}
impl UInternationalizationConditioningCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInternationalizationConditioningCommandlet")
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
pub struct UInternationalizationExportCommandlet {
    __padding_end: [u8; 272],
}
impl UInternationalizationExportCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInternationalizationExportCommandlet")
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
pub struct UIoStoreCommandlet {
    __padding_end: [u8; 136],
}
impl UIoStoreCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UIoStoreCommandlet")
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
pub struct UMakeBinaryConfigCommandlet {
    __padding_end: [u8; 136],
}
impl UMakeBinaryConfigCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMakeBinaryConfigCommandlet")
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
pub struct UMaterialLayerUsageCommandlet {
    __padding_end: [u8; 136],
}
impl UMaterialLayerUsageCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialLayerUsageCommandlet")
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
pub struct UMergeShaderPipelineCachesCommandlet {
    __padding_end: [u8; 136],
}
impl UMergeShaderPipelineCachesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMergeShaderPipelineCachesCommandlet")
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
pub struct UParticleSystemAuditCommandlet {
    __padding_end: [u8; 1232],
}
impl UParticleSystemAuditCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParticleSystemAuditCommandlet")
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
pub struct UPopulateDialogueWaveFromCharacterSheetCommandlet {
    __padding_end: [u8; 136],
}
impl UPopulateDialogueWaveFromCharacterSheetCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPopulateDialogueWaveFromCharacterSheetCommandlet")
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
pub struct UReplaceAssetsCommandlet {
    __padding_end: [u8; 136],
}
impl UReplaceAssetsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReplaceAssetsCommandlet")
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
pub struct USavePackageUtilitiesCommandlet {
    __padding_end: [u8; 136],
}
impl USavePackageUtilitiesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USavePackageUtilitiesCommandlet")
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
pub struct UShaderCodeLibraryToolsCommandlet {
    __padding_end: [u8; 136],
}
impl UShaderCodeLibraryToolsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShaderCodeLibraryToolsCommandlet")
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
pub struct UShaderPipelineCacheToolsCommandlet {
    __padding_end: [u8; 136],
}
impl UShaderPipelineCacheToolsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShaderPipelineCacheToolsCommandlet")
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
pub struct UStabilizeLocalizationKeysCommandlet {
    __padding_end: [u8; 136],
}
impl UStabilizeLocalizationKeysCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStabilizeLocalizationKeysCommandlet")
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
pub struct USubstrateCommandlet {
    __padding_end: [u8; 136],
}
impl USubstrateCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubstrateCommandlet")
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
pub struct USummarizeTraceCommandlet {
    __padding_end: [u8; 168],
}
impl USummarizeTraceCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USummarizeTraceCommandlet")
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
pub struct USummarizeTraceEditorWorkflowsCommandlet {
    __padding_end: [u8; 136],
}
impl USummarizeTraceEditorWorkflowsCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USummarizeTraceEditorWorkflowsCommandlet")
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
pub struct USwapSoundForDialogueInCuesCommandlet {
    __padding_end: [u8; 136],
}
impl USwapSoundForDialogueInCuesCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USwapSoundForDialogueInCuesCommandlet")
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
pub struct UTextAssetCommandlet {
    __padding_end: [u8; 136],
}
impl UTextAssetCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextAssetCommandlet")
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
pub struct UUnrealPakCommandlet {
    __padding_end: [u8; 136],
}
impl UUnrealPakCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnrealPakCommandlet")
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
pub struct UUpdateGameProjectCommandlet {
    __padding_end: [u8; 136],
}
impl UUpdateGameProjectCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUpdateGameProjectCommandlet")
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
pub struct UWorldPartitionBuilderCommandlet {
    __padding_end: [u8; 256],
}
impl UWorldPartitionBuilderCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionBuilderCommandlet")
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
pub struct UWorldPartitionConvertCommandlet {
    __padding_end: [u8; 944],
}
impl UWorldPartitionConvertCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionConvertCommandlet")
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
pub struct UCookFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UCookFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookFunctionLibrary")
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
    pub fn cook_asset(
        object: UPtr<crate::bindings::core_u_object::UObject>,
        for_platform: FString,
        destination_subfolder: FString,
        cook_commandline_args: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_cook_function_library_cook_asset,
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
                &for_platform,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_subfolder,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &cook_commandline_args,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UCookFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_cook_function_library_cook_asset,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEdGraphNode_Comment {
    __padding_end: [u8; 296],
}
impl UEdGraphNode_Comment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEdGraphNode_Comment")
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
pub struct UActorEditorContextActorFolderState {
    __padding_end: [u8; 64],
}
impl UActorEditorContextActorFolderState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorEditorContextActorFolderState")
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
pub struct UEditorDomainSaveCommandlet {
    __padding_end: [u8; 136],
}
impl UEditorDomainSaveCommandlet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorDomainSaveCommandlet")
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
pub struct UEditorEngine {
    __padding_end: [u8; 10032],
}
impl UEditorEngine {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorEngine")
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
pub struct UEditorLevelUtils {
    __padding_end: [u8; 48],
}
impl UEditorLevelUtils {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorLevelUtils")
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
    pub fn set_level_visibility(
        level: UPtr<crate::bindings::engine::ULevel>,
        b_should_be_visible: bool,
        b_force_layers_visible: bool,
        modify_mode: ELevelVisibilityDirtyMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_set_level_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevel>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_be_visible,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_layers_visible,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modify_mode,
                __buffer.add(10).cast::<ELevelVisibilityDirtyMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_set_level_visibility,
                __buffer,
            )
        };
    }
    pub fn set_levels_visibility(
        levels: &TArray<UPtr<crate::bindings::engine::ULevel>>,
        b_should_be_visible: &TArray<bool>,
        b_force_layers_visible: bool,
        modify_mode: ELevelVisibilityDirtyMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_set_levels_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                levels,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::ULevel>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_should_be_visible,
                __buffer.add(16).cast::<TArray<bool>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_layers_visible,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &modify_mode,
                __buffer.add(33).cast::<ELevelVisibilityDirtyMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_set_levels_visibility,
                __buffer,
            )
        };
    }
    pub fn move_selected_actors_to_level(
        dest_level: UPtr<crate::bindings::engine::ULevelStreaming>,
        b_warn_about_references: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_move_selected_actors_to_level,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dest_level,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevelStreaming>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_warn_about_references,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_move_selected_actors_to_level,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn move_actors_to_level(
        actors_to_move: &TArray<UPtr<crate::bindings::engine::AActor>>,
        dest_streaming_level: UPtr<crate::bindings::engine::ULevelStreaming>,
        b_warn_about_references: bool,
        b_warn_about_renaming: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_move_actors_to_level,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_move,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dest_streaming_level,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::ULevelStreaming>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_warn_about_references,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_warn_about_renaming,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_move_actors_to_level,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<i32>().read() }
    }
    pub fn make_level_current(
        in_streaming_level: UPtr<crate::bindings::engine::ULevelStreaming>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_make_level_current,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_streaming_level,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevelStreaming>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_make_level_current,
                __buffer,
            )
        };
    }
    pub fn remove_level_from_world(
        in_level: UPtr<crate::bindings::engine::ULevel>,
        b_clear_selection: bool,
        b_reset_transaction_buffer: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<11>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_k2_remove_level_from_world,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_level,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevel>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_clear_selection,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_transaction_buffer,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_k2_remove_level_from_world,
                __buffer,
            )
        };
        unsafe { __buffer.add(10).cast::<bool>().read() }
    }
    pub fn add_level_to_world_with_transform(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_package_name: FString,
        level_streaming_class: TSubclassOf<crate::bindings::engine::ULevelStreaming>,
        level_transform: &crate::bindings::core_u_object::FTransform,
    ) -> UPtr<crate::bindings::engine::ULevelStreaming> {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_k2_add_level_to_world_with_transform,
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
                &level_package_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_streaming_class,
                __buffer
                    .add(24)
                    .cast::<TSubclassOf<crate::bindings::engine::ULevelStreaming>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                level_transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_k2_add_level_to_world_with_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(128)
                .cast::<UPtr<crate::bindings::engine::ULevelStreaming>>()
                .read()
        }
    }
    pub fn add_level_to_world(
        world: UPtr<crate::bindings::engine::UWorld>,
        level_package_name: FString,
        level_streaming_class: TSubclassOf<crate::bindings::engine::ULevelStreaming>,
    ) -> UPtr<crate::bindings::engine::ULevelStreaming> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_k2_add_level_to_world,
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
                &level_package_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_streaming_class,
                __buffer
                    .add(24)
                    .cast::<TSubclassOf<crate::bindings::engine::ULevelStreaming>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_k2_add_level_to_world,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::engine::ULevelStreaming>>()
                .read()
        }
    }
    pub fn get_levels(
        world: UPtr<crate::bindings::engine::UWorld>,
    ) -> TArray<UPtr<crate::bindings::engine::ULevel>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_get_levels,
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
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_get_levels,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::ULevel>>>()
                .read()
        }
    }
    pub fn create_new_streaming_level(
        level_streaming_class: TSubclassOf<crate::bindings::engine::ULevelStreaming>,
        new_level_path: FString,
        b_move_selected_actors_into_new_level: bool,
    ) -> UPtr<crate::bindings::engine::ULevelStreaming> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_create_new_streaming_level,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level_streaming_class,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::engine::ULevelStreaming>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_level_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_move_selected_actors_into_new_level,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLevelUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_level_utils_create_new_streaming_level,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::engine::ULevelStreaming>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UEditorPerformanceSettings {
    __padding_end: [u8; 200],
}
impl UEditorPerformanceSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorPerformanceSettings")
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
pub struct UActorEditorContextEditorState {
    __padding_end: [u8; 64],
}
impl UActorEditorContextEditorState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorEditorContextEditorState")
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
pub struct UEditorStateSubsystem {
    __padding_end: [u8; 80],
}
impl UEditorStateSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorStateSubsystem")
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
pub struct UWorldEditorState {
    __padding_end: [u8; 96],
}
impl UWorldEditorState {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldEditorState")
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
pub struct UEditorWorldExtensionCollection {
    __padding_end: [u8; 80],
}
impl UEditorWorldExtensionCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorWorldExtensionCollection")
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
pub struct UEditorWorldExtensionManager {
    __padding_end: [u8; 64],
}
impl UEditorWorldExtensionManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorWorldExtensionManager")
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
pub struct UActorElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl UActorElementDetailsInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorElementDetailsInterface")
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
pub struct UActorElementEditorAssetDataInterface {
    __padding_end: [u8; 56],
}
impl UActorElementEditorAssetDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorElementEditorAssetDataInterface")
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
pub struct UActorElementsCopy {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub actors_to_copy: TArray<UPtr<crate::bindings::engine::AActor>>,
}
impl UActorElementsCopy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorElementsCopy")
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
pub struct UActorElementsExporterT3D {
    __padding_end: [u8; 128],
}
impl UActorElementsExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorElementsExporterT3D")
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
pub struct UActorElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl UActorElementEditorSelectionInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorElementEditorSelectionInterface")
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
pub struct UActorElementEditorWorldInterface {
    __padding_end: [u8; 56],
}
impl UActorElementEditorWorldInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorElementEditorWorldInterface")
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
pub struct UComponentElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementDetailsInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComponentElementDetailsInterface")
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
pub struct UComponentElementsCopy {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub components_to_copy: TArray<UPtr<crate::bindings::engine::UActorComponent>>,
}
impl UComponentElementsCopy {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComponentElementsCopy")
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
pub struct UComponentElementsExporterT3D {
    __padding_end: [u8; 128],
}
impl UComponentElementsExporterT3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComponentElementsExporterT3D")
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
pub struct UComponentElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementEditorSelectionInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComponentElementEditorSelectionInterface")
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
pub struct UComponentElementEditorWorldInterface {
    __padding_end: [u8; 56],
}
impl UComponentElementEditorWorldInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UComponentElementEditorWorldInterface")
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
pub struct UObjectElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementDetailsInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectElementDetailsInterface")
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
pub struct UObjectElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl UObjectElementEditorSelectionInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UObjectElementEditorSelectionInterface")
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
pub struct USMInstanceElementDetailsInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementDetailsInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USMInstanceElementDetailsInterface")
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
pub struct USMInstanceElementDetailsProxyObject {
    __padding_end: [u8; 192],
}
impl USMInstanceElementDetailsProxyObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USMInstanceElementDetailsProxyObject")
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
pub struct USMInstanceElementEditorSelectionInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementEditorSelectionInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USMInstanceElementEditorSelectionInterface")
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
pub struct USMInstanceElementEditorWorldInterface {
    __padding_end: [u8; 56],
}
impl USMInstanceElementEditorWorldInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USMInstanceElementEditorWorldInterface")
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
pub struct UActorFactoryPointLight {
    __padding_end: [u8; 144],
}
impl UActorFactoryPointLight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryPointLight")
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
pub struct UActorFactoryRectLight {
    __padding_end: [u8; 144],
}
impl UActorFactoryRectLight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactoryRectLight")
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
pub struct UActorFactorySpotLight {
    __padding_end: [u8; 144],
}
impl UActorFactorySpotLight {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorFactorySpotLight")
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
pub struct UAnimBankFactory {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UAnimBankFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBankFactory")
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
pub struct UTransformProviderDataFactory {
    __padding_end: [u8; 144],
}
impl UTransformProviderDataFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTransformProviderDataFactory")
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
pub struct UAnimBankDataFactory {
    __padding_end: [u8; 144],
}
impl UAnimBankDataFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBankDataFactory")
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
pub struct UAnimBlueprintFactory {
    __padding_end: [u8; 176],
}
impl UAnimBlueprintFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBlueprintFactory")
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
pub struct UAnimLayerInterfaceFactory {
    __padding_end: [u8; 176],
}
impl UAnimLayerInterfaceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimLayerInterfaceFactory")
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
pub struct UAnimBoneCompressionSettingsFactory {
    __padding_end: [u8; 136],
}
impl UAnimBoneCompressionSettingsFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBoneCompressionSettingsFactory")
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
pub struct UAnimCompositeFactory {
    __padding_end: [u8; 176],
}
impl UAnimCompositeFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimCompositeFactory")
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
pub struct UAnimCurveCompressionSettingsFactory {
    __padding_end: [u8; 136],
}
impl UAnimCurveCompressionSettingsFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimCurveCompressionSettingsFactory")
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
pub struct UAnimMontageFactory {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
    __padding_end: [u8; 24],
}
impl UAnimMontageFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimMontageFactory")
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
pub struct UAnimSequenceFactory {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub target_skeleton: UPtr<crate::bindings::engine::USkeleton>,
    pub preview_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    __padding_end: [u8; 16],
}
impl UAnimSequenceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequenceFactory")
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
pub struct UAnimStreamableFactory {
    __padding_end: [u8; 152],
}
impl UAnimStreamableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimStreamableFactory")
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
pub struct UCompositeCurveTableFactory {
    __padding_end: [u8; 136],
}
impl UCompositeCurveTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeCurveTableFactory")
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
pub struct UDataTableFactory {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub _struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
}
impl UDataTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDataTableFactory")
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
pub struct UCompositeDataTableFactory {
    __padding_end: [u8; 144],
}
impl UCompositeDataTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCompositeDataTableFactory")
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
pub struct UCurveTableFactory {
    __padding_end: [u8; 144],
}
impl UCurveTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveTableFactory")
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
pub struct UEditorStaticMeshFactory {
    __padding_end: [u8; 224],
}
impl UEditorStaticMeshFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorStaticMeshFactory")
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
pub struct UInterchangeReimportHandler {
    __padding_end: [u8; 80],
}
impl UInterchangeReimportHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeReimportHandler")
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
pub struct ULightWeightInstanceFactory {
    __padding_end: [u8; 144],
}
impl ULightWeightInstanceFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULightWeightInstanceFactory")
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
pub struct UMaterialImportHelpers {
    __padding_end: [u8; 48],
}
impl UMaterialImportHelpers {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialImportHelpers")
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
    pub fn find_existing_material_from_search_location(
        material_full_name: FString,
        base_package_path: FString,
        search_location: EMaterialSearchLocation,
        out_error: &mut FText,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_material_import_helpers_find_existing_material_from_search_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_full_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_package_path,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search_location,
                __buffer.add(32).cast::<EMaterialSearchLocation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error,
                __buffer.add(40).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UMaterialImportHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_material_import_helpers_find_existing_material_from_search_location,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<FText>().swap(out_error);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
    pub fn find_existing_material(
        base_path: FString,
        material_full_name: FString,
        b_recursive_paths: bool,
        out_error: &mut FText,
    ) -> UPtr<crate::bindings::engine::UMaterialInterface> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_material_import_helpers_find_existing_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_full_name,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recursive_paths,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error,
                __buffer.add(40).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UMaterialImportHelpers::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_material_import_helpers_find_existing_material,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<FText>().swap(out_error);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMeshDeformerCollectionFactory {
    __padding_end: [u8; 136],
}
impl UMeshDeformerCollectionFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMeshDeformerCollectionFactory")
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
pub struct UMirrorTableFindReplaceExpressions {
    __padding_end: [u8; 64],
}
impl UMirrorTableFindReplaceExpressions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorTableFindReplaceExpressions")
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
pub struct UMirrorDataTableFactory {
    __padding_end: [u8; 160],
}
impl UMirrorDataTableFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMirrorDataTableFactory")
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
pub struct UPackFactory {
    __padding_end: [u8; 136],
}
impl UPackFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPackFactory")
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
pub struct UPhysicsAssetFactory {
    __padding_end: [u8; 160],
}
impl UPhysicsAssetFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsAssetFactory")
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
pub struct UPoseAssetFactory {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub source_animation: UPtr<crate::bindings::engine::UAnimSequence>,
    pub pose_names: TArray<FString>,
    __padding_end: [u8; 16],
}
impl UPoseAssetFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPoseAssetFactory")
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
pub struct USkeletonFactory {
    __padding_end: [u8; 160],
}
impl USkeletonFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonFactory")
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
pub struct USlateBrushAssetFactory {
    __padding_end: [u8; 144],
}
impl USlateBrushAssetFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateBrushAssetFactory")
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
pub struct USlateWidgetStyleAssetFactory {
    __padding_end: [u8; 144],
}
impl USlateWidgetStyleAssetFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateWidgetStyleAssetFactory")
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
pub struct USparseVolumeTextureMaterialFactoryNew {
    __padding_end: [u8; 144],
}
impl USparseVolumeTextureMaterialFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USparseVolumeTextureMaterialFactoryNew")
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
pub struct USparseVolumeTextureMaterialInstanceFactoryNew {
    __padding_end: [u8; 200],
}
impl USparseVolumeTextureMaterialInstanceFactoryNew {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USparseVolumeTextureMaterialInstanceFactoryNew")
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
pub struct UTexture2DArrayFactory {
    __padding_end: [u8; 152],
}
impl UTexture2DArrayFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTexture2DArrayFactory")
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
pub struct UTextureCubeArrayFactory {
    __padding_end: [u8; 152],
}
impl UTextureCubeArrayFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureCubeArrayFactory")
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
pub struct UVariableFrameStrippingSettingsFactory {
    __padding_end: [u8; 136],
}
impl UVariableFrameStrippingSettingsFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVariableFrameStrippingSettingsFactory")
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
pub struct UVolumeTextureFactory {
    __padding_end: [u8; 144],
}
impl UVolumeTextureFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumeTextureFactory")
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
pub struct UWorldFactory {
    __padding_end: [u8; 144],
}
impl UWorldFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldFactory")
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
pub struct UFbxAssetImportData {
    #[doc(hidden)]
    pub(crate) __padding_96: [u8; 96],
    pub import_translation: crate::bindings::core_u_object::FVector,
    pub import_rotation: crate::bindings::core_u_object::FRotator,
    pub import_uniform_scale: f32,
    #[doc(hidden)]
    pub(crate) __padding_149: [u8; 1],
    pub b_convert_scene: bool,
    pub b_force_front_x_axis: bool,
    pub b_convert_scene_unit: bool,
    __padding_end: [u8; 24],
}
impl UFbxAssetImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxAssetImportData")
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
pub struct UFbxAnimSequenceImportData {
    __padding_end: [u8; 256],
}
impl UFbxAnimSequenceImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxAnimSequenceImportData")
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
pub struct UFbxExportOption {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub fbx_export_compatibility: EFbxExportCompatibility,
    #[doc(hidden)]
    pub(crate) __padding_52: [u8; 3],
    pub flags_52: u8,
    pub flags_53: u8,
    #[doc(hidden)]
    pub(crate) __padding_56: [u8; 2],
    pub bake_camera_and_light_animation: EMovieSceneBakeType,
    pub bake_actor_animation: EMovieSceneBakeType,
    pub bake_material_inputs: EFbxMaterialBakeMode,
    pub default_material_bake_size: FFbxMaterialBakeSize,
}
impl UFbxExportOption {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxExportOption")
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
pub struct UFbxMeshImportData {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub b_transform_vertex_to_absolute: bool,
    pub b_bake_pivot_in_vertex: bool,
    #[doc(hidden)]
    pub(crate) __padding_180: [u8; 2],
    pub flags_180: u8,
    #[doc(hidden)]
    pub(crate) __padding_184: [u8; 3],
    pub normal_import_method: EFBXNormalImportMethod,
    pub normal_generation_method: EFBXNormalGenerationMethod,
    #[doc(hidden)]
    pub(crate) __padding_188: [u8; 2],
    pub flags_188: u8,
    #[doc(hidden)]
    pub(crate) __padding_192: [u8; 3],
    pub b_reorder_material_to_fbx_order: bool,
    __padding_end: [u8; 39],
}
impl UFbxMeshImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxMeshImportData")
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
pub struct UFbxSceneImportData {
    __padding_end: [u8; 184],
}
impl UFbxSceneImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxSceneImportData")
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
pub struct UFbxSceneImportFactory {
    #[doc(hidden)]
    pub(crate) __padding_136: [u8; 136],
    pub scene_import_options: UPtr<UFbxSceneImportOptions>,
    pub scene_import_options_static_mesh: UPtr<UFbxSceneImportOptionsStaticMesh>,
    pub scene_import_options_skeletal_mesh: UPtr<UFbxSceneImportOptionsSkeletalMesh>,
    __padding_end: [u8; 240],
}
impl UFbxSceneImportFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxSceneImportFactory")
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
pub struct UFbxSceneImportOptions {
    __padding_end: [u8; 128],
}
impl UFbxSceneImportOptions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxSceneImportOptions")
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
pub struct UFbxSceneImportOptionsSkeletalMesh {
    __padding_end: [u8; 104],
}
impl UFbxSceneImportOptionsSkeletalMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxSceneImportOptionsSkeletalMesh")
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
pub struct UFbxSceneImportOptionsStaticMesh {
    __padding_end: [u8; 80],
}
impl UFbxSceneImportOptionsStaticMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxSceneImportOptionsStaticMesh")
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
pub struct UFbxSkeletalMeshImportData {
    #[doc(hidden)]
    pub(crate) __padding_234: [u8; 234],
    pub vertex_color_import_option: EVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    __padding_end: [u8; 24],
}
impl UFbxSkeletalMeshImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxSkeletalMeshImportData")
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
pub struct UFbxStaticMeshImportData {
    #[doc(hidden)]
    pub(crate) __padding_232: [u8; 232],
    pub static_mesh_lod_group: FName,
    pub vertex_color_import_option: EVertexColorImportOption,
    pub vertex_override_color: crate::bindings::core_u_object::FColor,
    pub flags_252: u8,
    pub distance_field_resolution_scale: f32,
}
impl UFbxStaticMeshImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxStaticMeshImportData")
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
pub struct UFbxTextureImportData {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub flags_176: u8,
    #[doc(hidden)]
    pub(crate) __padding_180: [u8; 3],
    pub material_search_location: EMaterialSearchLocation,
    pub base_material_name: crate::bindings::core_u_object::FSoftObjectPath,
    #[doc(hidden)]
    pub(crate) __padding_232: [u8; 8],
    pub base_color_name: FString,
    pub base_diffuse_texture_name: FString,
    pub base_normal_texture_name: FString,
    pub base_emissive_color_name: FString,
    pub base_emmisive_texture_name: FString,
    pub base_specular_texture_name: FString,
    pub base_opacity_texture_name: FString,
}
impl UFbxTextureImportData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxTextureImportData")
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
pub struct UReimportFbxSceneFactory {
    __padding_end: [u8; 464],
}
impl UReimportFbxSceneFactory {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UReimportFbxSceneFactory")
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
    pub fn script_reimport_helper(
        &mut self,
        obj: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_reimport_fbx_scene_factory_script_reimport_helper,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &obj,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_reimport_fbx_scene_factory_script_reimport_helper,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEditorLoadingAndSavingUtils {
    __padding_end: [u8; 48],
}
impl UEditorLoadingAndSavingUtils {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorLoadingAndSavingUtils")
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
    pub fn unload_packages(
        packages_to_unload: &TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
        b_out_any_packages_unloaded: &mut bool,
        out_error_message: &mut FText,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_unload_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                packages_to_unload,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_any_packages_unloaded,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error_message,
                __buffer.add(24).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_unload_packages,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_out_any_packages_unloaded);
        }
        unsafe {
            __buffer.add(24).cast::<FText>().swap(out_error_message);
        }
    }
    pub fn save_packages_with_dialog(
        packages_to_save: &TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
        b_only_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_packages_with_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                packages_to_save,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_packages_with_dialog,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn save_packages(
        packages_to_save: &TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
        b_only_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                packages_to_save,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_dirty,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_packages,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn save_map(
        world: UPtr<crate::bindings::engine::UWorld>,
        asset_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_map,
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
                &asset_path,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_map,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn save_dirty_packages_with_dialog(
        b_save_map_packages: bool,
        b_save_content_packages: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_dirty_packages_with_dialog,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_save_map_packages,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_save_content_packages,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_dirty_packages_with_dialog,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn save_dirty_packages(
        b_save_map_packages: bool,
        b_save_content_packages: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_dirty_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_save_map_packages,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_save_content_packages,
                __buffer.add(1).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_dirty_packages,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn save_current_level() -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_current_level,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_save_current_level,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn reload_packages(
        packages_to_reload: &TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
        b_out_any_packages_reloaded: &mut bool,
        out_error_message: &mut FText,
        interaction_mode: EReloadPackagesInteractionMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_reload_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                packages_to_reload,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_any_packages_reloaded,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_error_message,
                __buffer.add(24).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interaction_mode,
                __buffer.add(40).cast::<EReloadPackagesInteractionMode>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_reload_packages,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(b_out_any_packages_reloaded);
        }
        unsafe {
            __buffer.add(24).cast::<FText>().swap(out_error_message);
        }
    }
    pub fn new_map_from_template(
        path_to_template_level: FString,
        b_save_existing_map: bool,
    ) -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_new_map_from_template,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_to_template_level,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_save_existing_map,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_new_map_from_template,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<crate::bindings::engine::UWorld>>().read()
        }
    }
    pub fn new_blank_map(
        b_save_existing_map: bool,
    ) -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_new_blank_map,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_save_existing_map,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_new_blank_map,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<crate::bindings::engine::UWorld>>().read() }
    }
    pub fn load_map_with_dialog() -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_load_map_with_dialog,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_load_map_with_dialog,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>().read() }
    }
    pub fn load_map(filename: FString) -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_load_map,
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
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_load_map,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::UWorld>>().read()
        }
    }
    pub fn import_scene(filename: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_import_scene,
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
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_import_scene,
                __buffer,
            )
        };
    }
    pub fn get_dirty_map_packages(
        out_dirty_packages: &mut TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_get_dirty_map_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dirty_packages,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_get_dirty_map_packages,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>()
                .swap(out_dirty_packages);
        }
    }
    pub fn get_dirty_content_packages(
        out_dirty_packages: &mut TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_get_dirty_content_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dirty_packages,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_get_dirty_content_packages,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>()
                .swap(out_dirty_packages);
        }
    }
    pub fn fully_load_packages(
        packages: &TArray<UPtr<crate::bindings::core_u_object::UPackage>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_fully_load_packages,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                packages,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UPackage>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_fully_load_packages,
                __buffer,
            )
        };
    }
    pub fn fully_load_assets(
        assets: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_fully_load_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_fully_load_assets,
                __buffer,
            )
        };
    }
    pub fn export_scene(b_export_selected_actors_only: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_export_scene,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_export_selected_actors_only,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorLoadingAndSavingUtils::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_loading_and_saving_utils_export_scene,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct AGroupActor {
    __padding_end: [u8; 1176],
}
impl AGroupActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AGroupActor")
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
pub struct UHierarchicalLODSettings {
    __padding_end: [u8; 240],
}
impl UHierarchicalLODSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHierarchicalLODSettings")
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
pub struct AHierarchicalLODVolume {
    __padding_end: [u8; 1232],
}
impl AHierarchicalLODVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AHierarchicalLODVolume")
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
pub struct UEditorInstancedPlacementSettings {
    __padding_end: [u8; 800],
}
impl UEditorInstancedPlacementSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorInstancedPlacementSettings")
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
pub struct ULayersSubsystem {
    __padding_end: [u8; 136],
}
impl ULayersSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULayersSubsystem")
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
    pub fn update_all_view_visibility(&mut self, layer_that_changed: &FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_all_view_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_that_changed,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_all_view_visibility,
                __buffer,
            )
        };
    }
    pub fn update_all_actors_visibility(
        &mut self,
        b_notify_selection_change: bool,
        b_redraw_viewports: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_all_actors_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_notify_selection_change,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_redraw_viewports,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_all_actors_visibility,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn update_actor_visibility(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_out_selection_changed: &mut bool,
        b_out_actor_modified: &mut bool,
        b_notify_selection_change: bool,
        b_redraw_viewports: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_actor_visibility,
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
                b_out_selection_changed,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_actor_modified,
                __buffer.add(9).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_notify_selection_change,
                __buffer.add(10).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_redraw_viewports,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_actor_visibility,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<bool>().swap(b_out_selection_changed);
        }
        unsafe {
            __buffer.add(9).cast::<bool>().swap(b_out_actor_modified);
        }
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn update_actor_all_views_visibility(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_actor_all_views_visibility,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_update_actor_all_views_visibility,
                __buffer,
            )
        };
    }
    pub fn try_get_layer(
        &mut self,
        layer_name: &FName,
        out_layer: &mut UPtr<crate::bindings::engine::ULayer>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_try_get_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_layer,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::ULayer>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_try_get_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::ULayer>>()
                .swap(out_layer);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn toggle_layer_visibility(&mut self, layer_name: &FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_toggle_layer_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_toggle_layer_visibility,
                __buffer,
            )
        };
    }
    pub fn toggle_layers_visibility(&mut self, layer_names: &TArray<FName>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_toggle_layers_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_toggle_layers_visibility,
                __buffer,
            )
        };
    }
    pub fn set_layer_visibility(&mut self, layer_name: &FName, b_is_visible: bool) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_set_layer_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_visible,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_set_layer_visibility,
                __buffer,
            )
        };
    }
    pub fn set_layers_visibility(
        &mut self,
        layer_names: &TArray<FName>,
        b_is_visible: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_set_layers_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_visible,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_set_layers_visibility,
                __buffer,
            )
        };
    }
    pub fn select_actors_in_layers(
        &mut self,
        layer_names: &TArray<FName>,
        b_select: bool,
        b_notify: bool,
        b_select_even_if_hidden: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_select_actors_in_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(16).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_notify, __buffer.add(17).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_even_if_hidden,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_select_actors_in_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(19).cast::<bool>().read() }
    }
    pub fn select_actors_in_layer(
        &mut self,
        layer_name: &FName,
        b_select: bool,
        b_notify: bool,
        b_select_even_if_hidden: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_select_actors_in_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_select, __buffer.add(12).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_notify, __buffer.add(13).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_select_even_if_hidden,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_select_actors_in_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(15).cast::<bool>().read() }
    }
    pub fn rename_layer(
        &mut self,
        original_layer_name: &FName,
        new_layer_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_rename_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                original_layer_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_rename_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn remove_selected_actors_from_layers(
        &mut self,
        layer_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_selected_actors_from_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_selected_actors_from_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_selected_actors_from_layer(&mut self, layer_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_selected_actors_from_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_selected_actors_from_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn remove_level_layer_information(
        &mut self,
        level: UPtr<crate::bindings::engine::ULevel>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_level_layer_information,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevel>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_level_layer_information,
                __buffer,
            )
        };
    }
    pub fn remove_actors_from_layers(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        layer_names: &TArray<FName>,
        b_update_stats: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actors_from_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_update_stats,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actors_from_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn remove_actors_from_layer(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        layer_name: &FName,
        b_update_stats: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<30>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actors_from_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_update_stats,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actors_from_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(29).cast::<bool>().read() }
    }
    pub fn remove_actor_from_layers(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        layer_names: &TArray<FName>,
        b_update_stats: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actor_from_layers,
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
                layer_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_update_stats,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actor_from_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn remove_actor_from_layer(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        layer_to_remove: &FName,
        b_update_stats: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actor_from_layer,
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
                layer_to_remove,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_update_stats,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_remove_actor_from_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn make_all_layers_visible(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_make_all_layers_visible,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_make_all_layers_visible,
                __buffer,
            )
        };
    }
    pub fn is_layer(&mut self, layer_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_layers_subsystem_is_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_layers_subsystem_is_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn is_actor_valid_for_layer(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_is_actor_valid_for_layer,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_is_actor_valid_for_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn initialize_new_actor_layers(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_initialize_new_actor_layers,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_initialize_new_actor_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_world(&self) -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_layers_subsystem_get_world,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_layers_subsystem_get_world,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>().read() }
    }
    pub fn get_selected_actors(&self) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_get_selected_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_get_selected_actors,
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
    pub fn get_layer(
        &self,
        layer_name: &FName,
    ) -> UPtr<crate::bindings::engine::ULayer> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_layers_subsystem_get_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS.u_layers_subsystem_get_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::ULayer>>().read()
        }
    }
    pub fn get_actors_from_layers(
        &self,
        layer_names: &TArray<FName>,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_get_actors_from_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_get_actors_from_layers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn get_actors_from_layer(
        &self,
        layer_name: &FName,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_get_actors_from_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_get_actors_from_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn editor_refresh_layer_browser(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_editor_refresh_layer_browser,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_editor_refresh_layer_browser,
                __buffer,
            )
        };
    }
    pub fn editor_map_change(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_editor_map_change,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_editor_map_change,
                __buffer,
            )
        };
    }
    pub fn disassociate_actors_from_layers(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_disassociate_actors_from_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_disassociate_actors_from_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn disassociate_actor_from_layers(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_disassociate_actor_from_layers,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_disassociate_actor_from_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn delete_layers(&mut self, layers_to_delete: &TArray<FName>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_delete_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layers_to_delete,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_delete_layers,
                __buffer,
            )
        };
    }
    pub fn delete_layer(&mut self, layer_to_delete: &FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_delete_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_to_delete,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_delete_layer,
                __buffer,
            )
        };
    }
    pub fn create_layer(
        &mut self,
        layer_name: &FName,
    ) -> UPtr<crate::bindings::engine::ULayer> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_create_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_create_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::ULayer>>().read()
        }
    }
    pub fn append_actors_from_layers(
        &self,
        layer_names: &TArray<FName>,
        in_out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_append_actors_from_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
                __buffer.add(0).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_actors,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_append_actors_from_layers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(in_out_actors);
        }
    }
    pub fn append_actors_from_layer(
        &self,
        layer_name: &FName,
        in_out_actors: &mut TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_append_actors_from_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_out_actors,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_append_actors_from_layer,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .swap(in_out_actors);
        }
    }
    pub fn add_selected_actors_to_layers(
        &mut self,
        layer_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_selected_actors_to_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_selected_actors_to_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_selected_actors_to_layer(&mut self, layer_name: &FName) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_selected_actors_to_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_selected_actors_to_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn add_level_layer_information(
        &mut self,
        level: UPtr<crate::bindings::engine::ULevel>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_level_layer_information,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &level,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::ULevel>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_level_layer_information,
                __buffer,
            )
        };
    }
    pub fn add_all_layers_to(
        &self,
        out_layers: &mut TArray<UPtr<crate::bindings::engine::ULayer>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_all_layers_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_layers,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::ULayer>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_all_layers_to,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::ULayer>>>()
                .swap(out_layers);
        }
    }
    pub fn add_all_layer_names_to(&self, out_layer_names: &mut TArray<FName>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_all_layer_names_to,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_layer_names,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_all_layer_names_to,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FName>>().swap(out_layer_names);
        }
    }
    pub fn add_actor_to_layers(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        layer_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actor_to_layers,
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
                layer_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actor_to_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn add_actor_to_layer(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        layer_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actor_to_layer,
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
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actor_to_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn add_actors_to_layers(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        layer_names: &TArray<FName>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actors_to_layers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actors_to_layers,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_actors_to_layer(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        layer_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actors_to_layer,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                layer_name,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_layers_subsystem_add_actors_to_layer,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct ULevelEditorDragDropHandler {
    __padding_end: [u8; 104],
}
impl ULevelEditorDragDropHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorDragDropHandler")
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
pub struct ULightmassOptionsObject {
    __padding_end: [u8; 72],
}
impl ULightmassOptionsObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULightmassOptionsObject")
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
pub struct UMaterialEditorMeshComponent {
    __padding_end: [u8; 1888],
}
impl UMaterialEditorMeshComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialEditorMeshComponent")
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
pub struct UMaterialGraph {
    __padding_end: [u8; 328],
}
impl UMaterialGraph {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraph")
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
pub struct UMaterialGraphNode_Base {
    __padding_end: [u8; 192],
}
impl UMaterialGraphNode_Base {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_Base")
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
pub struct UMaterialGraphNode {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode")
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
pub struct UMaterialGraphNode_Comment {
    __padding_end: [u8; 328],
}
impl UMaterialGraphNode_Comment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_Comment")
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
pub struct UMaterialGraphNode_Composite {
    __padding_end: [u8; 288],
}
impl UMaterialGraphNode_Composite {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_Composite")
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
pub struct UMaterialGraphNode_Custom {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_Custom {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_Custom")
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
pub struct UMaterialGraphNode_Knot {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_Knot {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_Knot")
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
pub struct UMaterialGraphNode_Operator {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_Operator {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_Operator")
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
pub struct UMaterialGraphNode_PinBase {
    __padding_end: [u8; 280],
}
impl UMaterialGraphNode_PinBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_PinBase")
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
pub struct UMaterialGraphNode_Root {
    __padding_end: [u8; 200],
}
impl UMaterialGraphNode_Root {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphNode_Root")
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
pub struct UMaterialGraphSchema {
    __padding_end: [u8; 48],
}
impl UMaterialGraphSchema {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialGraphSchema")
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
pub struct UUndoableResolveHandler {
    __padding_end: [u8; 128],
}
impl UUndoableResolveHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUndoableResolveHandler")
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
pub struct UPackageTools {
    __padding_end: [u8; 48],
}
impl UPackageTools {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPackageTools")
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
    pub fn sanitize_package_name(in_package_name: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_package_tools_sanitize_package_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_package_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UPackageTools::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_package_tools_sanitize_package_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn package_name_to_filename(
        package_name: FString,
        extension: FString,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_package_tools_package_name_to_filename,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &package_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &extension,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UPackageTools::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_package_tools_package_name_to_filename,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FString>().read() }
    }
    pub fn filename_to_package_name(filename: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_package_tools_filename_to_package_name,
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
        let __object_ptr = crate::bindings::unreal_ed::UPackageTools::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_package_tools_filename_to_package_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
}
#[repr(C, align(8))]
pub struct UPreviewMaterial {
    __padding_end: [u8; 3400],
}
impl UPreviewMaterial {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPreviewMaterial")
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
pub struct UPropertyEditorTestInstancedObject {
    __padding_end: [u8; 56],
}
impl UPropertyEditorTestInstancedObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyEditorTestInstancedObject")
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
pub struct UFirstDerivedPropertyEditorTestObject {
    __padding_end: [u8; 72],
}
impl UFirstDerivedPropertyEditorTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFirstDerivedPropertyEditorTestObject")
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
pub struct USecondDerivedPropertyEditorTestObject {
    __padding_end: [u8; 64],
}
impl USecondDerivedPropertyEditorTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USecondDerivedPropertyEditorTestObject")
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
pub struct UPropertyEditorTestObject {
    __padding_end: [u8; 5008],
}
impl UPropertyEditorTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyEditorTestObject")
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
pub struct UHideCategoriesBase {
    __padding_end: [u8; 56],
}
impl UHideCategoriesBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UHideCategoriesBase")
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
pub struct UShowCategoriesTest {
    __padding_end: [u8; 64],
}
impl UShowCategoriesTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UShowCategoriesTest")
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
pub struct UBlueprintPropertyTestObject {
    __padding_end: [u8; 72],
}
impl UBlueprintPropertyTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintPropertyTestObject")
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
pub struct UBlueprintPropertyContainerTestObject {
    __padding_end: [u8; 64],
}
impl UBlueprintPropertyContainerTestObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintPropertyContainerTestObject")
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
pub struct UTestSparseClassDataBase {
    __padding_end: [u8; 48],
}
impl UTestSparseClassDataBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestSparseClassDataBase")
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
pub struct UTestSparseClassData {
    __padding_end: [u8; 48],
}
impl UTestSparseClassData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTestSparseClassData")
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
pub struct APropertyEditorTestActor {
    __padding_end: [u8; 1264],
}
impl APropertyEditorTestActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("APropertyEditorTestActor")
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
pub struct UPropertyEditorRowGeneratorTest {
    __padding_end: [u8; 96],
}
impl UPropertyEditorRowGeneratorTest {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyEditorRowGeneratorTest")
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
pub struct UUnrealEdViewportToolbarContext {
    __padding_end: [u8; 176],
}
impl UUnrealEdViewportToolbarContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnrealEdViewportToolbarContext")
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
pub struct UCommonViewportToolbarBaseMenuContext {
    __padding_end: [u8; 192],
}
impl UCommonViewportToolbarBaseMenuContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCommonViewportToolbarBaseMenuContext")
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
pub struct USelection {
    __padding_end: [u8; 72],
}
impl USelection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USelection")
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
pub struct UBlueprintEditorProjectSettings {
    __padding_end: [u8; 216],
}
impl UBlueprintEditorProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlueprintEditorProjectSettings")
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
pub struct UEditorPerProjectUserSettings {
    __padding_end: [u8; 320],
}
impl UEditorPerProjectUserSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorPerProjectUserSettings")
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
pub struct UEditorProjectAppearanceSettings {
    __padding_end: [u8; 184],
}
impl UEditorProjectAppearanceSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorProjectAppearanceSettings")
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
pub struct ULevelEditor2DSettings {
    __padding_end: [u8; 128],
}
impl ULevelEditor2DSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditor2DSettings")
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
pub struct ULevelEditorProjectSettings {
    __padding_end: [u8; 112],
}
impl ULevelEditorProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelEditorProjectSettings")
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
pub struct UEditorPerformanceProjectSettings {
    __padding_end: [u8; 136],
}
impl UEditorPerformanceProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorPerformanceProjectSettings")
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
pub struct UEditorProjectAssetSettings {
    __padding_end: [u8; 112],
}
impl UEditorProjectAssetSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorProjectAssetSettings")
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
pub struct UDDCProjectSettings {
    __padding_end: [u8; 112],
}
impl UDDCProjectSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDDCProjectSettings")
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
pub struct UEditorSettings {
    __padding_end: [u8; 360],
}
impl UEditorSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorSettings")
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
pub struct UActorEditorContextSubsystem {
    __padding_end: [u8; 128],
}
impl UActorEditorContextSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorEditorContextSubsystem")
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
pub struct UAssetEditorSubsystem {
    __padding_end: [u8; 1328],
}
impl UAssetEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetEditorSubsystem")
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
    pub fn open_editor_for_assets(
        &mut self,
        assets: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        opened_method: crate::bindings::asset_tools::EAssetTypeActivationOpenedMethod,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_editor_subsystem_open_editor_for_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &opened_method,
                __buffer
                    .add(16)
                    .cast::<
                        crate::bindings::asset_tools::EAssetTypeActivationOpenedMethod,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_editor_subsystem_open_editor_for_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn close_all_editors_for_asset(
        &mut self,
        asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_editor_subsystem_close_all_editors_for_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_asset_editor_subsystem_close_all_editors_for_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UBrowseToAssetOverrideSubsystem {
    __padding_end: [u8; 216],
}
impl UBrowseToAssetOverrideSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrowseToAssetOverrideSubsystem")
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
pub struct UBrushEditingSubsystem {
    __padding_end: [u8; 56],
}
impl UBrushEditingSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBrushEditingSubsystem")
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
pub struct UCollectionManagerScriptingSubsystem {
    __padding_end: [u8; 56],
}
impl UCollectionManagerScriptingSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCollectionManagerScriptingSubsystem")
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
    pub fn reparent_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        new_parent_collection: FCollectionScriptingRef,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_reparent_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_parent_collection,
                __buffer.add(28).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_reparent_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn rename_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        new_name: FName,
        new_share_type: crate::bindings::engine::ECollectionScriptingShareType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<42>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_rename_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_share_type,
                __buffer
                    .add(40)
                    .cast::<crate::bindings::engine::ECollectionScriptingShareType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_rename_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(41).cast::<bool>().read() }
    }
    pub fn remove_assets_from_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_paths: &TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_assets_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_paths,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FSoftObjectPath>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_assets_from_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn remove_asset_ptrs_from_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_ptrs: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_ptrs_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_ptrs,
                __buffer
                    .add(32)
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_ptrs_from_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn remove_asset_ptr_from_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_ptr: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_ptr_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_ptr,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_ptr_from_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn remove_asset_from_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_path: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path,
                __buffer
                    .add(32)
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_from_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn remove_asset_datas_from_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_datas: &TArray<crate::bindings::core_u_object::FAssetData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_datas_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_datas,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_datas_from_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn remove_asset_data_from_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<185>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_data_from_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_remove_asset_data_from_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(184).cast::<bool>().read() }
    }
    pub fn get_collections_containing_asset_ptr(
        &mut self,
        container: FCollectionScriptingContainerSource,
        asset_ptr: UPtr<crate::bindings::core_u_object::UObject>,
        out_collections: &mut TArray<FCollectionScriptingRef>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_containing_asset_ptr,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_ptr,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_collections,
                __buffer.add(40).cast::<TArray<FCollectionScriptingRef>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_containing_asset_ptr,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<FCollectionScriptingRef>>()
                .swap(out_collections);
        }
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn get_collections_containing_asset_data(
        &mut self,
        container: FCollectionScriptingContainerSource,
        asset_data: &crate::bindings::core_u_object::FAssetData,
        out_collections: &mut TArray<FCollectionScriptingRef>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<201>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_containing_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_collections,
                __buffer.add(184).cast::<TArray<FCollectionScriptingRef>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_containing_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(184)
                .cast::<TArray<FCollectionScriptingRef>>()
                .swap(out_collections);
        }
        unsafe { __buffer.add(200).cast::<bool>().read() }
    }
    pub fn get_collections_containing_asset(
        &mut self,
        container: FCollectionScriptingContainerSource,
        asset_path: &crate::bindings::core_u_object::FSoftObjectPath,
        out_collections: &mut TArray<FCollectionScriptingRef>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<89>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_containing_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::core_u_object::FSoftObjectPath>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_collections,
                __buffer.add(72).cast::<TArray<FCollectionScriptingRef>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_containing_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(72)
                .cast::<TArray<FCollectionScriptingRef>>()
                .swap(out_collections);
        }
        unsafe { __buffer.add(88).cast::<bool>().read() }
    }
    pub fn get_collections_by_name(
        &mut self,
        container: FCollectionScriptingContainerSource,
        collection: FName,
        out_collections: &mut TArray<FCollectionScriptingRef>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collection,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_collections,
                __buffer.add(48).cast::<TArray<FCollectionScriptingRef>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<FCollectionScriptingRef>>()
                .swap(out_collections);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn get_collections(
        &mut self,
        container: FCollectionScriptingContainerSource,
        out_collections: &mut TArray<FCollectionScriptingRef>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_collections,
                __buffer.add(32).cast::<TArray<FCollectionScriptingRef>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collections,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<FCollectionScriptingRef>>()
                .swap(out_collections);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_collection_containers(
        &mut self,
    ) -> TArray<FCollectionScriptingContainerSource> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collection_containers,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_collection_containers,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FCollectionScriptingContainerSource>>().read()
        }
    }
    pub fn get_base_game_collection_container(
        &self,
    ) -> FCollectionScriptingContainerSource {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_base_game_collection_container,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_base_game_collection_container,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FCollectionScriptingContainerSource>().read() }
    }
    pub fn get_assets_in_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        out_assets: &mut TArray<crate::bindings::core_u_object::FAssetData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_assets_in_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_assets,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_get_assets_in_collection,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(out_assets);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn empty_collection(&mut self, collection: &FCollectionScriptingRef) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_empty_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_empty_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn destroy_collection(&mut self, collection: &FCollectionScriptingRef) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_destroy_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_destroy_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn create_or_empty_collection(
        &mut self,
        container: FCollectionScriptingContainerSource,
        collection: FName,
        share_type: crate::bindings::engine::ECollectionScriptingShareType,
        out_new_or_empty_collection: &mut FCollectionScriptingRef,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<77>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_create_or_empty_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collection,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &share_type,
                __buffer
                    .add(44)
                    .cast::<crate::bindings::engine::ECollectionScriptingShareType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_new_or_empty_collection,
                __buffer.add(48).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_create_or_empty_collection,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<FCollectionScriptingRef>()
                .swap(out_new_or_empty_collection);
        }
        unsafe { __buffer.add(76).cast::<bool>().read() }
    }
    pub fn create_collection(
        &mut self,
        container: FCollectionScriptingContainerSource,
        collection: FName,
        share_type: crate::bindings::engine::ECollectionScriptingShareType,
        out_new_collection: &mut FCollectionScriptingRef,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<77>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_create_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collection,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &share_type,
                __buffer
                    .add(44)
                    .cast::<crate::bindings::engine::ECollectionScriptingShareType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_new_collection,
                __buffer.add(48).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_create_collection,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(48).cast::<FCollectionScriptingRef>().swap(out_new_collection);
        }
        unsafe { __buffer.add(76).cast::<bool>().read() }
    }
    pub fn collection_exists(
        &mut self,
        container: FCollectionScriptingContainerSource,
        collection: FName,
        share_type: crate::bindings::engine::ECollectionScriptingShareType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<46>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_collection_exists,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &container,
                __buffer.add(0).cast::<FCollectionScriptingContainerSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collection,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &share_type,
                __buffer
                    .add(44)
                    .cast::<crate::bindings::engine::ECollectionScriptingShareType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_collection_exists,
                __buffer,
            )
        };
        unsafe { __buffer.add(45).cast::<bool>().read() }
    }
    pub fn add_asset_to_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_path: &crate::bindings::core_u_object::FSoftObjectPath,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<73>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_to_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_path,
                __buffer
                    .add(32)
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_to_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(72).cast::<bool>().read() }
    }
    pub fn add_assets_to_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_paths: &TArray<crate::bindings::core_u_object::FSoftObjectPath>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_assets_to_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_paths,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FSoftObjectPath>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_assets_to_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn add_asset_ptr_to_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_ptr: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_ptr_to_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_ptr,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_ptr_to_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
    pub fn add_asset_ptrs_to_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_ptrs: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_ptrs_to_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_ptrs,
                __buffer
                    .add(32)
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_ptrs_to_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn add_asset_data_to_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_data: &crate::bindings::core_u_object::FAssetData,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<185>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_data_to_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_data,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FAssetData>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_data_to_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(184).cast::<bool>().read() }
    }
    pub fn add_asset_datas_to_collection(
        &mut self,
        collection: &FCollectionScriptingRef,
        asset_datas: &TArray<crate::bindings::core_u_object::FAssetData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_datas_to_collection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                collection,
                __buffer.add(0).cast::<FCollectionScriptingRef>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_datas,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_collection_manager_scripting_subsystem_add_asset_datas_to_collection,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UEditorActorSubsystem {
    __padding_end: [u8; 368],
}
impl UEditorActorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorActorSubsystem")
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
    pub fn spawn_actor_from_object(
        &mut self,
        object_to_use: UPtr<crate::bindings::core_u_object::UObject>,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        b_transient: bool,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_spawn_actor_from_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_use,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transient,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_spawn_actor_from_object,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn spawn_actor_from_class(
        &mut self,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        b_transient: bool,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_spawn_actor_from_class,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transient,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_spawn_actor_from_class,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(64).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn set_selected_level_actors(
        &mut self,
        actors_to_select: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_selected_level_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_select,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_selected_level_actors,
                __buffer,
            )
        };
    }
    pub fn set_component_transform(
        &mut self,
        in_scene_component: UPtr<crate::bindings::engine::USceneComponent>,
        in_world_transform: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_component_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_scene_component,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_world_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_component_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<bool>().read() }
    }
    pub fn set_actor_transform(
        &mut self,
        in_actor: UPtr<crate::bindings::engine::AActor>,
        in_world_transform: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<113>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_actor_transform,
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
                in_world_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_actor_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(112).cast::<bool>().read() }
    }
    pub fn set_actor_selection_state(
        &mut self,
        actor: UPtr<crate::bindings::engine::AActor>,
        b_should_be_selected: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_actor_selection_state,
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
                &b_should_be_selected,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_set_actor_selection_state,
                __buffer,
            )
        };
    }
    pub fn select_nothing(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_select_nothing,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_select_nothing,
                __buffer,
            )
        };
    }
    pub fn select_all_children(&mut self, b_recurse_children: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_select_all_children,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_recurse_children,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_select_all_children,
                __buffer,
            )
        };
    }
    pub fn select_all(&mut self, in_world: UPtr<crate::bindings::engine::UWorld>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_select_all,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_select_all,
                __buffer,
            )
        };
    }
    pub fn invert_selection(&mut self, in_world: UPtr<crate::bindings::engine::UWorld>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_invert_selection,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_invert_selection,
                __buffer,
            )
        };
    }
    pub fn get_selected_level_actors(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_selected_level_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_selected_level_actors,
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
    pub fn get_all_level_actors_components(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::engine::UActorComponent>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_all_level_actors_components,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_all_level_actors_components,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<UPtr<crate::bindings::engine::UActorComponent>>>()
                .read()
        }
    }
    pub fn get_all_level_actors(
        &mut self,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_all_level_actors,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_all_level_actors,
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
    pub fn get_actor_reference(
        &mut self,
        path_to_actor: FString,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_actor_reference,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &path_to_actor,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_get_actor_reference,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn duplicate_selected_actors(
        &mut self,
        in_world: UPtr<crate::bindings::engine::UWorld>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_duplicate_selected_actors,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_duplicate_selected_actors,
                __buffer,
            )
        };
    }
    pub fn duplicate_actors(
        &mut self,
        actors_to_duplicate: &TArray<UPtr<crate::bindings::engine::AActor>>,
        to_world: UPtr<crate::bindings::engine::UWorld>,
        offset: crate::bindings::core_u_object::FVector,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_duplicate_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_duplicate,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_world,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_duplicate_actors,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(48)
                .cast::<TArray<UPtr<crate::bindings::engine::AActor>>>()
                .read()
        }
    }
    pub fn duplicate_actor(
        &mut self,
        actor_to_duplicate: UPtr<crate::bindings::engine::AActor>,
        to_world: UPtr<crate::bindings::engine::UWorld>,
        offset: crate::bindings::core_u_object::FVector,
    ) -> UPtr<crate::bindings::engine::AActor> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_duplicate_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_to_duplicate,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_world,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UWorld>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_duplicate_actor,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<UPtr<crate::bindings::engine::AActor>>().read()
        }
    }
    pub fn destroy_actors(
        &mut self,
        actors_to_destroy: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_destroy_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors_to_destroy,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_destroy_actors,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn destroy_actor(
        &mut self,
        actor_to_destroy: UPtr<crate::bindings::engine::AActor>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_destroy_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_to_destroy,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_destroy_actor,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn delete_selected_actors(
        &mut self,
        in_world: UPtr<crate::bindings::engine::UWorld>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_delete_selected_actors,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_delete_selected_actors,
                __buffer,
            )
        };
    }
    pub fn convert_actors(
        &mut self,
        actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
        actor_class: TSubclassOf<crate::bindings::engine::AActor>,
        static_mesh_package_path: FString,
    ) -> TArray<UPtr<crate::bindings::engine::AActor>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_convert_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                actors,
                __buffer.add(0).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor_class,
                __buffer.add(16).cast::<TSubclassOf<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh_package_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_convert_actors,
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
    pub fn clear_actor_selection_set(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_clear_actor_selection_set,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_actor_subsystem_clear_actor_selection_set,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEditorAssetSubsystem {
    __padding_end: [u8; 104],
}
impl UEditorAssetSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorAssetSubsystem")
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
    pub fn sort_by_meta_data(
        &mut self,
        assets: &mut TArray<crate::bindings::core_u_object::FAssetData>,
        meta_data_tag: FName,
        meta_data_type: EEditorAssetMetaDataSortType,
        sort_order: EEditorAssetSortOrder,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<31>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_sort_by_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FAssetData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_tag,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_type,
                __buffer.add(28).cast::<EEditorAssetMetaDataSortType>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sort_order,
                __buffer.add(29).cast::<EEditorAssetSortOrder>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_sort_by_meta_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .swap(assets);
        }
        unsafe { __buffer.add(30).cast::<bool>().read() }
    }
    pub fn set_metadata_tag(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        tag: FName,
        value: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_set_metadata_tag,
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
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<FString>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_set_metadata_tag,
                __buffer,
            )
        };
    }
    pub fn set_dirty_flag(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        b_dirty_state: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_set_dirty_flag,
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
                &b_dirty_state,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_set_dirty_flag,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn save_loaded_assets(
        &mut self,
        assets_to_save: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
        b_only_if_is_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_loaded_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_save,
                __buffer
                    .add(0)
                    .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_loaded_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn save_loaded_asset(
        &mut self,
        asset_to_save: UPtr<crate::bindings::core_u_object::UObject>,
        b_only_if_is_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<10>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_save,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_loaded_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(9).cast::<bool>().read() }
    }
    pub fn save_directory(
        &mut self,
        directory_path: FString,
        b_only_if_is_dirty: bool,
        b_recursive: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<19>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_directory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_directory,
                __buffer,
            )
        };
        unsafe { __buffer.add(18).cast::<bool>().read() }
    }
    pub fn save_asset(
        &mut self,
        asset_to_save: FString,
        b_only_if_is_dirty: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_save,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_only_if_is_dirty,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_save_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn rename_loaded_asset(
        &mut self,
        source_asset: UPtr<crate::bindings::core_u_object::UObject>,
        destination_asset_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_rename_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_rename_loaded_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn rename_directory(
        &mut self,
        source_directory_path: FString,
        destination_directory_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_rename_directory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_directory_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_rename_directory,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn rename_asset(
        &mut self,
        source_asset_path: FString,
        destination_asset_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_rename_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_rename_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_on_extract_asset_from_file(
        &mut self,
        delegate: FRemoveOnExtractAssetFromFile_Delegate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_remove_on_extract_asset_from_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delegate,
                __buffer.add(0).cast::<FRemoveOnExtractAssetFromFile_Delegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_remove_on_extract_asset_from_file,
                __buffer,
            )
        };
    }
    pub fn remove_metadata_tag(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        tag: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_remove_metadata_tag,
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
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(8).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_remove_metadata_tag,
                __buffer,
            )
        };
    }
    pub fn make_directory(&mut self, directory_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_make_directory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_make_directory,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn load_blueprint_class(
        &mut self,
        asset_path: FString,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_load_blueprint_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_load_blueprint_class,
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
    pub fn load_asset(
        &mut self,
        asset_path: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_load_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_load_asset,
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
    pub fn list_assets_by_tag_value(
        &mut self,
        tag_name: FName,
        tag_value: FString,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_list_assets_by_tag_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tag_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tag_value,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_list_assets_by_tag_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<FString>>().read() }
    }
    pub fn list_assets(
        &mut self,
        directory_path: FString,
        b_recursive: bool,
        b_include_folder: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_list_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
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
                &b_include_folder,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_list_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn get_tag_values(&mut self, asset_path: FString) -> TMap<FName, FString> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_tag_values,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_tag_values,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<TMap<FName, FString>>().read() }
    }
    pub fn get_path_name_for_loaded_asset(
        &mut self,
        loaded_asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_path_name_for_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &loaded_asset,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_path_name_for_loaded_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_metadata_tag_values(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TMap<FName, FString> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_metadata_tag_values,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_metadata_tag_values,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<TMap<FName, FString>>().read() }
    }
    pub fn get_metadata_tag(
        &mut self,
        object: UPtr<crate::bindings::core_u_object::UObject>,
        tag: FName,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_metadata_tag,
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
            std::ptr::copy_nonoverlapping(&tag, __buffer.add(8).cast::<FName>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_metadata_tag,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_loaded_asset_filename_length_for_cooking(
        &mut self,
        asset: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_loaded_asset_filename_length_for_cooking,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_loaded_asset_filename_length_for_cooking,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_asset_filename_length_for_cooking(&mut self, asset_path: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_asset_filename_length_for_cooking,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_asset_filename_length_for_cooking,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_all_assets_by_meta_data_tags(
        &mut self,
        required_tags: &TSet<FName>,
        allowed_classes: &TSet<TSubclassOf<crate::bindings::core_u_object::UObject>>,
    ) -> TArray<crate::bindings::core_u_object::FAssetData> {
        let mut __stack = crate::core_data::StackAlloc::<176>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_all_assets_by_meta_data_tags,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                required_tags,
                __buffer.add(0).cast::<TSet<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                allowed_classes,
                __buffer
                    .add(80)
                    .cast::<
                        TSet<TSubclassOf<crate::bindings::core_u_object::UObject>>,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_get_all_assets_by_meta_data_tags,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(160)
                .cast::<TArray<crate::bindings::core_u_object::FAssetData>>()
                .read()
        }
    }
    pub fn find_package_referencers_for_asset(
        &mut self,
        asset_path: FString,
        b_load_assets_to_confirm: bool,
    ) -> TArray<FString> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_find_package_referencers_for_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_load_assets_to_confirm,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_find_package_referencers_for_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<FString>>().read() }
    }
    pub fn find_asset_data(
        &mut self,
        asset_path: FString,
    ) -> crate::bindings::core_u_object::FAssetData {
        let mut __stack = crate::core_data::StackAlloc::<168>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_find_asset_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_find_asset_data,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FAssetData>().read()
        }
    }
    pub fn duplicate_loaded_asset(
        &mut self,
        source_asset: UPtr<crate::bindings::core_u_object::UObject>,
        destination_asset_path: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_duplicate_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_duplicate_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn duplicate_directory(
        &mut self,
        source_directory_path: FString,
        destination_directory_path: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_duplicate_directory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_directory_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_directory_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_duplicate_directory,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn duplicate_asset(
        &mut self,
        source_asset_path: FString,
        destination_asset_path: FString,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_duplicate_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_asset_path,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_duplicate_asset,
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
    pub fn does_directory_exist(&mut self, directory_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_does_directory_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_does_directory_exist,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn does_directory_contain_assets(
        &mut self,
        directory_path: FString,
        b_recursive: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_does_directory_contain_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_does_directory_contain_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn does_asset_exist(&mut self, asset_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_does_asset_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_does_asset_exist,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn do_assets_exist(&mut self, asset_paths: &TArray<FString>) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_do_assets_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                asset_paths,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_do_assets_exist,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_loaded_assets(
        &mut self,
        assets_to_delete: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_loaded_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_delete,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_loaded_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_loaded_asset(
        &mut self,
        asset_to_delete: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_delete,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_loaded_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn delete_directory(&mut self, directory_path: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_directory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_directory,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn delete_asset(&mut self, asset_path_to_delete: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_path_to_delete,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_delete_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn consolidate_assets(
        &mut self,
        asset_to_consolidate_to: UPtr<crate::bindings::core_u_object::UObject>,
        assets_to_consolidate: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_consolidate_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_consolidate_to,
                __buffer.add(0).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_consolidate,
                __buffer
                    .add(8)
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_consolidate_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn checkout_loaded_assets(
        &mut self,
        assets_to_checkout: &TArray<UPtr<crate::bindings::core_u_object::UObject>>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_loaded_assets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                assets_to_checkout,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_loaded_assets,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn checkout_loaded_asset(
        &mut self,
        asset_to_checkout: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_loaded_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_checkout,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_loaded_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn checkout_directory(
        &mut self,
        directory_path: FString,
        b_recursive: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_directory,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &directory_path,
                __buffer.add(0).cast::<FString>(),
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_directory,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn checkout_asset(&mut self, asset_to_checkout: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &asset_to_checkout,
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
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_checkout_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_on_extract_asset_from_file(
        &mut self,
        delegate: FAddOnExtractAssetFromFile_Delegate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_add_on_extract_asset_from_file,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delegate,
                __buffer.add(0).cast::<FAddOnExtractAssetFromFile_Delegate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_asset_subsystem_add_on_extract_asset_from_file,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UEditorSubsystemBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UEditorSubsystemBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEditorSubsystemBlueprintLibrary")
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
    pub fn toggle_preview_platform() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_toggle_preview_platform,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::unreal_ed::UEditorSubsystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_toggle_preview_platform,
                __buffer,
            )
        };
    }
    pub fn set_preview_platform(preview_shader_platform_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_set_preview_platform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_shader_platform_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorSubsystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_set_preview_platform,
                __buffer,
            )
        };
    }
    pub fn get_editor_subsystem(
        class: TSubclassOf<crate::bindings::editor_subsystem::UEditorSubsystem>,
    ) -> UPtr<crate::bindings::editor_subsystem::UEditorSubsystem> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_get_editor_subsystem,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class,
                __buffer
                    .add(0)
                    .cast::<
                        TSubclassOf<crate::bindings::editor_subsystem::UEditorSubsystem>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::unreal_ed::UEditorSubsystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_get_editor_subsystem,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::editor_subsystem::UEditorSubsystem>>()
                .read()
        }
    }
    pub fn disable_preview_platform() {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_disable_preview_platform,
                __buffer,
            )
        };
        let __object_ptr = crate::bindings::unreal_ed::UEditorSubsystemBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_editor_subsystem_blueprint_library_disable_preview_platform,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UImportSubsystem {
    __padding_end: [u8; 304],
}
impl UImportSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImportSubsystem")
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
pub struct UPanelExtensionSubsystem {
    __padding_end: [u8; 296],
}
impl UPanelExtensionSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPanelExtensionSubsystem")
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
pub struct UPropertyVisibilityOverrideSubsystem {
    __padding_end: [u8; 136],
}
impl UPropertyVisibilityOverrideSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPropertyVisibilityOverrideSubsystem")
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
pub struct UUnrealEditorSubsystem {
    __padding_end: [u8; 56],
}
impl UUnrealEditorSubsystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnrealEditorSubsystem")
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
    pub fn set_level_viewport_camera_info(
        &mut self,
        camera_location: crate::bindings::core_u_object::FVector,
        camera_rotation: crate::bindings::core_u_object::FRotator,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_set_level_viewport_camera_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &camera_rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_set_level_viewport_camera_info,
                __buffer,
            )
        };
    }
    pub fn get_level_viewport_camera_info(
        &mut self,
        camera_location: &mut crate::bindings::core_u_object::FVector,
        camera_rotation: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_get_level_viewport_camera_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_location,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                camera_rotation,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_get_level_viewport_camera_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(camera_location);
        }
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(camera_rotation);
        }
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn get_game_world(&mut self) -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_get_game_world,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_get_game_world,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>().read() }
    }
    pub fn get_editor_world(&mut self) -> UPtr<crate::bindings::engine::UWorld> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_get_editor_world,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::unreal_ed::__FUNCTION_PTRS
                    .u_unreal_editor_subsystem_get_editor_world,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<crate::bindings::engine::UWorld>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_TemplateMapMetadata {
    __padding_end: [u8; 48],
}
impl UDEPRECATED_TemplateMapMetadata {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_TemplateMapMetadata")
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
pub struct UFbxTestPlan {
    __padding_end: [u8; 104],
}
impl UFbxTestPlan {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFbxTestPlan")
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
pub struct AAnimationThumbnailSkeletalMeshActor {
    __padding_end: [u8; 1264],
}
impl AAnimationThumbnailSkeletalMeshActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("AAnimationThumbnailSkeletalMeshActor")
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
pub struct UThumbnailManager {
    __padding_end: [u8; 336],
}
impl UThumbnailManager {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UThumbnailManager")
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
pub struct UAnimBlueprintThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UAnimBlueprintThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimBlueprintThumbnailRenderer")
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
pub struct UAnimSequenceThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UAnimSequenceThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimSequenceThumbnailRenderer")
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
pub struct UBlendSpaceThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UBlendSpaceThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBlendSpaceThumbnailRenderer")
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
pub struct UClassThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl UClassThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UClassThumbnailRenderer")
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
pub struct UCurveFloatThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UCurveFloatThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveFloatThumbnailRenderer")
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
pub struct UCurveVector3ThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UCurveVector3ThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveVector3ThumbnailRenderer")
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
pub struct UCurveLinearColorThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UCurveLinearColorThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCurveLinearColorThumbnailRenderer")
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
pub struct UFontThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UFontThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UFontThumbnailRenderer")
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
pub struct ULevelThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl ULevelThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelThumbnailRenderer")
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
pub struct UMaterialFunctionThumbnailRenderer {
    __padding_end: [u8; 80],
}
impl UMaterialFunctionThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialFunctionThumbnailRenderer")
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
pub struct UMaterialInstanceThumbnailRenderer {
    __padding_end: [u8; 80],
}
impl UMaterialInstanceThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialInstanceThumbnailRenderer")
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
pub struct UNeuralProfileRenderer {
    __padding_end: [u8; 48],
}
impl UNeuralProfileRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNeuralProfileRenderer")
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
pub struct UParticleSystemThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UParticleSystemThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UParticleSystemThumbnailRenderer")
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
pub struct UPhysicalMaterialMaskThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UPhysicalMaterialMaskThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicalMaterialMaskThumbnailRenderer")
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
pub struct UPhysicsAssetThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UPhysicsAssetThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPhysicsAssetThumbnailRenderer")
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
pub struct USceneThumbnailInfo {
    __padding_end: [u8; 64],
}
impl USceneThumbnailInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USceneThumbnailInfo")
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
pub struct USceneThumbnailInfoWithPrimitive {
    __padding_end: [u8; 120],
}
impl USceneThumbnailInfoWithPrimitive {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USceneThumbnailInfoWithPrimitive")
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
pub struct USkeletalMeshThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl USkeletalMeshThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshThumbnailRenderer")
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
pub struct USkeletonThumbnailRenderer {
    __padding_end: [u8; 136],
}
impl USkeletonThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletonThumbnailRenderer")
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
pub struct USlateBrushThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl USlateBrushThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USlateBrushThumbnailRenderer")
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
pub struct USoundWaveThumbnailRenderer {
    __padding_end: [u8; 56],
}
impl USoundWaveThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USoundWaveThumbnailRenderer")
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
pub struct USpecularProfileRenderer {
    __padding_end: [u8; 48],
}
impl USpecularProfileRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpecularProfileRenderer")
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
pub struct UStaticMeshThumbnailRenderer {
    __padding_end: [u8; 64],
}
impl UStaticMeshThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshThumbnailRenderer")
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
pub struct USubsurfaceProfileRenderer {
    __padding_end: [u8; 48],
}
impl USubsurfaceProfileRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USubsurfaceProfileRenderer")
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
pub struct UTextureCubeArrayThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTextureCubeArrayThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureCubeArrayThumbnailRenderer")
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
pub struct UTextureCubeThumbnailRenderer {
    __padding_end: [u8; 48],
}
impl UTextureCubeThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureCubeThumbnailRenderer")
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
pub struct UVolumeTextureThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UVolumeTextureThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumeTextureThumbnailRenderer")
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
pub struct UWorldThumbnailInfo {
    __padding_end: [u8; 72],
}
impl UWorldThumbnailInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldThumbnailInfo")
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
pub struct UWorldThumbnailRenderer {
    __padding_end: [u8; 72],
}
impl UWorldThumbnailRenderer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldThumbnailRenderer")
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
pub struct UAssetEditorContextObject {
    __padding_end: [u8; 64],
}
impl UAssetEditorContextObject {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetEditorContextObject")
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
pub struct UEdMode {
    __padding_end: [u8; 312],
}
impl UEdMode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS.wait().name_to_ptr.get("UEdMode").unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UBaseLegacyWidgetEdMode {
    __padding_end: [u8; 344],
}
impl UBaseLegacyWidgetEdMode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UBaseLegacyWidgetEdMode")
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
pub struct UEdModeDefault {
    __padding_end: [u8; 344],
}
impl UEdModeDefault {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEdModeDefault")
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
pub struct UAssetEdModeDefault {
    __padding_end: [u8; 344],
}
impl UAssetEdModeDefault {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetEdModeDefault")
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
pub struct UEdModeInteractiveToolsContext {
    __padding_end: [u8; 960],
}
impl UEdModeInteractiveToolsContext {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UEdModeInteractiveToolsContext")
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
pub struct ULegacyEdModeWrapper {
    __padding_end: [u8; 376],
}
impl ULegacyEdModeWrapper {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULegacyEdModeWrapper")
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
pub struct UAssetEditor {
    __padding_end: [u8; 64],
}
impl UAssetEditor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetEditor")
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
pub struct UUnrealEdEngine {
    __padding_end: [u8; 10736],
}
impl UUnrealEdEngine {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnrealEdEngine")
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
pub struct UUnrealEdTypes {
    __padding_end: [u8; 48],
}
impl UUnrealEdTypes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUnrealEdTypes")
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
pub struct UUserDefinedStructEditorData {
    __padding_end: [u8; 104],
}
impl UUserDefinedStructEditorData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UUserDefinedStructEditorData")
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
pub struct UWorldFolders {
    __padding_end: [u8; 296],
}
impl UWorldFolders {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldFolders")
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
pub struct UWorldPartitionFoliageBuilder {
    __padding_end: [u8; 472],
}
impl UWorldPartitionFoliageBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionFoliageBuilder")
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
pub struct UWorldPartitionHLODsBuilder {
    __padding_end: [u8; 888],
}
impl UWorldPartitionHLODsBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionHLODsBuilder")
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
pub struct UWorldPartitionLandscapeSplineMeshesBuilder {
    __padding_end: [u8; 472],
}
impl UWorldPartitionLandscapeSplineMeshesBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionLandscapeSplineMeshesBuilder")
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
pub struct UWorldPartitionMiniMapBuilder {
    __padding_end: [u8; 624],
}
impl UWorldPartitionMiniMapBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionMiniMapBuilder")
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
pub struct UWorldPartitionNavigationDataBuilder {
    __padding_end: [u8; 584],
}
impl UWorldPartitionNavigationDataBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionNavigationDataBuilder")
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
pub struct UWorldPartitionRenameDuplicateBuilder {
    __padding_end: [u8; 600],
}
impl UWorldPartitionRenameDuplicateBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionRenameDuplicateBuilder")
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
pub struct UWorldPartitionResaveActorsBuilder {
    __padding_end: [u8; 664],
}
impl UWorldPartitionResaveActorsBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionResaveActorsBuilder")
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
pub struct UWorldPartitionRuntimeVirtualTextureBuilder {
    __padding_end: [u8; 464],
}
impl UWorldPartitionRuntimeVirtualTextureBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionRuntimeVirtualTextureBuilder")
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
pub struct UWorldPartitionStaticLightingBuilder {
    __padding_end: [u8; 944],
}
impl UWorldPartitionStaticLightingBuilder {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UWorldPartitionStaticLightingBuilder")
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
pub struct UCookOnTheFlyServer {
    __padding_end: [u8; 1976],
}
impl UCookOnTheFlyServer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UCookOnTheFlyServer")
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
pub struct FAddOnExtractAssetFromFile_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FRemoveOnExtractAssetFromFile_Delegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnNewActorsDropped {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnNewActorsPlaced {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCutActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCutActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCopyActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditCopyActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditPasteActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnEditPasteActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDuplicateActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDuplicateActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDeleteActorsBegin {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnDeleteActorsEnd {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FEditorActorSubsystem_OnActorLabelChanged {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetPreImport_BP {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetPostImport_BP {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetReimport_BP {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FImportSubsystem_OnAssetPostLODImport_BP {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ELevelViewportType(pub u8);
impl ELevelViewportType {
    pub const LVT_ORTHO_XY: ELevelViewportType = ELevelViewportType(0);
    pub const LVT_ORTHO_XZ: ELevelViewportType = ELevelViewportType(1);
    pub const LVT_ORTHO_YZ: ELevelViewportType = ELevelViewportType(2);
    pub const LVT_PERSPECTIVE: ELevelViewportType = ELevelViewportType(3);
    pub const LVT_ORTHO_FREELOOK: ELevelViewportType = ELevelViewportType(4);
    pub const LVT_ORTHO_NEGATIVE_XY: ELevelViewportType = ELevelViewportType(5);
    pub const LVT_ORTHO_NEGATIVE_XZ: ELevelViewportType = ELevelViewportType(6);
    pub const LVT_ORTHO_NEGATIVE_YZ: ELevelViewportType = ELevelViewportType(7);
    pub const LVT_ORTHO_TOP: ELevelViewportType = ELevelViewportType(0);
    pub const LVT_ORTHO_LEFT: ELevelViewportType = ELevelViewportType(1);
    pub const LVT_ORTHO_FRONT: ELevelViewportType = ELevelViewportType(7);
    pub const LVT_ORTHO_BACK: ELevelViewportType = ELevelViewportType(2);
    pub const LVT_ORTHO_BOTTOM: ELevelViewportType = ELevelViewportType(5);
    pub const LVT_ORTHO_RIGHT: ELevelViewportType = ELevelViewportType(6);
    pub const LVT_NONE: ELevelViewportType = ELevelViewportType(255);
}
#[repr(transparent)]
pub struct EAnimationViewportCameraFollowMode(pub u8);
impl EAnimationViewportCameraFollowMode {
    pub const NONE: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        0,
    );
    pub const BOUNDS: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        1,
    );
    pub const BONE: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        2,
    );
    pub const ROOT: EAnimationViewportCameraFollowMode = EAnimationViewportCameraFollowMode(
        3,
    );
}
#[repr(transparent)]
pub struct ECSVImportType(pub u8);
impl ECSVImportType {
    pub const ECSV_DATA_TABLE: ECSVImportType = ECSVImportType(0);
    pub const ECSV_CURVE_TABLE: ECSVImportType = ECSVImportType(1);
    pub const ECSV_CURVE_FLOAT: ECSVImportType = ECSVImportType(2);
    pub const ECSV_CURVE_VECTOR: ECSVImportType = ECSVImportType(3);
    pub const ECSV_CURVE_LINEAR_COLOR: ECSVImportType = ECSVImportType(4);
}
#[repr(transparent)]
pub struct ETestEnumFlags(pub u8);
impl ETestEnumFlags {
    pub const NONE: ETestEnumFlags = ETestEnumFlags(0);
    pub const ONE: ETestEnumFlags = ETestEnumFlags(1);
    pub const TWO: ETestEnumFlags = ETestEnumFlags(2);
    pub const FOUR: ETestEnumFlags = ETestEnumFlags(4);
}
#[repr(transparent)]
pub struct NetworkEmulationTarget(pub i32);
impl NetworkEmulationTarget {
    pub const SERVER: NetworkEmulationTarget = NetworkEmulationTarget(0);
    pub const CLIENT: NetworkEmulationTarget = NetworkEmulationTarget(1);
    pub const ANY: NetworkEmulationTarget = NetworkEmulationTarget(2);
}
#[repr(transparent)]
pub struct EFBXExpectedResultPreset(pub u8);
impl EFBXExpectedResultPreset {
    pub const ERROR_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(0);
    pub const WARNING_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(1);
    pub const CREATED_STATICMESH_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        2,
    );
    pub const CREATED_SKELETALMESH_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        3,
    );
    pub const MATERIALS_CREATED_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        4,
    );
    pub const MATERIAL_SLOT_IMPORTED_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        5,
    );
    pub const VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(6);
    pub const LOD_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(7);
    pub const VERTEX_NUMBER_LOD: EFBXExpectedResultPreset = EFBXExpectedResultPreset(8);
    pub const MESH_MATERIALS_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        9,
    );
    pub const MESH_LOD_SECTION_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        10,
    );
    pub const MESH_LOD_SECTION_VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        11,
    );
    pub const MESH_LOD_SECTION_TRIANGLE_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        12,
    );
    pub const MESH_LOD_SECTION_MATERIAL_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        13,
    );
    pub const MESH_LOD_SECTION_MATERIAL_INDEX: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        14,
    );
    pub const MESH_LOD_SECTION_MATERIAL_IMPORTED_NAME: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        15,
    );
    pub const MESH_LOD_VERTEX_POSITION: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        16,
    );
    pub const MESH_LOD_VERTEX_NORMAL: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        17,
    );
    pub const LOD_UV_CHANNEL_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        18,
    );
    pub const BONE_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(19);
    pub const BONE_POSITION: EFBXExpectedResultPreset = EFBXExpectedResultPreset(20);
    pub const ANIMATION_FRAME_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        21,
    );
    pub const ANIMATION_LENGTH: EFBXExpectedResultPreset = EFBXExpectedResultPreset(22);
    pub const ANIMATION_CUSTOM_CURVE_KEY_VALUE: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        23,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_ARRIVE_TANGENT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        24,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_LEAVE_TANGENT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        25,
    );
    pub const SKIN_BY_BONE_VERTEX_NUMBER: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        26,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_ARRIVE_TANGENT_WEIGHT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        27,
    );
    pub const ANIMATION_CUSTOM_CURVE_KEY_LEAVE_TANGENT_WEIGHT: EFBXExpectedResultPreset = EFBXExpectedResultPreset(
        28,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorCenterOfMassViewMode(pub u8);
impl EPhysicsAssetEditorCenterOfMassViewMode {
    pub const ALL: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        0,
    );
    pub const SELECTED: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        1,
    );
    pub const NONE: EPhysicsAssetEditorCenterOfMassViewMode = EPhysicsAssetEditorCenterOfMassViewMode(
        2,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorCollisionViewMode(pub u8);
impl EPhysicsAssetEditorCollisionViewMode {
    pub const SOLID: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        0,
    );
    pub const WIREFRAME: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        1,
    );
    pub const SOLID_WIREFRAME: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        2,
    );
    pub const NONE: EPhysicsAssetEditorCollisionViewMode = EPhysicsAssetEditorCollisionViewMode(
        3,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorConstraintViewMode(pub u8);
impl EPhysicsAssetEditorConstraintViewMode {
    pub const NONE: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        0,
    );
    pub const ALL_POSITIONS: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        1,
    );
    pub const ALL_LIMITS: EPhysicsAssetEditorConstraintViewMode = EPhysicsAssetEditorConstraintViewMode(
        2,
    );
}
#[repr(transparent)]
pub struct EEditorAssetMetaDataSortType(pub u8);
impl EEditorAssetMetaDataSortType {
    pub const STRING: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(0);
    pub const NUMERIC: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(1);
    pub const DATE_TIME: EEditorAssetMetaDataSortType = EEditorAssetMetaDataSortType(2);
}
#[repr(transparent)]
pub struct EEditorAssetSortOrder(pub u8);
impl EEditorAssetSortOrder {
    pub const ASCENDING: EEditorAssetSortOrder = EEditorAssetSortOrder(0);
    pub const DESCENDING: EEditorAssetSortOrder = EEditorAssetSortOrder(1);
}
#[repr(transparent)]
pub struct ELevelVisibilityDirtyMode(pub u8);
impl ELevelVisibilityDirtyMode {
    pub const MODIFY_ON_CHANGE: ELevelVisibilityDirtyMode = ELevelVisibilityDirtyMode(0);
    pub const DONT_MODIFY: ELevelVisibilityDirtyMode = ELevelVisibilityDirtyMode(1);
}
#[repr(transparent)]
pub struct EMaterialSearchLocation(pub u8);
impl EMaterialSearchLocation {
    pub const LOCAL: EMaterialSearchLocation = EMaterialSearchLocation(0);
    pub const UNDER_PARENT: EMaterialSearchLocation = EMaterialSearchLocation(1);
    pub const UNDER_ROOT: EMaterialSearchLocation = EMaterialSearchLocation(2);
    pub const ALL_ASSETS: EMaterialSearchLocation = EMaterialSearchLocation(3);
    pub const DO_NOT_SEARCH: EMaterialSearchLocation = EMaterialSearchLocation(4);
}
#[repr(transparent)]
pub struct EReloadPackagesInteractionMode(pub u8);
impl EReloadPackagesInteractionMode {
    pub const INTERACTIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        0,
    );
    pub const ASSUME_POSITIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        1,
    );
    pub const ASSUME_NEGATIVE: EReloadPackagesInteractionMode = EReloadPackagesInteractionMode(
        2,
    );
}
#[repr(transparent)]
pub struct EFbxExportCompatibility(pub u8);
impl EFbxExportCompatibility {
    pub const FBX_2011: EFbxExportCompatibility = EFbxExportCompatibility(0);
    pub const FBX_2012: EFbxExportCompatibility = EFbxExportCompatibility(1);
    pub const FBX_2013: EFbxExportCompatibility = EFbxExportCompatibility(2);
    pub const FBX_2014: EFbxExportCompatibility = EFbxExportCompatibility(3);
    pub const FBX_2016: EFbxExportCompatibility = EFbxExportCompatibility(4);
    pub const FBX_2018: EFbxExportCompatibility = EFbxExportCompatibility(5);
    pub const FBX_2019: EFbxExportCompatibility = EFbxExportCompatibility(6);
    pub const FBX_2020: EFbxExportCompatibility = EFbxExportCompatibility(7);
}
#[repr(transparent)]
pub struct EBlueprintBreakpointReloadMethod(pub i32);
impl EBlueprintBreakpointReloadMethod {
    pub const RESTORE_ALL: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        0,
    );
    pub const RESTORE_ALL_AND_DISABLE: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        1,
    );
    pub const DISCARD_ALL: EBlueprintBreakpointReloadMethod = EBlueprintBreakpointReloadMethod(
        2,
    );
}
#[repr(transparent)]
pub struct ECommentBoxMode(pub u8);
impl ECommentBoxMode {
    pub const GROUP_MOVEMENT: ECommentBoxMode = ECommentBoxMode(0);
    pub const NO_GROUP_MOVEMENT: ECommentBoxMode = ECommentBoxMode(1);
}
#[repr(transparent)]
pub struct EClassViewerDeveloperType(pub u8);
impl EClassViewerDeveloperType {
    pub const CVDT_NONE: EClassViewerDeveloperType = EClassViewerDeveloperType(0);
    pub const CVDT_CURRENT_USER: EClassViewerDeveloperType = EClassViewerDeveloperType(
        1,
    );
    pub const CVDT_ALL: EClassViewerDeveloperType = EClassViewerDeveloperType(2);
}
#[repr(transparent)]
pub struct ELoadLevelAtStartup(pub u8);
impl ELoadLevelAtStartup {
    pub const NONE: ELoadLevelAtStartup = ELoadLevelAtStartup(0);
    pub const PROJECT_DEFAULT: ELoadLevelAtStartup = ELoadLevelAtStartup(1);
    pub const LAST_OPENED: ELoadLevelAtStartup = ELoadLevelAtStartup(2);
}
#[repr(transparent)]
pub struct ERestoreOpenAssetTabsMethod(pub u8);
impl ERestoreOpenAssetTabsMethod {
    pub const ALWAYS_PROMPT: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        0,
    );
    pub const ALWAYS_RESTORE: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        1,
    );
    pub const NEVER_RESTORE: ERestoreOpenAssetTabsMethod = ERestoreOpenAssetTabsMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EAutoSaveMethod(pub u8);
impl EAutoSaveMethod {
    pub const BACKUP_AND_RESTORE: EAutoSaveMethod = EAutoSaveMethod(0);
    pub const BACKUP_AND_OVERWRITE: EAutoSaveMethod = EAutoSaveMethod(1);
}
#[repr(transparent)]
pub struct EAssetEditorOpenLocation(pub u8);
impl EAssetEditorOpenLocation {
    pub const DEFAULT: EAssetEditorOpenLocation = EAssetEditorOpenLocation(0);
    pub const NEW_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(1);
    pub const MAIN_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(2);
    pub const CONTENT_BROWSER: EAssetEditorOpenLocation = EAssetEditorOpenLocation(3);
    pub const LAST_DOCKED_WINDOW_OR_NEW_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        4,
    );
    pub const LAST_DOCKED_WINDOW_OR_MAIN_WINDOW: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        5,
    );
    pub const LAST_DOCKED_WINDOW_OR_CONTENT_BROWSER: EAssetEditorOpenLocation = EAssetEditorOpenLocation(
        6,
    );
}
#[repr(transparent)]
pub struct EFBXImportType(pub u8);
impl EFBXImportType {
    pub const FBXIT_STATIC_MESH: EFBXImportType = EFBXImportType(0);
    pub const FBXIT_SKELETAL_MESH: EFBXImportType = EFBXImportType(1);
    pub const FBXIT_ANIMATION: EFBXImportType = EFBXImportType(2);
}
#[repr(transparent)]
pub struct ELabelAnchorMode(pub u8);
impl ELabelAnchorMode {
    pub const LABEL_ANCHOR_MODE_TOP_LEFT: ELabelAnchorMode = ELabelAnchorMode(0);
    pub const LABEL_ANCHOR_MODE_TOP_CENTER: ELabelAnchorMode = ELabelAnchorMode(1);
    pub const LABEL_ANCHOR_MODE_TOP_RIGHT: ELabelAnchorMode = ELabelAnchorMode(2);
    pub const LABEL_ANCHOR_MODE_CENTER_LEFT: ELabelAnchorMode = ELabelAnchorMode(3);
    pub const LABEL_ANCHOR_MODE_CENTERED: ELabelAnchorMode = ELabelAnchorMode(4);
    pub const LABEL_ANCHOR_MODE_CENTER_RIGHT: ELabelAnchorMode = ELabelAnchorMode(5);
    pub const LABEL_ANCHOR_MODE_BOTTOM_LEFT: ELabelAnchorMode = ELabelAnchorMode(6);
    pub const LABEL_ANCHOR_MODE_BOTTOM_CENTER: ELabelAnchorMode = ELabelAnchorMode(7);
    pub const LABEL_ANCHOR_MODE_BOTTOM_RIGHT: ELabelAnchorMode = ELabelAnchorMode(8);
}
#[repr(transparent)]
pub struct EPlayOnBuildMode(pub u8);
impl EPlayOnBuildMode {
    pub const PLAY_ON_BUILD_ALWAYS: EPlayOnBuildMode = EPlayOnBuildMode(0);
    pub const PLAY_ON_BUILD_NEVER: EPlayOnBuildMode = EPlayOnBuildMode(1);
    pub const PLAY_ON_BUILD_DEFAULT: EPlayOnBuildMode = EPlayOnBuildMode(2);
    pub const PLAY_ON_BUILD_IF_EDITOR_BUILT_LOCALLY: EPlayOnBuildMode = EPlayOnBuildMode(
        3,
    );
}
#[repr(transparent)]
pub struct EPlayOnLaunchConfiguration(pub u8);
impl EPlayOnLaunchConfiguration {
    pub const LAUNCH_CONFIG_DEFAULT: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        0,
    );
    pub const LAUNCH_CONFIG_DEBUG: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        1,
    );
    pub const LAUNCH_CONFIG_DEVELOPMENT: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        2,
    );
    pub const LAUNCH_CONFIG_TEST: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        3,
    );
    pub const LAUNCH_CONFIG_SHIPPING: EPlayOnLaunchConfiguration = EPlayOnLaunchConfiguration(
        4,
    );
}
#[repr(transparent)]
pub struct EPlayOnPakFileMode(pub u8);
impl EPlayOnPakFileMode {
    pub const NO_PAK: EPlayOnPakFileMode = EPlayOnPakFileMode(0);
    pub const PAK_NO_COMPRESS: EPlayOnPakFileMode = EPlayOnPakFileMode(1);
    pub const PAK_COMPRESS: EPlayOnPakFileMode = EPlayOnPakFileMode(2);
}
#[repr(transparent)]
pub struct EPlayNetMode(pub u8);
impl EPlayNetMode {
    pub const PIE_STANDALONE: EPlayNetMode = EPlayNetMode(0);
    pub const PIE_LISTEN_SERVER: EPlayNetMode = EPlayNetMode(1);
    pub const PIE_CLIENT: EPlayNetMode = EPlayNetMode(2);
}
#[repr(transparent)]
pub struct ELaunchModeType(pub u8);
impl ELaunchModeType {
    pub const LAUNCH_MODE_ON_DEVICE: ELaunchModeType = ELaunchModeType(0);
}
#[repr(transparent)]
pub struct EPlayModeLocations(pub u8);
impl EPlayModeLocations {
    pub const PLAY_LOCATION_CURRENT_CAMERA_LOCATION: EPlayModeLocations = EPlayModeLocations(
        0,
    );
    pub const PLAY_LOCATION_DEFAULT_PLAYER_START: EPlayModeLocations = EPlayModeLocations(
        1,
    );
}
#[repr(transparent)]
pub struct EPlayModeType(pub u8);
impl EPlayModeType {
    pub const PLAY_MODE_IN_VIEW_PORT: EPlayModeType = EPlayModeType(0);
    pub const PLAY_MODE_IN_EDITOR_FLOATING: EPlayModeType = EPlayModeType(1);
    pub const PLAY_MODE_IN_MOBILE_PREVIEW: EPlayModeType = EPlayModeType(2);
    pub const PLAY_MODE_IN_TARGETED_MOBILE_PREVIEW: EPlayModeType = EPlayModeType(3);
    pub const PLAY_MODE_IN_VULKAN_PREVIEW: EPlayModeType = EPlayModeType(4);
    pub const PLAY_MODE_IN_NEW_PROCESS: EPlayModeType = EPlayModeType(5);
    pub const PLAY_MODE_IN_VR: EPlayModeType = EPlayModeType(6);
    pub const PLAY_MODE_SIMULATE: EPlayModeType = EPlayModeType(7);
    pub const PLAY_MODE_QUICK_LAUNCH: EPlayModeType = EPlayModeType(8);
    pub const PLAY_MODE_COUNT: EPlayModeType = EPlayModeType(9);
}
#[repr(transparent)]
pub struct EWASDType(pub u8);
impl EWASDType {
    pub const WASD_ALWAYS: EWASDType = EWASDType(0);
    pub const WASD_RMB_ONLY: EWASDType = EWASDType(1);
    pub const WASD_NEVER: EWASDType = EWASDType(2);
}
#[repr(transparent)]
pub struct ELandscapeFoliageEditorControlType(pub u8);
impl ELandscapeFoliageEditorControlType {
    pub const IGNORE_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        0,
    );
    pub const REQUIRE_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        1,
    );
    pub const REQUIRE_NO_CTRL: ELandscapeFoliageEditorControlType = ELandscapeFoliageEditorControlType(
        2,
    );
}
#[repr(transparent)]
pub struct EScrollGestureDirection(pub u8);
impl EScrollGestureDirection {
    pub const USE_SYSTEM_SETTING: EScrollGestureDirection = EScrollGestureDirection(0);
    pub const STANDARD: EScrollGestureDirection = EScrollGestureDirection(1);
    pub const NATURAL: EScrollGestureDirection = EScrollGestureDirection(2);
}
#[repr(transparent)]
pub struct ERotationGridMode(pub u8);
impl ERotationGridMode {
    pub const GRID_MODE_DIVISIONS_OF360: ERotationGridMode = ERotationGridMode(0);
    pub const GRID_MODE_COMMON: ERotationGridMode = ERotationGridMode(1);
}
#[repr(transparent)]
pub struct EMarqueeSelectionMode(pub u8);
impl EMarqueeSelectionMode {
    pub const CROSSING: EMarqueeSelectionMode = EMarqueeSelectionMode(0);
    pub const WINDOW: EMarqueeSelectionMode = EMarqueeSelectionMode(1);
    pub const CROSS_LEFT: EMarqueeSelectionMode = EMarqueeSelectionMode(2);
    pub const CROSS_RIGHT: EMarqueeSelectionMode = EMarqueeSelectionMode(3);
}
#[repr(transparent)]
pub struct EMeasuringToolUnits(pub u8);
impl EMeasuringToolUnits {
    pub const MEASURE_UNITS_CENTIMETERS: EMeasuringToolUnits = EMeasuringToolUnits(0);
    pub const MEASURE_UNITS_METERS: EMeasuringToolUnits = EMeasuringToolUnits(1);
    pub const MEASURE_UNITS_KILOMETERS: EMeasuringToolUnits = EMeasuringToolUnits(2);
}
#[repr(transparent)]
pub struct EMaterialKind(pub u8);
impl EMaterialKind {
    pub const UNKNOWN: EMaterialKind = EMaterialKind(0);
    pub const BASE: EMaterialKind = EMaterialKind(1);
    pub const NORMAL: EMaterialKind = EMaterialKind(2);
    pub const SPECULAR: EMaterialKind = EMaterialKind(3);
    pub const EMISSIVE: EMaterialKind = EMaterialKind(4);
}
#[repr(transparent)]
pub struct EMaterialStatsDerivedMIOption(pub u8);
impl EMaterialStatsDerivedMIOption {
    pub const IGNORE: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(0);
    pub const COMPILE_ONLY: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        1,
    );
    pub const SHOW_STATS: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        2,
    );
    pub const INVALID_OR_MAX: EMaterialStatsDerivedMIOption = EMaterialStatsDerivedMIOption(
        3,
    );
}
#[repr(transparent)]
pub struct EPhysicsAssetEditorMeshViewMode(pub u8);
impl EPhysicsAssetEditorMeshViewMode {
    pub const SOLID: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(
        0,
    );
    pub const WIREFRAME: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(
        1,
    );
    pub const NONE: EPhysicsAssetEditorMeshViewMode = EPhysicsAssetEditorMeshViewMode(2);
}
#[repr(transparent)]
pub struct ETextureSourceColorSpace(pub i32);
impl ETextureSourceColorSpace {
    pub const AUTO: ETextureSourceColorSpace = ETextureSourceColorSpace(0);
    pub const LINEAR: ETextureSourceColorSpace = ETextureSourceColorSpace(1);
    pub const SRGB: ETextureSourceColorSpace = ETextureSourceColorSpace(2);
}
#[repr(transparent)]
pub struct ESheetAxis(pub u8);
impl ESheetAxis {
    pub const AX_HORIZONTAL: ESheetAxis = ESheetAxis(0);
    pub const AX_X_AXIS: ESheetAxis = ESheetAxis(1);
    pub const AX_Y_AXIS: ESheetAxis = ESheetAxis(2);
}
#[repr(transparent)]
pub struct EStructViewerDeveloperType(pub u8);
impl EStructViewerDeveloperType {
    pub const SVDT_NONE: EStructViewerDeveloperType = EStructViewerDeveloperType(0);
    pub const SVDT_CURRENT_USER: EStructViewerDeveloperType = EStructViewerDeveloperType(
        1,
    );
    pub const SVDT_ALL: EStructViewerDeveloperType = EStructViewerDeveloperType(2);
}
#[repr(transparent)]
pub struct ETexAlign(pub u8);
impl ETexAlign {
    pub const TEXALIGN_NONE: ETexAlign = ETexAlign(0);
    pub const TEXALIGN_DEFAULT: ETexAlign = ETexAlign(1);
    pub const TEXALIGN_BOX: ETexAlign = ETexAlign(2);
    pub const TEXALIGN_PLANAR: ETexAlign = ETexAlign(3);
    pub const TEXALIGN_FIT: ETexAlign = ETexAlign(4);
    pub const TEXALIGN_PLANAR_AUTO: ETexAlign = ETexAlign(5);
    pub const TEXALIGN_PLANAR_WALL: ETexAlign = ETexAlign(6);
    pub const TEXALIGN_PLANAR_FLOOR: ETexAlign = ETexAlign(7);
}
#[repr(transparent)]
pub struct ESkeletonDrawMode(pub u8);
impl ESkeletonDrawMode {
    pub const DEFAULT: ESkeletonDrawMode = ESkeletonDrawMode(0);
    pub const HIDDEN: ESkeletonDrawMode = ESkeletonDrawMode(1);
    pub const GREYED_OUT: ESkeletonDrawMode = ESkeletonDrawMode(2);
}
#[repr(transparent)]
pub struct EProcessRootMotionMode(pub u8);
impl EProcessRootMotionMode {
    pub const IGNORE: EProcessRootMotionMode = EProcessRootMotionMode(0);
    pub const LOOP: EProcessRootMotionMode = EProcessRootMotionMode(1);
    pub const LOOP_AND_RESET: EProcessRootMotionMode = EProcessRootMotionMode(2);
}
#[repr(transparent)]
pub struct EVisualizeRootMotionMode(pub u8);
impl EVisualizeRootMotionMode {
    pub const NONE: EVisualizeRootMotionMode = EVisualizeRootMotionMode(0);
    pub const TRAJECTORY: EVisualizeRootMotionMode = EVisualizeRootMotionMode(1);
    pub const TRAJECTORY_AND_ORIENTATION: EVisualizeRootMotionMode = EVisualizeRootMotionMode(
        2,
    );
}
#[repr(transparent)]
pub struct EEditorUserScreenPercentageModeOverride(pub i32);
impl EEditorUserScreenPercentageModeOverride {
    pub const PROJECT_DEFAULT: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        0,
    );
    pub const MANUAL: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        1,
    );
    pub const BASED_ON_DISPLAY_RESOLUTION: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        2,
    );
    pub const BASED_ON_DPI_SCALE: EEditorUserScreenPercentageModeOverride = EEditorUserScreenPercentageModeOverride(
        3,
    );
}
#[repr(transparent)]
pub struct ECoordinateSystemPolicy(pub u8);
impl ECoordinateSystemPolicy {
    pub const MATCH_UP_FORWARD_AXES: ECoordinateSystemPolicy = ECoordinateSystemPolicy(
        0,
    );
    pub const MATCH_UP_AXIS: ECoordinateSystemPolicy = ECoordinateSystemPolicy(1);
    pub const KEEP_XYZ_AXES: ECoordinateSystemPolicy = ECoordinateSystemPolicy(2);
}
#[repr(transparent)]
pub struct EFBXAnimationLengthImportType(pub u8);
impl EFBXAnimationLengthImportType {
    pub const FBXALIT_EXPORTED_TIME: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        0,
    );
    pub const FBXALIT_ANIMATED_KEY: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        1,
    );
    pub const FBXALIT_SET_RANGE: EFBXAnimationLengthImportType = EFBXAnimationLengthImportType(
        2,
    );
}
#[repr(transparent)]
pub struct EMovieSceneBakeType(pub u8);
impl EMovieSceneBakeType {
    pub const NONE: EMovieSceneBakeType = EMovieSceneBakeType(0);
    pub const BAKE_CHANNELS: EMovieSceneBakeType = EMovieSceneBakeType(1);
    pub const BAKE_TRANSFORMS: EMovieSceneBakeType = EMovieSceneBakeType(2);
    pub const BAKE_ALL: EMovieSceneBakeType = EMovieSceneBakeType(3);
}
#[repr(transparent)]
pub struct EFbxMaterialBakeMode(pub u8);
impl EFbxMaterialBakeMode {
    pub const DISABLED: EFbxMaterialBakeMode = EFbxMaterialBakeMode(0);
    pub const SIMPLE: EFbxMaterialBakeMode = EFbxMaterialBakeMode(1);
    pub const USE_MESH_DATA: EFbxMaterialBakeMode = EFbxMaterialBakeMode(2);
}
#[repr(transparent)]
pub struct EFBXNormalImportMethod(pub u8);
impl EFBXNormalImportMethod {
    pub const FBXNIM_COMPUTE_NORMALS: EFBXNormalImportMethod = EFBXNormalImportMethod(0);
    pub const FBXNIM_IMPORT_NORMALS: EFBXNormalImportMethod = EFBXNormalImportMethod(1);
    pub const FBXNIM_IMPORT_NORMALS_AND_TANGENTS: EFBXNormalImportMethod = EFBXNormalImportMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EFBXNormalGenerationMethod(pub u8);
impl EFBXNormalGenerationMethod {
    pub const BUILT_IN: EFBXNormalGenerationMethod = EFBXNormalGenerationMethod(0);
    pub const MIKK_T_SPACE: EFBXNormalGenerationMethod = EFBXNormalGenerationMethod(1);
}
#[repr(transparent)]
pub struct EFBXSceneOptionsCreateHierarchyType(pub u8);
impl EFBXSceneOptionsCreateHierarchyType {
    pub const FBXSOCHT_CREATE_LEVEL_ACTORS: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        0,
    );
    pub const FBXSOCHT_CREATE_ACTOR_COMPONENTS: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        1,
    );
    pub const FBXSOCHT_CREATE_BLUEPRINT: EFBXSceneOptionsCreateHierarchyType = EFBXSceneOptionsCreateHierarchyType(
        2,
    );
}
#[repr(transparent)]
pub struct EFbxSceneVertexColorImportOption(pub u8);
impl EFbxSceneVertexColorImportOption {
    pub const REPLACE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        0,
    );
    pub const IGNORE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        1,
    );
    pub const OVERRIDE: EFbxSceneVertexColorImportOption = EFbxSceneVertexColorImportOption(
        2,
    );
}
#[repr(transparent)]
pub struct EFBXSceneNormalImportMethod(pub u8);
impl EFBXSceneNormalImportMethod {
    pub const FBX_SCENE_NIM_COMPUTE_NORMALS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        0,
    );
    pub const FBX_SCENE_NIM_IMPORT_NORMALS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        1,
    );
    pub const FBX_SCENE_NIM_IMPORT_NORMALS_AND_TANGENTS: EFBXSceneNormalImportMethod = EFBXSceneNormalImportMethod(
        2,
    );
}
#[repr(transparent)]
pub struct EFBXSceneNormalGenerationMethod(pub u8);
impl EFBXSceneNormalGenerationMethod {
    pub const BUILT_IN: EFBXSceneNormalGenerationMethod = EFBXSceneNormalGenerationMethod(
        0,
    );
    pub const MIKK_T_SPACE: EFBXSceneNormalGenerationMethod = EFBXSceneNormalGenerationMethod(
        1,
    );
}
#[repr(transparent)]
pub struct EFBXImportContentType(pub u8);
impl EFBXImportContentType {
    pub const FBXICT_ALL: EFBXImportContentType = EFBXImportContentType(0);
    pub const FBXICT_GEOMETRY: EFBXImportContentType = EFBXImportContentType(1);
    pub const FBXICT_SKINNING_WEIGHTS: EFBXImportContentType = EFBXImportContentType(2);
}
#[repr(transparent)]
pub struct EVertexColorImportOption(pub u8);
impl EVertexColorImportOption {
    pub const REPLACE: EVertexColorImportOption = EVertexColorImportOption(0);
    pub const IGNORE: EVertexColorImportOption = EVertexColorImportOption(1);
    pub const OVERRIDE: EVertexColorImportOption = EVertexColorImportOption(2);
}
#[repr(transparent)]
pub struct EPropertyEditorTestEnum(pub u8);
impl EPropertyEditorTestEnum {
    pub const PROPERTY_EDITOR_TEST_ENUM1: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        0,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM2: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        1,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM3: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        2,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM4: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        3,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM5: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        4,
    );
    pub const PROPERTY_EDITOR_TEST_ENUM6: EPropertyEditorTestEnum = EPropertyEditorTestEnum(
        5,
    );
}
#[repr(transparent)]
pub struct EPropertyEditorTestEditColor(pub u8);
impl EPropertyEditorTestEditColor {
    pub const RED: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(0);
    pub const ORANGE: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(1);
    pub const YELLOW: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(2);
    pub const GREEN: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(3);
    pub const BLUE: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(4);
    pub const INDIGO: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(5);
    pub const VIOLET: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(6);
    pub const PINK: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(7);
    pub const MAGENTA: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(8);
    pub const CYAN: EPropertyEditorTestEditColor = EPropertyEditorTestEditColor(9);
}
#[repr(transparent)]
pub struct EPropertyEditorTestUnderscores(pub u8);
impl EPropertyEditorTestUnderscores {
    pub const ONE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(0);
    pub const TWO: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(1);
    pub const THREE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(2);
    pub const NOT_UNDERSCORE: EPropertyEditorTestUnderscores = EPropertyEditorTestUnderscores(
        3,
    );
}
#[repr(transparent)]
pub struct EReferenceViewerSettingMode(pub u8);
impl EReferenceViewerSettingMode {
    pub const NO_PREFERENCE: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        0,
    );
    pub const SHOW_BY_DEFAULT: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        1,
    );
    pub const HIDE_BY_DEFAULT: EReferenceViewerSettingMode = EReferenceViewerSettingMode(
        2,
    );
}
#[repr(transparent)]
pub struct EUnitDisplay(pub u8);
impl EUnitDisplay {
    pub const NONE: EUnitDisplay = EUnitDisplay(0);
    pub const METRIC: EUnitDisplay = EUnitDisplay(1);
    pub const IMPERIAL: EUnitDisplay = EUnitDisplay(2);
    pub const INVALID: EUnitDisplay = EUnitDisplay(3);
}
#[repr(transparent)]
pub struct EDefaultLocationUnit(pub u8);
impl EDefaultLocationUnit {
    pub const MICROMETERS: EDefaultLocationUnit = EDefaultLocationUnit(0);
    pub const MILLIMETERS: EDefaultLocationUnit = EDefaultLocationUnit(1);
    pub const CENTIMETERS: EDefaultLocationUnit = EDefaultLocationUnit(2);
    pub const METERS: EDefaultLocationUnit = EDefaultLocationUnit(3);
    pub const KILOMETERS: EDefaultLocationUnit = EDefaultLocationUnit(4);
    pub const INCHES: EDefaultLocationUnit = EDefaultLocationUnit(5);
    pub const FEET: EDefaultLocationUnit = EDefaultLocationUnit(6);
    pub const YARDS: EDefaultLocationUnit = EDefaultLocationUnit(7);
    pub const MILES: EDefaultLocationUnit = EDefaultLocationUnit(8);
    pub const INVALID: EDefaultLocationUnit = EDefaultLocationUnit(9);
}
#[repr(transparent)]
pub struct ELevelEditor2DAxis(pub u8);
impl ELevelEditor2DAxis {
    pub const X: ELevelEditor2DAxis = ELevelEditor2DAxis(0);
    pub const Y: ELevelEditor2DAxis = ELevelEditor2DAxis(1);
    pub const Z: ELevelEditor2DAxis = ELevelEditor2DAxis(2);
}
#[repr(transparent)]
pub struct EFBXTestPlanActionType(pub u8);
impl EFBXTestPlanActionType {
    pub const IMPORT: EFBXTestPlanActionType = EFBXTestPlanActionType(0);
    pub const REIMPORT: EFBXTestPlanActionType = EFBXTestPlanActionType(1);
    pub const ADD_LOD: EFBXTestPlanActionType = EFBXTestPlanActionType(2);
    pub const REIMPORT_LOD: EFBXTestPlanActionType = EFBXTestPlanActionType(3);
    pub const IMPORT_RELOAD: EFBXTestPlanActionType = EFBXTestPlanActionType(4);
    pub const ADD_ALTERNATE_SKINNIG: EFBXTestPlanActionType = EFBXTestPlanActionType(5);
}
#[repr(transparent)]
pub struct EThumbnailPrimType(pub u8);
impl EThumbnailPrimType {
    pub const TPT_NONE: EThumbnailPrimType = EThumbnailPrimType(0);
    pub const TPT_SPHERE: EThumbnailPrimType = EThumbnailPrimType(1);
    pub const TPT_CUBE: EThumbnailPrimType = EThumbnailPrimType(2);
    pub const TPT_PLANE: EThumbnailPrimType = EThumbnailPrimType(3);
    pub const TPT_CYLINDER: EThumbnailPrimType = EThumbnailPrimType(4);
}
#[repr(transparent)]
pub struct EOrthoThumbnailDirection(pub u8);
impl EOrthoThumbnailDirection {
    pub const TOP: EOrthoThumbnailDirection = EOrthoThumbnailDirection(0);
    pub const BOTTOM: EOrthoThumbnailDirection = EOrthoThumbnailDirection(1);
    pub const LEFT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(2);
    pub const RIGHT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(3);
    pub const FRONT: EOrthoThumbnailDirection = EOrthoThumbnailDirection(4);
    pub const BACK: EOrthoThumbnailDirection = EOrthoThumbnailDirection(5);
}
