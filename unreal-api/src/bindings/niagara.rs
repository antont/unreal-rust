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
    pub u_niagara_data_interface_array_distribution_int_set_niagara_array_distribution_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_mesh_set_niagara_array_mesh_sm: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_mesh_set_niagara_array_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_scene_capture2_d_set_scene_capture2_d_managed_show_only_actors: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_static_mesh_set_niagara_static_mesh_di_instance_index: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_static_mesh_on_source_end_play: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_u_object_property_reader_set_u_object_reader_property_remap: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_set_section_start_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_set_section_evaluate_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_set_section_end_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_set_allow_scalability: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_set_age_update_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_get_section_start_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_get_section_evaluate_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_get_section_end_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_get_allow_scalability: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_niagara_system_spawn_section_get_age_update_mode: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_actor_set_destroy_on_system_finish: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_actor_on_niagara_system_finished: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_actor_get_destroy_on_system_finish: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_vec4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_vec3: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_vec2: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_texture_render_target: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_texture: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_static_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_position: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_object: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_matrix: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_material: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_linear_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_bool: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_variable_actor: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_tick_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_system_fixed_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_sim_cache: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_seek_delta: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_rendering_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_random_seed_offset: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_preview_lod_distance: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_paused: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_occlusion_query_mode: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_vec4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_vec3: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_vec2: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_position: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_object: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_matrix: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_linear_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_bool: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_niagara_variable_actor: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_max_sim_time: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_lock_desired_age_delta_time_to_seek_delta: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_gpu_compute_debug: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_force_solo: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_force_local_player_effect: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_emitter_fixed_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_desired_age: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_custom_time_dilation: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_can_render_while_seeking: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_auto_destroy: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_asset: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_allow_scalability: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_set_age_update_mode: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_seek_to_desired_age: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_reset_system: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_reinitialize_system: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_is_paused: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_init_for_performance_baseline: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_vec4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_vec3: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_vec2: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_position: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_matrix: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_variable_bool: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_tick_behavior: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_system_fixed_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_sim_cache: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_seek_delta: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_random_seed_offset: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_preview_lod_distance_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_preview_lod_distance: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_occlusion_query_mode: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_max_sim_time: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_lock_desired_age_delta_time_to_seek_delta: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_force_solo: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_force_local_player_effect: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_emitter_fixed_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_desired_age: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_data_interface: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_custom_time_dilation: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_asset: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_allow_scalability: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_get_age_update_mode: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_clear_system_fixed_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_clear_sim_cache: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_clear_emitter_fixed_bounds: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_advance_simulation_by_time: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_component_advance_simulation: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_vector4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_vector: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_spawn_info: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_position: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_linear_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_id: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_enum: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_read_bool: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_num: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_reader_init_access: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_vector4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_vector: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_spawn_info: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_position: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_linear_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_int: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_id: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_enum: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_write_bool: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_num: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_writer_init_write: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_write_to_niagara_data_channel_single_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_write_to_niagara_data_channel_single: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_write_to_niagara_data_channel_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_write_to_niagara_data_channel: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_unsubscribe_from_niagara_data_channel: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_subscribe_to_niagara_data_channel_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_subscribe_to_niagara_data_channel: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_set_single_property_in_ndc_access_context_instance: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_set_members_in_ndc_access_context_instance: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_read_from_niagara_data_channel_single_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_read_from_niagara_data_channel_single: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_read_from_niagara_data_channel_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_read_from_niagara_data_channel: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_prepare_access_context_from_ndc_ref: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_make_ndc_access_context_instance: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_get_usable_access_context_from_ndc_ref: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_get_usable_access_context_from_ndc: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_get_single_property_in_ndc_access_context_instance: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_get_niagara_data_channel: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_get_members_in_ndc_access_context_instance: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_get_data_channel_element_count_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_library_get_data_channel_element_count: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_handler_unsubscribe_from_data_channel_updates: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_handler_subscribe_to_data_channel_updates_with_context: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_handler_subscribe_to_data_channel_updates: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_handler_get_data_channel_writer: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_channel_handler_get_data_channel_reader: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_vector_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_vector4_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_vector4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_vector: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_u_int8_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_u_int8: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_quat_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_position_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_position: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_matrix_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_matrix: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_int32_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_int32: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_float_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_color_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_bool_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_set_niagara_array_bool: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_vector_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_vector4_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_vector4: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_vector: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_u_int8_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_u_int8: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_quat_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_quat: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_position_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_position: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_matrix_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_matrix: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_int32_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_int32: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_float_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_float: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_color_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_color: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_bool_value: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_array_function_library_get_niagara_array_bool: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_particle_callback_handler_receive_particle_data: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid2_d_collection_get_texture_size: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid2_d_collection_get_raw_texture_size: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid2_d_collection_fill_texture2_d: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid2_d_collection_fill_raw_texture2_d: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid3_d_collection_get_texture_size: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid3_d_collection_get_raw_texture_size: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid3_d_collection_fill_volume_texture: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_grid3_d_collection_fill_raw_volume_texture: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_di_rigid_mesh_collision_function_library_set_source_actors: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_data_interface_skeletal_mesh_on_source_end_play: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_editor_preview_actor_calculate_rotation: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_editor_preview_actor_calculate_location: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_spawn_system_attached_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_spawn_system_attached: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_spawn_system_at_location_with_params: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_spawn_system_at_location: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_volume_texture_object: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_texture_object: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_texture2_d_array_object: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_skeletal_mesh_data_interface_sampling_regions: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_sockets: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_bones: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_scene_capture2_d_data_interface_managed_mode: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_component_niagara_gpu_ray_traced_collision_group: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_set_actor_niagara_gpu_ray_traced_collision_group: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_release_niagara_gpu_ray_traced_collision_group: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_override_system_user_variable_static_mesh_component: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_override_system_user_variable_static_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_override_system_user_variable_skeletal_mesh_component: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_get_niagara_parameter_collection: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_function_library_acquire_niagara_gpu_ray_traced_collision_group: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_vector_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_vector4_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_vector2_d_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_quat_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_int_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_float_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_color_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_set_bool_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_vector_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_vector4_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_vector2_d_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_quat_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_int_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_float_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_color_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_parameter_collection_instance_get_bool_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_baseline_controller_on_tick_test: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_baseline_controller_on_owner_tick: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_baseline_controller_on_end_test: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_baseline_controller_on_begin_test: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_baseline_controller_get_system: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_preview_base_set_system: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_preview_base_set_label_text: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_preview_axis_num: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_preview_axis_apply_to_preview: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_preview_grid_set_paused: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_preview_grid_get_previews: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_preview_grid_deactivate_previews: *mut crate::ffi::UFunctionOpague,
    pub a_niagara_preview_grid_activate_previews: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_script_raise_on_gpu_compilation_complete: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_vector_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_vector4_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_vector2_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_quat_attribute_with_rebase: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_quat_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_position_attribute_with_rebase: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_position_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_int_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_id_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_float_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_data_interface_as: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_read_color_attribute: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_is_empty: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_is_cache_valid: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_get_start_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_get_num_frames: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_get_num_emitters: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_get_emitter_names: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_get_emitter_name: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_get_attribute_capture_mode: *mut crate::ffi::UFunctionOpague,
    pub u_async_niagara_capture_sim_cache_on_capture_complete_delegate_signature: *mut crate::ffi::UFunctionOpague,
    pub u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_until_complete: *mut crate::ffi::UFunctionOpague,
    pub u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_multi_frame: *mut crate::ffi::UFunctionOpague,
    pub u_async_niagara_capture_sim_cache_capture_niagara_sim_cache: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_function_library_create_niagara_sim_cache: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_sim_cache_function_library_capture_niagara_sim_cache_immediate: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_system_collection_release: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_system_collection_num: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_system_collection_load_synchronous: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_system_collection_load_async: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_system_collection_get_systems: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_stateless_module_decal_attributes_is_orientation_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_stateless_module_mesh_index_needs_mesh_index_weights: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_stateless_module_scale_mesh_size_use_scale_curve_range: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_stateless_module_scale_ribbon_width_use_scale_curve_range: *mut crate::ffi::UFunctionOpague,
    pub u_niagara_stateless_module_scale_sprite_size_use_scale_curve_range: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_niagara_data_interface_array_distribution_int_set_niagara_array_distribution_int: std::ptr::null_mut(),
            u_niagara_data_interface_array_mesh_set_niagara_array_mesh_sm: std::ptr::null_mut(),
            u_niagara_data_interface_array_mesh_set_niagara_array_mesh: std::ptr::null_mut(),
            u_niagara_data_interface_scene_capture2_d_set_scene_capture2_d_managed_show_only_actors: std::ptr::null_mut(),
            u_niagara_data_interface_static_mesh_set_niagara_static_mesh_di_instance_index: std::ptr::null_mut(),
            u_niagara_data_interface_static_mesh_on_source_end_play: std::ptr::null_mut(),
            u_niagara_data_interface_u_object_property_reader_set_u_object_reader_property_remap: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_set_section_start_behavior: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_set_section_evaluate_behavior: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_set_section_end_behavior: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_set_allow_scalability: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_set_age_update_mode: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_get_section_start_behavior: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_get_section_evaluate_behavior: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_get_section_end_behavior: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_get_allow_scalability: std::ptr::null_mut(),
            u_movie_scene_niagara_system_spawn_section_get_age_update_mode: std::ptr::null_mut(),
            a_niagara_actor_set_destroy_on_system_finish: std::ptr::null_mut(),
            a_niagara_actor_on_niagara_system_finished: std::ptr::null_mut(),
            a_niagara_actor_get_destroy_on_system_finish: std::ptr::null_mut(),
            u_niagara_component_set_variable_vec4: std::ptr::null_mut(),
            u_niagara_component_set_variable_vec3: std::ptr::null_mut(),
            u_niagara_component_set_variable_vec2: std::ptr::null_mut(),
            u_niagara_component_set_variable_texture_render_target: std::ptr::null_mut(),
            u_niagara_component_set_variable_texture: std::ptr::null_mut(),
            u_niagara_component_set_variable_static_mesh: std::ptr::null_mut(),
            u_niagara_component_set_variable_quat: std::ptr::null_mut(),
            u_niagara_component_set_variable_position: std::ptr::null_mut(),
            u_niagara_component_set_variable_object: std::ptr::null_mut(),
            u_niagara_component_set_variable_matrix: std::ptr::null_mut(),
            u_niagara_component_set_variable_material: std::ptr::null_mut(),
            u_niagara_component_set_variable_linear_color: std::ptr::null_mut(),
            u_niagara_component_set_variable_int: std::ptr::null_mut(),
            u_niagara_component_set_variable_float: std::ptr::null_mut(),
            u_niagara_component_set_variable_bool: std::ptr::null_mut(),
            u_niagara_component_set_variable_actor: std::ptr::null_mut(),
            u_niagara_component_set_tick_behavior: std::ptr::null_mut(),
            u_niagara_component_set_system_fixed_bounds: std::ptr::null_mut(),
            u_niagara_component_set_sim_cache: std::ptr::null_mut(),
            u_niagara_component_set_seek_delta: std::ptr::null_mut(),
            u_niagara_component_set_rendering_enabled: std::ptr::null_mut(),
            u_niagara_component_set_random_seed_offset: std::ptr::null_mut(),
            u_niagara_component_set_preview_lod_distance: std::ptr::null_mut(),
            u_niagara_component_set_paused: std::ptr::null_mut(),
            u_niagara_component_set_occlusion_query_mode: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_vec4: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_vec3: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_vec2: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_quat: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_position: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_object: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_matrix: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_linear_color: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_int: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_float: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_bool: std::ptr::null_mut(),
            u_niagara_component_set_niagara_variable_actor: std::ptr::null_mut(),
            u_niagara_component_set_max_sim_time: std::ptr::null_mut(),
            u_niagara_component_set_lock_desired_age_delta_time_to_seek_delta: std::ptr::null_mut(),
            u_niagara_component_set_gpu_compute_debug: std::ptr::null_mut(),
            u_niagara_component_set_force_solo: std::ptr::null_mut(),
            u_niagara_component_set_force_local_player_effect: std::ptr::null_mut(),
            u_niagara_component_set_emitter_fixed_bounds: std::ptr::null_mut(),
            u_niagara_component_set_desired_age: std::ptr::null_mut(),
            u_niagara_component_set_custom_time_dilation: std::ptr::null_mut(),
            u_niagara_component_set_can_render_while_seeking: std::ptr::null_mut(),
            u_niagara_component_set_auto_destroy: std::ptr::null_mut(),
            u_niagara_component_set_asset: std::ptr::null_mut(),
            u_niagara_component_set_allow_scalability: std::ptr::null_mut(),
            u_niagara_component_set_age_update_mode: std::ptr::null_mut(),
            u_niagara_component_seek_to_desired_age: std::ptr::null_mut(),
            u_niagara_component_reset_system: std::ptr::null_mut(),
            u_niagara_component_reinitialize_system: std::ptr::null_mut(),
            u_niagara_component_is_paused: std::ptr::null_mut(),
            u_niagara_component_init_for_performance_baseline: std::ptr::null_mut(),
            u_niagara_component_get_variable_vec4: std::ptr::null_mut(),
            u_niagara_component_get_variable_vec3: std::ptr::null_mut(),
            u_niagara_component_get_variable_vec2: std::ptr::null_mut(),
            u_niagara_component_get_variable_quat: std::ptr::null_mut(),
            u_niagara_component_get_variable_position: std::ptr::null_mut(),
            u_niagara_component_get_variable_matrix: std::ptr::null_mut(),
            u_niagara_component_get_variable_int: std::ptr::null_mut(),
            u_niagara_component_get_variable_float: std::ptr::null_mut(),
            u_niagara_component_get_variable_color: std::ptr::null_mut(),
            u_niagara_component_get_variable_bool: std::ptr::null_mut(),
            u_niagara_component_get_tick_behavior: std::ptr::null_mut(),
            u_niagara_component_get_system_fixed_bounds: std::ptr::null_mut(),
            u_niagara_component_get_sim_cache: std::ptr::null_mut(),
            u_niagara_component_get_seek_delta: std::ptr::null_mut(),
            u_niagara_component_get_random_seed_offset: std::ptr::null_mut(),
            u_niagara_component_get_preview_lod_distance_enabled: std::ptr::null_mut(),
            u_niagara_component_get_preview_lod_distance: std::ptr::null_mut(),
            u_niagara_component_get_occlusion_query_mode: std::ptr::null_mut(),
            u_niagara_component_get_max_sim_time: std::ptr::null_mut(),
            u_niagara_component_get_lock_desired_age_delta_time_to_seek_delta: std::ptr::null_mut(),
            u_niagara_component_get_force_solo: std::ptr::null_mut(),
            u_niagara_component_get_force_local_player_effect: std::ptr::null_mut(),
            u_niagara_component_get_emitter_fixed_bounds: std::ptr::null_mut(),
            u_niagara_component_get_desired_age: std::ptr::null_mut(),
            u_niagara_component_get_data_interface: std::ptr::null_mut(),
            u_niagara_component_get_custom_time_dilation: std::ptr::null_mut(),
            u_niagara_component_get_asset: std::ptr::null_mut(),
            u_niagara_component_get_allow_scalability: std::ptr::null_mut(),
            u_niagara_component_get_age_update_mode: std::ptr::null_mut(),
            u_niagara_component_clear_system_fixed_bounds: std::ptr::null_mut(),
            u_niagara_component_clear_sim_cache: std::ptr::null_mut(),
            u_niagara_component_clear_emitter_fixed_bounds: std::ptr::null_mut(),
            u_niagara_component_advance_simulation_by_time: std::ptr::null_mut(),
            u_niagara_component_advance_simulation: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_vector4: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_vector2_d: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_vector: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_spawn_info: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_quat: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_position: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_linear_color: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_int: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_id: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_float: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_enum: std::ptr::null_mut(),
            u_niagara_data_channel_reader_read_bool: std::ptr::null_mut(),
            u_niagara_data_channel_reader_num: std::ptr::null_mut(),
            u_niagara_data_channel_reader_init_access: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_vector4: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_vector2_d: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_vector: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_spawn_info: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_quat: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_position: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_linear_color: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_int: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_id: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_float: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_enum: std::ptr::null_mut(),
            u_niagara_data_channel_writer_write_bool: std::ptr::null_mut(),
            u_niagara_data_channel_writer_num: std::ptr::null_mut(),
            u_niagara_data_channel_writer_init_write: std::ptr::null_mut(),
            u_niagara_data_channel_library_write_to_niagara_data_channel_single_with_context: std::ptr::null_mut(),
            u_niagara_data_channel_library_write_to_niagara_data_channel_single: std::ptr::null_mut(),
            u_niagara_data_channel_library_write_to_niagara_data_channel_with_context: std::ptr::null_mut(),
            u_niagara_data_channel_library_write_to_niagara_data_channel: std::ptr::null_mut(),
            u_niagara_data_channel_library_unsubscribe_from_niagara_data_channel: std::ptr::null_mut(),
            u_niagara_data_channel_library_subscribe_to_niagara_data_channel_with_context: std::ptr::null_mut(),
            u_niagara_data_channel_library_subscribe_to_niagara_data_channel: std::ptr::null_mut(),
            u_niagara_data_channel_library_set_single_property_in_ndc_access_context_instance: std::ptr::null_mut(),
            u_niagara_data_channel_library_set_members_in_ndc_access_context_instance: std::ptr::null_mut(),
            u_niagara_data_channel_library_read_from_niagara_data_channel_single_with_context: std::ptr::null_mut(),
            u_niagara_data_channel_library_read_from_niagara_data_channel_single: std::ptr::null_mut(),
            u_niagara_data_channel_library_read_from_niagara_data_channel_with_context: std::ptr::null_mut(),
            u_niagara_data_channel_library_read_from_niagara_data_channel: std::ptr::null_mut(),
            u_niagara_data_channel_library_prepare_access_context_from_ndc_ref: std::ptr::null_mut(),
            u_niagara_data_channel_library_make_ndc_access_context_instance: std::ptr::null_mut(),
            u_niagara_data_channel_library_get_usable_access_context_from_ndc_ref: std::ptr::null_mut(),
            u_niagara_data_channel_library_get_usable_access_context_from_ndc: std::ptr::null_mut(),
            u_niagara_data_channel_library_get_single_property_in_ndc_access_context_instance: std::ptr::null_mut(),
            u_niagara_data_channel_library_get_niagara_data_channel: std::ptr::null_mut(),
            u_niagara_data_channel_library_get_members_in_ndc_access_context_instance: std::ptr::null_mut(),
            u_niagara_data_channel_library_get_data_channel_element_count_with_context: std::ptr::null_mut(),
            u_niagara_data_channel_library_get_data_channel_element_count: std::ptr::null_mut(),
            u_niagara_data_channel_handler_unsubscribe_from_data_channel_updates: std::ptr::null_mut(),
            u_niagara_data_channel_handler_subscribe_to_data_channel_updates_with_context: std::ptr::null_mut(),
            u_niagara_data_channel_handler_subscribe_to_data_channel_updates: std::ptr::null_mut(),
            u_niagara_data_channel_handler_get_data_channel_writer: std::ptr::null_mut(),
            u_niagara_data_channel_handler_get_data_channel_reader: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_vector_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_vector4_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_vector4: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_vector: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_u_int8_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_u_int8: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_quat_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_quat: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_position_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_position: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_matrix_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_matrix: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_int32_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_int32: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_float_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_float: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_color_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_color: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_bool_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_set_niagara_array_bool: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_vector_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_vector4_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_vector4: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_vector: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_u_int8_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_u_int8: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_quat_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_quat: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_position_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_position: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_matrix_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_matrix: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_int32_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_int32: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_float_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_float: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_color_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_color: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_bool_value: std::ptr::null_mut(),
            u_niagara_data_interface_array_function_library_get_niagara_array_bool: std::ptr::null_mut(),
            u_niagara_particle_callback_handler_receive_particle_data: std::ptr::null_mut(),
            u_niagara_data_interface_grid2_d_collection_get_texture_size: std::ptr::null_mut(),
            u_niagara_data_interface_grid2_d_collection_get_raw_texture_size: std::ptr::null_mut(),
            u_niagara_data_interface_grid2_d_collection_fill_texture2_d: std::ptr::null_mut(),
            u_niagara_data_interface_grid2_d_collection_fill_raw_texture2_d: std::ptr::null_mut(),
            u_niagara_data_interface_grid3_d_collection_get_texture_size: std::ptr::null_mut(),
            u_niagara_data_interface_grid3_d_collection_get_raw_texture_size: std::ptr::null_mut(),
            u_niagara_data_interface_grid3_d_collection_fill_volume_texture: std::ptr::null_mut(),
            u_niagara_data_interface_grid3_d_collection_fill_raw_volume_texture: std::ptr::null_mut(),
            u_niagara_di_rigid_mesh_collision_function_library_set_source_actors: std::ptr::null_mut(),
            u_niagara_data_interface_skeletal_mesh_on_source_end_play: std::ptr::null_mut(),
            a_niagara_editor_preview_actor_calculate_rotation: std::ptr::null_mut(),
            a_niagara_editor_preview_actor_calculate_location: std::ptr::null_mut(),
            u_niagara_function_library_spawn_system_attached_with_params: std::ptr::null_mut(),
            u_niagara_function_library_spawn_system_attached: std::ptr::null_mut(),
            u_niagara_function_library_spawn_system_at_location_with_params: std::ptr::null_mut(),
            u_niagara_function_library_spawn_system_at_location: std::ptr::null_mut(),
            u_niagara_function_library_set_volume_texture_object: std::ptr::null_mut(),
            u_niagara_function_library_set_texture_object: std::ptr::null_mut(),
            u_niagara_function_library_set_texture2_d_array_object: std::ptr::null_mut(),
            u_niagara_function_library_set_skeletal_mesh_data_interface_sampling_regions: std::ptr::null_mut(),
            u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_sockets: std::ptr::null_mut(),
            u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_bones: std::ptr::null_mut(),
            u_niagara_function_library_set_scene_capture2_d_data_interface_managed_mode: std::ptr::null_mut(),
            u_niagara_function_library_set_component_niagara_gpu_ray_traced_collision_group: std::ptr::null_mut(),
            u_niagara_function_library_set_actor_niagara_gpu_ray_traced_collision_group: std::ptr::null_mut(),
            u_niagara_function_library_release_niagara_gpu_ray_traced_collision_group: std::ptr::null_mut(),
            u_niagara_function_library_override_system_user_variable_static_mesh_component: std::ptr::null_mut(),
            u_niagara_function_library_override_system_user_variable_static_mesh: std::ptr::null_mut(),
            u_niagara_function_library_override_system_user_variable_skeletal_mesh_component: std::ptr::null_mut(),
            u_niagara_function_library_get_niagara_parameter_collection: std::ptr::null_mut(),
            u_niagara_function_library_acquire_niagara_gpu_ray_traced_collision_group: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_vector_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_vector4_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_vector2_d_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_quat_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_int_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_float_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_color_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_set_bool_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_vector_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_vector4_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_vector2_d_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_quat_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_int_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_float_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_color_parameter: std::ptr::null_mut(),
            u_niagara_parameter_collection_instance_get_bool_parameter: std::ptr::null_mut(),
            u_niagara_baseline_controller_on_tick_test: std::ptr::null_mut(),
            u_niagara_baseline_controller_on_owner_tick: std::ptr::null_mut(),
            u_niagara_baseline_controller_on_end_test: std::ptr::null_mut(),
            u_niagara_baseline_controller_on_begin_test: std::ptr::null_mut(),
            u_niagara_baseline_controller_get_system: std::ptr::null_mut(),
            a_niagara_preview_base_set_system: std::ptr::null_mut(),
            a_niagara_preview_base_set_label_text: std::ptr::null_mut(),
            u_niagara_preview_axis_num: std::ptr::null_mut(),
            u_niagara_preview_axis_apply_to_preview: std::ptr::null_mut(),
            a_niagara_preview_grid_set_paused: std::ptr::null_mut(),
            a_niagara_preview_grid_get_previews: std::ptr::null_mut(),
            a_niagara_preview_grid_deactivate_previews: std::ptr::null_mut(),
            a_niagara_preview_grid_activate_previews: std::ptr::null_mut(),
            u_niagara_script_raise_on_gpu_compilation_complete: std::ptr::null_mut(),
            u_niagara_sim_cache_read_vector_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_vector4_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_vector2_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_quat_attribute_with_rebase: std::ptr::null_mut(),
            u_niagara_sim_cache_read_quat_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_position_attribute_with_rebase: std::ptr::null_mut(),
            u_niagara_sim_cache_read_position_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_int_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_id_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_float_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_read_data_interface_as: std::ptr::null_mut(),
            u_niagara_sim_cache_read_color_attribute: std::ptr::null_mut(),
            u_niagara_sim_cache_is_empty: std::ptr::null_mut(),
            u_niagara_sim_cache_is_cache_valid: std::ptr::null_mut(),
            u_niagara_sim_cache_get_start_seconds: std::ptr::null_mut(),
            u_niagara_sim_cache_get_num_frames: std::ptr::null_mut(),
            u_niagara_sim_cache_get_num_emitters: std::ptr::null_mut(),
            u_niagara_sim_cache_get_emitter_names: std::ptr::null_mut(),
            u_niagara_sim_cache_get_emitter_name: std::ptr::null_mut(),
            u_niagara_sim_cache_get_attribute_capture_mode: std::ptr::null_mut(),
            u_async_niagara_capture_sim_cache_on_capture_complete_delegate_signature: std::ptr::null_mut(),
            u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_until_complete: std::ptr::null_mut(),
            u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_multi_frame: std::ptr::null_mut(),
            u_async_niagara_capture_sim_cache_capture_niagara_sim_cache: std::ptr::null_mut(),
            u_niagara_sim_cache_function_library_create_niagara_sim_cache: std::ptr::null_mut(),
            u_niagara_sim_cache_function_library_capture_niagara_sim_cache_immediate: std::ptr::null_mut(),
            u_niagara_system_collection_release: std::ptr::null_mut(),
            u_niagara_system_collection_num: std::ptr::null_mut(),
            u_niagara_system_collection_load_synchronous: std::ptr::null_mut(),
            u_niagara_system_collection_load_async: std::ptr::null_mut(),
            u_niagara_system_collection_get_systems: std::ptr::null_mut(),
            u_niagara_stateless_module_decal_attributes_is_orientation_enabled: std::ptr::null_mut(),
            u_niagara_stateless_module_mesh_index_needs_mesh_index_weights: std::ptr::null_mut(),
            u_niagara_stateless_module_scale_mesh_size_use_scale_curve_range: std::ptr::null_mut(),
            u_niagara_stateless_module_scale_ribbon_width_use_scale_curve_range: std::ptr::null_mut(),
            u_niagara_stateless_module_scale_sprite_size_use_scale_curve_range: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceArrayDistributionInt::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayDistributionInt"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_distribution_int_set_niagara_array_distribution_int,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceArrayMesh::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayMeshSM"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_mesh_set_niagara_array_mesh_sm,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayMesh"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_mesh_set_niagara_array_mesh,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceSceneCapture2D::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSceneCapture2DManagedShowOnlyActors"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_scene_capture2_d_set_scene_capture2_d_managed_show_only_actors,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceStaticMesh::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraStaticMeshDIInstanceIndex"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_static_mesh_set_niagara_static_mesh_di_instance_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSourceEndPlay"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_static_mesh_on_source_end_play,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceUObjectPropertyReader::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetUObjectReaderPropertyRemap"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_u_object_property_reader_set_u_object_reader_property_remap,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneNiagaraSystemSpawnSection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionStartBehavior"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_set_section_start_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionEvaluateBehavior"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_set_section_evaluate_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionEndBehavior"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_set_section_end_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowScalability"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_set_allow_scalability,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAgeUpdateMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_set_age_update_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionStartBehavior"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_get_section_start_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionEvaluateBehavior"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_get_section_evaluate_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionEndBehavior"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_get_section_end_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllowScalability"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_get_allow_scalability,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAgeUpdateMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_niagara_system_spawn_section_get_age_update_mode,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ANiagaraActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDestroyOnSystemFinish"),
            &raw mut __FUNCTION_PTRS.a_niagara_actor_set_destroy_on_system_finish,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnNiagaraSystemFinished"),
            &raw mut __FUNCTION_PTRS.a_niagara_actor_on_niagara_system_finished,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDestroyOnSystemFinish"),
            &raw mut __FUNCTION_PTRS.a_niagara_actor_get_destroy_on_system_finish,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraComponent::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableVec4"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_vec4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableVec3"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_vec3,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableVec2"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_vec2,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableTextureRenderTarget"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_component_set_variable_texture_render_target,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableTexture"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_texture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableStaticMesh"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_static_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableQuat"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariablePosition"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableObject"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableMatrix"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_matrix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableMaterial"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableLinearColor"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_linear_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableInt"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableFloat"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableBool"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableActor"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_variable_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTickBehavior"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_tick_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSystemFixedBounds"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_system_fixed_bounds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSimCache"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_sim_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSeekDelta"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_seek_delta,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRenderingEnabled"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_rendering_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRandomSeedOffset"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_random_seed_offset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreviewLODDistance"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_preview_lod_distance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPaused"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_paused,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetOcclusionQueryMode"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_occlusion_query_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableVec4"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_vec4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableVec3"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_vec3,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableVec2"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_vec2,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableQuat"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariablePosition"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableObject"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableMatrix"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_matrix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableLinearColor"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_component_set_niagara_variable_linear_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableInt"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableFloat"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableBool"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraVariableActor"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_niagara_variable_actor,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaxSimTime"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_max_sim_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLockDesiredAgeDeltaTimeToSeekDelta"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_component_set_lock_desired_age_delta_time_to_seek_delta,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGpuComputeDebug"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_gpu_compute_debug,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForceSolo"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_force_solo,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForceLocalPlayerEffect"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_force_local_player_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEmitterFixedBounds"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_emitter_fixed_bounds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDesiredAge"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_desired_age,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTimeDilation"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_custom_time_dilation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCanRenderWhileSeeking"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_can_render_while_seeking,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAutoDestroy"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_auto_destroy,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAsset"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAllowScalability"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_allow_scalability,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAgeUpdateMode"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_set_age_update_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SeekToDesiredAge"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_seek_to_desired_age,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetSystem"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_reset_system,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReinitializeSystem"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_reinitialize_system,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPaused"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_is_paused,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitForPerformanceBaseline"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_init_for_performance_baseline,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableVec4"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_vec4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableVec3"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_vec3,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableVec2"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_vec2,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableQuat"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariablePosition"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableMatrix"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_matrix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableInt"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableFloat"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableColor"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableBool"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_variable_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTickBehavior"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_tick_behavior,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSystemFixedBounds"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_system_fixed_bounds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSimCache"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_sim_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSeekDelta"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_seek_delta,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRandomSeedOffset"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_random_seed_offset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewLODDistanceEnabled"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_preview_lod_distance_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviewLODDistance"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_preview_lod_distance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetOcclusionQueryMode"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_occlusion_query_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaxSimTime"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_max_sim_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLockDesiredAgeDeltaTimeToSeekDelta"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_component_get_lock_desired_age_delta_time_to_seek_delta,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetForceSolo"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_force_solo,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetForceLocalPlayerEffect"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_force_local_player_effect,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEmitterFixedBounds"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_emitter_fixed_bounds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDesiredAge"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_desired_age,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataInterface"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_data_interface,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTimeDilation"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_custom_time_dilation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAsset"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_asset,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllowScalability"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_allow_scalability,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAgeUpdateMode"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_get_age_update_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSystemFixedBounds"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_clear_system_fixed_bounds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearSimCache"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_clear_sim_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ClearEmitterFixedBounds"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_clear_emitter_fixed_bounds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AdvanceSimulationByTime"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_advance_simulation_by_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AdvanceSimulation"),
            &raw mut __FUNCTION_PTRS.u_niagara_component_advance_simulation,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataChannelReader::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadVector4"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadVector2D"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadVector"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadSpawnInfo"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_spawn_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadQuat"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadPosition"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadLinearColor"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_linear_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadInt"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadID"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadFloat"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadEnum"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_enum,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadBool"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_read_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Num"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_num,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitAccess"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_reader_init_access,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataChannelWriter::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteVector4"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteVector2D"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteVector"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteSpawnInfo"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_spawn_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteQuat"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WritePosition"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteLinearColor"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_linear_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteInt"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_int,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteID"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteFloat"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteEnum"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_enum,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteBool"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_write_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Num"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_num,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("InitWrite"),
            &raw mut __FUNCTION_PTRS.u_niagara_data_channel_writer_init_write,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataChannelLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteToNiagaraDataChannelSingle_WithContext"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_write_to_niagara_data_channel_single_with_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteToNiagaraDataChannelSingle"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_write_to_niagara_data_channel_single,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteToNiagaraDataChannel_WithContext"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_write_to_niagara_data_channel_with_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("WriteToNiagaraDataChannel"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_write_to_niagara_data_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnsubscribeFromNiagaraDataChannel"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_unsubscribe_from_niagara_data_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SubscribeToNiagaraDataChannel_WithContext"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_subscribe_to_niagara_data_channel_with_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SubscribeToNiagaraDataChannel"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_subscribe_to_niagara_data_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSinglePropertyInNDCAccessContextInstance"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_set_single_property_in_ndc_access_context_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMembersInNDCAccessContextInstance"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_set_members_in_ndc_access_context_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadFromNiagaraDataChannelSingle_WithContext"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_read_from_niagara_data_channel_single_with_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadFromNiagaraDataChannelSingle"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_read_from_niagara_data_channel_single,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadFromNiagaraDataChannel_WithContext"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_read_from_niagara_data_channel_with_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadFromNiagaraDataChannel"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_read_from_niagara_data_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("PrepareAccessContextFromNDCRef"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_prepare_access_context_from_ndc_ref,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeNDCAccessContextInstance"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_make_ndc_access_context_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUsableAccessContextFromNDCRef"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_get_usable_access_context_from_ndc_ref,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUsableAccessContextFromNDC"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_get_usable_access_context_from_ndc,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSinglePropertyInNDCAccessContextInstance"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_get_single_property_in_ndc_access_context_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraDataChannel"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_get_niagara_data_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMembersInNDCAccessContextInstance"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_get_members_in_ndc_access_context_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataChannelElementCount_WithContext"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_get_data_channel_element_count_with_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataChannelElementCount"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_library_get_data_channel_element_count,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataChannelHandler::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UnsubscribeFromDataChannelUpdates"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_handler_unsubscribe_from_data_channel_updates,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SubscribeToDataChannelUpdates_WithContext"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_handler_subscribe_to_data_channel_updates_with_context,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SubscribeToDataChannelUpdates"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_handler_subscribe_to_data_channel_updates,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataChannelWriter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_handler_get_data_channel_writer,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDataChannelReader"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_channel_handler_get_data_channel_reader,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceArrayFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayVectorValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_vector_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayVector4Value"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_vector4_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayVector4"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayVector2DValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayVector2D"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayVector"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayUInt8Value"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_u_int8_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayUInt8"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_u_int8,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayQuatValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_quat_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayQuat"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayPositionValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_position_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayPosition"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayMatrixValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_matrix_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayMatrix"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_matrix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayInt32Value"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_int32_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayInt32"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_int32,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayFloatValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_float_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayFloat"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayColorValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_color_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayColor"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayBoolValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_bool_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNiagaraArrayBool"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_set_niagara_array_bool,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayVectorValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_vector_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayVector4Value"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_vector4_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayVector4"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_vector4,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayVector2DValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayVector2D"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayVector"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_vector,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayUInt8Value"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_u_int8_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayUInt8"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_u_int8,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayQuatValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_quat_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayQuat"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_quat,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayPositionValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_position_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayPosition"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayMatrixValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_matrix_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayMatrix"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_matrix,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayInt32Value"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_int32_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayInt32"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_int32,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayFloatValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_float_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayFloat"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_float,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayColorValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_color_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayColor"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayBoolValue"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_bool_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraArrayBool"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_array_function_library_get_niagara_array_bool,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraParticleCallbackHandler::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReceiveParticleData"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_particle_callback_handler_receive_particle_data,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceGrid2DCollection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureSize"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid2_d_collection_get_texture_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRawTextureSize"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid2_d_collection_get_raw_texture_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FillTexture2D"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid2_d_collection_fill_texture2_d,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FillRawTexture2D"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid2_d_collection_fill_raw_texture2_d,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceGrid3DCollection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureSize"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid3_d_collection_get_texture_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRawTextureSize"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid3_d_collection_get_raw_texture_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FillVolumeTexture"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid3_d_collection_fill_volume_texture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FillRawVolumeTexture"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_grid3_d_collection_fill_raw_volume_texture,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDIRigidMeshCollisionFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSourceActors"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_di_rigid_mesh_collision_function_library_set_source_actors,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraDataInterfaceSkeletalMesh::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnSourceEndPlay"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_data_interface_skeletal_mesh_on_source_end_play,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ANiagaraEditorPreviewActor::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CalculateRotation"),
            &raw mut __FUNCTION_PTRS.a_niagara_editor_preview_actor_calculate_rotation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CalculateLocation"),
            &raw mut __FUNCTION_PTRS.a_niagara_editor_preview_actor_calculate_location,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnSystemAttachedWithParams"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_spawn_system_attached_with_params,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnSystemAttached"),
            &raw mut __FUNCTION_PTRS.u_niagara_function_library_spawn_system_attached,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnSystemAtLocationWithParams"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_spawn_system_at_location_with_params,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SpawnSystemAtLocation"),
            &raw mut __FUNCTION_PTRS.u_niagara_function_library_spawn_system_at_location,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVolumeTextureObject"),
            &raw mut __FUNCTION_PTRS.u_niagara_function_library_set_volume_texture_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTextureObject"),
            &raw mut __FUNCTION_PTRS.u_niagara_function_library_set_texture_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTexture2DArrayObject"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_set_texture2_d_array_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMeshDataInterfaceSamplingRegions"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_set_skeletal_mesh_data_interface_sampling_regions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMeshDataInterfaceFilteredSockets"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_sockets,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletalMeshDataInterfaceFilteredBones"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_bones,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSceneCapture2DDataInterfaceManagedMode"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_set_scene_capture2_d_data_interface_managed_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetComponentNiagaraGPURayTracedCollisionGroup"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_set_component_niagara_gpu_ray_traced_collision_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetActorNiagaraGPURayTracedCollisionGroup"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_set_actor_niagara_gpu_ray_traced_collision_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReleaseNiagaraGPURayTracedCollisionGroup"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_release_niagara_gpu_ray_traced_collision_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideSystemUserVariableStaticMeshComponent"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_override_system_user_variable_static_mesh_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideSystemUserVariableStaticMesh"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_override_system_user_variable_static_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OverrideSystemUserVariableSkeletalMeshComponent"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_override_system_user_variable_skeletal_mesh_component,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNiagaraParameterCollection"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_get_niagara_parameter_collection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AcquireNiagaraGPURayTracedCollisionGroup"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_function_library_acquire_niagara_gpu_ray_traced_collision_group,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraParameterCollectionInstance::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVectorParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_vector_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector4Parameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_vector4_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVector2DParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_vector2_d_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetQuatParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_quat_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIntParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_int_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFloatParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_float_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_color_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoolParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_set_bool_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_vector_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVector4Parameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_vector4_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVector2DParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_vector2_d_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetQuatParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_quat_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetIntParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_int_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloatParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_float_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetColorParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_color_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoolParameter"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_parameter_collection_instance_get_bool_parameter,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraBaselineController::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnTickTest"),
            &raw mut __FUNCTION_PTRS.u_niagara_baseline_controller_on_tick_test,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnOwnerTick"),
            &raw mut __FUNCTION_PTRS.u_niagara_baseline_controller_on_owner_tick,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnEndTest"),
            &raw mut __FUNCTION_PTRS.u_niagara_baseline_controller_on_end_test,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnBeginTest"),
            &raw mut __FUNCTION_PTRS.u_niagara_baseline_controller_on_begin_test,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSystem"),
            &raw mut __FUNCTION_PTRS.u_niagara_baseline_controller_get_system,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ANiagaraPreviewBase::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSystem"),
            &raw mut __FUNCTION_PTRS.a_niagara_preview_base_set_system,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLabelText"),
            &raw mut __FUNCTION_PTRS.a_niagara_preview_base_set_label_text,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraPreviewAxis::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Num"),
            &raw mut __FUNCTION_PTRS.u_niagara_preview_axis_num,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ApplyToPreview"),
            &raw mut __FUNCTION_PTRS.u_niagara_preview_axis_apply_to_preview,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ANiagaraPreviewGrid::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPaused"),
            &raw mut __FUNCTION_PTRS.a_niagara_preview_grid_set_paused,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreviews"),
            &raw mut __FUNCTION_PTRS.a_niagara_preview_grid_get_previews,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeactivatePreviews"),
            &raw mut __FUNCTION_PTRS.a_niagara_preview_grid_deactivate_previews,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ActivatePreviews"),
            &raw mut __FUNCTION_PTRS.a_niagara_preview_grid_activate_previews,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraScript::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RaiseOnGPUCompilationComplete"),
            &raw mut __FUNCTION_PTRS.u_niagara_script_raise_on_gpu_compilation_complete,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraSimCache::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadVectorAttribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_vector_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadVector4Attribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_vector4_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadVector2Attribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_vector2_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadQuatAttributeWithRebase"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_quat_attribute_with_rebase,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadQuatAttribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_quat_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadPositionAttributeWithRebase"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_sim_cache_read_position_attribute_with_rebase,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadPositionAttribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_position_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadIntAttribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_int_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadIDAttribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_id_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadFloatAttribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_float_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadDataInterfaceAs"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_data_interface_as,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReadColorAttribute"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_read_color_attribute,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsEmpty"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_is_empty,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsCacheValid"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_is_cache_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartSeconds"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_get_start_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumFrames"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_get_num_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumEmitters"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_get_num_emitters,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEmitterNames"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_get_emitter_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEmitterName"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_get_emitter_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAttributeCaptureMode"),
            &raw mut __FUNCTION_PTRS.u_niagara_sim_cache_get_attribute_capture_mode,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAsyncNiagaraCaptureSimCache::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("OnCaptureComplete__DelegateSignature"),
            &raw mut __FUNCTION_PTRS
                .u_async_niagara_capture_sim_cache_on_capture_complete_delegate_signature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureNiagaraSimCacheUntilComplete"),
            &raw mut __FUNCTION_PTRS
                .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_until_complete,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureNiagaraSimCacheMultiFrame"),
            &raw mut __FUNCTION_PTRS
                .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_multi_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureNiagaraSimCache"),
            &raw mut __FUNCTION_PTRS
                .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraSimCacheFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CreateNiagaraSimCache"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_sim_cache_function_library_create_niagara_sim_cache,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CaptureNiagaraSimCacheImmediate"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_sim_cache_function_library_capture_niagara_sim_cache_immediate,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraSystemCollection::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Release"),
            &raw mut __FUNCTION_PTRS.u_niagara_system_collection_release,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Num"),
            &raw mut __FUNCTION_PTRS.u_niagara_system_collection_num,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadSynchronous"),
            &raw mut __FUNCTION_PTRS.u_niagara_system_collection_load_synchronous,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LoadAsync"),
            &raw mut __FUNCTION_PTRS.u_niagara_system_collection_load_async,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSystems"),
            &raw mut __FUNCTION_PTRS.u_niagara_system_collection_get_systems,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraStatelessModule_DecalAttributes::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsOrientationEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_stateless_module_decal_attributes_is_orientation_enabled,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraStatelessModule_MeshIndex::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NeedsMeshIndexWeights"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_stateless_module_mesh_index_needs_mesh_index_weights,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraStatelessModule_ScaleMeshSize::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UseScaleCurveRange"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_stateless_module_scale_mesh_size_use_scale_curve_range,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraStatelessModule_ScaleRibbonWidth::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UseScaleCurveRange"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_stateless_module_scale_ribbon_width_use_scale_curve_range,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UNiagaraStatelessModule_ScaleSpriteSize::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("UseScaleCurveRange"),
            &raw mut __FUNCTION_PTRS
                .u_niagara_stateless_module_scale_sprite_size_use_scale_curve_range,
        );
    }
}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelUpdateContext {
    pub reader: UPtr<UNiagaraDataChannelReader>,
    pub first_new_data_index: i32,
    pub last_new_data_index: i32,
    pub new_element_count: i32,
}
impl FNiagaraDataChannelUpdateContext {}
#[repr(C, align(4))]
pub struct FNiagaraSimCacheCaptureParameters {
    pub flags_0: u8,
    pub num_frames: i32,
    pub capture_rate: i32,
    pub flags_12: u8,
    pub timeout_frame_count: i32,
    pub flags_20: u8,
    pub immediate_capture_delta_time: f32,
}
impl FNiagaraSimCacheCaptureParameters {}
#[repr(C, align(8))]
pub struct FNiagaraSimCacheCreateParameters {
    pub attribute_capture_mode: ENiagaraSimCacheAttributeCaptureMode,
    #[doc(hidden)]
    pub(crate) __padding_4: [u8; 3],
    pub flags_4: u8,
    pub rebase_include_attributes: TArray<FName>,
    pub rebase_exclude_attributes: TArray<FName>,
    pub interpolation_include_attributes: TArray<FName>,
    pub interpolation_exclude_attributes: TArray<FName>,
    pub explicit_capture_attributes: TArray<FName>,
}
impl FNiagaraSimCacheCreateParameters {}
#[repr(C, align(8))]
pub struct FNiagaraMeshRendererMeshPropertiesBase {
    pub mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    pub scale: crate::bindings::core_u_object::FVector,
    pub rotation: crate::bindings::core_u_object::FRotator,
    pub pivot_offset: crate::bindings::core_u_object::FVector,
    pub pivot_offset_space: ENiagaraMeshPivotOffsetSpace,
}
impl FNiagaraMeshRendererMeshPropertiesBase {}
#[repr(C, align(8))]
pub struct FNiagaraMeshRendererMeshProperties {
    pub(crate) __padding_end: [u8; 1232],
}
impl FNiagaraMeshRendererMeshProperties {}
#[repr(C, align(8))]
pub struct FNiagaraVariable {
    pub(crate) __padding_end: [u8; 72],
}
impl FNiagaraVariable {}
#[repr(C, align(4))]
pub struct FNDIDistributionIntArrayEntry {
    pub(crate) __padding_end: [u8; 8],
}
impl FNDIDistributionIntArrayEntry {}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelSearchParameters {
    pub owning_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub location: crate::bindings::core_u_object::FVector,
    pub flags_32: u8,
}
impl FNiagaraDataChannelSearchParameters {}
#[repr(C, align(8))]
pub struct FNDCAccessContextBase {
    pub owning_component: UPtr<crate::bindings::engine::USceneComponent>,
    pub(crate) __padding_end: [u8; 8],
}
impl FNDCAccessContextBase {}
#[repr(C, align(4))]
pub struct FNDCSpawnedSystemRef {
    pub spawned_system: TWeakObjectPtr<UNiagaraComponent>,
}
impl FNDCSpawnedSystemRef {}
#[repr(C, align(8))]
pub struct FNDCAccessContext {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub location: crate::bindings::core_u_object::FVector,
    pub flags_40: u8,
    pub system_to_spawn: UPtr<crate::bindings::core_u_object::UObject>,
    pub spawned_systems: TArray<FNDCSpawnedSystemRef>,
}
impl FNDCAccessContext {}
#[repr(C, align(8))]
pub struct FNDCAccessContextLegacy {
    #[doc(hidden)]
    pub(crate) __padding_16: [u8; 16],
    pub location: crate::bindings::core_u_object::FVector,
    pub flags_40: u8,
}
impl FNDCAccessContextLegacy {}
#[repr(C, align(8))]
pub struct FNDCAccessContextInst {
    pub access_context: crate::bindings::core_u_object::FInstancedStruct,
}
impl FNDCAccessContextInst {}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelReference {
    pub data_channel: UPtr<UNiagaraDataChannelAsset>,
    pub access_context: FNDCAccessContextInst,
    pub b_custom_access_context: bool,
}
impl FNiagaraDataChannelReference {}
#[repr(C, align(8))]
pub struct FNiagaraDataChannelVariable {
    pub(crate) __padding_end: [u8; 72],
}
impl FNiagaraDataChannelVariable {}
#[repr(C, align(8))]
pub struct FNDCAccessContext_MapBase {
    pub(crate) __padding_end: [u8; 72],
}
impl FNDCAccessContext_MapBase {}
#[repr(C, align(8))]
pub struct FNDCAccessContext_GameplayBurst {
    #[doc(hidden)]
    pub(crate) __padding_72: [u8; 72],
    pub flags_72: u8,
    pub cell_size_override: crate::bindings::core_u_object::FVector,
    pub system_bounds_padding: crate::bindings::core_u_object::FVector,
    pub gameplay_tag: crate::bindings::gameplay_tags::FGameplayTag,
}
impl FNDCAccessContext_GameplayBurst {}
#[repr(C, align(8))]
pub struct FBasicParticleData {
    pub position: crate::bindings::core_u_object::FVector,
    pub size: f32,
    pub velocity: crate::bindings::core_u_object::FVector,
}
impl FBasicParticleData {}
#[repr(C, align(8))]
pub struct FVersionedNiagaraEmitterData {
    #[doc(hidden)]
    pub(crate) __padding_112: [u8; 112],
    pub b_local_space: bool,
    pub b_determinism: bool,
    pub random_seed: i32,
    pub interpolated_spawn_mode: ENiagaraInterpolatedSpawnMode,
    #[doc(hidden)]
    pub(crate) __padding_128: [u8; 7],
    pub sim_target: ENiagaraSimTarget,
    pub fixed_bounds: crate::bindings::core_u_object::FBox,
    pub flags_192: u8,
    #[doc(hidden)]
    pub(crate) __padding_328: [u8; 132],
    pub max_gpu_particles_spawn_per_frame: i32,
    pub allocation_mode: EParticleAllocationMode,
    pub(crate) __padding_end: [u8; 1307],
}
impl FVersionedNiagaraEmitterData {}
#[repr(C, align(4))]
pub struct FNiagaraPerfBaselineStats {
    pub per_instance_avg_gt: f32,
    pub per_instance_avg_rt: f32,
    pub per_instance_max_gt: f32,
    pub per_instance_max_rt: f32,
}
impl FNiagaraPerfBaselineStats {}
#[repr(C, align(4))]
pub struct FNiagaraRendererReadbackParameters {
    pub b_export_position: bool,
    pub b_export_tangent_basis: bool,
    pub b_export_color: bool,
    pub export_num_tex_coords: i32,
    pub b_export_materials: bool,
    pub b_apply_wpo: bool,
    pub view_index_to_capture: TOptional<i32>,
}
impl FNiagaraRendererReadbackParameters {}
#[repr(C, align(8))]
pub struct FNiagaraSystemCollectionData {
    pub(crate) __padding_end: [u8; 48],
}
impl FNiagaraSystemCollectionData {}
#[repr(C, align(4))]
pub struct FNiagaraPosition {
    pub(crate) __padding_end: [u8; 12],
}
impl FNiagaraPosition {}
#[repr(C, align(4))]
pub struct FNiagaraSpawnInfo {
    pub count: i32,
    pub interp_start_dt: f32,
    pub interval_dt: f32,
    pub spawn_group: i32,
}
impl FNiagaraSpawnInfo {}
#[repr(C, align(4))]
pub struct FNiagaraID {
    pub index: i32,
    pub acquire_tag: i32,
}
impl FNiagaraID {}
#[repr(C, align(8))]
pub struct UNiagaraDataInterface {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfacePlatformSet {
    __padding_end: [u8; 248],
}
impl UNiagaraDataInterfacePlatformSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfacePlatformSet")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSystem {
    #[doc(hidden)]
    pub(crate) __padding_857: [u8; 857],
    pub flags_857: u8,
    #[doc(hidden)]
    pub(crate) __padding_860: [u8; 2],
    pub custom_depth_stencil_write_mask: crate::bindings::engine::ERendererStencilMask,
    pub custom_depth_stencil_value: i32,
    pub translucency_sort_priority: i32,
    pub translucency_sort_distance_offset: f32,
    __padding_end: [u8; 2428],
}
impl UNiagaraSystem {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystem")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraConvertInPlaceUtilityBase {
    __padding_end: [u8; 48],
}
impl UNiagaraConvertInPlaceUtilityBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraConvertInPlaceUtilityBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDynamicMeshSimCacheData {
    __padding_end: [u8; 96],
}
impl UNiagaraDataInterfaceDynamicMeshSimCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceDynamicMeshSimCacheData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraParameterDefinitionsBase {
    __padding_end: [u8; 96],
}
impl UNiagaraParameterDefinitionsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParameterDefinitionsBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct INiagaraRenderableMeshArrayInterface {}
#[repr(C, align(8))]
pub struct UNiagaraRenderableMeshArrayInterface {
    __padding_end: [u8; 48],
}
impl UNiagaraRenderableMeshArrayInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraRenderableMeshArrayInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct INiagaraRenderableMeshInterface {}
#[repr(C, align(8))]
pub struct UNiagaraRenderableMeshInterface {
    __padding_end: [u8; 48],
}
impl UNiagaraRenderableMeshInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraRenderableMeshInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraScriptSourceBase {
    __padding_end: [u8; 112],
}
impl UNiagaraScriptSourceBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScriptSourceBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct INiagaraSimCacheCustomStorageInterface {}
#[repr(C, align(8))]
pub struct UNiagaraSimCacheCustomStorageInterface {
    __padding_end: [u8; 48],
}
impl UNiagaraSimCacheCustomStorageInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSimCacheCustomStorageInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraValidationRuleSet {
    __padding_end: [u8; 64],
}
impl UNiagaraValidationRuleSet {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRuleSet")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNDIRenderTargetSimCacheData {
    __padding_end: [u8; 96],
}
impl UNDIRenderTargetSimCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNDIRenderTargetSimCacheData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceActorComponent {
    __padding_end: [u8; 264],
}
impl UNiagaraDataInterfaceActorComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceActorComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayDistributionInt {
    __padding_end: [u8; 192],
}
impl UNiagaraDataInterfaceArrayDistributionInt {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayDistributionInt")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_niagara_array_distribution_int(
        niagara_component: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<FNDIDistributionIntArrayEntry>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_distribution_int_set_niagara_array_distribution_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer.add(24).cast::<TArray<FNDIDistributionIntArrayEntry>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayDistributionInt::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_distribution_int_set_niagara_array_distribution_int,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRWBase {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceRWBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceRWBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArray {
    #[doc(hidden)]
    pub(crate) __padding_168: [u8; 168],
    pub gpu_sync_mode: ENiagaraGpuSyncMode,
    pub max_elements: i32,
}
impl UNiagaraDataInterfaceArray {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArray")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayMesh {
    __padding_end: [u8; 200],
}
impl UNiagaraDataInterfaceArrayMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayMesh")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_niagara_array_mesh_sm(
        niagara_component: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<UPtr<crate::bindings::engine::UStaticMesh>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_mesh_set_niagara_array_mesh_sm,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<UPtr<crate::bindings::engine::UStaticMesh>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayMesh::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_mesh_set_niagara_array_mesh_sm,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_mesh(
        niagara_component: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<FNiagaraMeshRendererMeshPropertiesBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_mesh_set_niagara_array_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<FNiagaraMeshRendererMeshPropertiesBase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayMesh::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_mesh_set_niagara_array_mesh,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAsyncGpuTrace {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceAsyncGpuTrace {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceAsyncGpuTrace")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceConsoleVariable {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceConsoleVariable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceConsoleVariable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDataChannelRead {
    __padding_end: [u8; 296],
}
impl UNiagaraDataInterfaceDataChannelRead {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceDataChannelRead")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNDIDataChannelWriteSimCacheData {
    __padding_end: [u8; 120],
}
impl UNDIDataChannelWriteSimCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNDIDataChannelWriteSimCacheData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDataChannelWrite {
    __padding_end: [u8; 384],
}
impl UNiagaraDataInterfaceDataChannelWrite {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceDataChannelWrite")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDataTable {
    __padding_end: [u8; 256],
}
impl UNiagaraDataInterfaceDataTable {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceDataTable")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDebugDraw {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceDebugDraw {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceDebugDraw")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceDynamicMesh {
    __padding_end: [u8; 272],
}
impl UNiagaraDataInterfaceDynamicMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceDynamicMesh")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceEmitterProperties {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceEmitterProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceEmitterProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGBuffer {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceGBuffer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceGBuffer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMemoryBuffer {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceMemoryBuffer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceMemoryBuffer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNDIMemoryBufferSimCacheData {
    __padding_end: [u8; 80],
}
impl UNDIMemoryBufferSimCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNDIMemoryBufferSimCacheData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfacePhysicsAsset {
    __padding_end: [u8; 320],
}
impl UNiagaraDataInterfacePhysicsAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfacePhysicsAsset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct INiagaraPhysicsAssetDICollectorInterface {}
#[repr(C, align(8))]
pub struct UNiagaraPhysicsAssetDICollectorInterface {
    __padding_end: [u8; 48],
}
impl UNiagaraPhysicsAssetDICollectorInterface {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPhysicsAssetDICollectorInterface")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSceneCapture2D {
    __padding_end: [u8; 448],
}
impl UNiagaraDataInterfaceSceneCapture2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSceneCapture2D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_scene_capture2_d_managed_show_only_actors(
        niagara_system: UPtr<UNiagaraComponent>,
        parameter_name: FName,
        show_only_actors: TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_scene_capture2_d_set_scene_capture2_d_managed_show_only_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &show_only_actors,
                __buffer.add(24).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceSceneCapture2D::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_scene_capture2_d_set_scene_capture2_d_managed_show_only_actors,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSimCacheReader {
    __padding_end: [u8; 248],
}
impl UNiagaraDataInterfaceSimCacheReader {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSimCacheReader")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSimpleCounter {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceSimpleCounter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSimpleCounter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNDISimpleCounterSimCacheData {
    __padding_end: [u8; 64],
}
impl UNDISimpleCounterSimCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNDISimpleCounterSimCacheData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSocketReader {
    __padding_end: [u8; 360],
}
impl UNiagaraDataInterfaceSocketReader {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSocketReader")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceStaticMesh {
    __padding_end: [u8; 480],
}
impl UNiagaraDataInterfaceStaticMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceStaticMesh")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_niagara_static_mesh_di_instance_index(
        niagara_system: UPtr<UNiagaraComponent>,
        user_parameter_name: FName,
        new_instance_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_static_mesh_set_niagara_static_mesh_di_instance_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_instance_index,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceStaticMesh::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_static_mesh_set_niagara_static_mesh_di_instance_index,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceUObjectPropertyReader {
    __padding_end: [u8; 312],
}
impl UNiagaraDataInterfaceUObjectPropertyReader {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceUObjectPropertyReader")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_u_object_reader_property_remap(
        niagara_component: UPtr<UNiagaraComponent>,
        user_parameter_name: FName,
        graph_name: FName,
        remap_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_u_object_property_reader_set_u_object_reader_property_remap,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &user_parameter_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &graph_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &remap_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceUObjectPropertyReader::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_u_object_property_reader_set_u_object_reader_property_remap,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVirtualTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceVirtualTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVirtualTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraSystemSpawnSection {
    __padding_end: [u8; 368],
}
impl UMovieSceneNiagaraSystemSpawnSection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraSystemSpawnSection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_section_start_behavior(
        &mut self,
        in_behavior: ENiagaraSystemSpawnSectionStartBehavior,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_section_start_behavior,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_behavior,
                __buffer.add(0).cast::<ENiagaraSystemSpawnSectionStartBehavior>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_section_start_behavior,
                __buffer,
            )
        };
    }
    pub fn set_section_evaluate_behavior(
        &mut self,
        in_behavior: ENiagaraSystemSpawnSectionEvaluateBehavior,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_section_evaluate_behavior,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_behavior,
                __buffer.add(0).cast::<ENiagaraSystemSpawnSectionEvaluateBehavior>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_section_evaluate_behavior,
                __buffer,
            )
        };
    }
    pub fn set_section_end_behavior(
        &mut self,
        in_behavior: ENiagaraSystemSpawnSectionEndBehavior,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_section_end_behavior,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_behavior,
                __buffer.add(0).cast::<ENiagaraSystemSpawnSectionEndBehavior>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_section_end_behavior,
                __buffer,
            )
        };
    }
    pub fn set_allow_scalability(&mut self, b_in_allow_scalability: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_allow_scalability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_allow_scalability,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_allow_scalability,
                __buffer,
            )
        };
    }
    pub fn set_age_update_mode(&mut self, in_mode: ENiagaraAgeUpdateMode) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_age_update_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mode,
                __buffer.add(0).cast::<ENiagaraAgeUpdateMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_set_age_update_mode,
                __buffer,
            )
        };
    }
    pub fn get_section_start_behavior(&self) -> ENiagaraSystemSpawnSectionStartBehavior {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_section_start_behavior,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_section_start_behavior,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<ENiagaraSystemSpawnSectionStartBehavior>().read()
        }
    }
    pub fn get_section_evaluate_behavior(
        &self,
    ) -> ENiagaraSystemSpawnSectionEvaluateBehavior {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_section_evaluate_behavior,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_section_evaluate_behavior,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<ENiagaraSystemSpawnSectionEvaluateBehavior>().read()
        }
    }
    pub fn get_section_end_behavior(&self) -> ENiagaraSystemSpawnSectionEndBehavior {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_section_end_behavior,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_section_end_behavior,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ENiagaraSystemSpawnSectionEndBehavior>().read() }
    }
    pub fn get_allow_scalability(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_allow_scalability,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_allow_scalability,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_age_update_mode(&self) -> ENiagaraAgeUpdateMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_age_update_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_movie_scene_niagara_system_spawn_section_get_age_update_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ENiagaraAgeUpdateMode>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraTrack {
    __padding_end: [u8; 400],
}
impl UMovieSceneNiagaraTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraSystemTrack {
    __padding_end: [u8; 408],
}
impl UMovieSceneNiagaraSystemTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraSystemTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraParameterTrack {
    __padding_end: [u8; 472],
}
impl UMovieSceneNiagaraParameterTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraParameterTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraBoolParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraBoolParameterTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraBoolParameterTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraColorParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraColorParameterTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraColorParameterTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraFloatParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraFloatParameterTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraFloatParameterTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraIntegerParameterTrack {
    __padding_end: [u8; 480],
}
impl UMovieSceneNiagaraIntegerParameterTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraIntegerParameterTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneNiagaraVectorParameterTrack {
    __padding_end: [u8; 488],
}
impl UMovieSceneNiagaraVectorParameterTrack {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneNiagaraVectorParameterTrack")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ANiagaraActor {
    #[doc(hidden)]
    pub(crate) __padding_1136: [u8; 1136],
    pub niagara_component: UPtr<UNiagaraComponent>,
    __padding_end: [u8; 24],
}
impl ANiagaraActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANiagaraActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_destroy_on_system_finish(
        &mut self,
        b_should_destroy_on_system_finish: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_actor_set_destroy_on_system_finish,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_should_destroy_on_system_finish,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_actor_set_destroy_on_system_finish,
                __buffer,
            )
        };
    }
    pub fn get_destroy_on_system_finish(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_actor_get_destroy_on_system_finish,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_actor_get_destroy_on_system_finish,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UDEPRECATED_NiagaraAssetTagDefinitions {
    __padding_end: [u8; 104],
}
impl UDEPRECATED_NiagaraAssetTagDefinitions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UDEPRECATED_NiagaraAssetTagDefinitions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutput {
    __padding_end: [u8; 64],
}
impl UNiagaraBakerOutput {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerOutput")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputSimCache {
    __padding_end: [u8; 168],
}
impl UNiagaraBakerOutputSimCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerOutputSimCache")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputSparseVolumeTexture {
    __padding_end: [u8; 480],
}
impl UNiagaraBakerOutputSparseVolumeTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerOutputSparseVolumeTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputStaticMesh {
    __padding_end: [u8; 104],
}
impl UNiagaraBakerOutputStaticMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerOutputStaticMesh")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputTexture2D {
    __padding_end: [u8; 176],
}
impl UNiagaraBakerOutputTexture2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerOutputTexture2D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerOutputVolumeTexture {
    __padding_end: [u8; 152],
}
impl UNiagaraBakerOutputVolumeTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerOutputVolumeTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBakerSettings {
    __padding_end: [u8; 512],
}
impl UNiagaraBakerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBakerSettings")
            .unwrap()
    }
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
pub struct UNiagaraComponent {
    #[doc(hidden)]
    pub(crate) __padding_2552: [u8; 2552],
    pub flags_2552: u8,
    #[doc(hidden)]
    pub(crate) __padding_2561: [u8; 8],
    pub occlusion_query_mode: ENiagaraOcclusionQueryMode,
    #[doc(hidden)]
    pub(crate) __padding_2592: [u8; 28],
    pub auto_attach_parent: TWeakObjectPtr<crate::bindings::engine::USceneComponent>,
    pub auto_attach_socket_name: FName,
    pub auto_attach_location_rule: crate::bindings::engine::EAttachmentRule,
    pub auto_attach_rotation_rule: crate::bindings::engine::EAttachmentRule,
    pub auto_attach_scale_rule: crate::bindings::engine::EAttachmentRule,
    #[doc(hidden)]
    pub(crate) __padding_2648: [u8; 33],
    pub flags_2648: u8,
    __padding_end: [u8; 311],
}
impl UNiagaraComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_variable_vec4(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_vec4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_vec4,
                __buffer,
            )
        };
    }
    pub fn set_variable_vec3(
        &mut self,
        in_variable_name: FName,
        in_value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_vec3,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_vec3,
                __buffer,
            )
        };
    }
    pub fn set_variable_vec2(
        &mut self,
        in_variable_name: FName,
        in_value: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_vec2,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_vec2,
                __buffer,
            )
        };
    }
    pub fn set_variable_texture_render_target(
        &mut self,
        in_variable_name: FName,
        texture_render_target: UPtr<crate::bindings::engine::UTextureRenderTarget>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_texture_render_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture_render_target,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_texture_render_target,
                __buffer,
            )
        };
    }
    pub fn set_variable_texture(
        &mut self,
        in_variable_name: FName,
        texture: UPtr<crate::bindings::engine::UTexture>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_texture,
                __buffer,
            )
        };
    }
    pub fn set_variable_static_mesh(
        &mut self,
        in_variable_name: FName,
        in_value: UPtr<crate::bindings::engine::UStaticMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_static_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_static_mesh,
                __buffer,
            )
        };
    }
    pub fn set_variable_quat(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_quat,
                __buffer,
            )
        };
    }
    pub fn set_variable_position(
        &mut self,
        in_variable_name: FName,
        in_value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_position,
                __buffer,
            )
        };
    }
    pub fn set_variable_object(
        &mut self,
        in_variable_name: FName,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_object,
                __buffer,
            )
        };
    }
    pub fn set_variable_matrix(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FMatrix,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_matrix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_matrix,
                __buffer,
            )
        };
    }
    pub fn set_variable_material(
        &mut self,
        in_variable_name: FName,
        object: UPtr<crate::bindings::engine::UMaterialInterface>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UMaterialInterface>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_material,
                __buffer,
            )
        };
    }
    pub fn set_variable_linear_color(
        &mut self,
        in_variable_name: FName,
        in_value: &crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_linear_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_linear_color,
                __buffer,
            )
        };
    }
    pub fn set_variable_int(&mut self, in_variable_name: FName, in_value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_int,
                __buffer,
            )
        };
    }
    pub fn set_variable_float(&mut self, in_variable_name: FName, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_float,
                __buffer,
            )
        };
    }
    pub fn set_variable_bool(&mut self, in_variable_name: FName, in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(12).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_bool,
                __buffer,
            )
        };
    }
    pub fn set_variable_actor(
        &mut self,
        in_variable_name: FName,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_variable_actor,
                __buffer,
            )
        };
    }
    pub fn set_tick_behavior(&mut self, new_tick_behavior: ENiagaraTickBehavior) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_tick_behavior,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_tick_behavior,
                __buffer.add(0).cast::<ENiagaraTickBehavior>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_tick_behavior,
                __buffer,
            )
        };
    }
    pub fn set_system_fixed_bounds(
        &mut self,
        local_bounds: crate::bindings::core_u_object::FBox,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_system_fixed_bounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_bounds,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_system_fixed_bounds,
                __buffer,
            )
        };
    }
    pub fn set_sim_cache(
        &mut self,
        sim_cache: UPtr<UNiagaraSimCache>,
        b_reset_system: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_sim_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_cache,
                __buffer.add(0).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_system,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_sim_cache,
                __buffer,
            )
        };
    }
    pub fn set_seek_delta(&mut self, in_seek_delta: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_seek_delta,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_seek_delta,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_seek_delta,
                __buffer,
            )
        };
    }
    pub fn set_rendering_enabled(&mut self, b_in_rendering_enabled: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_rendering_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_rendering_enabled,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_rendering_enabled,
                __buffer,
            )
        };
    }
    pub fn set_random_seed_offset(&mut self, new_random_seed_offset: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_random_seed_offset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_random_seed_offset,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_random_seed_offset,
                __buffer,
            )
        };
    }
    pub fn set_preview_lod_distance(
        &mut self,
        b_enable_preview_lod_distance: bool,
        preview_lod_distance: f32,
        preview_max_distance: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_preview_lod_distance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_preview_lod_distance,
                __buffer.add(0).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_lod_distance,
                __buffer.add(4).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_max_distance,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_preview_lod_distance,
                __buffer,
            )
        };
    }
    pub fn set_paused(&mut self, b_in_paused: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_set_paused,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_paused,
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
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_set_paused,
                __buffer,
            )
        };
    }
    pub fn set_occlusion_query_mode(&mut self, mode: ENiagaraOcclusionQueryMode) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_occlusion_query_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mode,
                __buffer.add(0).cast::<ENiagaraOcclusionQueryMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_occlusion_query_mode,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_vec4(
        &mut self,
        in_variable_name: FString,
        in_value: &crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_vec4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_vec4,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_vec3(
        &mut self,
        in_variable_name: FString,
        in_value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_vec3,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_vec3,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_vec2(
        &mut self,
        in_variable_name: FString,
        in_value: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_vec2,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_vec2,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_quat(
        &mut self,
        in_variable_name: FString,
        in_value: &crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_quat,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_position(
        &mut self,
        in_variable_name: FString,
        in_value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_position,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_object(
        &mut self,
        in_variable_name: FString,
        object: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_object,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_matrix(
        &mut self,
        in_variable_name: FString,
        in_value: &crate::bindings::core_u_object::FMatrix,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_matrix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_matrix,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_linear_color(
        &mut self,
        in_variable_name: FString,
        in_value: &crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_linear_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_linear_color,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_int(
        &mut self,
        in_variable_name: FString,
        in_value: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_int,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_float(
        &mut self,
        in_variable_name: FString,
        in_value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_float,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_bool(
        &mut self,
        in_variable_name: FString,
        in_value: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_bool,
                __buffer,
            )
        };
    }
    pub fn set_niagara_variable_actor(
        &mut self,
        in_variable_name: FString,
        actor: UPtr<crate::bindings::engine::AActor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_actor,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &actor,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_niagara_variable_actor,
                __buffer,
            )
        };
    }
    pub fn set_max_sim_time(&mut self, in_max_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_max_sim_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_max_time,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_max_sim_time,
                __buffer,
            )
        };
    }
    pub fn set_lock_desired_age_delta_time_to_seek_delta(&mut self, b_lock: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_lock_desired_age_delta_time_to_seek_delta,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_lock, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_lock_desired_age_delta_time_to_seek_delta,
                __buffer,
            )
        };
    }
    pub fn set_gpu_compute_debug(&mut self, b_enable_debug: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_gpu_compute_debug,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_enable_debug,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_gpu_compute_debug,
                __buffer,
            )
        };
    }
    pub fn set_force_solo(&mut self, b_in_force_solo: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_force_solo,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_force_solo,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_force_solo,
                __buffer,
            )
        };
    }
    pub fn set_force_local_player_effect(&mut self, b_is_player_effect: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_force_local_player_effect,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_player_effect,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_force_local_player_effect,
                __buffer,
            )
        };
    }
    pub fn set_emitter_fixed_bounds(
        &mut self,
        emitter_name: FName,
        local_bounds: crate::bindings::core_u_object::FBox,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_emitter_fixed_bounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &local_bounds,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FBox>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_emitter_fixed_bounds,
                __buffer,
            )
        };
    }
    pub fn set_desired_age(&mut self, in_desired_age: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_desired_age,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_desired_age,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_desired_age,
                __buffer,
            )
        };
    }
    pub fn set_custom_time_dilation(&mut self, dilation: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_custom_time_dilation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&dilation, __buffer.add(0).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_custom_time_dilation,
                __buffer,
            )
        };
    }
    pub fn set_can_render_while_seeking(&mut self, b_in_can_render_while_seeking: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_can_render_while_seeking,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_can_render_while_seeking,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_can_render_while_seeking,
                __buffer,
            )
        };
    }
    pub fn set_auto_destroy(&mut self, b_in_auto_destroy: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_auto_destroy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_auto_destroy,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_auto_destroy,
                __buffer,
            )
        };
    }
    pub fn set_asset(
        &mut self,
        in_asset: UPtr<UNiagaraSystem>,
        b_reset_existing_override_parameters: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_set_asset,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_asset,
                __buffer.add(0).cast::<UPtr<UNiagaraSystem>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_existing_override_parameters,
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
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_set_asset,
                __buffer,
            )
        };
    }
    pub fn set_allow_scalability(&mut self, b_allow: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_allow_scalability,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_allow, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_allow_scalability,
                __buffer,
            )
        };
    }
    pub fn set_age_update_mode(&mut self, in_age_update_mode: ENiagaraAgeUpdateMode) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_age_update_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_age_update_mode,
                __buffer.add(0).cast::<ENiagaraAgeUpdateMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_set_age_update_mode,
                __buffer,
            )
        };
    }
    pub fn seek_to_desired_age(&mut self, in_desired_age: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_seek_to_desired_age,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_desired_age,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_seek_to_desired_age,
                __buffer,
            )
        };
    }
    pub fn reset_system(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_reset_system,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_reset_system,
                __buffer,
            )
        };
    }
    pub fn reinitialize_system(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_reinitialize_system,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_reinitialize_system,
                __buffer,
            )
        };
    }
    pub fn is_paused(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_is_paused,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_is_paused,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn init_for_performance_baseline(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_init_for_performance_baseline,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_init_for_performance_baseline,
                __buffer,
            )
        };
    }
    pub fn get_variable_vec4(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_vec4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_vec4,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn get_variable_vec3(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_vec3,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_vec3,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_variable_vec2(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_vec2,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_vec2,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_variable_quat(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_quat,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_variable_position(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_variable_matrix(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FMatrix {
        let mut __stack = crate::core_data::StackAlloc::<144>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_matrix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_matrix,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FMatrix>().read()
        }
    }
    pub fn get_variable_int(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_int,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_variable_float(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_float,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_variable_color(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn get_variable_bool(
        &self,
        in_variable_name: FName,
        b_is_valid: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<14>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_is_valid,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_variable_bool,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(b_is_valid);
        }
        unsafe { __buffer.add(13).cast::<bool>().read() }
    }
    pub fn get_tick_behavior(&self) -> ENiagaraTickBehavior {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_tick_behavior,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_tick_behavior,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ENiagaraTickBehavior>().read() }
    }
    pub fn get_system_fixed_bounds(&self) -> crate::bindings::core_u_object::FBox {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_system_fixed_bounds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_system_fixed_bounds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<crate::bindings::core_u_object::FBox>().read() }
    }
    pub fn get_sim_cache(&self) -> UPtr<UNiagaraSimCache> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_sim_cache,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_sim_cache,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UNiagaraSimCache>>().read() }
    }
    pub fn get_seek_delta(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_seek_delta,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_seek_delta,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_random_seed_offset(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_random_seed_offset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_random_seed_offset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_preview_lod_distance_enabled(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_preview_lod_distance_enabled,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_preview_lod_distance_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_preview_lod_distance(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_preview_lod_distance,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_preview_lod_distance,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_occlusion_query_mode(&self) -> ENiagaraOcclusionQueryMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_occlusion_query_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_occlusion_query_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ENiagaraOcclusionQueryMode>().read() }
    }
    pub fn get_max_sim_time(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_max_sim_time,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_max_sim_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_lock_desired_age_delta_time_to_seek_delta(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_lock_desired_age_delta_time_to_seek_delta,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_lock_desired_age_delta_time_to_seek_delta,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_force_solo(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_force_solo,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_force_solo,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_force_local_player_effect(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_force_local_player_effect,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_force_local_player_effect,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_emitter_fixed_bounds(
        &self,
        emitter_name: FName,
    ) -> crate::bindings::core_u_object::FBox {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_emitter_fixed_bounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_emitter_fixed_bounds,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<crate::bindings::core_u_object::FBox>().read() }
    }
    pub fn get_desired_age(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_desired_age,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_desired_age,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_data_interface(&mut self, name: FString) -> UPtr<UNiagaraDataInterface> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_data_interface,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_data_interface,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UNiagaraDataInterface>>().read() }
    }
    pub fn get_custom_time_dilation(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_custom_time_dilation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_custom_time_dilation,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_asset(&self) -> UPtr<UNiagaraSystem> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_get_asset,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_component_get_asset,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UNiagaraSystem>>().read() }
    }
    pub fn get_allow_scalability(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_allow_scalability,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_allow_scalability,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_age_update_mode(&self) -> ENiagaraAgeUpdateMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_age_update_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_get_age_update_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ENiagaraAgeUpdateMode>().read() }
    }
    pub fn clear_system_fixed_bounds(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_clear_system_fixed_bounds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_clear_system_fixed_bounds,
                __buffer,
            )
        };
    }
    pub fn clear_sim_cache(&mut self, b_reset_system: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_clear_sim_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_system,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_clear_sim_cache,
                __buffer,
            )
        };
    }
    pub fn clear_emitter_fixed_bounds(&mut self, emitter_name: FName) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_clear_emitter_fixed_bounds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_clear_emitter_fixed_bounds,
                __buffer,
            )
        };
    }
    pub fn advance_simulation_by_time(
        &mut self,
        simulate_time: f32,
        tick_delta_seconds: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_advance_simulation_by_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &simulate_time,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tick_delta_seconds,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_advance_simulation_by_time,
                __buffer,
            )
        };
    }
    pub fn advance_simulation(&mut self, tick_count: i32, tick_delta_seconds: f32) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_advance_simulation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&tick_count, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tick_delta_seconds,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_component_advance_simulation,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraComponentPool {
    __padding_end: [u8; 176],
}
impl UNiagaraComponentPool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraComponentPool")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraRendererProperties {
    __padding_end: [u8; 672],
}
impl UNiagaraRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraRendererProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraComponentRendererProperties {
    __padding_end: [u8; 1600],
}
impl UNiagaraComponentRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraComponentRendererProperties")
            .unwrap()
    }
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
pub struct UNiagaraCullProxyComponent {
    __padding_end: [u8; 2992],
}
impl UNiagaraCullProxyComponent {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraCullProxyComponent")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel {
    __padding_end: [u8; 152],
}
impl UNiagaraDataChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelReader {
    __padding_end: [u8; 80],
}
impl UNiagaraDataChannelReader {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelReader")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn read_vector4(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_vector4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_vector4,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn read_vector2_d(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_vector2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn read_vector(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn read_spawn_info(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> FNiagaraSpawnInfo {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_spawn_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_spawn_info,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe { __buffer.add(20).cast::<FNiagaraSpawnInfo>().read() }
    }
    pub fn read_quat(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_quat,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn read_position(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_position,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn read_linear_color(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_linear_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_linear_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe {
            __buffer
                .add(20)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn read_int(&self, var_name: FName, index: i32, is_valid: &mut bool) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_int,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn read_id(
        &self,
        var_name: FName,
        index: i32,
        is_valid: &mut bool,
    ) -> FNiagaraID {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_id,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe { __buffer.add(20).cast::<FNiagaraID>().read() }
    }
    pub fn read_float(&self, var_name: FName, index: i32, is_valid: &mut bool) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_float,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe { __buffer.add(24).cast::<f64>().read() }
    }
    pub fn read_enum(&self, var_name: FName, index: i32, is_valid: &mut bool) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_enum,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe { __buffer.add(17).cast::<u8>().read() }
    }
    pub fn read_bool(&self, var_name: FName, index: i32, is_valid: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_read_bool,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(is_valid);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn num(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_num,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_reader_num,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelWriter {
    __padding_end: [u8; 80],
}
impl UNiagaraDataChannelWriter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelWriter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn write_vector4(
        &mut self,
        var_name: FName,
        index: i32,
        in_data: crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_vector4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_vector4,
                __buffer,
            )
        };
    }
    pub fn write_vector2_d(
        &mut self,
        var_name: FName,
        index: i32,
        in_data: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_vector2_d,
                __buffer,
            )
        };
    }
    pub fn write_vector(
        &mut self,
        var_name: FName,
        index: i32,
        in_data: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_vector,
                __buffer,
            )
        };
    }
    pub fn write_spawn_info(
        &mut self,
        var_name: FName,
        index: i32,
        in_data: FNiagaraSpawnInfo,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_spawn_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
                __buffer.add(16).cast::<FNiagaraSpawnInfo>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_spawn_info,
                __buffer,
            )
        };
    }
    pub fn write_quat(
        &mut self,
        var_name: FName,
        index: i32,
        in_data: crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_quat,
                __buffer,
            )
        };
    }
    pub fn write_position(
        &mut self,
        var_name: FName,
        index: i32,
        in_data: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_position,
                __buffer,
            )
        };
    }
    pub fn write_linear_color(
        &mut self,
        var_name: FName,
        index: i32,
        in_data: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_linear_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_linear_color,
                __buffer,
            )
        };
    }
    pub fn write_int(&mut self, var_name: FName, index: i32, in_data: i32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_int,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_data, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_int,
                __buffer,
            )
        };
    }
    pub fn write_id(&mut self, var_name: FName, index: i32, in_data: FNiagaraID) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_data,
                __buffer.add(16).cast::<FNiagaraID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_id,
                __buffer,
            )
        };
    }
    pub fn write_float(&mut self, var_name: FName, index: i32, in_data: f64) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_data, __buffer.add(16).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_float,
                __buffer,
            )
        };
    }
    pub fn write_enum(&mut self, var_name: FName, index: i32, in_data: u8) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_data, __buffer.add(16).cast::<u8>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_enum,
                __buffer,
            )
        };
    }
    pub fn write_bool(&mut self, var_name: FName, index: i32, in_data: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&var_name, __buffer.add(0).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_data, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_write_bool,
                __buffer,
            )
        };
    }
    pub fn num(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_num,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_writer_num,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelAsset {
    __padding_end: [u8; 64],
}
impl UNiagaraDataChannelAsset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelAsset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraDataChannelLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn write_to_niagara_data_channel_with_context(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        access_context: &mut FNDCAccessContextInst,
        count: i32,
        b_visible_to_blueprint: bool,
        b_visible_to_niagara_cpu: bool,
        b_visible_to_niagara_gpu: bool,
        debug_source: FString,
    ) -> UPtr<UNiagaraDataChannelWriter> {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_write_to_niagara_data_channel_with_context,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(16).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&count, __buffer.add(32).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_to_blueprint,
                __buffer.add(36).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_to_niagara_cpu,
                __buffer.add(37).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_to_niagara_gpu,
                __buffer.add(38).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &debug_source,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_write_to_niagara_data_channel_with_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FNDCAccessContextInst>().swap(access_context);
        }
        unsafe { __buffer.add(56).cast::<UPtr<UNiagaraDataChannelWriter>>().read() }
    }
    pub fn write_to_niagara_data_channel(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        search_params: FNiagaraDataChannelSearchParameters,
        count: i32,
        b_visible_to_game: bool,
        b_visible_to_cpu: bool,
        b_visible_to_gpu: bool,
        debug_source: FString,
    ) -> UPtr<UNiagaraDataChannelWriter> {
        let mut __stack = crate::core_data::StackAlloc::<88>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_write_to_niagara_data_channel,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search_params,
                __buffer.add(16).cast::<FNiagaraDataChannelSearchParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&count, __buffer.add(56).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_to_game,
                __buffer.add(60).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_to_cpu,
                __buffer.add(61).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_visible_to_gpu,
                __buffer.add(62).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &debug_source,
                __buffer.add(64).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_write_to_niagara_data_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(80).cast::<UPtr<UNiagaraDataChannelWriter>>().read() }
    }
    pub fn unsubscribe_from_niagara_data_channel(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        unsubscribe_token: &i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_unsubscribe_from_niagara_data_channel,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                unsubscribe_token,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_unsubscribe_from_niagara_data_channel,
                __buffer,
            )
        };
    }
    pub fn subscribe_to_niagara_data_channel_with_context(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        access_context: &mut FNDCAccessContextInst,
        update_delegate: &FSubscribeToNiagaraDataChannel_WithContext_UpdateDelegate,
        unsubscribe_token: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<68>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_subscribe_to_niagara_data_channel_with_context,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(16).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_delegate,
                __buffer
                    .add(32)
                    .cast::<FSubscribeToNiagaraDataChannel_WithContext_UpdateDelegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                unsubscribe_token,
                __buffer.add(64).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_subscribe_to_niagara_data_channel_with_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FNDCAccessContextInst>().swap(access_context);
        }
        unsafe {
            __buffer.add(64).cast::<i32>().swap(unsubscribe_token);
        }
    }
    pub fn subscribe_to_niagara_data_channel(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        search_params: FNiagaraDataChannelSearchParameters,
        update_delegate: &FSubscribeToNiagaraDataChannel_UpdateDelegate,
        unsubscribe_token: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_subscribe_to_niagara_data_channel,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search_params,
                __buffer.add(16).cast::<FNiagaraDataChannelSearchParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                update_delegate,
                __buffer.add(56).cast::<FSubscribeToNiagaraDataChannel_UpdateDelegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                unsubscribe_token,
                __buffer.add(88).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_subscribe_to_niagara_data_channel,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(88).cast::<i32>().swap(unsubscribe_token);
        }
    }
    pub fn set_single_property_in_ndc_access_context_instance(
        access_context: &mut FNDCAccessContextInst,
        context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        property_name: FName,
        value: &i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_set_single_property_in_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(0).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_struct,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(36).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_set_single_property_in_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FNDCAccessContextInst>().swap(access_context);
        }
    }
    pub fn set_members_in_ndc_access_context_instance(
        access_context: &mut FNDCAccessContextInst,
        context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_set_members_in_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(0).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_struct,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_set_members_in_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FNDCAccessContextInst>().swap(access_context);
        }
    }
    pub fn read_from_niagara_data_channel_with_context(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        access_context: &mut FNDCAccessContextInst,
        b_read_previous_frame: bool,
    ) -> UPtr<UNiagaraDataChannelReader> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_read_from_niagara_data_channel_with_context,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(16).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_read_previous_frame,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_read_from_niagara_data_channel_with_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FNDCAccessContextInst>().swap(access_context);
        }
        unsafe { __buffer.add(40).cast::<UPtr<UNiagaraDataChannelReader>>().read() }
    }
    pub fn read_from_niagara_data_channel(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        search_params: FNiagaraDataChannelSearchParameters,
        b_read_previous_frame: bool,
    ) -> UPtr<UNiagaraDataChannelReader> {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_read_from_niagara_data_channel,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search_params,
                __buffer.add(16).cast::<FNiagaraDataChannelSearchParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_read_previous_frame,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_read_from_niagara_data_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(64).cast::<UPtr<UNiagaraDataChannelReader>>().read() }
    }
    pub fn prepare_access_context_from_ndc_ref(
        ndc_ref: &mut FNiagaraDataChannelReference,
    ) -> FNDCAccessContextInst {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_prepare_access_context_from_ndc_ref,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ndc_ref,
                __buffer.add(0).cast::<FNiagaraDataChannelReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_prepare_access_context_from_ndc_ref,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FNiagaraDataChannelReference>().swap(ndc_ref);
        }
        unsafe { __buffer.add(32).cast::<FNDCAccessContextInst>().read() }
    }
    pub fn make_ndc_access_context_instance(
        context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) -> FNDCAccessContextInst {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_make_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_struct,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_make_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FNDCAccessContextInst>().read() }
    }
    pub fn get_usable_access_context_from_ndc_ref(
        ndc_ref: &FNiagaraDataChannelReference,
    ) -> FNDCAccessContextInst {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_usable_access_context_from_ndc_ref,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                ndc_ref,
                __buffer.add(0).cast::<FNiagaraDataChannelReference>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_usable_access_context_from_ndc_ref,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<FNDCAccessContextInst>().read() }
    }
    pub fn get_usable_access_context_from_ndc(
        data_channel: UPtr<UNiagaraDataChannelAsset>,
    ) -> FNDCAccessContextInst {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_usable_access_context_from_ndc,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &data_channel,
                __buffer.add(0).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_usable_access_context_from_ndc,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FNDCAccessContextInst>().read() }
    }
    pub fn get_single_property_in_ndc_access_context_instance(
        access_context: &FNDCAccessContextInst,
        context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
        property_name: FName,
        value: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_single_property_in_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(0).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_struct,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(36).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_single_property_in_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(36).cast::<i32>().swap(value);
        }
    }
    pub fn get_members_in_ndc_access_context_instance(
        access_context: &FNDCAccessContextInst,
        context_struct: UPtr<crate::bindings::core_u_object::UScriptStruct>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_members_in_ndc_access_context_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(0).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context_struct,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::core_u_object::UScriptStruct>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_members_in_ndc_access_context_instance,
                __buffer,
            )
        };
    }
    pub fn get_data_channel_element_count_with_context(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        access_context: &mut FNDCAccessContextInst,
        b_read_previous_frame: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_data_channel_element_count_with_context,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(16).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_read_previous_frame,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_data_channel_element_count_with_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FNDCAccessContextInst>().swap(access_context);
        }
        unsafe { __buffer.add(36).cast::<i32>().read() }
    }
    pub fn get_data_channel_element_count(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        channel: UPtr<UNiagaraDataChannelAsset>,
        search_params: FNiagaraDataChannelSearchParameters,
        b_read_previous_frame: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_data_channel_element_count,
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
                &channel,
                __buffer.add(8).cast::<UPtr<UNiagaraDataChannelAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search_params,
                __buffer.add(16).cast::<FNiagaraDataChannelSearchParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_read_previous_frame,
                __buffer.add(56).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataChannelLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_library_get_data_channel_element_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(60).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler {
    __padding_end: [u8; 160],
}
impl UNiagaraDataChannelHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelHandler")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn unsubscribe_from_data_channel_updates(&mut self, unsubscribe_token: &i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_unsubscribe_from_data_channel_updates,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                unsubscribe_token,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_unsubscribe_from_data_channel_updates,
                __buffer,
            )
        };
    }
    pub fn subscribe_to_data_channel_updates_with_context(
        &mut self,
        update_delegate: FSubscribeToDataChannelUpdates_WithContext_UpdateDelegate,
        access_context: &mut FNDCAccessContextInst,
        unsubscribe_token: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_subscribe_to_data_channel_updates_with_context,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &update_delegate,
                __buffer
                    .add(0)
                    .cast::<FSubscribeToDataChannelUpdates_WithContext_UpdateDelegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                access_context,
                __buffer.add(32).cast::<FNDCAccessContextInst>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                unsubscribe_token,
                __buffer.add(48).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_subscribe_to_data_channel_updates_with_context,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FNDCAccessContextInst>().swap(access_context);
        }
        unsafe {
            __buffer.add(48).cast::<i32>().swap(unsubscribe_token);
        }
    }
    pub fn subscribe_to_data_channel_updates(
        &mut self,
        update_delegate: FSubscribeToDataChannelUpdates_UpdateDelegate,
        search_params: FNiagaraDataChannelSearchParameters,
        unsubscribe_token: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_subscribe_to_data_channel_updates,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &update_delegate,
                __buffer.add(0).cast::<FSubscribeToDataChannelUpdates_UpdateDelegate>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &search_params,
                __buffer.add(32).cast::<FNiagaraDataChannelSearchParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                unsubscribe_token,
                __buffer.add(72).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_subscribe_to_data_channel_updates,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(72).cast::<i32>().swap(unsubscribe_token);
        }
    }
    pub fn get_data_channel_writer(&mut self) -> UPtr<UNiagaraDataChannelWriter> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_get_data_channel_writer,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_get_data_channel_writer,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UNiagaraDataChannelWriter>>().read() }
    }
    pub fn get_data_channel_reader(&mut self) -> UPtr<UNiagaraDataChannelReader> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_get_data_channel_reader,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_channel_handler_get_data_channel_reader,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UNiagaraDataChannelReader>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_MapBase {
    __padding_end: [u8; 160],
}
impl UNiagaraDataChannel_MapBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannel_MapBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_GameplayBurst {
    __padding_end: [u8; 264],
}
impl UNiagaraDataChannel_GameplayBurst {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannel_GameplayBurst")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_MapBase {
    __padding_end: [u8; 376],
}
impl UNiagaraDataChannelHandler_MapBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelHandler_MapBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_GameplayBurst {
    __padding_end: [u8; 480],
}
impl UNiagaraDataChannelHandler_GameplayBurst {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelHandler_GameplayBurst")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_Global {
    __padding_end: [u8; 152],
}
impl UNiagaraDataChannel_Global {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannel_Global")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_Global {
    __padding_end: [u8; 176],
}
impl UNiagaraDataChannelHandler_Global {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelHandler_Global")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannel_Islands {
    __padding_end: [u8; 288],
}
impl UNiagaraDataChannel_Islands {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannel_Islands")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataChannelHandler_Islands {
    __padding_end: [u8; 208],
}
impl UNiagaraDataChannelHandler_Islands {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataChannelHandler_Islands")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterface2DArrayTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterface2DArrayTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterface2DArrayTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNDIArraySimCacheData {
    __padding_end: [u8; 104],
}
impl UNDIArraySimCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNDIArraySimCacheData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub float_data: TArray<f32>,
}
impl UNiagaraDataInterfaceArrayFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayFloat")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat2 {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub float_data: TArray<crate::bindings::core_u_object::FVector2D>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayFloat2 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayFloat2")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat3 {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub float_data: TArray<crate::bindings::core_u_object::FVector>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayFloat3 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayFloat3")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayPosition {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub position_data: TArray<FNiagaraPosition>,
}
impl UNiagaraDataInterfaceArrayPosition {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayPosition")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFloat4 {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub float_data: TArray<crate::bindings::core_u_object::FVector4>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayFloat4 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayFloat4")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayColor {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub color_data: TArray<crate::bindings::core_u_object::FLinearColor>,
}
impl UNiagaraDataInterfaceArrayColor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayColor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayQuat {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub quat_data: TArray<crate::bindings::core_u_object::FQuat>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayQuat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayQuat")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayMatrix {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub matrix_data: TArray<crate::bindings::core_u_object::FMatrix>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayMatrix {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayMatrix")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraDataInterfaceArrayFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_niagara_array_vector_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &crate::bindings::core_u_object::FVector,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_vector4_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &crate::bindings::core_u_object::FVector4,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector4_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector4_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_vector4(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<crate::bindings::core_u_object::FVector4>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector4>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector4,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_vector2_d_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &crate::bindings::core_u_object::FVector2D,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_vector2_d(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<crate::bindings::core_u_object::FVector2D>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector2_d,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_vector(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_vector,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_u_int8_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: i32,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_u_int8_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_u_int8_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_u_int8(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<i32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_u_int8,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer.add(24).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_u_int8,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_quat_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &crate::bindings::core_u_object::FQuat,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_quat_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(64).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_quat_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_quat(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<crate::bindings::core_u_object::FQuat>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer.add(24).cast::<TArray<crate::bindings::core_u_object::FQuat>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_quat,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_position_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &crate::bindings::core_u_object::FVector,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_position_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(48).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_position_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_position(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_position,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_matrix_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &crate::bindings::core_u_object::FMatrix,
        b_size_to_fit: bool,
        b_apply_lwc_rebase: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<162>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_matrix_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(160).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_lwc_rebase,
                __buffer.add(161).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_matrix_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_matrix(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<crate::bindings::core_u_object::FMatrix>,
        b_apply_lwc_rebase: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_matrix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FMatrix>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_lwc_rebase,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_matrix,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_int32_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: i32,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_int32_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_int32_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_int32(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<i32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_int32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer.add(24).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_int32,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_float_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: f32,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_float_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_float_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_float(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_float,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_color_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &crate::bindings::core_u_object::FLinearColor,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_color_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                value,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_color_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_color(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<crate::bindings::core_u_object::FLinearColor>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_color,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_bool_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        value: &bool,
        b_size_to_fit: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_bool_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(24).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_size_to_fit,
                __buffer.add(25).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_bool_value,
                __buffer,
            )
        };
    }
    pub fn set_niagara_array_bool(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        array_data: &TArray<bool>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                array_data,
                __buffer.add(24).cast::<TArray<bool>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_set_niagara_array_bool,
                __buffer,
            )
        };
    }
    pub fn get_niagara_array_vector_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_niagara_array_vector4_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector4_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector4_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn get_niagara_array_vector4(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FVector4> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector4,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector4,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector4>>()
                .read()
        }
    }
    pub fn get_niagara_array_vector2_d_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_niagara_array_vector2_d(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FVector2D> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .read()
        }
    }
    pub fn get_niagara_array_vector(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_vector,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_niagara_array_u_int8_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_u_int8_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_u_int8_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<i32>().read() }
    }
    pub fn get_niagara_array_u_int8(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_u_int8,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_u_int8,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<i32>>().read() }
    }
    pub fn get_niagara_array_quat_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_quat_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_quat_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_niagara_array_quat(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FQuat> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_quat,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_quat,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FQuat>>()
                .read()
        }
    }
    pub fn get_niagara_array_position_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_position_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_position_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_niagara_array_position(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FVector> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_position,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_position,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .read()
        }
    }
    pub fn get_niagara_array_matrix_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
        b_apply_lwc_rebase: bool,
    ) -> crate::bindings::core_u_object::FMatrix {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_matrix_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_lwc_rebase,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_matrix_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<crate::bindings::core_u_object::FMatrix>().read()
        }
    }
    pub fn get_niagara_array_matrix(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        b_apply_lwc_rebase: bool,
    ) -> TArray<crate::bindings::core_u_object::FMatrix> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_matrix,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_apply_lwc_rebase,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_matrix,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FMatrix>>()
                .read()
        }
    }
    pub fn get_niagara_array_int32_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_int32_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_int32_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<i32>().read() }
    }
    pub fn get_niagara_array_int32(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_int32,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_int32,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<i32>>().read() }
    }
    pub fn get_niagara_array_float_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_float_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_float_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<f32>().read() }
    }
    pub fn get_niagara_array_float(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_float,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_float,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<f32>>().read() }
    }
    pub fn get_niagara_array_color_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_color_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_color_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .read()
        }
    }
    pub fn get_niagara_array_color(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<crate::bindings::core_u_object::FLinearColor> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_color,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>()
                .read()
        }
    }
    pub fn get_niagara_array_bool_value(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_bool_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_bool_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_niagara_array_bool(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
    ) -> TArray<bool> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_bool,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDataInterfaceArrayFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_array_function_library_get_niagara_array_bool,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<TArray<bool>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayInt32 {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub int_data: TArray<i32>,
}
impl UNiagaraDataInterfaceArrayInt32 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayInt32")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayUInt8 {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub int_data: TArray<i32>,
    __padding_end: [u8; 16],
}
impl UNiagaraDataInterfaceArrayUInt8 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayUInt8")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayBool {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub bool_data: TArray<bool>,
}
impl UNiagaraDataInterfaceArrayBool {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayBool")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceArrayNiagaraID {
    #[doc(hidden)]
    pub(crate) __padding_176: [u8; 176],
    pub int_data: TArray<FNiagaraID>,
}
impl UNiagaraDataInterfaceArrayNiagaraID {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceArrayNiagaraID")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioSubmix {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceAudioSubmix {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceAudioSubmix")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioOscilloscope {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceAudioOscilloscope {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceAudioOscilloscope")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioPlayerSettings {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub b_override_concurrency: bool,
    pub concurrency: UPtr<crate::bindings::engine::USoundConcurrency>,
    pub b_override_attenuation_settings: bool,
    pub attenuation_settings: crate::bindings::engine::FSoundAttenuationSettings,
}
impl UNiagaraDataInterfaceAudioPlayerSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceAudioPlayerSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioPlayer {
    __padding_end: [u8; 280],
}
impl UNiagaraDataInterfaceAudioPlayer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceAudioPlayer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceAudioSpectrum {
    __padding_end: [u8; 176],
}
impl UNiagaraDataInterfaceAudioSpectrum {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceAudioSpectrum")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCamera {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceCamera {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceCamera")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCollisionQuery {
    __padding_end: [u8; 160],
}
impl UNiagaraDataInterfaceCollisionQuery {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceCollisionQuery")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCurveBase {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceCurveBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceCurveBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceColorCurve {
    __padding_end: [u8; 1256],
}
impl UNiagaraDataInterfaceColorCurve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceColorCurve")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCubeTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceCubeTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceCubeTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCurlNoise {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceCurlNoise {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceCurlNoise")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceCurve {
    __padding_end: [u8; 488],
}
impl UNiagaraDataInterfaceCurve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceCurve")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
pub struct INiagaraParticleCallbackHandler {}
#[repr(C, align(8))]
pub struct UNiagaraParticleCallbackHandler {
    __padding_end: [u8; 48],
}
impl UNiagaraParticleCallbackHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParticleCallbackHandler")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn receive_particle_data(
        &mut self,
        data: &TArray<FBasicParticleData>,
        niagara_system: UPtr<UNiagaraSystem>,
        simulation_position_offset: &crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_particle_callback_handler_receive_particle_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                data,
                __buffer.add(0).cast::<TArray<FBasicParticleData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(16).cast::<UPtr<UNiagaraSystem>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                simulation_position_offset,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_particle_callback_handler_receive_particle_data,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceExport {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceExport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceExport")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid2D {
    __padding_end: [u8; 192],
}
impl UNiagaraDataInterfaceGrid2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceGrid2D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid2DCollection {
    __padding_end: [u8; 448],
}
impl UNiagaraDataInterfaceGrid2DCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceGrid2DCollection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_texture_size(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        size_x: &mut i32,
        size_y: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_get_texture_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_x, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_y, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_get_texture_size,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(size_x);
        }
        unsafe {
            __buffer.add(12).cast::<i32>().swap(size_y);
        }
    }
    pub fn get_raw_texture_size(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        size_x: &mut i32,
        size_y: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_get_raw_texture_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_x, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_y, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_get_raw_texture_size,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(size_x);
        }
        unsafe {
            __buffer.add(12).cast::<i32>().swap(size_y);
        }
    }
    pub fn fill_texture2_d(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        dest: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        attribute_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_fill_texture2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dest,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_fill_texture2_d,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn fill_raw_texture2_d(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        dest: UPtr<crate::bindings::engine::UTextureRenderTarget2D>,
        tiles_x: &mut i32,
        tiles_y: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_fill_raw_texture2_d,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dest,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UTextureRenderTarget2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tiles_x, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tiles_y, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid2_d_collection_fill_raw_texture2_d,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(tiles_x);
        }
        unsafe {
            __buffer.add(20).cast::<i32>().swap(tiles_y);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid2DCollectionReader {
    __padding_end: [u8; 480],
}
impl UNiagaraDataInterfaceGrid2DCollectionReader {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceGrid2DCollectionReader")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid3D {
    __padding_end: [u8; 208],
}
impl UNiagaraDataInterfaceGrid3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceGrid3D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid3DCollection {
    __padding_end: [u8; 384],
}
impl UNiagaraDataInterfaceGrid3DCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceGrid3DCollection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_texture_size(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        size_x: &mut i32,
        size_y: &mut i32,
        size_z: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_get_texture_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_x, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_y, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_z, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_get_texture_size,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(size_x);
        }
        unsafe {
            __buffer.add(12).cast::<i32>().swap(size_y);
        }
        unsafe {
            __buffer.add(16).cast::<i32>().swap(size_z);
        }
    }
    pub fn get_raw_texture_size(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        size_x: &mut i32,
        size_y: &mut i32,
        size_z: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_get_raw_texture_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_x, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_y, __buffer.add(12).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(size_z, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_get_raw_texture_size,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(size_x);
        }
        unsafe {
            __buffer.add(12).cast::<i32>().swap(size_y);
        }
        unsafe {
            __buffer.add(16).cast::<i32>().swap(size_z);
        }
    }
    pub fn fill_volume_texture(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        dest: UPtr<crate::bindings::engine::UVolumeTexture>,
        attribute_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_fill_volume_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dest,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UVolumeTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_fill_volume_texture,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn fill_raw_volume_texture(
        &mut self,
        component: UPtr<UNiagaraComponent>,
        dest: UPtr<crate::bindings::engine::UVolumeTexture>,
        tiles_x: &mut i32,
        tiles_y: &mut i32,
        tile_z: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_fill_raw_volume_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &dest,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UVolumeTexture>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tiles_x, __buffer.add(16).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tiles_y, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(tile_z, __buffer.add(24).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_data_interface_grid3_d_collection_fill_raw_volume_texture,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<i32>().swap(tiles_x);
        }
        unsafe {
            __buffer.add(20).cast::<i32>().swap(tiles_y);
        }
        unsafe {
            __buffer.add(24).cast::<i32>().swap(tile_z);
        }
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceGrid3DCollectionReader {
    __padding_end: [u8; 416],
}
impl UNiagaraDataInterfaceGrid3DCollectionReader {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceGrid3DCollectionReader")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceIntRenderTarget2D {
    __padding_end: [u8; 256],
}
impl UNiagaraDataInterfaceIntRenderTarget2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceIntRenderTarget2D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceLandscape {
    __padding_end: [u8; 192],
}
impl UNiagaraDataInterfaceLandscape {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceLandscape")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNDILandscapeSimCacheData {
    __padding_end: [u8; 64],
}
impl UNDILandscapeSimCacheData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNDILandscapeSimCacheData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMaterialInstanceDynamic {
    __padding_end: [u8; 496],
}
impl UNiagaraDataInterfaceMaterialInstanceDynamic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceMaterialInstanceDynamic")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMaterialParameterCollection {
    __padding_end: [u8; 496],
}
impl UNiagaraDataInterfaceMaterialParameterCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceMaterialParameterCollection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceMeshRendererInfo {
    __padding_end: [u8; 184],
}
impl UNiagaraDataInterfaceMeshRendererInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceMeshRendererInfo")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceNeighborGrid3D {
    __padding_end: [u8; 216],
}
impl UNiagaraDataInterfaceNeighborGrid3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceNeighborGrid3D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceOcclusion {
    __padding_end: [u8; 152],
}
impl UNiagaraDataInterfaceOcclusion {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceOcclusion")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceParticleRead {
    __padding_end: [u8; 184],
}
impl UNiagaraDataInterfaceParticleRead {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceParticleRead")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRasterizationGrid3D {
    __padding_end: [u8; 224],
}
impl UNiagaraDataInterfaceRasterizationGrid3D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceRasterizationGrid3D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTarget2D {
    __padding_end: [u8; 328],
}
impl UNiagaraDataInterfaceRenderTarget2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceRenderTarget2D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTarget2DArray {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceRenderTarget2DArray {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceRenderTarget2DArray")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTargetCube {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceRenderTargetCube {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceRenderTargetCube")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRenderTargetVolume {
    __padding_end: [u8; 248],
}
impl UNiagaraDataInterfaceRenderTargetVolume {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceRenderTargetVolume")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceRigidMeshCollisionQuery {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceRigidMeshCollisionQuery {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceRigidMeshCollisionQuery")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDIRigidMeshCollisionFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraDIRigidMeshCollisionFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDIRigidMeshCollisionFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_source_actors(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FName,
        source_actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_di_rigid_mesh_collision_function_library_set_source_actors,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_actors,
                __buffer.add(24).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraDIRigidMeshCollisionFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_di_rigid_mesh_collision_function_library_set_source_actors,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSkeletalMesh {
    __padding_end: [u8; 464],
}
impl UNiagaraDataInterfaceSkeletalMesh {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSkeletalMesh")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSparseVolumeTexture {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceSparseVolumeTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSparseVolumeTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSpline {
    __padding_end: [u8; 368],
}
impl UNiagaraDataInterfaceSpline {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSpline")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceSpriteRendererInfo {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceSpriteRendererInfo {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceSpriteRendererInfo")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVector2DCurve {
    __padding_end: [u8; 744],
}
impl UNiagaraDataInterfaceVector2DCurve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVector2DCurve")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVector4Curve {
    __padding_end: [u8; 1256],
}
impl UNiagaraDataInterfaceVector4Curve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVector4Curve")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVectorCurve {
    __padding_end: [u8; 1000],
}
impl UNiagaraDataInterfaceVectorCurve {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVectorCurve")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVectorField {
    __padding_end: [u8; 168],
}
impl UNiagaraDataInterfaceVectorField {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVectorField")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVirtualTextureSample {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceVirtualTextureSample {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVirtualTextureSample")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVolumeCache {
    __padding_end: [u8; 240],
}
impl UNiagaraDataInterfaceVolumeCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVolumeCache")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDataInterfaceVolumeTexture {
    __padding_end: [u8; 232],
}
impl UNiagaraDataInterfaceVolumeTexture {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDataInterfaceVolumeTexture")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDebugHUDSettings {
    __padding_end: [u8; 664],
}
impl UNiagaraDebugHUDSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDebugHUDSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraDecalRendererProperties {
    __padding_end: [u8; 4312],
}
impl UNiagaraDecalRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraDecalRendererProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraEditorDataBase {
    __padding_end: [u8; 80],
}
impl UNiagaraEditorDataBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEditorDataBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraEditorParametersAdapterBase {
    __padding_end: [u8; 48],
}
impl UNiagaraEditorParametersAdapterBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEditorParametersAdapterBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ANiagaraEditorPreviewActor {
    #[doc(hidden)]
    pub(crate) __padding_1288: [u8; 1288],
    pub niagara_component: UPtr<UNiagaraComponent>,
    __padding_end: [u8; 8],
}
impl ANiagaraEditorPreviewActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANiagaraEditorPreviewActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn calculate_rotation(
        &mut self,
        motion_time: f32,
        out_rotation: &mut crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_editor_preview_actor_calculate_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_time,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_rotation,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_editor_preview_actor_calculate_rotation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FQuat>()
                .swap(out_rotation);
        }
    }
    pub fn calculate_location(
        &mut self,
        motion_time: f32,
        out_location: &mut crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_editor_preview_actor_calculate_location,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &motion_time,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_location,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_editor_preview_actor_calculate_location,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(out_location);
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSignificanceHandler {
    __padding_end: [u8; 48],
}
impl UNiagaraSignificanceHandler {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSignificanceHandler")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSignificanceHandlerDistance {
    __padding_end: [u8; 48],
}
impl UNiagaraSignificanceHandlerDistance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSignificanceHandlerDistance")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSignificanceHandlerAge {
    __padding_end: [u8; 48],
}
impl UNiagaraSignificanceHandlerAge {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSignificanceHandlerAge")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraEffectType {
    __padding_end: [u8; 264],
}
impl UNiagaraEffectType {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEffectType")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraEmitterBase {
    __padding_end: [u8; 96],
}
impl UNiagaraEmitterBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEmitterBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraEmitter {
    __padding_end: [u8; 1720],
}
impl UNiagaraEmitter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEmitter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraEventReceiverEmitterAction {
    __padding_end: [u8; 48],
}
impl UNiagaraEventReceiverEmitterAction {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEventReceiverEmitterAction")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraEventReceiverEmitterAction_SpawnParticles {
    __padding_end: [u8; 56],
}
impl UNiagaraEventReceiverEmitterAction_SpawnParticles {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraEventReceiverEmitterAction_SpawnParticles")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn spawn_system_attached_with_params(
        spawn_params: &crate::bindings::engine::FFXSystemSpawnParameters,
    ) -> UPtr<UNiagaraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_attached_with_params,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_params,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::FFXSystemSpawnParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_attached_with_params,
                __buffer,
            )
        };
        unsafe { __buffer.add(120).cast::<UPtr<UNiagaraComponent>>().read() }
    }
    pub fn spawn_system_attached(
        system_template: UPtr<UNiagaraSystem>,
        attach_to_component: UPtr<crate::bindings::engine::USceneComponent>,
        attach_point_name: FName,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        location_type: crate::bindings::engine::EAttachLocation,
        b_auto_destroy: bool,
        b_auto_activate: bool,
        pooling_method: ENCPoolMethod,
        b_pre_cull_check: bool,
    ) -> UPtr<UNiagaraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_attached,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &system_template,
                __buffer.add(0).cast::<UPtr<UNiagaraSystem>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attach_to_component,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USceneComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attach_point_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
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
            std::ptr::copy_nonoverlapping(
                &location_type,
                __buffer.add(80).cast::<crate::bindings::engine::EAttachLocation>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_destroy,
                __buffer.add(81).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_activate,
                __buffer.add(82).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pooling_method,
                __buffer.add(83).cast::<ENCPoolMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_pre_cull_check,
                __buffer.add(84).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_attached,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<UPtr<UNiagaraComponent>>().read() }
    }
    pub fn spawn_system_at_location_with_params(
        spawn_params: &crate::bindings::engine::FFXSystemSpawnParameters,
    ) -> UPtr<UNiagaraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_at_location_with_params,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawn_params,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::FFXSystemSpawnParameters>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_at_location_with_params,
                __buffer,
            )
        };
        unsafe { __buffer.add(120).cast::<UPtr<UNiagaraComponent>>().read() }
    }
    pub fn spawn_system_at_location(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        system_template: UPtr<UNiagaraSystem>,
        location: crate::bindings::core_u_object::FVector,
        rotation: crate::bindings::core_u_object::FRotator,
        scale: crate::bindings::core_u_object::FVector,
        b_auto_destroy: bool,
        b_auto_activate: bool,
        pooling_method: ENCPoolMethod,
        b_pre_cull_check: bool,
    ) -> UPtr<UNiagaraComponent> {
        let mut __stack = crate::core_data::StackAlloc::<104>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_at_location,
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
                &system_template,
                __buffer.add(8).cast::<UPtr<UNiagaraSystem>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &location,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &rotation,
                __buffer.add(40).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scale,
                __buffer.add(64).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_destroy,
                __buffer.add(88).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_auto_activate,
                __buffer.add(89).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pooling_method,
                __buffer.add(90).cast::<ENCPoolMethod>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_pre_cull_check,
                __buffer.add(91).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_spawn_system_at_location,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<UPtr<UNiagaraComponent>>().read() }
    }
    pub fn set_volume_texture_object(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        texture: UPtr<crate::bindings::engine::UVolumeTexture>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_volume_texture_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UVolumeTexture>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_volume_texture_object,
                __buffer,
            )
        };
    }
    pub fn set_texture_object(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        texture: UPtr<crate::bindings::engine::UTexture>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_texture_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UTexture>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_texture_object,
                __buffer,
            )
        };
    }
    pub fn set_texture2_d_array_object(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        texture: UPtr<crate::bindings::engine::UTexture2DArray>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_texture2_d_array_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UTexture2DArray>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_texture2_d_array_object,
                __buffer,
            )
        };
    }
    pub fn set_skeletal_mesh_data_interface_sampling_regions(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        sampling_regions: &TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_skeletal_mesh_data_interface_sampling_regions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sampling_regions,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_skeletal_mesh_data_interface_sampling_regions,
                __buffer,
            )
        };
    }
    pub fn set_skeletal_mesh_data_interface_filtered_sockets(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        filtered_sockets: &TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_sockets,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                filtered_sockets,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_sockets,
                __buffer,
            )
        };
    }
    pub fn set_skeletal_mesh_data_interface_filtered_bones(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        filtered_bones: &TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                filtered_bones,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_skeletal_mesh_data_interface_filtered_bones,
                __buffer,
            )
        };
    }
    pub fn set_scene_capture2_d_data_interface_managed_mode(
        niagara_system: UPtr<UNiagaraComponent>,
        di_name: &FName,
        managed_capture_source: crate::bindings::engine::ESceneCaptureSource,
        managed_texture_size: crate::bindings::core_u_object::FIntPoint,
        managed_texture_format: crate::bindings::engine::ETextureRenderTargetFormat,
        managed_projection_type: crate::bindings::engine::ECameraProjectionMode,
        managed_fov_angle: f32,
        managed_ortho_width: f32,
        b_managed_capture_every_frame: bool,
        b_managed_capture_on_movement: bool,
        show_only_actors: &TArray<UPtr<crate::bindings::engine::AActor>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_scene_capture2_d_data_interface_managed_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(di_name, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &managed_capture_source,
                __buffer.add(20).cast::<crate::bindings::engine::ESceneCaptureSource>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &managed_texture_size,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FIntPoint>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &managed_texture_format,
                __buffer
                    .add(32)
                    .cast::<crate::bindings::engine::ETextureRenderTargetFormat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &managed_projection_type,
                __buffer
                    .add(33)
                    .cast::<crate::bindings::engine::ECameraProjectionMode>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &managed_fov_angle,
                __buffer.add(36).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &managed_ortho_width,
                __buffer.add(40).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_managed_capture_every_frame,
                __buffer.add(44).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_managed_capture_on_movement,
                __buffer.add(45).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                show_only_actors,
                __buffer.add(48).cast::<TArray<UPtr<crate::bindings::engine::AActor>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_scene_capture2_d_data_interface_managed_mode,
                __buffer,
            )
        };
    }
    pub fn set_component_niagara_gpu_ray_traced_collision_group(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        primitive: UPtr<crate::bindings::engine::UPrimitiveComponent>,
        collision_group: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_component_niagara_gpu_ray_traced_collision_group,
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
                &primitive,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UPrimitiveComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collision_group,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_component_niagara_gpu_ray_traced_collision_group,
                __buffer,
            )
        };
    }
    pub fn set_actor_niagara_gpu_ray_traced_collision_group(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        actor: UPtr<crate::bindings::engine::AActor>,
        collision_group: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_actor_niagara_gpu_ray_traced_collision_group,
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
                &actor,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::AActor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &collision_group,
                __buffer.add(16).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_set_actor_niagara_gpu_ray_traced_collision_group,
                __buffer,
            )
        };
    }
    pub fn release_niagara_gpu_ray_traced_collision_group(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        collision_group: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_release_niagara_gpu_ray_traced_collision_group,
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
                &collision_group,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_release_niagara_gpu_ray_traced_collision_group,
                __buffer,
            )
        };
    }
    pub fn override_system_user_variable_static_mesh_component(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        static_mesh_component: UPtr<crate::bindings::engine::UStaticMeshComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_override_system_user_variable_static_mesh_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh_component,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::UStaticMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_override_system_user_variable_static_mesh_component,
                __buffer,
            )
        };
    }
    pub fn override_system_user_variable_static_mesh(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        static_mesh: UPtr<crate::bindings::engine::UStaticMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_override_system_user_variable_static_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &static_mesh,
                __buffer.add(24).cast::<UPtr<crate::bindings::engine::UStaticMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_override_system_user_variable_static_mesh,
                __buffer,
            )
        };
    }
    pub fn override_system_user_variable_skeletal_mesh_component(
        niagara_system: UPtr<UNiagaraComponent>,
        override_name: FString,
        skeletal_mesh_component: UPtr<crate::bindings::engine::USkeletalMeshComponent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_override_system_user_variable_skeletal_mesh_component,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_system,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeletal_mesh_component,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::engine::USkeletalMeshComponent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_override_system_user_variable_skeletal_mesh_component,
                __buffer,
            )
        };
    }
    pub fn get_niagara_parameter_collection(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
        collection: UPtr<UNiagaraParameterCollection>,
    ) -> UPtr<UNiagaraParameterCollectionInstance> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_get_niagara_parameter_collection,
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
                &collection,
                __buffer.add(8).cast::<UPtr<UNiagaraParameterCollection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_get_niagara_parameter_collection,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<UNiagaraParameterCollectionInstance>>().read()
        }
    }
    pub fn acquire_niagara_gpu_ray_traced_collision_group(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_acquire_niagara_gpu_ray_traced_collision_group,
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
        let __object_ptr = crate::bindings::niagara::UNiagaraFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_function_library_acquire_niagara_gpu_ray_traced_collision_group,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
#[repr(C, align(16))]
pub struct ANiagaraLensEffectBase {
    __padding_end: [u8; 1312],
}
impl ANiagaraLensEffectBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANiagaraLensEffectBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraLightRendererProperties {
    #[doc(hidden)]
    pub(crate) __padding_708: [u8; 708],
    pub inverse_exposure_blend: f32,
    __padding_end: [u8; 3608],
}
impl UNiagaraLightRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraLightRendererProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraMeshRendererProperties {
    __padding_end: [u8; 9648],
}
impl UNiagaraMeshRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraMeshRendererProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraMessageDataBase {
    __padding_end: [u8; 48],
}
impl UNiagaraMessageDataBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraMessageDataBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraParameterCollectionInstance {
    __padding_end: [u8; 624],
}
impl UNiagaraParameterCollectionInstance {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParameterCollectionInstance")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_vector_parameter(
        &mut self,
        in_variable_name: FString,
        in_value: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_vector_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_vector_parameter,
                __buffer,
            )
        };
    }
    pub fn set_vector4_parameter(
        &mut self,
        in_variable_name: FString,
        in_value: &crate::bindings::core_u_object::FVector4,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_vector4_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_vector4_parameter,
                __buffer,
            )
        };
    }
    pub fn set_vector2_d_parameter(
        &mut self,
        in_variable_name: FString,
        in_value: crate::bindings::core_u_object::FVector2D,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_vector2_d_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_vector2_d_parameter,
                __buffer,
            )
        };
    }
    pub fn set_quat_parameter(
        &mut self,
        in_variable_name: FString,
        in_value: &crate::bindings::core_u_object::FQuat,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_quat_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_quat_parameter,
                __buffer,
            )
        };
    }
    pub fn set_int_parameter(&mut self, in_variable_name: FString, in_value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_int_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_int_parameter,
                __buffer,
            )
        };
    }
    pub fn set_float_parameter(&mut self, in_variable_name: FString, in_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_float_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_float_parameter,
                __buffer,
            )
        };
    }
    pub fn set_color_parameter(
        &mut self,
        in_variable_name: FString,
        in_value: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_color_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_value,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_color_parameter,
                __buffer,
            )
        };
    }
    pub fn set_bool_parameter(&mut self, in_variable_name: FString, in_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_bool_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_value, __buffer.add(16).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_set_bool_parameter,
                __buffer,
            )
        };
    }
    pub fn get_vector_parameter(
        &mut self,
        in_variable_name: FString,
    ) -> crate::bindings::core_u_object::FVector {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_vector_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_vector_parameter,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector>().read()
        }
    }
    pub fn get_vector4_parameter(
        &mut self,
        in_variable_name: FString,
    ) -> crate::bindings::core_u_object::FVector4 {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_vector4_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_vector4_parameter,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector4>().read()
        }
    }
    pub fn get_vector2_d_parameter(
        &mut self,
        in_variable_name: FString,
    ) -> crate::bindings::core_u_object::FVector2D {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_vector2_d_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_vector2_d_parameter,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FVector2D>().read()
        }
    }
    pub fn get_quat_parameter(
        &mut self,
        in_variable_name: FString,
    ) -> crate::bindings::core_u_object::FQuat {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_quat_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_quat_parameter,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>().read()
        }
    }
    pub fn get_int_parameter(&mut self, in_variable_name: FString) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_int_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_int_parameter,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn get_float_parameter(&mut self, in_variable_name: FString) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_float_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_float_parameter,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f32>().read() }
    }
    pub fn get_color_parameter(
        &mut self,
        in_variable_name: FString,
    ) -> crate::bindings::core_u_object::FLinearColor {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_color_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_color_parameter,
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
    pub fn get_bool_parameter(&mut self, in_variable_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_bool_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_variable_name,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_parameter_collection_instance_get_bool_parameter,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraParameterCollection {
    __padding_end: [u8; 312],
}
impl UNiagaraParameterCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraParameterCollection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBaselineController {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub test_duration: f32,
    pub effect_type: UPtr<UNiagaraEffectType>,
    pub owner: UPtr<ANiagaraPerfBaselineActor>,
    __padding_end: [u8; 48],
}
impl UNiagaraBaselineController {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBaselineController")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn on_tick_test(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_tick_test,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_tick_test,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn on_owner_tick(&mut self, delta_time: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_owner_tick,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_owner_tick,
                __buffer,
            )
        };
    }
    pub fn on_end_test(&mut self, stats: FNiagaraPerfBaselineStats) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_end_test,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &stats,
                __buffer.add(0).cast::<FNiagaraPerfBaselineStats>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_end_test,
                __buffer,
            )
        };
    }
    pub fn on_begin_test(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_begin_test,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_on_begin_test,
                __buffer,
            )
        };
    }
    pub fn get_system(&mut self) -> UPtr<UNiagaraSystem> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_get_system,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_baseline_controller_get_system,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<UPtr<UNiagaraSystem>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraBaselineController_Basic {
    __padding_end: [u8; 144],
}
impl UNiagaraBaselineController_Basic {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraBaselineController_Basic")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ANiagaraPerfBaselineActor {
    __padding_end: [u8; 1152],
}
impl ANiagaraPerfBaselineActor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANiagaraPerfBaselineActor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPrecompileContainer {
    __padding_end: [u8; 72],
}
impl UNiagaraPrecompileContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPrecompileContainer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ANiagaraPreviewBase {
    __padding_end: [u8; 1136],
}
impl ANiagaraPreviewBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANiagaraPreviewBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_system(&mut self, in_system: UPtr<UNiagaraSystem>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_base_set_system,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_system,
                __buffer.add(0).cast::<UPtr<UNiagaraSystem>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_base_set_system,
                __buffer,
            )
        };
    }
    pub fn set_label_text(&mut self, in_x_axis_text: &FText, in_y_axis_text: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_base_set_label_text,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_x_axis_text,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_y_axis_text,
                __buffer.add(16).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_base_set_label_text,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis {
    __padding_end: [u8; 48],
}
impl UNiagaraPreviewAxis {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn num(&mut self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_preview_axis_num,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_preview_axis_num,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn apply_to_preview(
        &mut self,
        preview_component: UPtr<UNiagaraComponent>,
        preview_index: i32,
        b_is_x_axis: bool,
        out_label_text: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_preview_axis_apply_to_preview,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_component,
                __buffer.add(0).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_x_axis,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_label_text,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_preview_axis_apply_to_preview,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(out_label_text);
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamBase {
    __padding_end: [u8; 64],
}
impl UNiagaraPreviewAxis_InterpParamBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis_InterpParamBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamInt32 {
    __padding_end: [u8; 72],
}
impl UNiagaraPreviewAxis_InterpParamInt32 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis_InterpParamInt32")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamFloat {
    __padding_end: [u8; 72],
}
impl UNiagaraPreviewAxis_InterpParamFloat {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis_InterpParamFloat")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamVector2D {
    __padding_end: [u8; 96],
}
impl UNiagaraPreviewAxis_InterpParamVector2D {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis_InterpParamVector2D")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamVector {
    __padding_end: [u8; 112],
}
impl UNiagaraPreviewAxis_InterpParamVector {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis_InterpParamVector")
            .unwrap()
    }
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
pub struct UNiagaraPreviewAxis_InterpParamVector4 {
    __padding_end: [u8; 128],
}
impl UNiagaraPreviewAxis_InterpParamVector4 {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis_InterpParamVector4")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraPreviewAxis_InterpParamLinearColor {
    __padding_end: [u8; 96],
}
impl UNiagaraPreviewAxis_InterpParamLinearColor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraPreviewAxis_InterpParamLinearColor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ANiagaraPreviewGrid {
    __padding_end: [u8; 1232],
}
impl ANiagaraPreviewGrid {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ANiagaraPreviewGrid")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_paused(&mut self, b_paused: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_set_paused,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_paused, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_set_paused,
                __buffer,
            )
        };
    }
    pub fn get_previews(&mut self, out_previews: &mut TArray<UPtr<UNiagaraComponent>>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_get_previews,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_previews,
                __buffer.add(0).cast::<TArray<UPtr<UNiagaraComponent>>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_get_previews,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<UPtr<UNiagaraComponent>>>().swap(out_previews);
        }
    }
    pub fn deactivate_previews(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_deactivate_previews,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_deactivate_previews,
                __buffer,
            )
        };
    }
    pub fn activate_previews(&mut self, b_reset: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_activate_previews,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&b_reset, __buffer.add(0).cast::<bool>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .a_niagara_preview_grid_activate_previews,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UNiagaraRibbonRendererProperties {
    __padding_end: [u8; 10248],
}
impl UNiagaraRibbonRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraRibbonRendererProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraScratchPadContainer {
    __padding_end: [u8; 64],
}
impl UNiagaraScratchPadContainer {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScratchPadContainer")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraScript {
    __padding_end: [u8; 4112],
}
impl UNiagaraScript {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraScript")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSettings {
    #[doc(hidden)]
    pub(crate) __padding_171: [u8; 171],
    pub b_limit_delta_time: bool,
    pub max_delta_time_per_tick: f32,
    __padding_end: [u8; 256],
}
impl UNiagaraSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSimCache {
    __padding_end: [u8; 568],
}
impl UNiagaraSimCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSimCache")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn read_vector_attribute(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FVector>,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_vector_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_vector_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(out_values);
        }
    }
    pub fn read_vector4_attribute(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FVector4>,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_vector4_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector4>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_vector4_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector4>>()
                .swap(out_values);
        }
    }
    pub fn read_vector2_attribute(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FVector2D>,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_vector2_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector2D>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_vector2_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector2D>>()
                .swap(out_values);
        }
    }
    pub fn read_quat_attribute_with_rebase(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FQuat>,
        quat: crate::bindings::core_u_object::FQuat,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<76>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_quat_attribute_with_rebase,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer.add(0).cast::<TArray<crate::bindings::core_u_object::FQuat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &quat,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FQuat>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(48).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(60).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
                __buffer.add(72).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_quat_attribute_with_rebase,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FQuat>>()
                .swap(out_values);
        }
    }
    pub fn read_quat_attribute(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FQuat>,
        attribute_name: FName,
        emitter_name: FName,
        b_local_space_to_world: bool,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_quat_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer.add(0).cast::<TArray<crate::bindings::core_u_object::FQuat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_local_space_to_world,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
                __buffer.add(44).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_quat_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FQuat>>()
                .swap(out_values);
        }
    }
    pub fn read_position_attribute_with_rebase(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FVector>,
        transform: crate::bindings::core_u_object::FTransform,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<140>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_position_attribute_with_rebase,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
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
                &attribute_name,
                __buffer.add(112).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(124).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
                __buffer.add(136).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_position_attribute_with_rebase,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(out_values);
        }
    }
    pub fn read_position_attribute(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FVector>,
        attribute_name: FName,
        emitter_name: FName,
        b_local_space_to_world: bool,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_position_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_local_space_to_world,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
                __buffer.add(44).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_position_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(out_values);
        }
    }
    pub fn read_int_attribute(
        &self,
        out_values: &mut TArray<i32>,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_int_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer.add(0).cast::<TArray<i32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_int_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<i32>>().swap(out_values);
        }
    }
    pub fn read_id_attribute(
        &self,
        out_values: &mut TArray<FNiagaraID>,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_id_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer.add(0).cast::<TArray<FNiagaraID>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_id_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FNiagaraID>>().swap(out_values);
        }
    }
    pub fn read_float_attribute(
        &self,
        out_values: &mut TArray<f32>,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_float_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer.add(0).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_float_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<f32>>().swap(out_values);
        }
    }
    pub fn read_data_interface_as(
        &self,
        requested_type: TSubclassOf<crate::bindings::core_u_object::UObject>,
        attribute_name: FName,
        frame_index: i32,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_data_interface_as,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &requested_type,
                __buffer
                    .add(0)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
                __buffer.add(20).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_data_interface_as,
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
    pub fn read_color_attribute(
        &self,
        out_values: &mut TArray<crate::bindings::core_u_object::FLinearColor>,
        attribute_name: FName,
        emitter_name: FName,
        frame_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_color_attribute,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_values,
                __buffer
                    .add(0)
                    .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_name,
                __buffer.add(16).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_name,
                __buffer.add(28).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_read_color_attribute,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FLinearColor>>()
                .swap(out_values);
        }
    }
    pub fn is_empty(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_sim_cache_is_empty,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS.u_niagara_sim_cache_is_empty,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_cache_valid(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_is_cache_valid,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_is_cache_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_start_seconds(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_start_seconds,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_start_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_num_frames(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_num_frames,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_num_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_num_emitters(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_num_emitters,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_num_emitters,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_emitter_names(&self) -> TArray<FName> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_emitter_names,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_emitter_names,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<FName>>().read() }
    }
    pub fn get_emitter_name(&self, emitter_index: i32) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_emitter_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &emitter_index,
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
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_emitter_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<FName>().read() }
    }
    pub fn get_attribute_capture_mode(&self) -> ENiagaraSimCacheAttributeCaptureMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_attribute_capture_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_get_attribute_capture_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<ENiagaraSimCacheAttributeCaptureMode>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSimCacheDebugData {
    __padding_end: [u8; 64],
}
impl UNiagaraSimCacheDebugData {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSimCacheDebugData")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAsyncNiagaraCaptureSimCache {
    __padding_end: [u8; 328],
}
impl UAsyncNiagaraCaptureSimCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAsyncNiagaraCaptureSimCache")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn capture_niagara_sim_cache_until_complete(
        sim_cache: UPtr<UNiagaraSimCache>,
        create_parameters: FNiagaraSimCacheCreateParameters,
        niagara_component: UPtr<UNiagaraComponent>,
        out_sim_cache: &mut UPtr<UNiagaraSimCache>,
        capture_rate: i32,
        b_advance_simulation: bool,
        advance_delta_time: f32,
    ) -> UPtr<UAsyncNiagaraCaptureSimCache> {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_until_complete,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_cache,
                __buffer.add(0).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &create_parameters,
                __buffer.add(8).cast::<FNiagaraSimCacheCreateParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(96).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_sim_cache,
                __buffer.add(104).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capture_rate,
                __buffer.add(112).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_advance_simulation,
                __buffer.add(116).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &advance_delta_time,
                __buffer.add(120).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UAsyncNiagaraCaptureSimCache::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_until_complete,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(104).cast::<UPtr<UNiagaraSimCache>>().swap(out_sim_cache);
        }
        unsafe { __buffer.add(128).cast::<UPtr<UAsyncNiagaraCaptureSimCache>>().read() }
    }
    pub fn capture_niagara_sim_cache_multi_frame(
        sim_cache: UPtr<UNiagaraSimCache>,
        create_parameters: FNiagaraSimCacheCreateParameters,
        niagara_component: UPtr<UNiagaraComponent>,
        out_sim_cache: &mut UPtr<UNiagaraSimCache>,
        num_frames: i32,
        capture_rate: i32,
        b_advance_simulation: bool,
        advance_delta_time: f32,
    ) -> UPtr<UAsyncNiagaraCaptureSimCache> {
        let mut __stack = crate::core_data::StackAlloc::<136>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_multi_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_cache,
                __buffer.add(0).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &create_parameters,
                __buffer.add(8).cast::<FNiagaraSimCacheCreateParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(96).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_sim_cache,
                __buffer.add(104).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &num_frames,
                __buffer.add(112).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capture_rate,
                __buffer.add(116).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_advance_simulation,
                __buffer.add(120).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &advance_delta_time,
                __buffer.add(124).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UAsyncNiagaraCaptureSimCache::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache_multi_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(104).cast::<UPtr<UNiagaraSimCache>>().swap(out_sim_cache);
        }
        unsafe { __buffer.add(128).cast::<UPtr<UAsyncNiagaraCaptureSimCache>>().read() }
    }
    pub fn capture_niagara_sim_cache(
        sim_cache: UPtr<UNiagaraSimCache>,
        create_parameters: FNiagaraSimCacheCreateParameters,
        niagara_component: UPtr<UNiagaraComponent>,
        capture_parameters: FNiagaraSimCacheCaptureParameters,
        out_sim_cache: &mut UPtr<UNiagaraSimCache>,
    ) -> UPtr<UAsyncNiagaraCaptureSimCache> {
        let mut __stack = crate::core_data::StackAlloc::<152>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_cache,
                __buffer.add(0).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &create_parameters,
                __buffer.add(8).cast::<FNiagaraSimCacheCreateParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(96).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &capture_parameters,
                __buffer.add(104).cast::<FNiagaraSimCacheCaptureParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_sim_cache,
                __buffer.add(136).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UAsyncNiagaraCaptureSimCache::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_async_niagara_capture_sim_cache_capture_niagara_sim_cache,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(136).cast::<UPtr<UNiagaraSimCache>>().swap(out_sim_cache);
        }
        unsafe { __buffer.add(144).cast::<UPtr<UAsyncNiagaraCaptureSimCache>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSimCacheFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UNiagaraSimCacheFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSimCacheFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn create_niagara_sim_cache(
        world_context_object: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> UPtr<UNiagaraSimCache> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_function_library_create_niagara_sim_cache,
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
        let __object_ptr = crate::bindings::niagara::UNiagaraSimCacheFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_function_library_create_niagara_sim_cache,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<UPtr<UNiagaraSimCache>>().read() }
    }
    pub fn capture_niagara_sim_cache_immediate(
        sim_cache: UPtr<UNiagaraSimCache>,
        create_parameters: FNiagaraSimCacheCreateParameters,
        niagara_component: UPtr<UNiagaraComponent>,
        out_sim_cache: &mut UPtr<UNiagaraSimCache>,
        b_advance_simulation: bool,
        advance_delta_time: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<121>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_function_library_capture_niagara_sim_cache_immediate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sim_cache,
                __buffer.add(0).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &create_parameters,
                __buffer.add(8).cast::<FNiagaraSimCacheCreateParameters>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &niagara_component,
                __buffer.add(96).cast::<UPtr<UNiagaraComponent>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_sim_cache,
                __buffer.add(104).cast::<UPtr<UNiagaraSimCache>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_advance_simulation,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &advance_delta_time,
                __buffer.add(116).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::niagara::UNiagaraSimCacheFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_sim_cache_function_library_capture_niagara_sim_cache_immediate,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(104).cast::<UPtr<UNiagaraSimCache>>().swap(out_sim_cache);
        }
        unsafe { __buffer.add(120).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSimulationStageBase {
    __padding_end: [u8; 136],
}
impl UNiagaraSimulationStageBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSimulationStageBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSimulationStageGeneric {
    __padding_end: [u8; 5496],
}
impl UNiagaraSimulationStageGeneric {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSimulationStageGeneric")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSpriteRendererProperties {
    #[doc(hidden)]
    pub(crate) __padding_11512: [u8; 11512],
    pub b_use_material_cutout_texture: bool,
    __padding_end: [u8; 127],
}
impl UNiagaraSpriteRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSpriteRendererProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraSystemCollection {
    __padding_end: [u8; 112],
}
impl UNiagaraSystemCollection {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraSystemCollection")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn release(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_release,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_release,
                __buffer,
            )
        };
    }
    pub fn num(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_num,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_num,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn load_synchronous(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_load_synchronous,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_load_synchronous,
                __buffer,
            )
        };
    }
    pub fn load_async(&self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_load_async,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_load_async,
                __buffer,
            )
        };
    }
    pub fn get_systems(&self) -> TArray<UPtr<UNiagaraSystem>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_get_systems,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::niagara::__FUNCTION_PTRS
                    .u_niagara_system_collection_get_systems,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UNiagaraSystem>>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraValidationRule {
    __padding_end: [u8; 56],
}
impl UNiagaraValidationRule {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraValidationRule")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraVolumeRendererProperties {
    __padding_end: [u8; 3520],
}
impl UNiagaraVolumeRendererProperties {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraVolumeRendererProperties")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule {
    __padding_end: [u8; 104],
}
impl UNiagaraStatelessModule {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_AccelerationForce {
    __padding_end: [u8; 264],
}
impl UNiagaraStatelessModule_AccelerationForce {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_AccelerationForce")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_AddVelocity {
    __padding_end: [u8; 728],
}
impl UNiagaraStatelessModule_AddVelocity {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_AddVelocity")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_CalculateAccurateVelocity {
    __padding_end: [u8; 104],
}
impl UNiagaraStatelessModule_CalculateAccurateVelocity {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_CalculateAccurateVelocity")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_CameraOffset {
    __padding_end: [u8; 256],
}
impl UNiagaraStatelessModule_CameraOffset {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_CameraOffset")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_CurlNoiseForce {
    __padding_end: [u8; 112],
}
impl UNiagaraStatelessModule_CurlNoiseForce {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_CurlNoiseForce")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_DecalAttributes {
    __padding_end: [u8; 576],
}
impl UNiagaraStatelessModule_DecalAttributes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_DecalAttributes")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_Drag {
    __padding_end: [u8; 240],
}
impl UNiagaraStatelessModule_Drag {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_Drag")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_DynamicMaterialParameters {
    __padding_end: [u8; 2576],
}
impl UNiagaraStatelessModule_DynamicMaterialParameters {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_DynamicMaterialParameters")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_GravityForce {
    __padding_end: [u8; 256],
}
impl UNiagaraStatelessModule_GravityForce {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_GravityForce")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_InitializeParticle {
    __padding_end: [u8; 1256],
}
impl UNiagaraStatelessModule_InitializeParticle {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_InitializeParticle")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_InitialMeshOrientation {
    __padding_end: [u8; 584],
}
impl UNiagaraStatelessModule_InitialMeshOrientation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_InitialMeshOrientation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_LightAttributes {
    __padding_end: [u8; 872],
}
impl UNiagaraStatelessModule_LightAttributes {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_LightAttributes")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_MeshIndex {
    __padding_end: [u8; 208],
}
impl UNiagaraStatelessModule_MeshIndex {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_MeshIndex")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_MeshRotationRate {
    __padding_end: [u8; 424],
}
impl UNiagaraStatelessModule_MeshRotationRate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_MeshRotationRate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_RotateAroundPoint {
    __padding_end: [u8; 832],
}
impl UNiagaraStatelessModule_RotateAroundPoint {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_RotateAroundPoint")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleColor {
    __padding_end: [u8; 256],
}
impl UNiagaraStatelessModule_ScaleColor {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_ScaleColor")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleMeshSize {
    __padding_end: [u8; 608],
}
impl UNiagaraStatelessModule_ScaleMeshSize {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_ScaleMeshSize")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleMeshSizeBySpeed {
    __padding_end: [u8; 704],
}
impl UNiagaraStatelessModule_ScaleMeshSizeBySpeed {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_ScaleMeshSizeBySpeed")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleRibbonWidth {
    __padding_end: [u8; 608],
}
impl UNiagaraStatelessModule_ScaleRibbonWidth {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_ScaleRibbonWidth")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleSpriteSize {
    __padding_end: [u8; 608],
}
impl UNiagaraStatelessModule_ScaleSpriteSize {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_ScaleSpriteSize")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ScaleSpriteSizeBySpeed {
    __padding_end: [u8; 688],
}
impl UNiagaraStatelessModule_ScaleSpriteSizeBySpeed {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_ScaleSpriteSizeBySpeed")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_ShapeLocation {
    __padding_end: [u8; 2112],
}
impl UNiagaraStatelessModule_ShapeLocation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_ShapeLocation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SolveVelocitiesAndForces {
    __padding_end: [u8; 104],
}
impl UNiagaraStatelessModule_SolveVelocitiesAndForces {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_SolveVelocitiesAndForces")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SpriteFacingAndAlignment {
    __padding_end: [u8; 416],
}
impl UNiagaraStatelessModule_SpriteFacingAndAlignment {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_SpriteFacingAndAlignment")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SpriteRotationRate {
    __padding_end: [u8; 408],
}
impl UNiagaraStatelessModule_SpriteRotationRate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_SpriteRotationRate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessModule_SubUVAnimation {
    __padding_end: [u8; 224],
}
impl UNiagaraStatelessModule_SubUVAnimation {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessModule_SubUVAnimation")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessEmitter {
    __padding_end: [u8; 816],
}
impl UNiagaraStatelessEmitter {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessEmitter")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessEmitterTemplate {
    __padding_end: [u8; 136],
}
impl UNiagaraStatelessEmitterTemplate {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessEmitterTemplate")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UNiagaraStatelessEmitterDefault {
    __padding_end: [u8; 136],
}
impl UNiagaraStatelessEmitterDefault {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UNiagaraStatelessEmitterDefault")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UVolumeCache {
    __padding_end: [u8; 104],
}
impl UVolumeCache {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UVolumeCache")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct FSubscribeToNiagaraDataChannel_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToNiagaraDataChannel_WithContext_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToDataChannelUpdates_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FSubscribeToDataChannelUpdates_WithContext_UpdateDelegate {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FNiagaraComponent_OnSystemFinished {
    _opague: [u8; 24],
}
#[repr(C, align(8))]
pub struct FAsyncNiagaraCaptureSimCache_CaptureComplete {
    _opague: [u8; 24],
}
#[repr(transparent)]
pub struct ENiagaraSimCacheAttributeCaptureMode(pub u8);
impl ENiagaraSimCacheAttributeCaptureMode {
    pub const ALL: ENiagaraSimCacheAttributeCaptureMode = ENiagaraSimCacheAttributeCaptureMode(
        0,
    );
    pub const RENDERING_ONLY: ENiagaraSimCacheAttributeCaptureMode = ENiagaraSimCacheAttributeCaptureMode(
        1,
    );
    pub const EXPLICIT_ATTRIBUTES: ENiagaraSimCacheAttributeCaptureMode = ENiagaraSimCacheAttributeCaptureMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraDataSetType(pub u8);
impl ENiagaraDataSetType {
    pub const PARTICLE_DATA: ENiagaraDataSetType = ENiagaraDataSetType(0);
    pub const SHARED: ENiagaraDataSetType = ENiagaraDataSetType(1);
    pub const EVENT: ENiagaraDataSetType = ENiagaraDataSetType(2);
}
#[repr(transparent)]
pub struct ENiagaraSimTarget(pub u8);
impl ENiagaraSimTarget {
    pub const CPU_SIM: ENiagaraSimTarget = ENiagaraSimTarget(0);
    pub const GPU_COMPUTE_SIM: ENiagaraSimTarget = ENiagaraSimTarget(1);
}
#[repr(transparent)]
pub struct ENiagaraMeshPivotOffsetSpace(pub u8);
impl ENiagaraMeshPivotOffsetSpace {
    pub const MESH: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(0);
    pub const SIMULATION: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(1);
    pub const WORLD: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(2);
    pub const LOCAL: ENiagaraMeshPivotOffsetSpace = ENiagaraMeshPivotOffsetSpace(3);
}
#[repr(transparent)]
pub struct ENiagaraMeshLODMode(pub u8);
impl ENiagaraMeshLODMode {
    pub const LOD_LEVEL: ENiagaraMeshLODMode = ENiagaraMeshLODMode(0);
    pub const LOD_BIAS: ENiagaraMeshLODMode = ENiagaraMeshLODMode(1);
    pub const BY_COMPONENT_BOUNDS: ENiagaraMeshLODMode = ENiagaraMeshLODMode(2);
    pub const COMPONENT_ORIGIN: ENiagaraMeshLODMode = ENiagaraMeshLODMode(3);
    pub const PER_PARTICLE: ENiagaraMeshLODMode = ENiagaraMeshLODMode(4);
}
#[repr(transparent)]
pub struct ENiagaraSystemSpawnSectionStartBehavior(pub u8);
impl ENiagaraSystemSpawnSectionStartBehavior {
    pub const ACTIVATE: ENiagaraSystemSpawnSectionStartBehavior = ENiagaraSystemSpawnSectionStartBehavior(
        0,
    );
}
#[repr(transparent)]
pub struct ENiagaraSystemSpawnSectionEvaluateBehavior(pub u8);
impl ENiagaraSystemSpawnSectionEvaluateBehavior {
    pub const ACTIVATE_IF_INACTIVE: ENiagaraSystemSpawnSectionEvaluateBehavior = ENiagaraSystemSpawnSectionEvaluateBehavior(
        0,
    );
    pub const NONE: ENiagaraSystemSpawnSectionEvaluateBehavior = ENiagaraSystemSpawnSectionEvaluateBehavior(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraSystemSpawnSectionEndBehavior(pub u8);
impl ENiagaraSystemSpawnSectionEndBehavior {
    pub const SET_SYSTEM_INACTIVE: ENiagaraSystemSpawnSectionEndBehavior = ENiagaraSystemSpawnSectionEndBehavior(
        0,
    );
    pub const DEACTIVATE: ENiagaraSystemSpawnSectionEndBehavior = ENiagaraSystemSpawnSectionEndBehavior(
        1,
    );
    pub const NONE: ENiagaraSystemSpawnSectionEndBehavior = ENiagaraSystemSpawnSectionEndBehavior(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraAgeUpdateMode(pub u8);
impl ENiagaraAgeUpdateMode {
    pub const TICK_DELTA_TIME: ENiagaraAgeUpdateMode = ENiagaraAgeUpdateMode(0);
    pub const DESIRED_AGE: ENiagaraAgeUpdateMode = ENiagaraAgeUpdateMode(1);
    pub const DESIRED_AGE_NO_SEEK: ENiagaraAgeUpdateMode = ENiagaraAgeUpdateMode(2);
}
#[repr(transparent)]
pub struct ENiagaraAssetTagDefinitionImportance(pub u8);
impl ENiagaraAssetTagDefinitionImportance {
    pub const PRIMARY: ENiagaraAssetTagDefinitionImportance = ENiagaraAssetTagDefinitionImportance(
        0,
    );
    pub const SECONDARY: ENiagaraAssetTagDefinitionImportance = ENiagaraAssetTagDefinitionImportance(
        1,
    );
    pub const INTERNAL: ENiagaraAssetTagDefinitionImportance = ENiagaraAssetTagDefinitionImportance(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraBakerViewMode(pub i32);
impl ENiagaraBakerViewMode {
    pub const PERSPECTIVE: ENiagaraBakerViewMode = ENiagaraBakerViewMode(0);
    pub const ORTHO_FRONT: ENiagaraBakerViewMode = ENiagaraBakerViewMode(1);
    pub const ORTHO_BACK: ENiagaraBakerViewMode = ENiagaraBakerViewMode(2);
    pub const ORTHO_LEFT: ENiagaraBakerViewMode = ENiagaraBakerViewMode(3);
    pub const ORTHO_RIGHT: ENiagaraBakerViewMode = ENiagaraBakerViewMode(4);
    pub const ORTHO_TOP: ENiagaraBakerViewMode = ENiagaraBakerViewMode(5);
    pub const ORTHO_BOTTOM: ENiagaraBakerViewMode = ENiagaraBakerViewMode(6);
    pub const NUM: ENiagaraBakerViewMode = ENiagaraBakerViewMode(7);
}
#[repr(transparent)]
pub struct ENiagaraBindingSource(pub u8);
impl ENiagaraBindingSource {
    pub const IMPLICIT_FROM_SOURCE: ENiagaraBindingSource = ENiagaraBindingSource(0);
    pub const EXPLICIT_PARTICLES: ENiagaraBindingSource = ENiagaraBindingSource(1);
    pub const EXPLICIT_EMITTER: ENiagaraBindingSource = ENiagaraBindingSource(2);
    pub const EXPLICIT_SYSTEM: ENiagaraBindingSource = ENiagaraBindingSource(3);
    pub const EXPLICIT_USER: ENiagaraBindingSource = ENiagaraBindingSource(4);
    pub const MAX_BINDING_SOURCE: ENiagaraBindingSource = ENiagaraBindingSource(5);
}
#[repr(transparent)]
pub struct ENiagaraStructConversionType(pub u8);
impl ENiagaraStructConversionType {
    pub const COPY_ONLY: ENiagaraStructConversionType = ENiagaraStructConversionType(0);
    pub const DOUBLE_TO_FLOAT: ENiagaraStructConversionType = ENiagaraStructConversionType(
        1,
    );
    pub const VECTOR2: ENiagaraStructConversionType = ENiagaraStructConversionType(2);
    pub const VECTOR3: ENiagaraStructConversionType = ENiagaraStructConversionType(3);
    pub const VECTOR4: ENiagaraStructConversionType = ENiagaraStructConversionType(4);
    pub const QUAT: ENiagaraStructConversionType = ENiagaraStructConversionType(5);
}
#[repr(transparent)]
pub struct ENiagaraDataInterfaceEmitterBindingMode(pub i32);
impl ENiagaraDataInterfaceEmitterBindingMode {
    pub const SELF: ENiagaraDataInterfaceEmitterBindingMode = ENiagaraDataInterfaceEmitterBindingMode(
        0,
    );
    pub const OTHER: ENiagaraDataInterfaceEmitterBindingMode = ENiagaraDataInterfaceEmitterBindingMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraExecutionState(pub u32);
impl ENiagaraExecutionState {
    pub const ACTIVE: ENiagaraExecutionState = ENiagaraExecutionState(0);
    pub const INACTIVE: ENiagaraExecutionState = ENiagaraExecutionState(1);
    pub const INACTIVE_CLEAR: ENiagaraExecutionState = ENiagaraExecutionState(2);
    pub const COMPLETE: ENiagaraExecutionState = ENiagaraExecutionState(3);
    pub const DISABLED: ENiagaraExecutionState = ENiagaraExecutionState(4);
    pub const NUM: ENiagaraExecutionState = ENiagaraExecutionState(5);
}
#[repr(transparent)]
pub struct ENCPoolMethod(pub u8);
impl ENCPoolMethod {
    pub const NONE: ENCPoolMethod = ENCPoolMethod(0);
    pub const AUTO_RELEASE: ENCPoolMethod = ENCPoolMethod(1);
    pub const MANUAL_RELEASE: ENCPoolMethod = ENCPoolMethod(2);
    pub const MANUAL_RELEASE_ON_COMPLETE: ENCPoolMethod = ENCPoolMethod(3);
    pub const FREE_IN_POOL: ENCPoolMethod = ENCPoolMethod(4);
}
#[repr(transparent)]
pub struct ENiagaraGpuComputeTickStage(pub u8);
impl ENiagaraGpuComputeTickStage {
    pub const PRE_INIT_VIEWS: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(
        0,
    );
    pub const POST_INIT_VIEWS: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(
        1,
    );
    pub const POST_OPAQUE_RENDER: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(
        2,
    );
    pub const MAX: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(3);
    pub const FIRST: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(0);
    pub const LAST: ENiagaraGpuComputeTickStage = ENiagaraGpuComputeTickStage(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHudFont(pub i32);
impl ENiagaraDebugHudFont {
    pub const SMALL: ENiagaraDebugHudFont = ENiagaraDebugHudFont(0);
    pub const NORMAL: ENiagaraDebugHudFont = ENiagaraDebugHudFont(1);
}
#[repr(transparent)]
pub struct ENiagaraDebugHudHAlign(pub u8);
impl ENiagaraDebugHudHAlign {
    pub const LEFT: ENiagaraDebugHudHAlign = ENiagaraDebugHudHAlign(0);
    pub const CENTER: ENiagaraDebugHudHAlign = ENiagaraDebugHudHAlign(1);
    pub const RIGHT: ENiagaraDebugHudHAlign = ENiagaraDebugHudHAlign(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHudVAlign(pub u8);
impl ENiagaraDebugHudVAlign {
    pub const TOP: ENiagaraDebugHudVAlign = ENiagaraDebugHudVAlign(0);
    pub const CENTER: ENiagaraDebugHudVAlign = ENiagaraDebugHudVAlign(1);
    pub const BOTTOM: ENiagaraDebugHudVAlign = ENiagaraDebugHudVAlign(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDOverviewMode(pub i32);
impl ENiagaraDebugHUDOverviewMode {
    pub const OVERVIEW: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(0);
    pub const SCALABILITY: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        1,
    );
    pub const PERFORMANCE: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        2,
    );
    pub const PERFORMANCE_GRAPH: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        3,
    );
    pub const GPU_COMPUTE_PERFORMANCE: ENiagaraDebugHUDOverviewMode = ENiagaraDebugHUDOverviewMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDDOverviewSort(pub i32);
impl ENiagaraDebugHUDDOverviewSort {
    pub const NAME: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(0);
    pub const NUMBER_REGISTERED: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        1,
    );
    pub const NUMBER_ACTIVE: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        2,
    );
    pub const NUMBER_SCALABILITY: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        3,
    );
    pub const MEMORY_USAGE: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        4,
    );
    pub const RECENTLY_VISIBILTY: ENiagaraDebugHUDDOverviewSort = ENiagaraDebugHUDDOverviewSort(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraDebugHudVerbosity(pub i32);
impl ENiagaraDebugHudVerbosity {
    pub const NONE: ENiagaraDebugHudVerbosity = ENiagaraDebugHudVerbosity(0);
    pub const BASIC: ENiagaraDebugHudVerbosity = ENiagaraDebugHudVerbosity(1);
    pub const VERBOSE: ENiagaraDebugHudVerbosity = ENiagaraDebugHudVerbosity(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDPerfSampleMode(pub i32);
impl ENiagaraDebugHUDPerfSampleMode {
    pub const FRAME_TOTAL: ENiagaraDebugHUDPerfSampleMode = ENiagaraDebugHUDPerfSampleMode(
        0,
    );
    pub const PER_INSTANCE_AVERAGE: ENiagaraDebugHUDPerfSampleMode = ENiagaraDebugHUDPerfSampleMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDPerfUnits(pub i32);
impl ENiagaraDebugHUDPerfUnits {
    pub const MICROSECONDS: ENiagaraDebugHUDPerfUnits = ENiagaraDebugHUDPerfUnits(0);
    pub const MILLISECONDS: ENiagaraDebugHUDPerfUnits = ENiagaraDebugHUDPerfUnits(1);
}
#[repr(transparent)]
pub struct ENiagaraDebugHUDPerfGraphMode(pub i32);
impl ENiagaraDebugHUDPerfGraphMode {
    pub const GAME_THREAD: ENiagaraDebugHUDPerfGraphMode = ENiagaraDebugHUDPerfGraphMode(
        0,
    );
    pub const RENDER_THREAD: ENiagaraDebugHUDPerfGraphMode = ENiagaraDebugHUDPerfGraphMode(
        1,
    );
    pub const GPU: ENiagaraDebugHUDPerfGraphMode = ENiagaraDebugHUDPerfGraphMode(2);
}
#[repr(transparent)]
pub struct ENiagaraDebugPlaybackMode(pub u8);
impl ENiagaraDebugPlaybackMode {
    pub const PLAY: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(0);
    pub const LOOP: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(1);
    pub const PAUSED: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(2);
    pub const STEP: ENiagaraDebugPlaybackMode = ENiagaraDebugPlaybackMode(3);
}
#[repr(transparent)]
pub struct ENiagaraCVarConditionResponse(pub u8);
impl ENiagaraCVarConditionResponse {
    pub const NONE: ENiagaraCVarConditionResponse = ENiagaraCVarConditionResponse(0);
    pub const ENABLE: ENiagaraCVarConditionResponse = ENiagaraCVarConditionResponse(1);
    pub const DISABLE: ENiagaraCVarConditionResponse = ENiagaraCVarConditionResponse(2);
}
#[repr(transparent)]
pub struct ENiagaraCullProxyMode(pub u32);
impl ENiagaraCullProxyMode {
    pub const NONE: ENiagaraCullProxyMode = ENiagaraCullProxyMode(0);
    pub const INSTANCED_RENDERED: ENiagaraCullProxyMode = ENiagaraCullProxyMode(1);
}
#[repr(transparent)]
pub struct EScriptExecutionMode(pub u8);
impl EScriptExecutionMode {
    pub const EVERY_PARTICLE: EScriptExecutionMode = EScriptExecutionMode(0);
    pub const SPAWNED_PARTICLES: EScriptExecutionMode = EScriptExecutionMode(1);
    pub const SINGLE_PARTICLE: EScriptExecutionMode = EScriptExecutionMode(2);
}
#[repr(transparent)]
pub struct ENiagaraPythonUpdateScriptReference(pub u8);
impl ENiagaraPythonUpdateScriptReference {
    pub const NONE: ENiagaraPythonUpdateScriptReference = ENiagaraPythonUpdateScriptReference(
        0,
    );
    pub const SCRIPT_ASSET: ENiagaraPythonUpdateScriptReference = ENiagaraPythonUpdateScriptReference(
        1,
    );
    pub const DIRECT_TEXT_ENTRY: ENiagaraPythonUpdateScriptReference = ENiagaraPythonUpdateScriptReference(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraInterpolatedSpawnMode(pub u8);
impl ENiagaraInterpolatedSpawnMode {
    pub const NO_INTERPOLATION: ENiagaraInterpolatedSpawnMode = ENiagaraInterpolatedSpawnMode(
        0,
    );
    pub const RUN_UPDATE_SCRIPT: ENiagaraInterpolatedSpawnMode = ENiagaraInterpolatedSpawnMode(
        1,
    );
    pub const INTERPOLATION: ENiagaraInterpolatedSpawnMode = ENiagaraInterpolatedSpawnMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraEmitterCalculateBoundMode(pub u8);
impl ENiagaraEmitterCalculateBoundMode {
    pub const DYNAMIC: ENiagaraEmitterCalculateBoundMode = ENiagaraEmitterCalculateBoundMode(
        0,
    );
    pub const FIXED: ENiagaraEmitterCalculateBoundMode = ENiagaraEmitterCalculateBoundMode(
        1,
    );
    pub const PROGRAMMABLE: ENiagaraEmitterCalculateBoundMode = ENiagaraEmitterCalculateBoundMode(
        2,
    );
}
#[repr(transparent)]
pub struct EParticleAllocationMode(pub u8);
impl EParticleAllocationMode {
    pub const AUTOMATIC_ESTIMATE: EParticleAllocationMode = EParticleAllocationMode(0);
    pub const MANUAL_ESTIMATE: EParticleAllocationMode = EParticleAllocationMode(1);
    pub const FIXED_COUNT: EParticleAllocationMode = EParticleAllocationMode(2);
}
#[repr(transparent)]
pub struct ENiagaraEmitterDefaultSummaryState(pub u8);
impl ENiagaraEmitterDefaultSummaryState {
    pub const DEFAULT: ENiagaraEmitterDefaultSummaryState = ENiagaraEmitterDefaultSummaryState(
        0,
    );
    pub const SUMMARY: ENiagaraEmitterDefaultSummaryState = ENiagaraEmitterDefaultSummaryState(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraEmitterMode(pub u8);
impl ENiagaraEmitterMode {
    pub const STANDARD: ENiagaraEmitterMode = ENiagaraEmitterMode(0);
    pub const STATELESS: ENiagaraEmitterMode = ENiagaraEmitterMode(1);
}
#[repr(transparent)]
pub struct ENiagaraDeviceProfileRedirectMode(pub u8);
impl ENiagaraDeviceProfileRedirectMode {
    pub const C_VAR: ENiagaraDeviceProfileRedirectMode = ENiagaraDeviceProfileRedirectMode(
        0,
    );
    pub const DEVICE_PROFILE: ENiagaraDeviceProfileRedirectMode = ENiagaraDeviceProfileRedirectMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraRibbonUVDistributionMode(pub u8);
impl ENiagaraRibbonUVDistributionMode {
    pub const SCALED_UNIFORMLY: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        0,
    );
    pub const SCALED_USING_RIBBON_SEGMENT_LENGTH: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        1,
    );
    pub const TILED_OVER_RIBBON_LENGTH: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        2,
    );
    pub const TILED_FROM_START_OVER_RIBBON_LENGTH: ENiagaraRibbonUVDistributionMode = ENiagaraRibbonUVDistributionMode(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagaraRibbonUVEdgeMode(pub u8);
impl ENiagaraRibbonUVEdgeMode {
    pub const SMOOTH_TRANSITION: ENiagaraRibbonUVEdgeMode = ENiagaraRibbonUVEdgeMode(0);
    pub const LOCKED: ENiagaraRibbonUVEdgeMode = ENiagaraRibbonUVEdgeMode(1);
}
#[repr(transparent)]
pub struct ENiagaraModuleDependencyType(pub u8);
impl ENiagaraModuleDependencyType {
    pub const PRE_DEPENDENCY: ENiagaraModuleDependencyType = ENiagaraModuleDependencyType(
        0,
    );
    pub const POST_DEPENDENCY: ENiagaraModuleDependencyType = ENiagaraModuleDependencyType(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraModuleDependencyScriptConstraint(pub u8);
impl ENiagaraModuleDependencyScriptConstraint {
    pub const SAME_SCRIPT: ENiagaraModuleDependencyScriptConstraint = ENiagaraModuleDependencyScriptConstraint(
        0,
    );
    pub const ALL_SCRIPTS: ENiagaraModuleDependencyScriptConstraint = ENiagaraModuleDependencyScriptConstraint(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraScriptUsage(pub u8);
impl ENiagaraScriptUsage {
    pub const FUNCTION: ENiagaraScriptUsage = ENiagaraScriptUsage(0);
    pub const MODULE: ENiagaraScriptUsage = ENiagaraScriptUsage(1);
    pub const DYNAMIC_INPUT: ENiagaraScriptUsage = ENiagaraScriptUsage(2);
    pub const PARTICLE_SPAWN_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(3);
    pub const PARTICLE_SPAWN_SCRIPT_INTERPOLATED: ENiagaraScriptUsage = ENiagaraScriptUsage(
        4,
    );
    pub const PARTICLE_UPDATE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(5);
    pub const PARTICLE_EVENT_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(6);
    pub const PARTICLE_SIMULATION_STAGE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(
        7,
    );
    pub const PARTICLE_GPU_COMPUTE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(8);
    pub const EMITTER_SPAWN_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(9);
    pub const EMITTER_UPDATE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(10);
    pub const SYSTEM_SPAWN_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(11);
    pub const SYSTEM_UPDATE_SCRIPT: ENiagaraScriptUsage = ENiagaraScriptUsage(12);
}
#[repr(transparent)]
pub struct ENiagaraScriptCompileStatus(pub u8);
impl ENiagaraScriptCompileStatus {
    pub const NCS_UNKNOWN: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(0);
    pub const NCS_DIRTY: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(1);
    pub const NCS_ERROR: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(2);
    pub const NCS_UP_TO_DATE: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        3,
    );
    pub const NCS_BEING_CREATED: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        4,
    );
    pub const NCS_UP_TO_DATE_WITH_WARNINGS: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        5,
    );
    pub const NCS_COMPUTE_UP_TO_DATE_WITH_WARNINGS: ENiagaraScriptCompileStatus = ENiagaraScriptCompileStatus(
        6,
    );
}
#[repr(transparent)]
pub struct ENiagaraInlineDynamicInputFormatTokenUsage(pub i32);
impl ENiagaraInlineDynamicInputFormatTokenUsage {
    pub const INPUT: ENiagaraInlineDynamicInputFormatTokenUsage = ENiagaraInlineDynamicInputFormatTokenUsage(
        0,
    );
    pub const DECORATOR: ENiagaraInlineDynamicInputFormatTokenUsage = ENiagaraInlineDynamicInputFormatTokenUsage(
        1,
    );
    pub const LINE_BREAK: ENiagaraInlineDynamicInputFormatTokenUsage = ENiagaraInlineDynamicInputFormatTokenUsage(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraScriptLibraryVisibility(pub u8);
impl ENiagaraScriptLibraryVisibility {
    pub const INVALID: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        0,
    );
    pub const UNEXPOSED: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        1,
    );
    pub const LIBRARY: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        2,
    );
    pub const HIDDEN: ENiagaraScriptLibraryVisibility = ENiagaraScriptLibraryVisibility(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagaraNumericOutputTypeSelectionMode(pub u8);
impl ENiagaraNumericOutputTypeSelectionMode {
    pub const NONE: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        0,
    );
    pub const LARGEST: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        1,
    );
    pub const SMALLEST: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        2,
    );
    pub const SCALAR: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        3,
    );
    pub const CUSTOM: ENiagaraNumericOutputTypeSelectionMode = ENiagaraNumericOutputTypeSelectionMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraSystemInactiveResponse(pub u8);
impl ENiagaraSystemInactiveResponse {
    pub const COMPLETE: ENiagaraSystemInactiveResponse = ENiagaraSystemInactiveResponse(
        0,
    );
    pub const KILL: ENiagaraSystemInactiveResponse = ENiagaraSystemInactiveResponse(1);
}
#[repr(transparent)]
pub struct ENiagaraLoopBehavior(pub u8);
impl ENiagaraLoopBehavior {
    pub const INFINITE: ENiagaraLoopBehavior = ENiagaraLoopBehavior(0);
    pub const MULTIPLE: ENiagaraLoopBehavior = ENiagaraLoopBehavior(1);
    pub const ONCE: ENiagaraLoopBehavior = ENiagaraLoopBehavior(2);
}
#[repr(transparent)]
pub struct ENiagaraDistributionMode(pub u8);
impl ENiagaraDistributionMode {
    pub const ARRAY: ENiagaraDistributionMode = ENiagaraDistributionMode(0);
    pub const BINDING: ENiagaraDistributionMode = ENiagaraDistributionMode(1);
    pub const EXPRESSION: ENiagaraDistributionMode = ENiagaraDistributionMode(2);
    pub const UNIFORM_CONSTANT: ENiagaraDistributionMode = ENiagaraDistributionMode(3);
    pub const NON_UNIFORM_CONSTANT: ENiagaraDistributionMode = ENiagaraDistributionMode(
        4,
    );
    pub const UNIFORM_RANGE: ENiagaraDistributionMode = ENiagaraDistributionMode(5);
    pub const NON_UNIFORM_RANGE: ENiagaraDistributionMode = ENiagaraDistributionMode(6);
    pub const UNIFORM_CURVE: ENiagaraDistributionMode = ENiagaraDistributionMode(7);
    pub const NON_UNIFORM_CURVE: ENiagaraDistributionMode = ENiagaraDistributionMode(8);
    pub const COLOR_GRADIENT: ENiagaraDistributionMode = ENiagaraDistributionMode(9);
}
#[repr(transparent)]
pub struct ENiagaraEmitterInactiveResponse(pub u8);
impl ENiagaraEmitterInactiveResponse {
    pub const COMPLETE: ENiagaraEmitterInactiveResponse = ENiagaraEmitterInactiveResponse(
        0,
    );
    pub const KILL: ENiagaraEmitterInactiveResponse = ENiagaraEmitterInactiveResponse(1);
}
#[repr(transparent)]
pub struct ENiagaraLoopDurationMode(pub u8);
impl ENiagaraLoopDurationMode {
    pub const FIXED: ENiagaraLoopDurationMode = ENiagaraLoopDurationMode(0);
    pub const INFINITE: ENiagaraLoopDurationMode = ENiagaraLoopDurationMode(1);
}
#[repr(transparent)]
pub struct ENiagaraExecutionStateManagement(pub u32);
impl ENiagaraExecutionStateManagement {
    pub const AWAKEN: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        0,
    );
    pub const SLEEP_AND_LET_PARTICLES_FINISH: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        1,
    );
    pub const SLEEP_AND_CLEAR_PARTICLES: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        2,
    );
    pub const KILL_IMMEDIATELY: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        3,
    );
    pub const KILL_AFTER_PARTICLES_FINISH: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        4,
    );
    pub const NUM: ENiagaraExecutionStateManagement = ENiagaraExecutionStateManagement(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraInputWidgetType(pub u8);
impl ENiagaraInputWidgetType {
    pub const DEFAULT: ENiagaraInputWidgetType = ENiagaraInputWidgetType(0);
    pub const SLIDER: ENiagaraInputWidgetType = ENiagaraInputWidgetType(1);
    pub const VOLUME: ENiagaraInputWidgetType = ENiagaraInputWidgetType(2);
    pub const NUMERIC_DROPDOWN: ENiagaraInputWidgetType = ENiagaraInputWidgetType(3);
    pub const ENUM_STYLE: ENiagaraInputWidgetType = ENiagaraInputWidgetType(4);
    pub const SEGMENTED_BUTTONS: ENiagaraInputWidgetType = ENiagaraInputWidgetType(5);
}
#[repr(transparent)]
pub struct ENiagaraBoolDisplayMode(pub u8);
impl ENiagaraBoolDisplayMode {
    pub const DISPLAY_ALWAYS: ENiagaraBoolDisplayMode = ENiagaraBoolDisplayMode(0);
    pub const DISPLAY_IF_TRUE: ENiagaraBoolDisplayMode = ENiagaraBoolDisplayMode(1);
    pub const DISPLAY_IF_FALSE: ENiagaraBoolDisplayMode = ENiagaraBoolDisplayMode(2);
}
#[repr(transparent)]
pub struct ENiagaraVariantMode(pub i32);
impl ENiagaraVariantMode {
    pub const NONE: ENiagaraVariantMode = ENiagaraVariantMode(0);
    pub const OBJECT: ENiagaraVariantMode = ENiagaraVariantMode(1);
    pub const DATA_INTERFACE: ENiagaraVariantMode = ENiagaraVariantMode(2);
    pub const BYTES: ENiagaraVariantMode = ENiagaraVariantMode(3);
}
#[repr(transparent)]
pub struct ENiagaraStatelessSpawnInfoType(pub i32);
impl ENiagaraStatelessSpawnInfoType {
    pub const BURST: ENiagaraStatelessSpawnInfoType = ENiagaraStatelessSpawnInfoType(0);
    pub const RATE: ENiagaraStatelessSpawnInfoType = ENiagaraStatelessSpawnInfoType(1);
}
#[repr(transparent)]
pub struct ENiagaraDefaultMode(pub u8);
impl ENiagaraDefaultMode {
    pub const VALUE: ENiagaraDefaultMode = ENiagaraDefaultMode(0);
    pub const BINDING: ENiagaraDefaultMode = ENiagaraDefaultMode(1);
    pub const CUSTOM: ENiagaraDefaultMode = ENiagaraDefaultMode(2);
    pub const FAIL_IF_PREVIOUSLY_NOT_SET: ENiagaraDefaultMode = ENiagaraDefaultMode(3);
}
#[repr(transparent)]
pub struct ENiagaraOcclusionQueryMode(pub u8);
impl ENiagaraOcclusionQueryMode {
    pub const DEFAULT: ENiagaraOcclusionQueryMode = ENiagaraOcclusionQueryMode(0);
    pub const ALWAYS_ENABLED: ENiagaraOcclusionQueryMode = ENiagaraOcclusionQueryMode(1);
    pub const ALWAYS_DISABLED: ENiagaraOcclusionQueryMode = ENiagaraOcclusionQueryMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraTickBehavior(pub u8);
impl ENiagaraTickBehavior {
    pub const USE_PREREQS: ENiagaraTickBehavior = ENiagaraTickBehavior(0);
    pub const USE_COMPONENT_TICK_GROUP: ENiagaraTickBehavior = ENiagaraTickBehavior(1);
    pub const FORCE_TICK_FIRST: ENiagaraTickBehavior = ENiagaraTickBehavior(2);
    pub const FORCE_TICK_LAST: ENiagaraTickBehavior = ENiagaraTickBehavior(3);
}
#[repr(transparent)]
pub struct ENiagartaDataChannelReadResult(pub u8);
impl ENiagartaDataChannelReadResult {
    pub const SUCCESS: ENiagartaDataChannelReadResult = ENiagartaDataChannelReadResult(
        0,
    );
    pub const FAILURE: ENiagartaDataChannelReadResult = ENiagartaDataChannelReadResult(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraScriptTemplateSpecification(pub u8);
impl ENiagaraScriptTemplateSpecification {
    pub const NONE: ENiagaraScriptTemplateSpecification = ENiagaraScriptTemplateSpecification(
        0,
    );
    pub const TEMPLATE: ENiagaraScriptTemplateSpecification = ENiagaraScriptTemplateSpecification(
        1,
    );
    pub const BEHAVIOR: ENiagaraScriptTemplateSpecification = ENiagaraScriptTemplateSpecification(
        2,
    );
}
#[repr(transparent)]
pub struct ENDIActorComponentSourceMode(pub u8);
impl ENDIActorComponentSourceMode {
    pub const DEFAULT: ENDIActorComponentSourceMode = ENDIActorComponentSourceMode(0);
    pub const ATTACH_PARENT: ENDIActorComponentSourceMode = ENDIActorComponentSourceMode(
        1,
    );
    pub const LOCAL_PLAYER: ENDIActorComponentSourceMode = ENDIActorComponentSourceMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraGpuSyncMode(pub u8);
impl ENiagaraGpuSyncMode {
    pub const NONE: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(0);
    pub const SYNC_CPU_TO_GPU: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(1);
    pub const SYNC_GPU_TO_CPU: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(2);
    pub const SYNC_BOTH: ENiagaraGpuSyncMode = ENiagaraGpuSyncMode(3);
}
#[repr(transparent)]
pub struct ENDICollisionQuery_AsyncGpuTraceProvider(pub u8);
impl ENDICollisionQuery_AsyncGpuTraceProvider {
    pub const DEFAULT: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        0,
    );
    pub const HWRT: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        1,
    );
    pub const GSDF: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        2,
    );
    pub const NONE: ENDICollisionQuery_AsyncGpuTraceProvider = ENDICollisionQuery_AsyncGpuTraceProvider(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagaraDataChannelAllocationMode(pub u8);
impl ENiagaraDataChannelAllocationMode {
    pub const STATIC: ENiagaraDataChannelAllocationMode = ENiagaraDataChannelAllocationMode(
        0,
    );
    pub const DYNAMIC: ENiagaraDataChannelAllocationMode = ENiagaraDataChannelAllocationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENDISceneCapture2DSourceMode(pub u8);
impl ENDISceneCapture2DSourceMode {
    pub const USER_PARAMETER_THEN_ATTACH_PARENT: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(
        0,
    );
    pub const USER_PARAMETER_ONLY: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(
        1,
    );
    pub const ATTACH_PARENT_ONLY: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(
        2,
    );
    pub const MANAGED: ENDISceneCapture2DSourceMode = ENDISceneCapture2DSourceMode(3);
}
#[repr(transparent)]
pub struct ENDISceneCapture2DOffsetMode(pub u8);
impl ENDISceneCapture2DOffsetMode {
    pub const DISABLED: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(0);
    pub const RELATIVE_LOCAL: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(
        1,
    );
    pub const RELATIVE_WORLD: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(
        2,
    );
    pub const ABSOLUTE_WORLD: ENDISceneCapture2DOffsetMode = ENDISceneCapture2DOffsetMode(
        3,
    );
}
#[repr(transparent)]
pub struct ENDISocketReaderSourceMode(pub u8);
impl ENDISocketReaderSourceMode {
    pub const DEFAULT: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(0);
    pub const PARAMETER_BINDING_ONLY: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(
        1,
    );
    pub const ATTACHED_PARENT_ONLY: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(
        2,
    );
    pub const SOURCE_ONLY: ENDISocketReaderSourceMode = ENDISocketReaderSourceMode(3);
}
#[repr(transparent)]
pub struct ENDIStaticMesh_SourceMode(pub u8);
impl ENDIStaticMesh_SourceMode {
    pub const DEFAULT: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(0);
    pub const SOURCE: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(1);
    pub const ATTACH_PARENT: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(2);
    pub const DEFAULT_MESH_ONLY: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(
        3,
    );
    pub const MESH_PARAMETER_BINDING: ENDIStaticMesh_SourceMode = ENDIStaticMesh_SourceMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENDIObjectPropertyReaderSourceMode(pub u8);
impl ENDIObjectPropertyReaderSourceMode {
    pub const BINDING: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        0,
    );
    pub const ATTACH_PARENT_ACTOR: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        1,
    );
    pub const BINDING_THEN_ATTACH_PARENT_ACTOR: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        2,
    );
    pub const ATTACH_PARENT: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        3,
    );
    pub const BINDING_THEN_ATTACH_PARENT: ENDIObjectPropertyReaderSourceMode = ENDIObjectPropertyReaderSourceMode(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraRendererMotionVectorSetting(pub u8);
impl ENiagaraRendererMotionVectorSetting {
    pub const AUTO_DETECT: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        0,
    );
    pub const PRECISE: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        1,
    );
    pub const APPROXIMATE: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        2,
    );
    pub const DISABLE: ENiagaraRendererMotionVectorSetting = ENiagaraRendererMotionVectorSetting(
        3,
    );
}
#[repr(transparent)]
pub struct ENiagraDataChannel_IslandMode(pub u8);
impl ENiagraDataChannel_IslandMode {
    pub const ALIGNED_STATIC: ENiagraDataChannel_IslandMode = ENiagraDataChannel_IslandMode(
        0,
    );
    pub const DYNAMIC: ENiagraDataChannel_IslandMode = ENiagraDataChannel_IslandMode(1);
}
#[repr(transparent)]
pub struct ENDIExport_GPUAllocationMode(pub u8);
impl ENDIExport_GPUAllocationMode {
    pub const FIXED_SIZE: ENDIExport_GPUAllocationMode = ENDIExport_GPUAllocationMode(0);
    pub const PER_PARTICLE: ENDIExport_GPUAllocationMode = ENDIExport_GPUAllocationMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraGpuBufferFormat(pub u8);
impl ENiagaraGpuBufferFormat {
    pub const FLOAT: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(0);
    pub const HALF_FLOAT: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(1);
    pub const UNSIGNED_NORMALIZED_BYTE: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(
        2,
    );
    pub const MAX: ENiagaraGpuBufferFormat = ENiagaraGpuBufferFormat(3);
}
#[repr(transparent)]
pub struct ESetResolutionMethod(pub i32);
impl ESetResolutionMethod {
    pub const INDEPENDENT: ESetResolutionMethod = ESetResolutionMethod(0);
    pub const MAX_AXIS: ESetResolutionMethod = ESetResolutionMethod(1);
    pub const CELL_SIZE: ESetResolutionMethod = ESetResolutionMethod(2);
}
#[repr(transparent)]
pub struct ENDILandscape_SourceMode(pub u8);
impl ENDILandscape_SourceMode {
    pub const DEFAULT: ENDILandscape_SourceMode = ENDILandscape_SourceMode(0);
    pub const SOURCE: ENDILandscape_SourceMode = ENDILandscape_SourceMode(1);
    pub const ATTACH_PARENT: ENDILandscape_SourceMode = ENDILandscape_SourceMode(2);
}
#[repr(transparent)]
pub struct ENiagaraMipMapGeneration(pub u8);
impl ENiagaraMipMapGeneration {
    pub const DISABLED: ENiagaraMipMapGeneration = ENiagaraMipMapGeneration(0);
    pub const POST_STAGE: ENiagaraMipMapGeneration = ENiagaraMipMapGeneration(1);
    pub const POST_SIMULATE: ENiagaraMipMapGeneration = ENiagaraMipMapGeneration(2);
}
#[repr(transparent)]
pub struct ENDISkeletalMesh_SourceMode(pub u8);
impl ENDISkeletalMesh_SourceMode {
    pub const DEFAULT: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(0);
    pub const SOURCE: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(1);
    pub const ATTACH_PARENT: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(
        2,
    );
    pub const DEFAULT_MESH_ONLY: ENDISkeletalMesh_SourceMode = ENDISkeletalMesh_SourceMode(
        3,
    );
}
#[repr(transparent)]
pub struct ENDISkeletalMesh_SkinningMode(pub u8);
impl ENDISkeletalMesh_SkinningMode {
    pub const INVALID: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(
        255,
    );
    pub const NONE: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(0);
    pub const SKIN_ON_THE_FLY: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(
        1,
    );
    pub const PRE_SKIN: ENDISkeletalMesh_SkinningMode = ENDISkeletalMesh_SkinningMode(2);
}
#[repr(transparent)]
pub struct ENiagaraRendererSourceDataMode(pub u8);
impl ENiagaraRendererSourceDataMode {
    pub const PARTICLES: ENiagaraRendererSourceDataMode = ENiagaraRendererSourceDataMode(
        0,
    );
    pub const EMITTER: ENiagaraRendererSourceDataMode = ENiagaraRendererSourceDataMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraEditorPreviewActorPlaybackType(pub i32);
impl ENiagaraEditorPreviewActorPlaybackType {
    pub const ONCE: ENiagaraEditorPreviewActorPlaybackType = ENiagaraEditorPreviewActorPlaybackType(
        0,
    );
    pub const LOOPING: ENiagaraEditorPreviewActorPlaybackType = ENiagaraEditorPreviewActorPlaybackType(
        1,
    );
    pub const PING_PONG: ENiagaraEditorPreviewActorPlaybackType = ENiagaraEditorPreviewActorPlaybackType(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraEditorPreviewActorShapeType(pub i32);
impl ENiagaraEditorPreviewActorShapeType {
    pub const CIRCLE: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        0,
    );
    pub const SQUARE: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        1,
    );
    pub const TRIANGLE: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        2,
    );
    pub const CUSTOM: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        3,
    );
    pub const BLUEPRINT: ENiagaraEditorPreviewActorShapeType = ENiagaraEditorPreviewActorShapeType(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraEditorPreviewActorRotationMode(pub i32);
impl ENiagaraEditorPreviewActorRotationMode {
    pub const NONE: ENiagaraEditorPreviewActorRotationMode = ENiagaraEditorPreviewActorRotationMode(
        0,
    );
    pub const DIRECTION_OF_TRAVEL: ENiagaraEditorPreviewActorRotationMode = ENiagaraEditorPreviewActorRotationMode(
        1,
    );
    pub const BLUEPRINT: ENiagaraEditorPreviewActorRotationMode = ENiagaraEditorPreviewActorRotationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraScalabilityUpdateFrequency(pub i32);
impl ENiagaraScalabilityUpdateFrequency {
    pub const SPAWN_ONLY: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        0,
    );
    pub const LOW: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        1,
    );
    pub const MEDIUM: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        2,
    );
    pub const HIGH: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        3,
    );
    pub const CONTINUOUS: ENiagaraScalabilityUpdateFrequency = ENiagaraScalabilityUpdateFrequency(
        4,
    );
}
#[repr(transparent)]
pub struct ENiagaraCullReaction(pub i32);
impl ENiagaraCullReaction {
    pub const DEACTIVATE: ENiagaraCullReaction = ENiagaraCullReaction(0);
    pub const DEACTIVATE_IMMEDIATE: ENiagaraCullReaction = ENiagaraCullReaction(1);
    pub const DEACTIVATE_RESUME: ENiagaraCullReaction = ENiagaraCullReaction(2);
    pub const DEACTIVATE_IMMEDIATE_RESUME: ENiagaraCullReaction = ENiagaraCullReaction(
        3,
    );
    pub const PAUSE_RESUME: ENiagaraCullReaction = ENiagaraCullReaction(4);
}
#[repr(transparent)]
pub struct ENiagaraSortMode(pub u8);
impl ENiagaraSortMode {
    pub const NONE: ENiagaraSortMode = ENiagaraSortMode(0);
    pub const VIEW_DEPTH: ENiagaraSortMode = ENiagaraSortMode(1);
    pub const VIEW_DISTANCE: ENiagaraSortMode = ENiagaraSortMode(2);
    pub const CUSTOM_ASCENDING: ENiagaraSortMode = ENiagaraSortMode(3);
    pub const CUSTOM_DECENDING: ENiagaraSortMode = ENiagaraSortMode(4);
}
#[repr(transparent)]
pub struct ENiagaraRendererSortPrecision(pub u8);
impl ENiagaraRendererSortPrecision {
    pub const DEFAULT: ENiagaraRendererSortPrecision = ENiagaraRendererSortPrecision(0);
    pub const LOW: ENiagaraRendererSortPrecision = ENiagaraRendererSortPrecision(1);
    pub const HIGH: ENiagaraRendererSortPrecision = ENiagaraRendererSortPrecision(2);
}
#[repr(transparent)]
pub struct ENiagaraRendererGpuTranslucentLatency(pub u8);
impl ENiagaraRendererGpuTranslucentLatency {
    pub const PROJECT_DEFAULT: ENiagaraRendererGpuTranslucentLatency = ENiagaraRendererGpuTranslucentLatency(
        0,
    );
    pub const IMMEDIATE: ENiagaraRendererGpuTranslucentLatency = ENiagaraRendererGpuTranslucentLatency(
        1,
    );
    pub const LATENT: ENiagaraRendererGpuTranslucentLatency = ENiagaraRendererGpuTranslucentLatency(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraMeshFacingMode(pub u8);
impl ENiagaraMeshFacingMode {
    pub const DEFAULT: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(0);
    pub const VELOCITY: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(1);
    pub const CAMERA_POSITION: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(2);
    pub const CAMERA_PLANE: ENiagaraMeshFacingMode = ENiagaraMeshFacingMode(3);
}
#[repr(transparent)]
pub struct ENiagaraMeshLockedAxisSpace(pub u8);
impl ENiagaraMeshLockedAxisSpace {
    pub const SIMULATION: ENiagaraMeshLockedAxisSpace = ENiagaraMeshLockedAxisSpace(0);
    pub const WORLD: ENiagaraMeshLockedAxisSpace = ENiagaraMeshLockedAxisSpace(1);
    pub const LOCAL: ENiagaraMeshLockedAxisSpace = ENiagaraMeshLockedAxisSpace(2);
}
#[repr(transparent)]
pub struct ENiagaraPreviewGridResetMode(pub u8);
impl ENiagaraPreviewGridResetMode {
    pub const NEVER: ENiagaraPreviewGridResetMode = ENiagaraPreviewGridResetMode(0);
    pub const INDIVIDUAL: ENiagaraPreviewGridResetMode = ENiagaraPreviewGridResetMode(1);
    pub const ALL: ENiagaraPreviewGridResetMode = ENiagaraPreviewGridResetMode(2);
}
#[repr(transparent)]
pub struct ENiagaraRibbonFacingMode(pub u8);
impl ENiagaraRibbonFacingMode {
    pub const SCREEN: ENiagaraRibbonFacingMode = ENiagaraRibbonFacingMode(0);
    pub const CUSTOM: ENiagaraRibbonFacingMode = ENiagaraRibbonFacingMode(1);
    pub const CUSTOM_SIDE_VECTOR: ENiagaraRibbonFacingMode = ENiagaraRibbonFacingMode(2);
}
#[repr(transparent)]
pub struct ENiagaraRibbonAgeOffsetMode(pub u8);
impl ENiagaraRibbonAgeOffsetMode {
    pub const SCALE: ENiagaraRibbonAgeOffsetMode = ENiagaraRibbonAgeOffsetMode(0);
    pub const CLIP: ENiagaraRibbonAgeOffsetMode = ENiagaraRibbonAgeOffsetMode(1);
}
#[repr(transparent)]
pub struct ENiagaraRibbonDrawDirection(pub u8);
impl ENiagaraRibbonDrawDirection {
    pub const FRONT_TO_BACK: ENiagaraRibbonDrawDirection = ENiagaraRibbonDrawDirection(
        0,
    );
    pub const BACK_TO_FRONT: ENiagaraRibbonDrawDirection = ENiagaraRibbonDrawDirection(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraRibbonShapeMode(pub u8);
impl ENiagaraRibbonShapeMode {
    pub const PLANE: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(0);
    pub const MULTI_PLANE: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(1);
    pub const TUBE: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(2);
    pub const CUSTOM: ENiagaraRibbonShapeMode = ENiagaraRibbonShapeMode(3);
}
#[repr(transparent)]
pub struct ENiagaraRibbonTessellationMode(pub u8);
impl ENiagaraRibbonTessellationMode {
    pub const AUTOMATIC: ENiagaraRibbonTessellationMode = ENiagaraRibbonTessellationMode(
        0,
    );
    pub const CUSTOM: ENiagaraRibbonTessellationMode = ENiagaraRibbonTessellationMode(1);
    pub const DISABLED: ENiagaraRibbonTessellationMode = ENiagaraRibbonTessellationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraLwcTileUpdateMode(pub u8);
impl ENiagaraLwcTileUpdateMode {
    pub const RESET_SIMULATION: ENiagaraLwcTileUpdateMode = ENiagaraLwcTileUpdateMode(0);
    pub const REBASE: ENiagaraLwcTileUpdateMode = ENiagaraLwcTileUpdateMode(1);
    pub const REBASE_OR_RESET_SIMULATION: ENiagaraLwcTileUpdateMode = ENiagaraLwcTileUpdateMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraCompileErrorSeverity(pub u8);
impl ENiagaraCompileErrorSeverity {
    pub const IGNORE: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(0);
    pub const LOG_ONLY: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(1);
    pub const WARNING: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(2);
    pub const ERROR: ENiagaraCompileErrorSeverity = ENiagaraCompileErrorSeverity(3);
}
#[repr(transparent)]
pub struct ENiagaraStripScriptByteCodeOption(pub u8);
impl ENiagaraStripScriptByteCodeOption {
    pub const DEFAULT: ENiagaraStripScriptByteCodeOption = ENiagaraStripScriptByteCodeOption(
        0,
    );
    pub const STRIP_ORIGINAL: ENiagaraStripScriptByteCodeOption = ENiagaraStripScriptByteCodeOption(
        1,
    );
    pub const STRIP_EXPERIMENTAL: ENiagaraStripScriptByteCodeOption = ENiagaraStripScriptByteCodeOption(
        2,
    );
}
#[repr(transparent)]
pub struct ENiagaraCompilationMode(pub i32);
impl ENiagaraCompilationMode {
    pub const ORIGINAL: ENiagaraCompilationMode = ENiagaraCompilationMode(0);
    pub const ASYNC_TASKS: ENiagaraCompilationMode = ENiagaraCompilationMode(1);
    pub const VERIFY: ENiagaraCompilationMode = ENiagaraCompilationMode(2);
}
#[repr(transparent)]
pub struct ENiagaraDefaultRendererMotionVectorSetting(pub u8);
impl ENiagaraDefaultRendererMotionVectorSetting {
    pub const PRECISE: ENiagaraDefaultRendererMotionVectorSetting = ENiagaraDefaultRendererMotionVectorSetting(
        0,
    );
    pub const APPROXIMATE: ENiagaraDefaultRendererMotionVectorSetting = ENiagaraDefaultRendererMotionVectorSetting(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraDefaultRendererPixelCoverageMode(pub u8);
impl ENiagaraDefaultRendererPixelCoverageMode {
    pub const ENABLED: ENiagaraDefaultRendererPixelCoverageMode = ENiagaraDefaultRendererPixelCoverageMode(
        0,
    );
    pub const DISABLED: ENiagaraDefaultRendererPixelCoverageMode = ENiagaraDefaultRendererPixelCoverageMode(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraDefaultSortPrecision(pub u8);
impl ENiagaraDefaultSortPrecision {
    pub const LOW: ENiagaraDefaultSortPrecision = ENiagaraDefaultSortPrecision(0);
    pub const HIGH: ENiagaraDefaultSortPrecision = ENiagaraDefaultSortPrecision(1);
}
#[repr(transparent)]
pub struct ENiagaraDefaultGpuTranslucentLatency(pub u8);
impl ENiagaraDefaultGpuTranslucentLatency {
    pub const IMMEDIATE: ENiagaraDefaultGpuTranslucentLatency = ENiagaraDefaultGpuTranslucentLatency(
        0,
    );
    pub const LATENT: ENiagaraDefaultGpuTranslucentLatency = ENiagaraDefaultGpuTranslucentLatency(
        1,
    );
}
#[repr(transparent)]
pub struct ENDISkelMesh_GpuMaxInfluences(pub u8);
impl ENDISkelMesh_GpuMaxInfluences {
    pub const ALLOW_MAX4: ENDISkelMesh_GpuMaxInfluences = ENDISkelMesh_GpuMaxInfluences(
        0,
    );
    pub const ALLOW_MAX8: ENDISkelMesh_GpuMaxInfluences = ENDISkelMesh_GpuMaxInfluences(
        1,
    );
    pub const UNLIMITED: ENDISkelMesh_GpuMaxInfluences = ENDISkelMesh_GpuMaxInfluences(
        2,
    );
}
#[repr(transparent)]
pub struct ENDISkelMesh_GpuUniformSamplingFormat(pub u8);
impl ENDISkelMesh_GpuUniformSamplingFormat {
    pub const FULL: ENDISkelMesh_GpuUniformSamplingFormat = ENDISkelMesh_GpuUniformSamplingFormat(
        0,
    );
    pub const LIMITED_24_8: ENDISkelMesh_GpuUniformSamplingFormat = ENDISkelMesh_GpuUniformSamplingFormat(
        1,
    );
    pub const LIMITED_23_9: ENDISkelMesh_GpuUniformSamplingFormat = ENDISkelMesh_GpuUniformSamplingFormat(
        2,
    );
}
#[repr(transparent)]
pub struct ENDISkelMesh_AdjacencyTriangleIndexFormat(pub u8);
impl ENDISkelMesh_AdjacencyTriangleIndexFormat {
    pub const FULL: ENDISkelMesh_AdjacencyTriangleIndexFormat = ENDISkelMesh_AdjacencyTriangleIndexFormat(
        0,
    );
    pub const HALF: ENDISkelMesh_AdjacencyTriangleIndexFormat = ENDISkelMesh_AdjacencyTriangleIndexFormat(
        1,
    );
}
#[repr(transparent)]
pub struct ENiagaraSpriteAlignment(pub u8);
impl ENiagaraSpriteAlignment {
    pub const UNALIGNED: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(0);
    pub const VELOCITY_ALIGNED: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(1);
    pub const CUSTOM_ALIGNMENT: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(2);
    pub const AUTOMATIC: ENiagaraSpriteAlignment = ENiagaraSpriteAlignment(3);
}
#[repr(transparent)]
pub struct ENiagaraSpriteFacingMode(pub u8);
impl ENiagaraSpriteFacingMode {
    pub const FACE_CAMERA: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(0);
    pub const FACE_CAMERA_PLANE: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(1);
    pub const CUSTOM_FACING_VECTOR: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(
        2,
    );
    pub const FACE_CAMERA_POSITION: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(
        3,
    );
    pub const FACE_CAMERA_DISTANCE_BLEND: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(
        4,
    );
    pub const AUTOMATIC: ENiagaraSpriteFacingMode = ENiagaraSpriteFacingMode(5);
}
#[repr(transparent)]
pub struct ENiagaraRendererPixelCoverageMode(pub u8);
impl ENiagaraRendererPixelCoverageMode {
    pub const AUTOMATIC: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        0,
    );
    pub const DISABLED: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        1,
    );
    pub const ENABLED: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        2,
    );
    pub const ENABLED_RGBA: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        3,
    );
    pub const ENABLED_RGB: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        4,
    );
    pub const ENABLED_A: ENiagaraRendererPixelCoverageMode = ENiagaraRendererPixelCoverageMode(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraCoordinateSpace(pub u32);
impl ENiagaraCoordinateSpace {
    pub const SIMULATION: ENiagaraCoordinateSpace = ENiagaraCoordinateSpace(0);
    pub const WORLD: ENiagaraCoordinateSpace = ENiagaraCoordinateSpace(1);
    pub const LOCAL: ENiagaraCoordinateSpace = ENiagaraCoordinateSpace(2);
}
#[repr(transparent)]
pub struct ENSM_VelocityType(pub i32);
impl ENSM_VelocityType {
    pub const LINEAR: ENSM_VelocityType = ENSM_VelocityType(0);
    pub const FROM_POINT: ENSM_VelocityType = ENSM_VelocityType(1);
    pub const IN_CONE: ENSM_VelocityType = ENSM_VelocityType(2);
}
#[repr(transparent)]
pub struct ENSMInitialMeshOrientationMode(pub i32);
impl ENSMInitialMeshOrientationMode {
    pub const NONE: ENSMInitialMeshOrientationMode = ENSMInitialMeshOrientationMode(0);
    pub const RANDOM: ENSMInitialMeshOrientationMode = ENSMInitialMeshOrientationMode(1);
    pub const ORIENT_TO_AXIS: ENSMInitialMeshOrientationMode = ENSMInitialMeshOrientationMode(
        2,
    );
}
#[repr(transparent)]
pub struct ENSM_ShapePrimitive(pub u8);
impl ENSM_ShapePrimitive {
    pub const BOX: ENSM_ShapePrimitive = ENSM_ShapePrimitive(0);
    pub const CYLINDER: ENSM_ShapePrimitive = ENSM_ShapePrimitive(1);
    pub const PLANE: ENSM_ShapePrimitive = ENSM_ShapePrimitive(2);
    pub const RING: ENSM_ShapePrimitive = ENSM_ShapePrimitive(3);
    pub const SPHERE: ENSM_ShapePrimitive = ENSM_ShapePrimitive(4);
    pub const MAX: ENSM_ShapePrimitive = ENSM_ShapePrimitive(5);
}
#[repr(transparent)]
pub struct ENSM_SurfaceExpansionMode(pub u8);
impl ENSM_SurfaceExpansionMode {
    pub const INNER: ENSM_SurfaceExpansionMode = ENSM_SurfaceExpansionMode(0);
    pub const CENTERED: ENSM_SurfaceExpansionMode = ENSM_SurfaceExpansionMode(1);
    pub const OUTSIDE: ENSM_SurfaceExpansionMode = ENSM_SurfaceExpansionMode(2);
}
#[repr(transparent)]
pub struct ENSMSubUVAnimation_Mode(pub i32);
impl ENSMSubUVAnimation_Mode {
    pub const DIRECT_SET: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(0);
    pub const INFINITE_LOOP: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(1);
    pub const LINEAR: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(2);
    pub const RANDOM: ENSMSubUVAnimation_Mode = ENSMSubUVAnimation_Mode(3);
}
#[repr(transparent)]
pub struct EVolumeCacheType(pub u8);
impl EVolumeCacheType {
    pub const OPEN_VDB: EVolumeCacheType = EVolumeCacheType(0);
}
#[repr(transparent)]
pub struct ENiagaraFunctionDebugState(pub u8);
impl ENiagaraFunctionDebugState {
    pub const NO_DEBUG: ENiagaraFunctionDebugState = ENiagaraFunctionDebugState(0);
    pub const BASIC: ENiagaraFunctionDebugState = ENiagaraFunctionDebugState(1);
}
#[repr(transparent)]
pub struct ENiagaraInputNodeUsage(pub u8);
impl ENiagaraInputNodeUsage {
    pub const UNDEFINED: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(0);
    pub const PARAMETER: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(1);
    pub const ATTRIBUTE: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(2);
    pub const SYSTEM_CONSTANT: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(3);
    pub const TRANSLATOR_CONSTANT: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(4);
    pub const RAPID_ITERATION_PARAMETER: ENiagaraInputNodeUsage = ENiagaraInputNodeUsage(
        5,
    );
}
#[repr(transparent)]
pub struct ENiagaraValidationSeverity(pub i32);
impl ENiagaraValidationSeverity {
    pub const INFO: ENiagaraValidationSeverity = ENiagaraValidationSeverity(0);
    pub const WARNING: ENiagaraValidationSeverity = ENiagaraValidationSeverity(1);
    pub const ERROR: ENiagaraValidationSeverity = ENiagaraValidationSeverity(2);
}
