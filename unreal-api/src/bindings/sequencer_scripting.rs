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
    pub u_movie_scene_scripting_actor_reference_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_actor_reference_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_get_num_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_evaluate_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_compute_effective_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_bool_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_byte_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_tangent_weight_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_tangent_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_leave_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_leave_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_interpolation_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_arrive_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_set_arrive_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_tangent_weight_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_tangent_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_leave_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_leave_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_interpolation_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_arrive_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_key_get_arrive_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_set_pre_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_set_post_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_get_pre_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_get_post_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_get_num_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_evaluate_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_compute_effective_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_double_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_event_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_tangent_weight_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_tangent_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_leave_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_leave_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_interpolation_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_arrive_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_set_arrive_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_tangent_weight_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_tangent_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_leave_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_leave_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_interpolation_mode: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_arrive_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_key_get_arrive_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_set_pre_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_set_post_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_get_pre_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_get_post_infinity_extrapolation: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_get_num_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_evaluate_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_compute_effective_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_float_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_set_interpolate_linear_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_get_num_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_get_interpolate_linear_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_evaluate_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_integer_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_object_path_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_get_num_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_evaluate_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_compute_effective_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_particle_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_string_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_key_set_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_key_set_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_key_get_value: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_key_get_time: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_transform: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_set_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_remove_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_remove_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_has_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_get_keys_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_get_keys: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_get_default: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_scripting_text_channel_add_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_set_spawnable_binding_id: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_set_sorting_order: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_set_parent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_set_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_set_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_remove_track: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_remove: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_not_equal_movie_scene_binding_proxy: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_move_binding_contents: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_sorting_order: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_possessed_object_class: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_parent: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_object_template: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_id: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_get_child_possessables: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_find_tracks_by_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_find_tracks_by_exact_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_equal_equal_movie_scene_binding_proxy: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_binding_extensions_add_track: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_event_track_extensions_get_bound_object_property_class: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_event_track_extensions_add_event_trigger_section: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_event_track_extensions_add_event_repeater_section: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_set_folder_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_set_folder_color: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_remove_child_track: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_remove_child_object_binding: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_remove_child_folder: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_get_folder_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_get_folder_color: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_get_child_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_get_child_object_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_get_child_folders: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_add_child_track: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_add_child_object_binding: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_folder_extensions_add_child_folder: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_material_track_extensions_set_material_info: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_material_track_extensions_set_material_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_material_track_extensions_get_material_info: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_material_track_extensions_get_material_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_primitive_material_track_extensions_set_material_info: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_primitive_material_track_extensions_set_material_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_primitive_material_track_extensions_get_material_info: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_primitive_material_track_extensions_get_material_index: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_set_property_name_and_path: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_set_object_property_class: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_set_byte_track_enum: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_get_unique_track_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_get_property_path: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_get_property_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_get_object_property_class: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_property_track_extensions_get_byte_track_enum: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_start_frame_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_start_frame_bounded: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_range_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_end_frame_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_end_frame_bounded: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_set_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_has_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_has_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_start_frame_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_parent_sequence_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_end_frame_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_channels_by_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_channel: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_auto_size_start_frame_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_auto_size_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_auto_size_has_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_auto_size_has_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_auto_size_end_frame_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_auto_size_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_section_extensions_get_all_channels: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_sort_marked_frames: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_work_range_start: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_work_range_end: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_view_range_start: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_view_range_end: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_tick_resolution_directly: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_tick_resolution: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_read_only: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_playback_start_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_playback_start: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_playback_range_locked: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_playback_end_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_playback_end: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_marked_frames_locked: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_marked_frame_in_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_marked_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_evaluation_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_display_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_set_clock_source: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_resolve_binding_id: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_remove_track: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_remove_root_folder_from_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_make_range_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_make_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_locate_bound_objects: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_is_read_only: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_is_playback_range_locked: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_work_range_start: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_work_range_end: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_view_range_start: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_view_range_end: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_tick_resolution: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_spawnables: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_root_folders_in_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_possessables: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_portable_binding_id: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_playback_start_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_playback_start: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_playback_range: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_playback_end_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_playback_end: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_movie_scene: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_marked_frames_from_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_marked_frames: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_evaluation_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_display_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_custom_clock: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_clock_source: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_bindings: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_get_binding_id: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_tracks_by_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_tracks_by_exact_type: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_next_marked_frame_in_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_next_marked_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_marked_frame_by_label: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number_in_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_binding_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_find_binding_by_id: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_delete_marked_frames: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_delete_marked_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_are_marked_frames_locked: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_add_track: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_add_spawnable_from_instance: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_add_spawnable_from_class: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_add_root_folder_to_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_add_possessable: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_add_marked_frame_to_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_sequence_extensions_add_marked_frame: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_time_warp_extensions_to_fixed_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_time_warp_extensions_set_fixed_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_time_warp_extensions_make_time_warp: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_time_warp_extensions_conv_time_warp_variant_to_play_rate: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_time_warp_extensions_conv_play_rate_to_time_warp_variant: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_time_warp_extensions_break_time_warp: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_set_track_row_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_set_sorting_order: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_set_section_to_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_set_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_set_color_tint: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_remove_section: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_get_track_row_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_get_sorting_order: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_get_section_to_key: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_get_sections: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_get_display_name: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_get_color_tint: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_track_extensions_add_section: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_float_vector_track_extensions_set_num_channels_used: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_float_vector_track_extensions_get_num_channels_used: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_double_vector_track_extensions_set_num_channels_used: *mut crate::ffi::UFunctionOpague,
    pub u_movie_scene_double_vector_track_extensions_get_num_channels_used: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_set_start_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_set_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_set_end_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_set_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_remove_start: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_remove_end: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_has_start: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_has_end: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_get_start_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_get_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_get_end_seconds: *mut crate::ffi::UFunctionOpague,
    pub u_sequencer_scripting_range_extensions_get_end_frame: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_movie_scene_scripting_actor_reference_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_actor_reference_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_get_num_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_evaluate_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_compute_effective_range: std::ptr::null_mut(),
            u_movie_scene_scripting_bool_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_byte_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_tangent_weight_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_tangent_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_leave_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_leave_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_interpolation_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_arrive_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_set_arrive_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_tangent_weight_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_tangent_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_leave_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_leave_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_interpolation_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_arrive_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_double_key_get_arrive_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_set_pre_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_set_post_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_get_pre_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_get_post_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_get_num_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_evaluate_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_compute_effective_range: std::ptr::null_mut(),
            u_movie_scene_scripting_double_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_event_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_event_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_event_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_event_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_event_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_event_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_event_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_event_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_event_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_tangent_weight_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_tangent_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_leave_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_leave_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_interpolation_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_arrive_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_set_arrive_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_tangent_weight_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_tangent_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_leave_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_leave_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_interpolation_mode: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_arrive_tangent_weight: std::ptr::null_mut(),
            u_movie_scene_scripting_float_key_get_arrive_tangent: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_set_pre_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_set_post_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_get_pre_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_get_post_infinity_extrapolation: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_get_num_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_evaluate_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_compute_effective_range: std::ptr::null_mut(),
            u_movie_scene_scripting_float_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_set_interpolate_linear_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_get_num_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_get_interpolate_linear_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_evaluate_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_integer_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_object_path_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_get_num_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_evaluate_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_compute_effective_range: std::ptr::null_mut(),
            u_movie_scene_scripting_particle_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_string_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_string_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_string_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_string_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_string_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_scripting_text_key_set_value: std::ptr::null_mut(),
            u_movie_scene_scripting_text_key_set_time: std::ptr::null_mut(),
            u_movie_scene_scripting_text_key_get_value: std::ptr::null_mut(),
            u_movie_scene_scripting_text_key_get_time: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_transform: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_set_default: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_remove_key: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_remove_default: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_has_default: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_get_keys_by_index: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_get_keys: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_get_default: std::ptr::null_mut(),
            u_movie_scene_scripting_text_channel_add_key: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_set_spawnable_binding_id: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_set_sorting_order: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_set_parent: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_set_name: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_set_display_name: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_remove_track: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_remove: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_not_equal_movie_scene_binding_proxy: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_move_binding_contents: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_is_valid: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_tracks: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_sorting_order: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_possessed_object_class: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_parent: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_object_template: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_name: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_id: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_display_name: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_get_child_possessables: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_find_tracks_by_type: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_find_tracks_by_exact_type: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_equal_equal_movie_scene_binding_proxy: std::ptr::null_mut(),
            u_movie_scene_binding_extensions_add_track: std::ptr::null_mut(),
            u_movie_scene_event_track_extensions_get_bound_object_property_class: std::ptr::null_mut(),
            u_movie_scene_event_track_extensions_add_event_trigger_section: std::ptr::null_mut(),
            u_movie_scene_event_track_extensions_add_event_repeater_section: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_set_folder_name: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_set_folder_color: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_remove_child_track: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_remove_child_object_binding: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_remove_child_folder: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_get_folder_name: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_get_folder_color: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_get_child_tracks: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_get_child_object_bindings: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_get_child_folders: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_add_child_track: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_add_child_object_binding: std::ptr::null_mut(),
            u_movie_scene_folder_extensions_add_child_folder: std::ptr::null_mut(),
            u_movie_scene_material_track_extensions_set_material_info: std::ptr::null_mut(),
            u_movie_scene_material_track_extensions_set_material_index: std::ptr::null_mut(),
            u_movie_scene_material_track_extensions_get_material_info: std::ptr::null_mut(),
            u_movie_scene_material_track_extensions_get_material_index: std::ptr::null_mut(),
            u_movie_scene_primitive_material_track_extensions_set_material_info: std::ptr::null_mut(),
            u_movie_scene_primitive_material_track_extensions_set_material_index: std::ptr::null_mut(),
            u_movie_scene_primitive_material_track_extensions_get_material_info: std::ptr::null_mut(),
            u_movie_scene_primitive_material_track_extensions_get_material_index: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_set_property_name_and_path: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_set_object_property_class: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_set_byte_track_enum: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_get_unique_track_name: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_get_property_path: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_get_property_name: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_get_object_property_class: std::ptr::null_mut(),
            u_movie_scene_property_track_extensions_get_byte_track_enum: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_start_frame_seconds: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_start_frame_bounded: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_start_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_range_seconds: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_range: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_end_frame_seconds: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_end_frame_bounded: std::ptr::null_mut(),
            u_movie_scene_section_extensions_set_end_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_has_start_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_has_end_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_start_frame_seconds: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_start_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_parent_sequence_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_end_frame_seconds: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_end_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_channels_by_type: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_channel: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_auto_size_start_frame_seconds: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_auto_size_start_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_auto_size_has_start_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_auto_size_has_end_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_auto_size_end_frame_seconds: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_auto_size_end_frame: std::ptr::null_mut(),
            u_movie_scene_section_extensions_get_all_channels: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_sort_marked_frames: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_work_range_start: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_work_range_end: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_view_range_start: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_view_range_end: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_tick_resolution_directly: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_tick_resolution: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_read_only: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_playback_start_seconds: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_playback_start: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_playback_range_locked: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_playback_end_seconds: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_playback_end: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_marked_frames_locked: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_marked_frame_in_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_marked_frame: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_evaluation_type: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_display_rate: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_set_clock_source: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_resolve_binding_id: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_remove_track: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_remove_root_folder_from_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_make_range_seconds: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_make_range: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_locate_bound_objects: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_is_read_only: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_is_playback_range_locked: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_work_range_start: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_work_range_end: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_view_range_start: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_view_range_end: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_tracks: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_tick_resolution: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_spawnables: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_root_folders_in_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_possessables: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_portable_binding_id: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_playback_start_seconds: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_playback_start: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_playback_range: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_playback_end_seconds: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_playback_end: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_movie_scene: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_marked_frames_from_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_marked_frames: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_evaluation_type: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_display_rate: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_custom_clock: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_clock_source: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_bindings: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_get_binding_id: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_tracks_by_type: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_tracks_by_exact_type: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_next_marked_frame_in_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_next_marked_frame: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_marked_frame_by_label: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number_in_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_binding_by_name: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_find_binding_by_id: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_delete_marked_frames: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_delete_marked_frame: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_are_marked_frames_locked: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_add_track: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_add_spawnable_from_instance: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_add_spawnable_from_class: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_add_root_folder_to_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_add_possessable: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_add_marked_frame_to_sequence: std::ptr::null_mut(),
            u_movie_scene_sequence_extensions_add_marked_frame: std::ptr::null_mut(),
            u_movie_scene_time_warp_extensions_to_fixed_play_rate: std::ptr::null_mut(),
            u_movie_scene_time_warp_extensions_set_fixed_play_rate: std::ptr::null_mut(),
            u_movie_scene_time_warp_extensions_make_time_warp: std::ptr::null_mut(),
            u_movie_scene_time_warp_extensions_conv_time_warp_variant_to_play_rate: std::ptr::null_mut(),
            u_movie_scene_time_warp_extensions_conv_play_rate_to_time_warp_variant: std::ptr::null_mut(),
            u_movie_scene_time_warp_extensions_break_time_warp: std::ptr::null_mut(),
            u_movie_scene_track_extensions_set_track_row_display_name: std::ptr::null_mut(),
            u_movie_scene_track_extensions_set_sorting_order: std::ptr::null_mut(),
            u_movie_scene_track_extensions_set_section_to_key: std::ptr::null_mut(),
            u_movie_scene_track_extensions_set_display_name: std::ptr::null_mut(),
            u_movie_scene_track_extensions_set_color_tint: std::ptr::null_mut(),
            u_movie_scene_track_extensions_remove_section: std::ptr::null_mut(),
            u_movie_scene_track_extensions_get_track_row_display_name: std::ptr::null_mut(),
            u_movie_scene_track_extensions_get_sorting_order: std::ptr::null_mut(),
            u_movie_scene_track_extensions_get_section_to_key: std::ptr::null_mut(),
            u_movie_scene_track_extensions_get_sections: std::ptr::null_mut(),
            u_movie_scene_track_extensions_get_display_name: std::ptr::null_mut(),
            u_movie_scene_track_extensions_get_color_tint: std::ptr::null_mut(),
            u_movie_scene_track_extensions_add_section: std::ptr::null_mut(),
            u_movie_scene_float_vector_track_extensions_set_num_channels_used: std::ptr::null_mut(),
            u_movie_scene_float_vector_track_extensions_get_num_channels_used: std::ptr::null_mut(),
            u_movie_scene_double_vector_track_extensions_set_num_channels_used: std::ptr::null_mut(),
            u_movie_scene_double_vector_track_extensions_get_num_channels_used: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_set_start_seconds: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_set_start_frame: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_set_end_seconds: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_set_end_frame: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_remove_start: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_remove_end: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_has_start: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_has_end: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_get_start_seconds: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_get_start_frame: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_get_end_seconds: std::ptr::null_mut(),
            u_sequencer_scripting_range_extensions_get_end_frame: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingActorReferenceKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_actor_reference_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_actor_reference_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingActorReferenceChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_actor_reference_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingBoolKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingBoolChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_get_num_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_bool_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_evaluate_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputeEffectiveRange"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_bool_channel_compute_effective_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_bool_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingByteKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingByteChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_byte_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_byte_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingDoubleKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTangentWeightMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_set_tangent_weight_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTangentMode"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_set_tangent_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLeaveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_set_leave_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLeaveTangent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_set_leave_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInterpolationMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_set_interpolation_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetArriveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_set_arrive_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetArriveTangent"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_set_arrive_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_get_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTangentWeightMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_get_tangent_weight_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTangentMode"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_get_tangent_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLeaveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_get_leave_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLeaveTangent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_key_get_leave_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInterpolationMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_get_interpolation_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetArriveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_get_arrive_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetArriveTangent"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_key_get_arrive_tangent,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingDoubleChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_channel_set_pre_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPostInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_channel_set_post_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_channel_get_pre_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPostInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_channel_get_post_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_get_num_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_evaluate_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputeEffectiveRange"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_double_channel_compute_effective_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_double_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingEventKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingEventChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_event_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_event_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingFloatKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTangentWeightMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_set_tangent_weight_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTangentMode"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_set_tangent_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLeaveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_set_leave_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetLeaveTangent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_set_leave_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInterpolationMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_set_interpolation_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetArriveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_set_arrive_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetArriveTangent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_set_arrive_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_get_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTangentWeightMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_get_tangent_weight_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTangentMode"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_get_tangent_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLeaveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_get_leave_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetLeaveTangent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_get_leave_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInterpolationMode"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_get_interpolation_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetArriveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_key_get_arrive_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetArriveTangent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_key_get_arrive_tangent,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingFloatChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPreInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_channel_set_pre_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPostInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_channel_set_post_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPreInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_channel_get_pre_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPostInfinityExtrapolation"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_channel_get_post_infinity_extrapolation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_get_num_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_evaluate_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputeEffectiveRange"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_float_channel_compute_effective_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_float_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingIntegerKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingIntegerChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetInterpolateLinearKeys"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_integer_channel_set_interpolate_linear_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_integer_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_get_num_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_integer_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInterpolateLinearKeys"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_integer_channel_get_interpolate_linear_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateKeys"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_integer_channel_evaluate_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_integer_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingObjectPathKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_object_path_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_object_path_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_object_path_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_object_path_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingObjectPathChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_object_path_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_object_path_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_object_path_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_object_path_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_object_path_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_object_path_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_object_path_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_object_path_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_object_path_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingParticleKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingParticleChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_particle_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKeys"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_particle_channel_get_num_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_particle_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateKeys"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_particle_channel_evaluate_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ComputeEffectiveRange"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_particle_channel_compute_effective_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_particle_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingStringKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingStringChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_string_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_string_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_string_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingTextKey::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_key_set_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_key_set_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetValue"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_key_get_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTime"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_key_get_time,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneScriptingTextChannel::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Transform"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_set_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_remove_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_remove_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_has_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeysByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_scripting_text_channel_get_keys_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetKeys"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_get_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDefault"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_get_default,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_scripting_text_channel_add_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneBindingExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSpawnableBindingID"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_binding_extensions_set_spawnable_binding_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSortingOrder"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_set_sorting_order,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetParent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_set_parent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_set_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_set_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTrack"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_remove_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Remove"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_remove,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("NotEqual_MovieSceneBindingProxy"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_binding_extensions_not_equal_movie_scene_binding_proxy,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MoveBindingContents"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_binding_extensions_move_binding_contents,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_is_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTracks"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_get_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSortingOrder"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_get_sorting_order,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPossessedObjectClass"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_binding_extensions_get_possessed_object_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParent"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_get_parent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectTemplate"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_get_object_template,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_get_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetId"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_get_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_get_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildPossessables"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_binding_extensions_get_child_possessables,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindTracksByType"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_find_tracks_by_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindTracksByExactType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_binding_extensions_find_tracks_by_exact_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EqualEqual_MovieSceneBindingProxy"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_binding_extensions_equal_equal_movie_scene_binding_proxy,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTrack"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_binding_extensions_add_track,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneEventTrackExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoundObjectPropertyClass"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_event_track_extensions_get_bound_object_property_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddEventTriggerSection"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_event_track_extensions_add_event_trigger_section,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddEventRepeaterSection"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_event_track_extensions_add_event_repeater_section,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneFolderExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFolderName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_set_folder_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFolderColor"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_set_folder_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChildTrack"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_remove_child_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChildObjectBinding"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_folder_extensions_remove_child_object_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveChildFolder"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_remove_child_folder,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFolderName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_get_folder_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFolderColor"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_get_folder_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildTracks"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_get_child_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildObjectBindings"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_folder_extensions_get_child_object_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChildFolders"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_get_child_folders,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildTrack"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_add_child_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildObjectBinding"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_folder_extensions_add_child_object_binding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddChildFolder"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_folder_extensions_add_child_folder,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneMaterialTrackExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInfo"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_material_track_extensions_set_material_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_material_track_extensions_set_material_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInfo"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_material_track_extensions_get_material_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_material_track_extensions_get_material_index,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieScenePrimitiveMaterialTrackExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialInfo"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_primitive_material_track_extensions_set_material_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMaterialIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_primitive_material_track_extensions_set_material_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialInfo"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_primitive_material_track_extensions_get_material_info,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMaterialIndex"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_primitive_material_track_extensions_get_material_index,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieScenePropertyTrackExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPropertyNameAndPath"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_set_property_name_and_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetObjectPropertyClass"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_set_object_property_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetByteTrackEnum"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_set_byte_track_enum,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUniqueTrackName"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_get_unique_track_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertyPath"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_get_property_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPropertyName"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_get_property_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetObjectPropertyClass"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_get_object_property_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetByteTrackEnum"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_property_track_extensions_get_byte_track_enum,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneSectionExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartFrameSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_set_start_frame_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartFrameBounded"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_set_start_frame_bounded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_set_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRangeSeconds"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_set_range_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRange"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_set_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEndFrameSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_set_end_frame_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEndFrameBounded"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_set_end_frame_bounded,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEndFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_set_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasStartFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_has_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasEndFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_has_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartFrameSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_start_frame_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_get_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentSequenceFrame"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_parent_sequence_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEndFrameSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_end_frame_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEndFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_get_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannelsByType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_channels_by_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetChannel"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_get_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoSizeStartFrameSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_auto_size_start_frame_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoSizeStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_auto_size_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoSizeHasStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_auto_size_has_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoSizeHasEndFrame"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_auto_size_has_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoSizeEndFrameSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_auto_size_end_frame_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAutoSizeEndFrame"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_section_extensions_get_auto_size_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAllChannels"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_section_extensions_get_all_channels,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneSequenceExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SortMarkedFrames"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_sort_marked_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWorkRangeStart"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_work_range_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetWorkRangeEnd"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_work_range_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetViewRangeStart"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_view_range_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetViewRangeEnd"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_view_range_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTickResolutionDirectly"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_tick_resolution_directly,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTickResolution"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_tick_resolution,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetReadOnly"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_read_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackStartSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_playback_start_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackStart"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_playback_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackRangeLocked"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_playback_range_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackEndSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_playback_end_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPlaybackEnd"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_playback_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMarkedFramesLocked"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_marked_frames_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMarkedFrameInSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_marked_frame_in_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMarkedFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_marked_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEvaluationType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_set_evaluation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayRate"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_display_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetClockSource"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_set_clock_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResolveBindingID"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_resolve_binding_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveTrack"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_remove_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveRootFolderFromSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_remove_root_folder_from_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeRangeSeconds"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_make_range_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeRange"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_make_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("LocateBoundObjects"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_locate_bound_objects,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsReadOnly"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_is_read_only,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsPlaybackRangeLocked"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_is_playback_range_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorkRangeStart"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_work_range_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetWorkRangeEnd"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_work_range_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewRangeStart"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_view_range_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetViewRangeEnd"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_view_range_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTracks"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTickResolution"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_tick_resolution,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSpawnables"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_spawnables,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootFoldersInSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_root_folders_in_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPossessables"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_possessables,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPortableBindingID"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_portable_binding_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackStartSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_playback_start_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackStart"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_playback_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackRange"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_playback_range,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackEndSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_playback_end_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPlaybackEnd"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_playback_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMovieScene"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_movie_scene,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMarkedFramesFromSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_marked_frames_from_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMarkedFrames"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_marked_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEvaluationType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_get_evaluation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayRate"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_display_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomClock"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_custom_clock,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetClockSource"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_clock_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBindings"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_bindings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBindingID"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_get_binding_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindTracksByType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_tracks_by_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindTracksByExactType"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_tracks_by_exact_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNextMarkedFrameInSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_next_marked_frame_in_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindNextMarkedFrame"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_next_marked_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindMarkedFrameByLabel"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_marked_frame_by_label,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindMarkedFrameByFrameNumberInSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number_in_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindMarkedFrameByFrameNumber"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindBindingByName"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_find_binding_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindBindingById"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_find_binding_by_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteMarkedFrames"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_delete_marked_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DeleteMarkedFrame"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_delete_marked_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AreMarkedFramesLocked"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_are_marked_frames_locked,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTrack"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_add_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSpawnableFromInstance"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_add_spawnable_from_instance,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSpawnableFromClass"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_add_spawnable_from_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddRootFolderToSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_add_root_folder_to_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddPossessable"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_add_possessable,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMarkedFrameToSequence"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_sequence_extensions_add_marked_frame_to_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMarkedFrame"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_sequence_extensions_add_marked_frame,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneTimeWarpExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ToFixedPlayRate"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_time_warp_extensions_to_fixed_play_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetFixedPlayRate"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_time_warp_extensions_set_fixed_play_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeTimeWarp"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_time_warp_extensions_make_time_warp,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_TimeWarpVariantToPlayRate"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_time_warp_extensions_conv_time_warp_variant_to_play_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("Conv_PlayRateToTimeWarpVariant"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_time_warp_extensions_conv_play_rate_to_time_warp_variant,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("BreakTimeWarp"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_time_warp_extensions_break_time_warp,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneTrackExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetTrackRowDisplayName"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_track_extensions_set_track_row_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSortingOrder"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_set_sorting_order,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSectionToKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_set_section_to_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetDisplayName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_set_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetColorTint"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_set_color_tint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSection"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_remove_section,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTrackRowDisplayName"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_track_extensions_get_track_row_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSortingOrder"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_get_sorting_order,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSectionToKey"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_get_section_to_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSections"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_get_sections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetDisplayName"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_get_display_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetColorTint"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_get_color_tint,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSection"),
            &raw mut __FUNCTION_PTRS.u_movie_scene_track_extensions_add_section,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneFloatVectorTrackExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumChannelsUsed"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_float_vector_track_extensions_set_num_channels_used,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumChannelsUsed"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_float_vector_track_extensions_get_num_channels_used,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMovieSceneDoubleVectorTrackExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetNumChannelsUsed"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_double_vector_track_extensions_set_num_channels_used,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumChannelsUsed"),
            &raw mut __FUNCTION_PTRS
                .u_movie_scene_double_vector_track_extensions_get_num_channels_used,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USequencerScriptingRangeExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_scripting_range_extensions_set_start_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_scripting_range_extensions_set_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEndSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_scripting_range_extensions_set_end_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetEndFrame"),
            &raw mut __FUNCTION_PTRS.u_sequencer_scripting_range_extensions_set_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveStart"),
            &raw mut __FUNCTION_PTRS.u_sequencer_scripting_range_extensions_remove_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveEnd"),
            &raw mut __FUNCTION_PTRS.u_sequencer_scripting_range_extensions_remove_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasStart"),
            &raw mut __FUNCTION_PTRS.u_sequencer_scripting_range_extensions_has_start,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasEnd"),
            &raw mut __FUNCTION_PTRS.u_sequencer_scripting_range_extensions_has_end,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_scripting_range_extensions_get_start_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_scripting_range_extensions_get_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEndSeconds"),
            &raw mut __FUNCTION_PTRS
                .u_sequencer_scripting_range_extensions_get_end_seconds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetEndFrame"),
            &raw mut __FUNCTION_PTRS.u_sequencer_scripting_range_extensions_get_end_frame,
        );
    }
}
#[repr(C, align(4))]
pub struct FSequencerScriptingRange {
    pub flags_0: u8,
    pub inclusive_start: i32,
    pub exclusive_end: i32,
    pub(crate) __padding_end: [u8; 8],
}
impl FSequencerScriptingRange {}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingKey {
    __padding_end: [u8; 72],
}
impl UMovieSceneScriptingKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingActorReferenceKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingActorReferenceKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingActorReferenceKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(
        &mut self,
        in_new_value: &crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> crate::bindings::movie_scene::FMovieSceneObjectBindingID {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_get_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>()
                .read()
        }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingChannel {
    #[doc(hidden)]
    pub(crate) __padding_48: [u8; 48],
    pub channel_name: FName,
}
impl UMovieSceneScriptingChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingActorReferenceChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingActorReferenceChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingActorReferenceChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_default(
        &mut self,
        in_default_value: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(
        &self,
    ) -> crate::bindings::movie_scene::FMovieSceneObjectBindingID {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_get_default,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>()
                .read()
        }
    }
    pub fn add_key(
        &mut self,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        new_value: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingActorReferenceKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_value,
                __buffer
                    .add(4)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(32).cast::<f32>(), 1);
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_actor_reference_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<UPtr<UMovieSceneScriptingActorReferenceKey>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingBoolKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingBoolKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingBoolKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_new_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingBoolChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingBoolChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingBoolChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_default(&mut self, in_default_value: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_num_keys(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_num_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_num_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_get_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn evaluate_keys(
        &self,
        range: FSequencerScriptingRange,
        frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) -> TArray<bool> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_rate,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<bool>>().read() }
    }
    pub fn compute_effective_range(&self) -> FSequencerScriptingRange {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_compute_effective_range,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_compute_effective_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FSequencerScriptingRange>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: bool,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingBoolKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_value, __buffer.add(4).cast::<bool>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(8).cast::<f32>(), 1);
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_bool_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMovieSceneScriptingBoolKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingByteKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingByteKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingByteKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_new_value: u8) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<u8>().read() }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingByteChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingByteChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingByteChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_default(&mut self, in_default_value: u8) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> u8 {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_get_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<u8>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: u8,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        in_interpolation: crate::bindings::movie_scene::EMovieSceneKeyInterpolation,
    ) -> UPtr<UMovieSceneScriptingByteKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_value, __buffer.add(4).cast::<u8>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(8).cast::<f32>(), 1);
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_interpolation,
                __buffer
                    .add(13)
                    .cast::<crate::bindings::movie_scene::EMovieSceneKeyInterpolation>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_byte_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMovieSceneScriptingByteKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingDoubleKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingDoubleKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingDoubleKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_new_value: f64) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_time,
                __buffer,
            )
        };
    }
    pub fn set_tangent_weight_mode(
        &mut self,
        in_new_value: crate::bindings::engine::ERichCurveTangentWeightMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_tangent_weight_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::ERichCurveTangentWeightMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_tangent_weight_mode,
                __buffer,
            )
        };
    }
    pub fn set_tangent_mode(
        &mut self,
        in_new_value: crate::bindings::engine::ERichCurveTangentMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_tangent_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
                __buffer.add(0).cast::<crate::bindings::engine::ERichCurveTangentMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_tangent_mode,
                __buffer,
            )
        };
    }
    pub fn set_leave_tangent_weight(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_leave_tangent_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_leave_tangent_weight,
                __buffer,
            )
        };
    }
    pub fn set_leave_tangent(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_leave_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_leave_tangent,
                __buffer,
            )
        };
    }
    pub fn set_interpolation_mode(
        &mut self,
        in_new_value: crate::bindings::engine::ERichCurveInterpMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_interpolation_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
                __buffer.add(0).cast::<crate::bindings::engine::ERichCurveInterpMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_interpolation_mode,
                __buffer,
            )
        };
    }
    pub fn set_arrive_tangent_weight(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_arrive_tangent_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_arrive_tangent_weight,
                __buffer,
            )
        };
    }
    pub fn set_arrive_tangent(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_arrive_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_set_arrive_tangent,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f64>().read() }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
    pub fn get_tangent_weight_mode(
        &self,
    ) -> crate::bindings::engine::ERichCurveTangentWeightMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_tangent_weight_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_tangent_weight_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveTangentWeightMode>()
                .read()
        }
    }
    pub fn get_tangent_mode(&self) -> crate::bindings::engine::ERichCurveTangentMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_tangent_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_tangent_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveTangentMode>()
                .read()
        }
    }
    pub fn get_leave_tangent_weight(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_leave_tangent_weight,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_leave_tangent_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_leave_tangent(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_leave_tangent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_leave_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_interpolation_mode(
        &self,
    ) -> crate::bindings::engine::ERichCurveInterpMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_interpolation_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_interpolation_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveInterpMode>()
                .read()
        }
    }
    pub fn get_arrive_tangent_weight(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_arrive_tangent_weight,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_arrive_tangent_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_arrive_tangent(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_arrive_tangent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_key_get_arrive_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingDoubleChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingDoubleChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingDoubleChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_pre_infinity_extrapolation(
        &mut self,
        in_extrapolation: crate::bindings::engine::ERichCurveExtrapolation,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_set_pre_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extrapolation,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::ERichCurveExtrapolation>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_set_pre_infinity_extrapolation,
                __buffer,
            )
        };
    }
    pub fn set_post_infinity_extrapolation(
        &mut self,
        in_extrapolation: crate::bindings::engine::ERichCurveExtrapolation,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_set_post_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extrapolation,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::ERichCurveExtrapolation>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_set_post_infinity_extrapolation,
                __buffer,
            )
        };
    }
    pub fn set_default(&mut self, in_default_value: f64) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_pre_infinity_extrapolation(
        &self,
    ) -> crate::bindings::engine::ERichCurveExtrapolation {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_pre_infinity_extrapolation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_pre_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveExtrapolation>()
                .read()
        }
    }
    pub fn get_post_infinity_extrapolation(
        &self,
    ) -> crate::bindings::engine::ERichCurveExtrapolation {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_post_infinity_extrapolation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_post_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveExtrapolation>()
                .read()
        }
    }
    pub fn get_num_keys(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_num_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_num_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_get_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f64>().read() }
    }
    pub fn evaluate_keys(
        &self,
        range: FSequencerScriptingRange,
        frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) -> TArray<f64> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_rate,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<f64>>().read() }
    }
    pub fn compute_effective_range(&self) -> FSequencerScriptingRange {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_compute_effective_range,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_compute_effective_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FSequencerScriptingRange>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: f64,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        in_interpolation: crate::bindings::movie_scene::EMovieSceneKeyInterpolation,
    ) -> UPtr<UMovieSceneScriptingDoubleKey> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_value, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(16).cast::<f32>(), 1);
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
                &in_interpolation,
                __buffer
                    .add(21)
                    .cast::<crate::bindings::movie_scene::EMovieSceneKeyInterpolation>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_double_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UMovieSceneScriptingDoubleKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingEventKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingEventKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingEventKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(
        &mut self,
        in_new_value: &crate::bindings::movie_scene_tracks::FMovieSceneEvent,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene_tracks::FMovieSceneEvent>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> crate::bindings::movie_scene_tracks::FMovieSceneEvent {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_get_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::movie_scene_tracks::FMovieSceneEvent>()
                .read()
        }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingEventChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingEventChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingEventChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_transform,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: crate::bindings::movie_scene_tracks::FMovieSceneEvent,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingEventKey> {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_value,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene_tracks::FMovieSceneEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sub_frame,
                __buffer.add(216).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(220)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_event_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(224).cast::<UPtr<UMovieSceneScriptingEventKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingFloatKey {
    __padding_end: [u8; 72],
}
impl UMovieSceneScriptingFloatKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingFloatKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_time,
                __buffer,
            )
        };
    }
    pub fn set_tangent_weight_mode(
        &mut self,
        in_new_value: crate::bindings::engine::ERichCurveTangentWeightMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_tangent_weight_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::ERichCurveTangentWeightMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_tangent_weight_mode,
                __buffer,
            )
        };
    }
    pub fn set_tangent_mode(
        &mut self,
        in_new_value: crate::bindings::engine::ERichCurveTangentMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_tangent_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
                __buffer.add(0).cast::<crate::bindings::engine::ERichCurveTangentMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_tangent_mode,
                __buffer,
            )
        };
    }
    pub fn set_leave_tangent_weight(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_leave_tangent_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_leave_tangent_weight,
                __buffer,
            )
        };
    }
    pub fn set_leave_tangent(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_leave_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_leave_tangent,
                __buffer,
            )
        };
    }
    pub fn set_interpolation_mode(
        &mut self,
        in_new_value: crate::bindings::engine::ERichCurveInterpMode,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_interpolation_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
                __buffer.add(0).cast::<crate::bindings::engine::ERichCurveInterpMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_interpolation_mode,
                __buffer,
            )
        };
    }
    pub fn set_arrive_tangent_weight(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_arrive_tangent_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_arrive_tangent_weight,
                __buffer,
            )
        };
    }
    pub fn set_arrive_tangent(&mut self, in_new_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_arrive_tangent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_set_arrive_tangent,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
    pub fn get_tangent_weight_mode(
        &self,
    ) -> crate::bindings::engine::ERichCurveTangentWeightMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_tangent_weight_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_tangent_weight_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveTangentWeightMode>()
                .read()
        }
    }
    pub fn get_tangent_mode(&self) -> crate::bindings::engine::ERichCurveTangentMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_tangent_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_tangent_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveTangentMode>()
                .read()
        }
    }
    pub fn get_leave_tangent_weight(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_leave_tangent_weight,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_leave_tangent_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_leave_tangent(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_leave_tangent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_leave_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_interpolation_mode(
        &self,
    ) -> crate::bindings::engine::ERichCurveInterpMode {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_interpolation_mode,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_interpolation_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveInterpMode>()
                .read()
        }
    }
    pub fn get_arrive_tangent_weight(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_arrive_tangent_weight,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_arrive_tangent_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn get_arrive_tangent(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_arrive_tangent,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_key_get_arrive_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingActualFloatKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingActualFloatKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingActualFloatKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingDoubleAsFloatKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingDoubleAsFloatKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingDoubleAsFloatKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingFloatChannel {
    __padding_end: [u8; 144],
}
impl UMovieSceneScriptingFloatChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingFloatChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_pre_infinity_extrapolation(
        &mut self,
        in_extrapolation: crate::bindings::engine::ERichCurveExtrapolation,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_set_pre_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extrapolation,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::ERichCurveExtrapolation>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_set_pre_infinity_extrapolation,
                __buffer,
            )
        };
    }
    pub fn set_post_infinity_extrapolation(
        &mut self,
        in_extrapolation: crate::bindings::engine::ERichCurveExtrapolation,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_set_post_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_extrapolation,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::engine::ERichCurveExtrapolation>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_set_post_infinity_extrapolation,
                __buffer,
            )
        };
    }
    pub fn set_default(&mut self, in_default_value: f32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_pre_infinity_extrapolation(
        &self,
    ) -> crate::bindings::engine::ERichCurveExtrapolation {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_pre_infinity_extrapolation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_pre_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveExtrapolation>()
                .read()
        }
    }
    pub fn get_post_infinity_extrapolation(
        &self,
    ) -> crate::bindings::engine::ERichCurveExtrapolation {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_post_infinity_extrapolation,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_post_infinity_extrapolation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::engine::ERichCurveExtrapolation>()
                .read()
        }
    }
    pub fn get_num_keys(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_num_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_num_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_get_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<f32>().read() }
    }
    pub fn evaluate_keys(
        &self,
        range: FSequencerScriptingRange,
        frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) -> TArray<f32> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_rate,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<f32>>().read() }
    }
    pub fn compute_effective_range(&self) -> FSequencerScriptingRange {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_compute_effective_range,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_compute_effective_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FSequencerScriptingRange>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: f32,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
        in_interpolation: crate::bindings::movie_scene::EMovieSceneKeyInterpolation,
    ) -> UPtr<UMovieSceneScriptingFloatKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_value, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(8).cast::<f32>(), 1);
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_interpolation,
                __buffer
                    .add(13)
                    .cast::<crate::bindings::movie_scene::EMovieSceneKeyInterpolation>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_float_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMovieSceneScriptingFloatKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingIntegerKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingIntegerKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingIntegerKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_new_value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingIntegerChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingIntegerChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingIntegerChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_interpolate_linear_keys(&mut self, b_in_interpolate_linear_keys: bool) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_set_interpolate_linear_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_interpolate_linear_keys,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_set_interpolate_linear_keys,
                __buffer,
            )
        };
    }
    pub fn set_default(&mut self, in_default_value: i32) {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_num_keys(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_num_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_num_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_interpolate_linear_keys(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_interpolate_linear_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_interpolate_linear_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_default(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_get_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn evaluate_keys(
        &self,
        range: FSequencerScriptingRange,
        frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) -> TArray<i32> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_rate,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<TArray<i32>>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: i32,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingIntegerKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&new_value, __buffer.add(4).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(8).cast::<f32>(), 1);
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_integer_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<UPtr<UMovieSceneScriptingIntegerKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingObjectPathKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingObjectPathKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingObjectPathKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(
        &mut self,
        in_new_value: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_get_value,
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
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingObjectPathChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingObjectPathChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingObjectPathChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_default(
        &mut self,
        in_default_value: UPtr<crate::bindings::core_u_object::UObject>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_get_default,
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
    pub fn add_key(
        &mut self,
        in_time: crate::bindings::core_u_object::FFrameNumber,
        new_value: UPtr<crate::bindings::core_u_object::UObject>,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingObjectPathKey> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_value,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(16).cast::<f32>(), 1);
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_object_path_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<UPtr<UMovieSceneScriptingObjectPathKey>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingParticleKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingParticleKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingParticleKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(
        &mut self,
        in_new_value: crate::bindings::movie_scene_tracks::EParticleKey,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene_tracks::EParticleKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> crate::bindings::movie_scene_tracks::EParticleKey {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_get_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::movie_scene_tracks::EParticleKey>()
                .read()
        }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingParticleChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingParticleChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingParticleChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_default(
        &mut self,
        in_default_value: crate::bindings::movie_scene_tracks::EParticleKey,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene_tracks::EParticleKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_num_keys(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_num_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_num_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> crate::bindings::movie_scene_tracks::EParticleKey {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_get_default,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::movie_scene_tracks::EParticleKey>()
                .read()
        }
    }
    pub fn evaluate_keys(
        &self,
        range: FSequencerScriptingRange,
        frame_rate: crate::bindings::core_u_object::FFrameRate,
    ) -> TArray<crate::bindings::movie_scene_tracks::EParticleKey> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_rate,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_evaluate_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::movie_scene_tracks::EParticleKey>>()
                .read()
        }
    }
    pub fn compute_effective_range(&self) -> FSequencerScriptingRange {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_compute_effective_range,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_compute_effective_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FSequencerScriptingRange>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_particle_value: crate::bindings::movie_scene_tracks::EParticleKey,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingParticleKey> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_particle_value,
                __buffer
                    .add(4)
                    .cast::<crate::bindings::movie_scene_tracks::EParticleKey>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(8).cast::<f32>(), 1);
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_particle_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<UPtr<UMovieSceneScriptingParticleKey>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingStringKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingStringKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingStringKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_new_value: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_new_value,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingStringChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingStringChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingStringChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_default(&mut self, in_default_value: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_set_default,
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
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_get_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FString>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: FString,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingStringKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_value,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(24).cast::<f32>(), 1);
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_string_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UMovieSceneScriptingStringKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingTextKey {
    __padding_end: [u8; 104],
}
impl UMovieSceneScriptingTextKey {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingTextKey")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_value(&mut self, in_new_value: &FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_set_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_new_value,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_set_value,
                __buffer,
            )
        };
    }
    pub fn set_time(
        &mut self,
        new_frame_number: &crate::bindings::core_u_object::FFrameNumber,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_set_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_frame_number,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(4).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_set_time,
                __buffer,
            )
        };
    }
    pub fn get_value(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_get_value,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_get_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn get_time(
        &self,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> crate::bindings::core_u_object::FFrameTime {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_get_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_key_get_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<crate::bindings::core_u_object::FFrameTime>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneScriptingTextChannel {
    __padding_end: [u8; 112],
}
impl UMovieSceneScriptingTextChannel {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneScriptingTextChannel")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn transform(
        &mut self,
        offset_frame: crate::bindings::core_u_object::FFrameNumber,
        scale: f64,
        pivot_frame: crate::bindings::core_u_object::FFrameNumber,
        scripting_range: FSequencerScriptingRange,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &offset_frame,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&scale, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pivot_frame,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scripting_range,
                __buffer.add(20).cast::<FSequencerScriptingRange>(),
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_transform,
                __buffer,
            )
        };
    }
    pub fn set_default(&mut self, in_default_value: FText) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_set_default,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_default_value,
                __buffer.add(0).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_set_default,
                __buffer,
            )
        };
    }
    pub fn remove_key(&mut self, key: UPtr<UMovieSceneScriptingKey>) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_remove_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &key,
                __buffer.add(0).cast::<UPtr<UMovieSceneScriptingKey>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_remove_key,
                __buffer,
            )
        };
    }
    pub fn remove_default(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_remove_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_remove_default,
                __buffer,
            )
        };
    }
    pub fn has_default(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_has_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_has_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_keys_by_index(
        &self,
        indices: &TArray<i32>,
    ) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_get_keys_by_index,
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_get_keys_by_index,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read()
        }
    }
    pub fn get_keys(&self) -> TArray<UPtr<UMovieSceneScriptingKey>> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_get_keys,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_get_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TArray<UPtr<UMovieSceneScriptingKey>>>().read() }
    }
    pub fn get_default(&self) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_get_default,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_get_default,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<FText>().read() }
    }
    pub fn add_key(
        &mut self,
        in_time: &crate::bindings::core_u_object::FFrameNumber,
        new_value: FText,
        sub_frame: f32,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> UPtr<UMovieSceneScriptingTextKey> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_add_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_time,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_value,
                __buffer.add(8).cast::<FText>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&sub_frame, __buffer.add(24).cast::<f32>(), 1);
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
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_scripting_text_channel_add_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<UPtr<UMovieSceneScriptingTextKey>>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneBindingExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneBindingExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneBindingExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_spawnable_binding_id(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        spawnable_binding_id: &crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<52>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_spawnable_binding_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                spawnable_binding_id,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_spawnable_binding_id,
                __buffer,
            )
        };
    }
    pub fn set_sorting_order(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        sorting_order: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_sorting_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sorting_order,
                __buffer.add(24).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_sorting_order,
                __buffer,
            )
        };
    }
    pub fn set_parent(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        in_parent_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_parent_binding,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_parent,
                __buffer,
            )
        };
    }
    pub fn set_name(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        in_name: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_name,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_name,
                __buffer,
            )
        };
    }
    pub fn set_display_name(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        in_display_name: &FText,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_display_name,
                __buffer.add(24).cast::<FText>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_set_display_name,
                __buffer,
            )
        };
    }
    pub fn remove_track(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        track_to_remove: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_remove_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_to_remove,
                __buffer
                    .add(24)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_remove_track,
                __buffer,
            )
        };
    }
    pub fn remove(in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_remove,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_remove,
                __buffer,
            )
        };
    }
    pub fn not_equal_movie_scene_binding_proxy(
        a: crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_not_equal_movie_scene_binding_proxy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &a,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_not_equal_movie_scene_binding_proxy,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn move_binding_contents(
        source_binding_id: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        destination_binding_id: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_move_binding_contents,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_binding_id,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                destination_binding_id,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_move_binding_contents,
                __buffer,
            )
        };
    }
    pub fn is_valid(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_is_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_is_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_tracks(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_tracks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_tracks,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn get_sorting_order(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_sorting_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_sorting_order,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<i32>().read() }
    }
    pub fn get_possessed_object_class(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_possessed_object_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_possessed_object_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_parent(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_parent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_parent,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn get_object_template(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> UPtr<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_object_template,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_object_template,
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
    pub fn get_name(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FString>().read() }
    }
    pub fn get_id(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> crate::bindings::core_u_object::FGuid {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_id,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<crate::bindings::core_u_object::FGuid>().read()
        }
    }
    pub fn get_display_name(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<FText>().read() }
    }
    pub fn get_child_possessables(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_child_possessables,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_get_child_possessables,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .read()
        }
    }
    pub fn find_tracks_by_type(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        track_type: TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_find_tracks_by_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer
                    .add(24)
                    .cast::<
                        TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_find_tracks_by_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn find_tracks_by_exact_type(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        track_type: TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_find_tracks_by_exact_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer
                    .add(24)
                    .cast::<
                        TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_find_tracks_by_exact_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn equal_equal_movie_scene_binding_proxy(
        a: crate::bindings::movie_scene::FMovieSceneBindingProxy,
        b: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<49>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_equal_equal_movie_scene_binding_proxy,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &a,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b,
                __buffer
                    .add(24)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_equal_equal_movie_scene_binding_proxy,
                __buffer,
            )
        };
        unsafe { __buffer.add(48).cast::<bool>().read() }
    }
    pub fn add_track(
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        track_type: TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneTrack> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_add_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer
                    .add(24)
                    .cast::<
                        TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneBindingExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_binding_extensions_add_track,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneEventTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneEventTrackExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneEventTrackExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_bound_object_property_class(
        event_key: &crate::bindings::movie_scene_tracks::FMovieSceneEvent,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<216>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_event_track_extensions_get_bound_object_property_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                event_key,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene_tracks::FMovieSceneEvent>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneEventTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_event_track_extensions_get_bound_object_property_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(208)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn add_event_trigger_section(
        in_track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneEventTrack>,
    ) -> UPtr<crate::bindings::movie_scene_tracks::UMovieSceneEventTriggerSection> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_event_track_extensions_add_event_trigger_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::movie_scene_tracks::UMovieSceneEventTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneEventTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_event_track_extensions_add_event_trigger_section,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    UPtr<
                        crate::bindings::movie_scene_tracks::UMovieSceneEventTriggerSection,
                    >,
                >()
                .read()
        }
    }
    pub fn add_event_repeater_section(
        in_track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneEventTrack>,
    ) -> UPtr<crate::bindings::movie_scene_tracks::UMovieSceneEventRepeaterSection> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_event_track_extensions_add_event_repeater_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::movie_scene_tracks::UMovieSceneEventTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneEventTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_event_track_extensions_add_event_repeater_section,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<
                    UPtr<
                        crate::bindings::movie_scene_tracks::UMovieSceneEventRepeaterSection,
                    >,
                >()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneFolderExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneFolderExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneFolderExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_folder_name(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        in_folder_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_set_folder_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_folder_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_set_folder_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_folder_color(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        in_folder_color: crate::bindings::core_u_object::FColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_set_folder_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_folder_color,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_set_folder_color,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn remove_child_track(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        in_track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_remove_child_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_remove_child_track,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_child_object_binding(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        in_object_binding: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_remove_child_object_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object_binding,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_remove_child_object_binding,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_child_folder(
        target_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        folder_to_remove: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_remove_child_folder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder_to_remove,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_remove_child_folder,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_folder_name(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_folder_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_folder_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FName>().read() }
    }
    pub fn get_folder_color(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) -> crate::bindings::core_u_object::FColor {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_folder_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_folder_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FColor>().read()
        }
    }
    pub fn get_child_tracks(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_child_tracks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_child_tracks,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn get_child_object_bindings(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_child_object_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_child_object_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .read()
        }
    }
    pub fn get_child_folders(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_child_folders,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_get_child_folders,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>>()
                .read()
        }
    }
    pub fn add_child_track(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        in_track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_add_child_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_track,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_add_child_track,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_child_object_binding(
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        in_object_binding: crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_add_child_object_binding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object_binding,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_add_child_object_binding,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_child_folder(
        target_folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
        folder_to_add: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_add_child_folder,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_folder,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder_to_add,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFolderExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_folder_extensions_add_child_folder,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneMaterialTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneMaterialTrackExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneMaterialTrackExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_material_info(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
        >,
        material_info: &crate::bindings::movie_scene_tracks::FComponentMaterialInfo,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_set_material_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                material_info,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::movie_scene_tracks::FComponentMaterialInfo,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_set_material_info,
                __buffer,
            )
        };
    }
    pub fn set_material_index(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
        >,
        material_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_set_material_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_set_material_index,
                __buffer,
            )
        };
    }
    pub fn get_material_info(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
        >,
    ) -> crate::bindings::movie_scene_tracks::FComponentMaterialInfo {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_get_material_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_get_material_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::movie_scene_tracks::FComponentMaterialInfo>()
                .read()
        }
    }
    pub fn get_material_index(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
        >,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_get_material_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneComponentMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_material_track_extensions_get_material_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieScenePrimitiveMaterialTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieScenePrimitiveMaterialTrackExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePrimitiveMaterialTrackExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_material_info(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
        >,
        material_info: &crate::bindings::movie_scene_tracks::FComponentMaterialInfo,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_set_material_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                material_info,
                __buffer
                    .add(8)
                    .cast::<
                        crate::bindings::movie_scene_tracks::FComponentMaterialInfo,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePrimitiveMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_set_material_info,
                __buffer,
            )
        };
    }
    pub fn set_material_index(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
        >,
        material_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_set_material_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePrimitiveMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_set_material_index,
                __buffer,
            )
        };
    }
    pub fn get_material_info(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
        >,
    ) -> crate::bindings::movie_scene_tracks::FComponentMaterialInfo {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_get_material_info,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePrimitiveMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_get_material_info,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::movie_scene_tracks::FComponentMaterialInfo>()
                .read()
        }
    }
    pub fn get_material_index(
        track: UPtr<
            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
        >,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_get_material_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePrimitiveMaterialTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePrimitiveMaterialTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_primitive_material_track_extensions_get_material_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieScenePropertyTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieScenePropertyTrackExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieScenePropertyTrackExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_property_name_and_path(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack>,
        in_property_name: &FName,
        in_property_path: FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_set_property_name_and_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_property_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_property_path,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_set_property_name_and_path,
                __buffer,
            )
        };
    }
    pub fn set_object_property_class(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneObjectPropertyTrack>,
        property_class: TSubclassOf<crate::bindings::core_u_object::UObject>,
        b_in_class_property: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_set_object_property_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneObjectPropertyTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_class_property,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_set_object_property_class,
                __buffer,
            )
        };
    }
    pub fn set_byte_track_enum(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneByteTrack>,
        in_enum: UPtr<crate::bindings::core_u_object::UEnum>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_set_byte_track_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::movie_scene_tracks::UMovieSceneByteTrack>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_enum,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UEnum>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_set_byte_track_enum,
                __buffer,
            )
        };
    }
    pub fn get_unique_track_name(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack>,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_unique_track_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_unique_track_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FName>().read() }
    }
    pub fn get_property_path(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_property_path,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_property_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
    pub fn get_property_name(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack>,
    ) -> FName {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_property_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieScenePropertyTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_property_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FName>().read() }
    }
    pub fn get_object_property_class(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneObjectPropertyTrack>,
    ) -> TSubclassOf<crate::bindings::core_u_object::UObject> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_object_property_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneObjectPropertyTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_object_property_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>()
                .read()
        }
    }
    pub fn get_byte_track_enum(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneByteTrack>,
    ) -> UPtr<crate::bindings::core_u_object::UEnum> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_byte_track_enum,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::movie_scene_tracks::UMovieSceneByteTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieScenePropertyTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_property_track_extensions_get_byte_track_enum,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UEnum>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSectionExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneSectionExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSectionExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_start_frame_seconds(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        start_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_start_frame_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_start_frame_seconds,
                __buffer,
            )
        };
    }
    pub fn set_start_frame_bounded(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        b_is_bounded: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_start_frame_bounded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_bounded,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_start_frame_bounded,
                __buffer,
            )
        };
    }
    pub fn set_start_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        start_frame: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_start_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_frame,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_start_frame,
                __buffer,
            )
        };
    }
    pub fn set_range_seconds(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        start_time: f32,
        end_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_range_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_time, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_range_seconds,
                __buffer,
            )
        };
    }
    pub fn set_range(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        start_frame: i32,
        end_frame: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_frame,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_frame, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_range,
                __buffer,
            )
        };
    }
    pub fn set_end_frame_seconds(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        end_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_end_frame_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_time, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_end_frame_seconds,
                __buffer,
            )
        };
    }
    pub fn set_end_frame_bounded(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        b_is_bounded: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_end_frame_bounded,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_bounded,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_end_frame_bounded,
                __buffer,
            )
        };
    }
    pub fn set_end_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        end_frame: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_end_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_frame, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_set_end_frame,
                __buffer,
            )
        };
    }
    pub fn has_start_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_has_start_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_has_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn has_end_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_has_end_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_has_end_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_start_frame_seconds(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_start_frame_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_start_frame_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_start_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_start_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_parent_sequence_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSubSection>,
        in_frame: i32,
        parent_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_parent_sequence_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSubSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&in_frame, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parent_sequence,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_parent_sequence_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<i32>().read() }
    }
    pub fn get_end_frame_seconds(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_end_frame_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_end_frame_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_end_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_end_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_end_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_channels_by_type(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        channel_type: TSubclassOf<UMovieSceneScriptingChannel>,
    ) -> TArray<UPtr<UMovieSceneScriptingChannel>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_channels_by_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &channel_type,
                __buffer.add(8).cast::<TSubclassOf<UMovieSceneScriptingChannel>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_channels_by_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<UPtr<UMovieSceneScriptingChannel>>>().read()
        }
    }
    pub fn get_channel(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
        channel_name: &FName,
    ) -> UPtr<UMovieSceneScriptingChannel> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_channel,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                channel_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<UPtr<UMovieSceneScriptingChannel>>().read() }
    }
    pub fn get_auto_size_start_frame_seconds(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_start_frame_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_start_frame_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_auto_size_start_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_start_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_auto_size_has_start_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_has_start_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_has_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_auto_size_has_end_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_has_end_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_has_end_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_auto_size_end_frame_seconds(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_end_frame_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_end_frame_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_auto_size_end_frame(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_end_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_auto_size_end_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_all_channels(
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) -> TArray<UPtr<UMovieSceneScriptingChannel>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_all_channels,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSectionExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_section_extensions_get_all_channels,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<UPtr<UMovieSceneScriptingChannel>>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneSequenceExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneSequenceExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneSequenceExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn sort_marked_frames(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_sort_marked_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_sort_marked_frames,
                __buffer,
            )
        };
    }
    pub fn set_work_range_start(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        start_time_in_seconds: f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_work_range_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_time_in_seconds,
                __buffer.add(8).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_work_range_start,
                __buffer,
            )
        };
    }
    pub fn set_work_range_end(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        end_time_in_seconds: f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_work_range_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_time_in_seconds,
                __buffer.add(8).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_work_range_end,
                __buffer,
            )
        };
    }
    pub fn set_view_range_start(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        start_time_in_seconds: f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_view_range_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_time_in_seconds,
                __buffer.add(8).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_view_range_start,
                __buffer,
            )
        };
    }
    pub fn set_view_range_end(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        end_time_in_seconds: f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_view_range_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &end_time_in_seconds,
                __buffer.add(8).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_view_range_end,
                __buffer,
            )
        };
    }
    pub fn set_tick_resolution_directly(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        tick_resolution: crate::bindings::core_u_object::FFrameRate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_tick_resolution_directly,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tick_resolution,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_tick_resolution_directly,
                __buffer,
            )
        };
    }
    pub fn set_tick_resolution(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        tick_resolution: crate::bindings::core_u_object::FFrameRate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_tick_resolution,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &tick_resolution,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_tick_resolution,
                __buffer,
            )
        };
    }
    pub fn set_read_only(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        b_in_read_only: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_read_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_read_only,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_read_only,
                __buffer,
            )
        };
    }
    pub fn set_playback_start_seconds(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        start_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_start_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_start_seconds,
                __buffer,
            )
        };
    }
    pub fn set_playback_start(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        start_frame: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_frame,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_start,
                __buffer,
            )
        };
    }
    pub fn set_playback_range_locked(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        b_in_locked: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_range_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_locked,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_range_locked,
                __buffer,
            )
        };
    }
    pub fn set_playback_end_seconds(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        end_time: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_end_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_time, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_end_seconds,
                __buffer,
            )
        };
    }
    pub fn set_playback_end(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        end_frame: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end_frame, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_playback_end,
                __buffer,
            )
        };
    }
    pub fn set_marked_frames_locked(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        b_in_locked: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_marked_frames_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_locked,
                __buffer.add(8).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_marked_frames_locked,
                __buffer,
            )
        };
    }
    pub fn set_marked_frame_in_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_mark_index: i32,
        in_frame_number: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_marked_frame_in_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mark_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_number,
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
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_marked_frame_in_sequence,
                __buffer,
            )
        };
    }
    pub fn set_marked_frame(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_mark_index: i32,
        in_frame_number: crate::bindings::core_u_object::FFrameNumber,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_marked_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_mark_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_number,
                __buffer.add(12).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_marked_frame,
                __buffer,
            )
        };
    }
    pub fn set_evaluation_type(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_evaluation_type: crate::bindings::movie_scene::EMovieSceneEvaluationType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_evaluation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_evaluation_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneEvaluationType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_evaluation_type,
                __buffer,
            )
        };
    }
    pub fn set_display_rate(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        display_rate: crate::bindings::core_u_object::FFrameRate,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_display_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &display_rate,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameRate>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_display_rate,
                __buffer,
            )
        };
    }
    pub fn set_clock_source(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_clock_source: crate::bindings::movie_scene::EUpdateClockSource,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_clock_source,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_clock_source,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EUpdateClockSource>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_set_clock_source,
                __buffer,
            )
        };
    }
    pub fn resolve_binding_id(
        root_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_object_binding_id: crate::bindings::movie_scene::FMovieSceneObjectBindingID,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<64>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_resolve_binding_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &root_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_object_binding_id,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_resolve_binding_id,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn remove_track(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_remove_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_remove_track,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_root_folder_from_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        folder: UPtr<crate::bindings::movie_scene::UMovieSceneFolder>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_remove_root_folder_from_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &folder,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_remove_root_folder_from_sequence,
                __buffer,
            )
        };
    }
    pub fn make_range_seconds(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        start_time: f32,
        duration: f32,
    ) -> FSequencerScriptingRange {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_make_range_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_make_range_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FSequencerScriptingRange>().read() }
    }
    pub fn make_range(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        start_frame: i32,
        duration: i32,
    ) -> FSequencerScriptingRange {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_make_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_frame,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_make_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FSequencerScriptingRange>().read() }
    }
    pub fn locate_bound_objects(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
        context: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> TArray<UPtr<crate::bindings::core_u_object::UObject>> {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_locate_bound_objects,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &context,
                __buffer.add(32).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_locate_bound_objects,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<UPtr<crate::bindings::core_u_object::UObject>>>()
                .read()
        }
    }
    pub fn is_read_only(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_is_read_only,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_is_read_only,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_playback_range_locked(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_is_playback_range_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_is_playback_range_locked,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_work_range_start(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_work_range_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_work_range_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f64>().read() }
    }
    pub fn get_work_range_end(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_work_range_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_work_range_end,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f64>().read() }
    }
    pub fn get_view_range_start(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_view_range_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_view_range_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f64>().read() }
    }
    pub fn get_view_range_end(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_view_range_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_view_range_end,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f64>().read() }
    }
    pub fn get_tracks(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_tracks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_tracks,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn get_tick_resolution(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> crate::bindings::core_u_object::FFrameRate {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_tick_resolution,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_tick_resolution,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameRate>().read()
        }
    }
    pub fn get_spawnables(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_spawnables,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_spawnables,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .read()
        }
    }
    pub fn get_root_folders_in_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_root_folders_in_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_root_folders_in_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>>()
                .read()
        }
    }
    pub fn get_possessables(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_possessables,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_possessables,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .read()
        }
    }
    pub fn get_portable_binding_id(
        root_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        destination_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> crate::bindings::movie_scene::FMovieSceneObjectBindingID {
        let mut __stack = crate::core_data::StackAlloc::<68>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_portable_binding_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &root_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_sequence,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
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
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_portable_binding_id,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>()
                .read()
        }
    }
    pub fn get_playback_start_seconds(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_start_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_start_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_playback_start(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_playback_range(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> FSequencerScriptingRange {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_range,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_range,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FSequencerScriptingRange>().read() }
    }
    pub fn get_playback_end_seconds(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_end_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_end_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<f32>().read() }
    }
    pub fn get_playback_end(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_playback_end,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_movie_scene(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> UPtr<crate::bindings::movie_scene::UMovieScene> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_movie_scene,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_movie_scene,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieScene>>()
                .read()
        }
    }
    pub fn get_marked_frames_from_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneMarkedFrame> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_marked_frames_from_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_marked_frames_from_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneMarkedFrame>>()
                .read()
        }
    }
    pub fn get_marked_frames(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneMarkedFrame> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_marked_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_marked_frames,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneMarkedFrame>>()
                .read()
        }
    }
    pub fn get_evaluation_type(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> crate::bindings::movie_scene::EMovieSceneEvaluationType {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_evaluation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_evaluation_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::movie_scene::EMovieSceneEvaluationType>()
                .read()
        }
    }
    pub fn get_display_rate(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> crate::bindings::core_u_object::FFrameRate {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_display_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_display_rate,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameRate>().read()
        }
    }
    pub fn get_custom_clock(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneClock> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_custom_clock,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_custom_clock,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneClock>>()
                .read()
        }
    }
    pub fn get_clock_source(
        in_sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> crate::bindings::movie_scene::EUpdateClockSource {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_clock_source,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_clock_source,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::movie_scene::EUpdateClockSource>()
                .read()
        }
    }
    pub fn get_bindings(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_bindings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_bindings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::movie_scene::FMovieSceneBindingProxy>>()
                .read()
        }
    }
    pub fn get_binding_id(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_binding: &crate::bindings::movie_scene::FMovieSceneBindingProxy,
    ) -> crate::bindings::movie_scene::FMovieSceneObjectBindingID {
        let mut __stack = crate::core_data::StackAlloc::<60>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_binding_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_binding,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_get_binding_id,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::movie_scene::FMovieSceneObjectBindingID>()
                .read()
        }
    }
    pub fn find_tracks_by_type(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        track_type: TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_tracks_by_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer
                    .add(8)
                    .cast::<
                        TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_tracks_by_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn find_tracks_by_exact_type(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        track_type: TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_tracks_by_exact_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer
                    .add(8)
                    .cast::<
                        TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_tracks_by_exact_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>>()
                .read()
        }
    }
    pub fn find_next_marked_frame_in_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_frame_number: crate::bindings::core_u_object::FFrameNumber,
        b_forward: bool,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_next_marked_frame_in_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_number,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_forward,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &time_unit,
                __buffer
                    .add(13)
                    .cast::<crate::bindings::movie_scene::EMovieSceneTimeUnit>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_next_marked_frame_in_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn find_next_marked_frame(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_frame_number: crate::bindings::core_u_object::FFrameNumber,
        b_forward: bool,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_next_marked_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_number,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_forward,
                __buffer.add(12).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_next_marked_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn find_marked_frame_by_label(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_label: FString,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_marked_frame_by_label,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_label,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_marked_frame_by_label,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<i32>().read() }
    }
    pub fn find_marked_frame_by_frame_number_in_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_frame_number: crate::bindings::core_u_object::FFrameNumber,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number_in_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_number,
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
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number_in_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<i32>().read() }
    }
    pub fn find_marked_frame_by_frame_number(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_frame_number: crate::bindings::core_u_object::FFrameNumber,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_frame_number,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FFrameNumber>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_marked_frame_by_frame_number,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<i32>().read() }
    }
    pub fn find_binding_by_name(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        name: FString,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_binding_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&name, __buffer.add(8).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_binding_by_name,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn find_binding_by_id(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        binding_id: crate::bindings::core_u_object::FGuid,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_binding_by_id,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &binding_id,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FGuid>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_find_binding_by_id,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn delete_marked_frames(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_delete_marked_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_delete_marked_frames,
                __buffer,
            )
        };
    }
    pub fn delete_marked_frame(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        delete_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_delete_marked_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &delete_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_delete_marked_frame,
                __buffer,
            )
        };
    }
    pub fn are_marked_frames_locked(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_are_marked_frames_locked,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_are_marked_frames_locked,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn add_track(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        track_type: TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneTrack> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_type,
                __buffer
                    .add(8)
                    .cast::<
                        TSubclassOf<crate::bindings::movie_scene::UMovieSceneTrack>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_track,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>()
                .read()
        }
    }
    pub fn add_spawnable_from_instance(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        object_to_spawn: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_spawnable_from_instance,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_spawn,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_spawnable_from_instance,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn add_spawnable_from_class(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        class_to_spawn: TSubclassOf<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_spawnable_from_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &class_to_spawn,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_spawnable_from_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn add_root_folder_to_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        new_folder_name: FString,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneFolder> {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_root_folder_to_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_folder_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_root_folder_to_sequence,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneFolder>>()
                .read()
        }
    }
    pub fn add_possessable(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        object_to_possess: UPtr<crate::bindings::core_u_object::UObject>,
    ) -> crate::bindings::movie_scene::FMovieSceneBindingProxy {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_possessable,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &object_to_possess,
                __buffer.add(8).cast::<UPtr<crate::bindings::core_u_object::UObject>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_possessable,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::movie_scene::FMovieSceneBindingProxy>()
                .read()
        }
    }
    pub fn add_marked_frame_to_sequence(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_marked_frame: &crate::bindings::movie_scene::FMovieSceneMarkedFrame,
        time_unit: crate::bindings::movie_scene::EMovieSceneTimeUnit,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_marked_frame_to_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_marked_frame,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::FMovieSceneMarkedFrame>(),
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
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_marked_frame_to_sequence,
                __buffer,
            )
        };
        unsafe { __buffer.add(92).cast::<i32>().read() }
    }
    pub fn add_marked_frame(
        sequence: UPtr<crate::bindings::movie_scene::UMovieSceneSequence>,
        in_marked_frame: &crate::bindings::movie_scene::FMovieSceneMarkedFrame,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<92>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_marked_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_marked_frame,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::movie_scene::FMovieSceneMarkedFrame>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneSequenceExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_sequence_extensions_add_marked_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(88).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTimeWarpExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneTimeWarpExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTimeWarpExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn to_fixed_play_rate(
        time_warp: &crate::bindings::movie_scene::FMovieSceneTimeWarpVariant,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_to_fixed_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_warp,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneTimeWarpVariant>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTimeWarpExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_to_fixed_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f64>().read() }
    }
    pub fn set_fixed_play_rate(
        time_warp: &mut crate::bindings::movie_scene::FMovieSceneTimeWarpVariant,
        fixed_play_rate: f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_set_fixed_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_warp,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneTimeWarpVariant>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fixed_play_rate,
                __buffer.add(16).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTimeWarpExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_set_fixed_play_rate,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::movie_scene::FMovieSceneTimeWarpVariant>()
                .swap(time_warp);
        }
    }
    pub fn make_time_warp(
        fixed_play_rate: f64,
    ) -> crate::bindings::movie_scene::FMovieSceneTimeWarpVariant {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_make_time_warp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &fixed_play_rate,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTimeWarpExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_make_time_warp,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::movie_scene::FMovieSceneTimeWarpVariant>()
                .read()
        }
    }
    pub fn conv_time_warp_variant_to_play_rate(
        time_warp: &crate::bindings::movie_scene::FMovieSceneTimeWarpVariant,
    ) -> f64 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_conv_time_warp_variant_to_play_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_warp,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneTimeWarpVariant>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTimeWarpExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_conv_time_warp_variant_to_play_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<f64>().read() }
    }
    pub fn conv_play_rate_to_time_warp_variant(
        constant_play_rate: f64,
    ) -> crate::bindings::movie_scene::FMovieSceneTimeWarpVariant {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_conv_play_rate_to_time_warp_variant,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &constant_play_rate,
                __buffer.add(0).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTimeWarpExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_conv_play_rate_to_time_warp_variant,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::movie_scene::FMovieSceneTimeWarpVariant>()
                .read()
        }
    }
    pub fn break_time_warp(
        time_warp: &crate::bindings::movie_scene::FMovieSceneTimeWarpVariant,
        fixed_play_rate: &mut f64,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_break_time_warp,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                time_warp,
                __buffer
                    .add(0)
                    .cast::<crate::bindings::movie_scene::FMovieSceneTimeWarpVariant>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                fixed_play_rate,
                __buffer.add(16).cast::<f64>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTimeWarpExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_time_warp_extensions_break_time_warp,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<f64>().swap(fixed_play_rate);
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneTrackExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneTrackExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_track_row_display_name(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        in_name: &FText,
        row_index: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_track_row_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(8).cast::<FText>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&row_index, __buffer.add(24).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_track_row_display_name,
                __buffer,
            )
        };
    }
    pub fn set_sorting_order(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        sorting_order: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_sorting_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sorting_order,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_sorting_order,
                __buffer,
            )
        };
    }
    pub fn set_section_to_key(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_section_to_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_section_to_key,
                __buffer,
            )
        };
    }
    pub fn set_display_name(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        in_name: &FText,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(in_name, __buffer.add(8).cast::<FText>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_display_name,
                __buffer,
            )
        };
    }
    pub fn set_color_tint(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        color_tint: &crate::bindings::core_u_object::FColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_color_tint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                color_tint,
                __buffer.add(8).cast::<crate::bindings::core_u_object::FColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_set_color_tint,
                __buffer,
            )
        };
    }
    pub fn remove_section(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        section: UPtr<crate::bindings::movie_scene::UMovieSceneSection>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_remove_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &section,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_remove_section,
                __buffer,
            )
        };
    }
    pub fn get_track_row_display_name(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
        row_index: i32,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_track_row_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&row_index, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_track_row_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FText>().read() }
    }
    pub fn get_sorting_order(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_sorting_order,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_sorting_order,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_section_to_key(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneSection> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_section_to_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_section_to_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>()
                .read()
        }
    }
    pub fn get_sections(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>> {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_sections,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_sections,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>>()
                .read()
        }
    }
    pub fn get_display_name(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> FText {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_display_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_display_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FText>().read() }
    }
    pub fn get_color_tint(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> crate::bindings::core_u_object::FColor {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_color_tint,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_get_color_tint,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<crate::bindings::core_u_object::FColor>().read()
        }
    }
    pub fn add_section(
        track: UPtr<crate::bindings::movie_scene::UMovieSceneTrack>,
    ) -> UPtr<crate::bindings::movie_scene::UMovieSceneSection> {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_add_section,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneTrack>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_track_extensions_add_section,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::movie_scene::UMovieSceneSection>>()
                .read()
        }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneFloatVectorTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneFloatVectorTrackExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneFloatVectorTrackExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_num_channels_used(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneFloatVectorTrack>,
        in_num_channels_used: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_float_vector_track_extensions_set_num_channels_used,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneFloatVectorTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_channels_used,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFloatVectorTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_float_vector_track_extensions_set_num_channels_used,
                __buffer,
            )
        };
    }
    pub fn get_num_channels_used(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneFloatVectorTrack>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_float_vector_track_extensions_get_num_channels_used,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneFloatVectorTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneFloatVectorTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_float_vector_track_extensions_get_num_channels_used,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct UMovieSceneDoubleVectorTrackExtensions {
    __padding_end: [u8; 48],
}
impl UMovieSceneDoubleVectorTrackExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMovieSceneDoubleVectorTrackExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_num_channels_used(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneDoubleVectorTrack>,
        in_num_channels_used: i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_double_vector_track_extensions_set_num_channels_used,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneDoubleVectorTrack,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_num_channels_used,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneDoubleVectorTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_double_vector_track_extensions_set_num_channels_used,
                __buffer,
            )
        };
    }
    pub fn get_num_channels_used(
        track: UPtr<crate::bindings::movie_scene_tracks::UMovieSceneDoubleVectorTrack>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_double_vector_track_extensions_get_num_channels_used,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::movie_scene_tracks::UMovieSceneDoubleVectorTrack,
                        >,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::UMovieSceneDoubleVectorTrackExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_movie_scene_double_vector_track_extensions_get_num_channels_used,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
}
#[repr(C, align(8))]
pub struct USequencerScriptingRangeExtensions {
    __padding_end: [u8; 48],
}
impl USequencerScriptingRangeExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USequencerScriptingRangeExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_start_seconds(range: &mut FSequencerScriptingRange, start: f32) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_start_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_start_seconds,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSequencerScriptingRange>().swap(range);
        }
    }
    pub fn set_start_frame(range: &mut FSequencerScriptingRange, start: i32) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_start_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_start_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSequencerScriptingRange>().swap(range);
        }
    }
    pub fn set_end_seconds(range: &mut FSequencerScriptingRange, end: f32) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_end_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_end_seconds,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSequencerScriptingRange>().swap(range);
        }
    }
    pub fn set_end_frame(range: &mut FSequencerScriptingRange, end: i32) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_end_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&end, __buffer.add(20).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_set_end_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSequencerScriptingRange>().swap(range);
        }
    }
    pub fn remove_start(range: &mut FSequencerScriptingRange) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_remove_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_remove_start,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSequencerScriptingRange>().swap(range);
        }
    }
    pub fn remove_end(range: &mut FSequencerScriptingRange) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_remove_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_remove_end,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FSequencerScriptingRange>().swap(range);
        }
    }
    pub fn has_start(range: &FSequencerScriptingRange) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_has_start,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_has_start,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn has_end(range: &FSequencerScriptingRange) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_has_end,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_has_end,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_start_seconds(range: &FSequencerScriptingRange) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_start_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_start_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
    pub fn get_start_frame(range: &FSequencerScriptingRange) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_start_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn get_end_seconds(range: &FSequencerScriptingRange) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_end_seconds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_end_seconds,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<f32>().read() }
    }
    pub fn get_end_frame(range: &FSequencerScriptingRange) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_end_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                range,
                __buffer.add(0).cast::<FSequencerScriptingRange>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::sequencer_scripting::USequencerScriptingRangeExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::sequencer_scripting::__FUNCTION_PTRS
                    .u_sequencer_scripting_range_extensions_get_end_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
}
