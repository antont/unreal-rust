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
    pub u_interchange_physical_camera_node_set_custom_sensor_width: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_physical_camera_node_set_custom_sensor_height: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_physical_camera_node_set_custom_focal_length: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_physical_camera_node_set_custom_enable_depth_of_field: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_physical_camera_node_get_custom_sensor_width: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_physical_camera_node_get_custom_sensor_height: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_physical_camera_node_get_custom_focal_length: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_physical_camera_node_get_custom_enable_depth_of_field: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_set_custom_width: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_set_custom_projection_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_set_custom_near_clip_plane: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_set_custom_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_set_custom_far_clip_plane: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_set_custom_aspect_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_get_custom_width: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_get_custom_projection_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_get_custom_near_clip_plane: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_get_custom_field_of_view: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_get_custom_far_clip_plane: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_standard_camera_node_get_custom_aspect_ratio: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_node_set_custom_shader_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_node_get_custom_shader_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_node_add_string_input: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_node_add_linear_color_input: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_node_add_float_input: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_decal_node_set_custom_sort_order: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_decal_node_set_custom_decal_size: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_decal_node_set_custom_decal_material_path_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_decal_node_get_custom_sort_order: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_decal_node_get_custom_decal_size: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_decal_node_get_custom_decal_material_path_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_set_pay_load_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_set_custom_srgb: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_set_custom_filter: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_set_custom_color_space: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_set_customb_flip_green_channel: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_get_custom_srgb: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_get_custom_filter: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_get_custom_color_space: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture_node_get_customb_flip_green_channel: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_node_set_custom_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_node_remove_custom_animation_track_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_node_get_custom_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_node_get_custom_animation_track_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_node_get_custom_animation_track_uid_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_node_get_custom_animation_track_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_node_add_custom_animation_track_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_base_node_set_custom_completion_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_base_node_get_custom_completion_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_set_custom_track_set_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_set_custom_time_scale: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_set_custom_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_set_custom_duration: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_get_custom_track_set_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_get_custom_time_scale: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_get_custom_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_set_instance_node_get_custom_duration: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_set_custom_property_track: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_set_custom_frame_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_set_custom_animation_payload_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_set_custom_actor_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_get_custom_property_track: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_get_custom_frame_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_get_custom_animation_payload_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_animation_track_node_get_custom_actor_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_transform_animation_track_node_set_custom_used_channels: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_transform_animation_track_node_get_custom_used_channels: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_stop_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_start_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_custom_skeleton_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_custom_animation_stop_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_custom_animation_start_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_custom_animation_sample_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_scene_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_morph_target_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_is_node_animated_with_baked_curve: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_scene_node_animation_payload_keys: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_morph_target_node_animation_payload_keys: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_stop_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_start_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_custom_skeleton_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_custom_animation_stop_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_custom_animation_start_time: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_skeletal_animation_track_node_get_custom_animation_sample_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_set_payload_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_set_custom_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_set_custom_num_frames: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_set_custom_groom_cache_attributes: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_set_custom_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_set_custom_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_get_custom_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_get_custom_num_frames: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_get_custom_groom_cache_attributes: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_get_custom_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_groom_node_get_custom_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_set_custom_use_temperature: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_set_custom_temperature: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_set_custom_light_color: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_set_custom_intensity: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_get_custom_use_temperature: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_get_custom_temperature: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_get_custom_light_color: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_base_light_node_get_custom_intensity: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_set_custom_use_ies_brightness: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_set_custom_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_set_custom_intensity_units: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_set_custom_ies_texture: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_set_custom_ies_brightness_scale: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_set_custom_attenuation_radius: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_get_custom_use_ies_brightness: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_get_custom_rotation: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_get_custom_intensity_units: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_get_custom_ies_texture: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_get_custom_ies_brightness_scale: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_light_node_get_custom_attenuation_radius: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_point_light_node_set_custom_use_inverse_squared_falloff: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_point_light_node_set_custom_light_falloff_exponent: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_point_light_node_get_custom_use_inverse_squared_falloff: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_point_light_node_get_custom_light_falloff_exponent: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_spot_light_node_set_custom_outer_cone_angle: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_spot_light_node_set_custom_inner_cone_angle: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_spot_light_node_get_custom_outer_cone_angle: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_spot_light_node_get_custom_inner_cone_angle: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_rect_light_node_set_custom_source_width: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_rect_light_node_set_custom_source_height: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_rect_light_node_get_custom_source_width: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_rect_light_node_get_custom_source_height: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_sky_light_node_set_custom_source_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_sky_light_node_set_custom_cubemap_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_sky_light_node_get_custom_source_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_sky_light_node_get_custom_cubemap_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_set_custom_parent: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_set_custom_blend_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_get_vector_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_get_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_get_static_switch_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_get_scalar_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_get_custom_parent: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_get_custom_blend_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_add_vector_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_add_texture_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_add_static_switch_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_instance_node_add_scalar_parameter_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_reference_node_set_custom_content_path: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_material_reference_node_get_custom_content_path: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_lod_container_node_reset_mesh_lod_node_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_lod_container_node_remove_mesh_lod_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_lod_container_node_get_mesh_lod_node_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_lod_container_node_add_mesh_lod_node_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_slot_material_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_skinned_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_skeleton_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_scene_instance_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_pay_load_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_morph_target_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_morph_target_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_morph_target: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_vertex_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_uv_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_polygon_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_has_vertex_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_has_vertex_normal: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_has_vertex_color: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_has_vertex_binormal: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_has_smooth_group: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_collision_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_set_custom_bounding_box: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_remove_slot_material_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_remove_skeleton_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_remove_scene_instance_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_remove_morph_target_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_remove_assembly_part_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_is_skinned_mesh: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_is_morph_target: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_slot_material_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_slot_material_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_skeleton_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_skeleton_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_skeleton_dependecies_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_scene_instance_uids_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_scene_instance_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_scene_instance_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_morph_target_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_morph_target_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_morph_target_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_morph_target_dependecies_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_vertex_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_uv_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_polygon_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_has_vertex_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_has_vertex_normal: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_has_vertex_color: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_has_vertex_binormal: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_has_smooth_group: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_collision_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_custom_bounding_box: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_assembly_part_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_assembly_part_dependencies_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_get_assembly_part_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_mesh_node_add_assembly_part_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_set_custom_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_set_custom_has_constant_topology: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_set_custom_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_set_custom_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_get_custom_start_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_get_custom_has_constant_topology: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_get_custom_frame_rate: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_geometry_cache_node_get_custom_end_frame: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_component_node_set_custom_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_component_node_set_custom_component_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_component_node_get_parent_scene_node_and_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_component_node_get_custom_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_component_node_get_custom_component_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_component_node_get_component_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_component_node_add_component_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_static_mesh_component_node_set_custom_instanced_asset_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_static_mesh_component_node_get_instance_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_static_mesh_component_node_get_custom_instanced_asset_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_static_mesh_component_node_add_instance_transforms: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_instanced_static_mesh_component_node_add_instance_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_slot_material_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_morph_target_curve_weight: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_global_bind_pose_reference_for_mesh_ui_ds: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_time_zero_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_pivot_node_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_is_scene_root: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_has_bind_pose: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_global_matrix_for_t0_rebinding: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_geometric_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_component_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_bind_pose_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_asset_instance_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_animation_asset_uid_to_play: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_custom_actor_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_set_animation_curve_type_for_curve_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_remove_specialized_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_remove_slot_material_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_is_specialized_type_contains: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_specialized_types: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_specialized_type_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_specialized_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_slot_material_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_slot_material_dependencies: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_morph_target_curve_weights: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_global_bind_pose_reference_for_mesh_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_time_zero_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_time_zero_global_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_pivot_node_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_is_scene_root: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_has_bind_pose: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_global_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_global_matrix_for_t0_rebinding: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_geometric_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_component_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_bind_pose_local_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_bind_pose_global_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_asset_instance_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_animation_asset_uid_to_play: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_custom_actor_visibility: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_component_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_get_animation_curve_type_for_curve_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_add_specialized_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_node_add_component_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_make_input_value_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_make_input_parameter_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_make_input_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_make_input_connection_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_is_a_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_is_an_input: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_has_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_has_input: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_get_input_connection: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_gather_inputs: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_disconnect_input_from_output_node: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_disconnect_input: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_connect_ouput_to_input_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_connect_ouput_to_input_by_index: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_ports_api_connect_default_ouput_to_input: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_function_call_shader_node_set_custom_material_function: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_function_call_shader_node_get_custom_material_function: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_set_custom_two_sided_transmission: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_set_custom_two_sided: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_set_custom_screen_space_reflections: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_set_custom_opacity_mask_clip_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_set_custom_is_a_shader_function: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_set_custom_displacement_center_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_set_custom_blend_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_get_custom_two_sided_transmission: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_get_custom_two_sided: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_get_custom_screen_space_reflections: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_get_custom_opacity_mask_clip_value: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_get_custom_is_a_shader_function: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_get_custom_displacement_center_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_shader_graph_node_get_custom_blend_mode: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_specular_profile_node_set_custom_texture: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_specular_profile_node_set_custom_format: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_specular_profile_node_get_custom_texture: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_specular_profile_node_get_custom_format: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture2_d_node_set_force_long_lat_cubemap: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture2_d_node_set_custom_wrap_v: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture2_d_node_set_custom_wrap_u: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture2_d_node_get_source_blocks: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture2_d_node_get_force_long_lat_cubemap: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture2_d_node_get_custom_wrap_v: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_texture2_d_node_get_custom_wrap_u: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_set_custom_variants_payload_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_set_custom_display_text: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_remove_custom_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_get_custom_variants_payload_key: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_get_custom_display_text: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_get_custom_dependency_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_get_custom_dependency_uid_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_get_custom_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_variant_set_node_add_custom_dependency_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_variant_sets_node_remove_custom_variant_set_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_variant_sets_node_get_custom_variant_set_uids: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_variant_sets_node_get_custom_variant_set_uid_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_variant_sets_node_get_custom_variant_set_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_scene_variant_sets_node_add_custom_variant_set_uid: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_set_custom_file_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_set_custom_animation_id: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_remove_custom_grid_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_remove_custom_frame_index_in_animation: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_get_custom_grid_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_get_custom_grid_dependecies_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_get_custom_grid_dependecies: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_get_custom_frame_indices_in_animation: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_get_custom_frame_index_in_animation: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_get_custom_file_name: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_get_custom_animation_id: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_add_custom_grid_dependency: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_node_add_custom_frame_index_in_animation: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_set_custom_num_components: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_set_custom_grid_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_set_custom_grid_active_dimensions: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_set_custom_grid_active_aabb_min: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_set_custom_grid_active_aabb_max: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_set_custom_element_type: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_get_custom_num_components: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_get_custom_grid_transform: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_get_custom_grid_active_dimensions: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_get_custom_grid_active_aabb_min: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_get_custom_grid_active_aabb_max: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_volume_grid_node_get_custom_element_type: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_interchange_physical_camera_node_set_custom_sensor_width: std::ptr::null_mut(),
            u_interchange_physical_camera_node_set_custom_sensor_height: std::ptr::null_mut(),
            u_interchange_physical_camera_node_set_custom_focal_length: std::ptr::null_mut(),
            u_interchange_physical_camera_node_set_custom_enable_depth_of_field: std::ptr::null_mut(),
            u_interchange_physical_camera_node_get_custom_sensor_width: std::ptr::null_mut(),
            u_interchange_physical_camera_node_get_custom_sensor_height: std::ptr::null_mut(),
            u_interchange_physical_camera_node_get_custom_focal_length: std::ptr::null_mut(),
            u_interchange_physical_camera_node_get_custom_enable_depth_of_field: std::ptr::null_mut(),
            u_interchange_standard_camera_node_set_custom_width: std::ptr::null_mut(),
            u_interchange_standard_camera_node_set_custom_projection_mode: std::ptr::null_mut(),
            u_interchange_standard_camera_node_set_custom_near_clip_plane: std::ptr::null_mut(),
            u_interchange_standard_camera_node_set_custom_field_of_view: std::ptr::null_mut(),
            u_interchange_standard_camera_node_set_custom_far_clip_plane: std::ptr::null_mut(),
            u_interchange_standard_camera_node_set_custom_aspect_ratio: std::ptr::null_mut(),
            u_interchange_standard_camera_node_get_custom_width: std::ptr::null_mut(),
            u_interchange_standard_camera_node_get_custom_projection_mode: std::ptr::null_mut(),
            u_interchange_standard_camera_node_get_custom_near_clip_plane: std::ptr::null_mut(),
            u_interchange_standard_camera_node_get_custom_field_of_view: std::ptr::null_mut(),
            u_interchange_standard_camera_node_get_custom_far_clip_plane: std::ptr::null_mut(),
            u_interchange_standard_camera_node_get_custom_aspect_ratio: std::ptr::null_mut(),
            u_interchange_shader_node_set_custom_shader_type: std::ptr::null_mut(),
            u_interchange_shader_node_get_custom_shader_type: std::ptr::null_mut(),
            u_interchange_shader_node_add_string_input: std::ptr::null_mut(),
            u_interchange_shader_node_add_linear_color_input: std::ptr::null_mut(),
            u_interchange_shader_node_add_float_input: std::ptr::null_mut(),
            u_interchange_decal_node_set_custom_sort_order: std::ptr::null_mut(),
            u_interchange_decal_node_set_custom_decal_size: std::ptr::null_mut(),
            u_interchange_decal_node_set_custom_decal_material_path_name: std::ptr::null_mut(),
            u_interchange_decal_node_get_custom_sort_order: std::ptr::null_mut(),
            u_interchange_decal_node_get_custom_decal_size: std::ptr::null_mut(),
            u_interchange_decal_node_get_custom_decal_material_path_name: std::ptr::null_mut(),
            u_interchange_texture_node_set_pay_load_key: std::ptr::null_mut(),
            u_interchange_texture_node_set_custom_srgb: std::ptr::null_mut(),
            u_interchange_texture_node_set_custom_filter: std::ptr::null_mut(),
            u_interchange_texture_node_set_custom_color_space: std::ptr::null_mut(),
            u_interchange_texture_node_set_customb_flip_green_channel: std::ptr::null_mut(),
            u_interchange_texture_node_get_custom_srgb: std::ptr::null_mut(),
            u_interchange_texture_node_get_custom_filter: std::ptr::null_mut(),
            u_interchange_texture_node_get_custom_color_space: std::ptr::null_mut(),
            u_interchange_texture_node_get_customb_flip_green_channel: std::ptr::null_mut(),
            u_interchange_animation_track_set_node_set_custom_frame_rate: std::ptr::null_mut(),
            u_interchange_animation_track_set_node_remove_custom_animation_track_uid: std::ptr::null_mut(),
            u_interchange_animation_track_set_node_get_custom_frame_rate: std::ptr::null_mut(),
            u_interchange_animation_track_set_node_get_custom_animation_track_uids: std::ptr::null_mut(),
            u_interchange_animation_track_set_node_get_custom_animation_track_uid_count: std::ptr::null_mut(),
            u_interchange_animation_track_set_node_get_custom_animation_track_uid: std::ptr::null_mut(),
            u_interchange_animation_track_set_node_add_custom_animation_track_uid: std::ptr::null_mut(),
            u_interchange_animation_track_base_node_set_custom_completion_mode: std::ptr::null_mut(),
            u_interchange_animation_track_base_node_get_custom_completion_mode: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_set_custom_track_set_dependency_uid: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_set_custom_time_scale: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_set_custom_start_frame: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_set_custom_duration: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_get_custom_track_set_dependency_uid: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_get_custom_time_scale: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_get_custom_start_frame: std::ptr::null_mut(),
            u_interchange_animation_track_set_instance_node_get_custom_duration: std::ptr::null_mut(),
            u_interchange_animation_track_node_set_custom_property_track: std::ptr::null_mut(),
            u_interchange_animation_track_node_set_custom_frame_count: std::ptr::null_mut(),
            u_interchange_animation_track_node_set_custom_animation_payload_key: std::ptr::null_mut(),
            u_interchange_animation_track_node_set_custom_actor_dependency_uid: std::ptr::null_mut(),
            u_interchange_animation_track_node_get_custom_property_track: std::ptr::null_mut(),
            u_interchange_animation_track_node_get_custom_frame_count: std::ptr::null_mut(),
            u_interchange_animation_track_node_get_custom_animation_payload_key: std::ptr::null_mut(),
            u_interchange_animation_track_node_get_custom_actor_dependency_uid: std::ptr::null_mut(),
            u_interchange_transform_animation_track_node_set_custom_used_channels: std::ptr::null_mut(),
            u_interchange_transform_animation_track_node_get_custom_used_channels: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_stop_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_start_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_custom_skeleton_node_uid: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_custom_animation_stop_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_custom_animation_start_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_custom_animation_sample_rate: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_scene_node_uid: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_morph_target_node_uid: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_is_node_animated_with_baked_curve: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_scene_node_animation_payload_keys: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_morph_target_node_animation_payload_keys: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_stop_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_start_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_custom_skeleton_node_uid: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_custom_animation_stop_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_custom_animation_start_time: std::ptr::null_mut(),
            u_interchange_skeletal_animation_track_node_get_custom_animation_sample_rate: std::ptr::null_mut(),
            u_interchange_groom_node_set_payload_key: std::ptr::null_mut(),
            u_interchange_groom_node_set_custom_start_frame: std::ptr::null_mut(),
            u_interchange_groom_node_set_custom_num_frames: std::ptr::null_mut(),
            u_interchange_groom_node_set_custom_groom_cache_attributes: std::ptr::null_mut(),
            u_interchange_groom_node_set_custom_frame_rate: std::ptr::null_mut(),
            u_interchange_groom_node_set_custom_end_frame: std::ptr::null_mut(),
            u_interchange_groom_node_get_custom_start_frame: std::ptr::null_mut(),
            u_interchange_groom_node_get_custom_num_frames: std::ptr::null_mut(),
            u_interchange_groom_node_get_custom_groom_cache_attributes: std::ptr::null_mut(),
            u_interchange_groom_node_get_custom_frame_rate: std::ptr::null_mut(),
            u_interchange_groom_node_get_custom_end_frame: std::ptr::null_mut(),
            u_interchange_base_light_node_set_custom_use_temperature: std::ptr::null_mut(),
            u_interchange_base_light_node_set_custom_temperature: std::ptr::null_mut(),
            u_interchange_base_light_node_set_custom_light_color: std::ptr::null_mut(),
            u_interchange_base_light_node_set_custom_intensity: std::ptr::null_mut(),
            u_interchange_base_light_node_get_custom_use_temperature: std::ptr::null_mut(),
            u_interchange_base_light_node_get_custom_temperature: std::ptr::null_mut(),
            u_interchange_base_light_node_get_custom_light_color: std::ptr::null_mut(),
            u_interchange_base_light_node_get_custom_intensity: std::ptr::null_mut(),
            u_interchange_light_node_set_custom_use_ies_brightness: std::ptr::null_mut(),
            u_interchange_light_node_set_custom_rotation: std::ptr::null_mut(),
            u_interchange_light_node_set_custom_intensity_units: std::ptr::null_mut(),
            u_interchange_light_node_set_custom_ies_texture: std::ptr::null_mut(),
            u_interchange_light_node_set_custom_ies_brightness_scale: std::ptr::null_mut(),
            u_interchange_light_node_set_custom_attenuation_radius: std::ptr::null_mut(),
            u_interchange_light_node_get_custom_use_ies_brightness: std::ptr::null_mut(),
            u_interchange_light_node_get_custom_rotation: std::ptr::null_mut(),
            u_interchange_light_node_get_custom_intensity_units: std::ptr::null_mut(),
            u_interchange_light_node_get_custom_ies_texture: std::ptr::null_mut(),
            u_interchange_light_node_get_custom_ies_brightness_scale: std::ptr::null_mut(),
            u_interchange_light_node_get_custom_attenuation_radius: std::ptr::null_mut(),
            u_interchange_point_light_node_set_custom_use_inverse_squared_falloff: std::ptr::null_mut(),
            u_interchange_point_light_node_set_custom_light_falloff_exponent: std::ptr::null_mut(),
            u_interchange_point_light_node_get_custom_use_inverse_squared_falloff: std::ptr::null_mut(),
            u_interchange_point_light_node_get_custom_light_falloff_exponent: std::ptr::null_mut(),
            u_interchange_spot_light_node_set_custom_outer_cone_angle: std::ptr::null_mut(),
            u_interchange_spot_light_node_set_custom_inner_cone_angle: std::ptr::null_mut(),
            u_interchange_spot_light_node_get_custom_outer_cone_angle: std::ptr::null_mut(),
            u_interchange_spot_light_node_get_custom_inner_cone_angle: std::ptr::null_mut(),
            u_interchange_rect_light_node_set_custom_source_width: std::ptr::null_mut(),
            u_interchange_rect_light_node_set_custom_source_height: std::ptr::null_mut(),
            u_interchange_rect_light_node_get_custom_source_width: std::ptr::null_mut(),
            u_interchange_rect_light_node_get_custom_source_height: std::ptr::null_mut(),
            u_interchange_sky_light_node_set_custom_source_type: std::ptr::null_mut(),
            u_interchange_sky_light_node_set_custom_cubemap_dependency: std::ptr::null_mut(),
            u_interchange_sky_light_node_get_custom_source_type: std::ptr::null_mut(),
            u_interchange_sky_light_node_get_custom_cubemap_dependency: std::ptr::null_mut(),
            u_interchange_material_instance_node_set_custom_parent: std::ptr::null_mut(),
            u_interchange_material_instance_node_set_custom_blend_mode: std::ptr::null_mut(),
            u_interchange_material_instance_node_get_vector_parameter_value: std::ptr::null_mut(),
            u_interchange_material_instance_node_get_texture_parameter_value: std::ptr::null_mut(),
            u_interchange_material_instance_node_get_static_switch_parameter_value: std::ptr::null_mut(),
            u_interchange_material_instance_node_get_scalar_parameter_value: std::ptr::null_mut(),
            u_interchange_material_instance_node_get_custom_parent: std::ptr::null_mut(),
            u_interchange_material_instance_node_get_custom_blend_mode: std::ptr::null_mut(),
            u_interchange_material_instance_node_add_vector_parameter_value: std::ptr::null_mut(),
            u_interchange_material_instance_node_add_texture_parameter_value: std::ptr::null_mut(),
            u_interchange_material_instance_node_add_static_switch_parameter_value: std::ptr::null_mut(),
            u_interchange_material_instance_node_add_scalar_parameter_value: std::ptr::null_mut(),
            u_interchange_material_reference_node_set_custom_content_path: std::ptr::null_mut(),
            u_interchange_material_reference_node_get_custom_content_path: std::ptr::null_mut(),
            u_interchange_mesh_lod_container_node_reset_mesh_lod_node_uids: std::ptr::null_mut(),
            u_interchange_mesh_lod_container_node_remove_mesh_lod_node_uid: std::ptr::null_mut(),
            u_interchange_mesh_lod_container_node_get_mesh_lod_node_uids: std::ptr::null_mut(),
            u_interchange_mesh_lod_container_node_add_mesh_lod_node_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_set_slot_material_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_set_skinned_mesh: std::ptr::null_mut(),
            u_interchange_mesh_node_set_skeleton_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_set_scene_instance_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_set_pay_load_key: std::ptr::null_mut(),
            u_interchange_mesh_node_set_morph_target_name: std::ptr::null_mut(),
            u_interchange_mesh_node_set_morph_target_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_set_morph_target: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_vertex_count: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_uv_count: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_polygon_count: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_has_vertex_tangent: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_has_vertex_normal: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_has_vertex_color: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_has_vertex_binormal: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_has_smooth_group: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_collision_type: std::ptr::null_mut(),
            u_interchange_mesh_node_set_custom_bounding_box: std::ptr::null_mut(),
            u_interchange_mesh_node_remove_slot_material_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_remove_skeleton_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_remove_scene_instance_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_remove_morph_target_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_remove_assembly_part_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_is_skinned_mesh: std::ptr::null_mut(),
            u_interchange_mesh_node_is_morph_target: std::ptr::null_mut(),
            u_interchange_mesh_node_get_slot_material_dependency_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_get_slot_material_dependencies: std::ptr::null_mut(),
            u_interchange_mesh_node_get_skeleton_dependency: std::ptr::null_mut(),
            u_interchange_mesh_node_get_skeleton_dependencies: std::ptr::null_mut(),
            u_interchange_mesh_node_get_skeleton_dependecies_count: std::ptr::null_mut(),
            u_interchange_mesh_node_get_scene_instance_uids_count: std::ptr::null_mut(),
            u_interchange_mesh_node_get_scene_instance_uids: std::ptr::null_mut(),
            u_interchange_mesh_node_get_scene_instance_uid: std::ptr::null_mut(),
            u_interchange_mesh_node_get_morph_target_name: std::ptr::null_mut(),
            u_interchange_mesh_node_get_morph_target_dependency: std::ptr::null_mut(),
            u_interchange_mesh_node_get_morph_target_dependencies: std::ptr::null_mut(),
            u_interchange_mesh_node_get_morph_target_dependecies_count: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_vertex_count: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_uv_count: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_polygon_count: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_has_vertex_tangent: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_has_vertex_normal: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_has_vertex_color: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_has_vertex_binormal: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_has_smooth_group: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_collision_type: std::ptr::null_mut(),
            u_interchange_mesh_node_get_custom_bounding_box: std::ptr::null_mut(),
            u_interchange_mesh_node_get_assembly_part_dependency: std::ptr::null_mut(),
            u_interchange_mesh_node_get_assembly_part_dependencies_count: std::ptr::null_mut(),
            u_interchange_mesh_node_get_assembly_part_dependencies: std::ptr::null_mut(),
            u_interchange_mesh_node_add_assembly_part_dependency_uid: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_set_custom_start_frame: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_set_custom_has_constant_topology: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_set_custom_frame_rate: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_set_custom_end_frame: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_get_custom_start_frame: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_get_custom_has_constant_topology: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_get_custom_frame_rate: std::ptr::null_mut(),
            u_interchange_geometry_cache_node_get_custom_end_frame: std::ptr::null_mut(),
            u_interchange_scene_component_node_set_custom_local_transform: std::ptr::null_mut(),
            u_interchange_scene_component_node_set_custom_component_visibility: std::ptr::null_mut(),
            u_interchange_scene_component_node_get_parent_scene_node_and_transform: std::ptr::null_mut(),
            u_interchange_scene_component_node_get_custom_local_transform: std::ptr::null_mut(),
            u_interchange_scene_component_node_get_custom_component_visibility: std::ptr::null_mut(),
            u_interchange_scene_component_node_get_component_uids: std::ptr::null_mut(),
            u_interchange_scene_component_node_add_component_uid: std::ptr::null_mut(),
            u_interchange_instanced_static_mesh_component_node_set_custom_instanced_asset_uid: std::ptr::null_mut(),
            u_interchange_instanced_static_mesh_component_node_get_instance_transforms: std::ptr::null_mut(),
            u_interchange_instanced_static_mesh_component_node_get_custom_instanced_asset_uid: std::ptr::null_mut(),
            u_interchange_instanced_static_mesh_component_node_add_instance_transforms: std::ptr::null_mut(),
            u_interchange_instanced_static_mesh_component_node_add_instance_transform: std::ptr::null_mut(),
            u_interchange_scene_node_set_slot_material_dependency_uid: std::ptr::null_mut(),
            u_interchange_scene_node_set_morph_target_curve_weight: std::ptr::null_mut(),
            u_interchange_scene_node_set_global_bind_pose_reference_for_mesh_ui_ds: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_time_zero_local_transform: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_pivot_node_transform: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_local_transform: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_is_scene_root: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_has_bind_pose: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_global_matrix_for_t0_rebinding: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_geometric_transform: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_component_visibility: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_bind_pose_local_transform: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_asset_instance_uid: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_animation_asset_uid_to_play: std::ptr::null_mut(),
            u_interchange_scene_node_set_custom_actor_visibility: std::ptr::null_mut(),
            u_interchange_scene_node_set_animation_curve_type_for_curve_name: std::ptr::null_mut(),
            u_interchange_scene_node_remove_specialized_type: std::ptr::null_mut(),
            u_interchange_scene_node_remove_slot_material_dependency_uid: std::ptr::null_mut(),
            u_interchange_scene_node_is_specialized_type_contains: std::ptr::null_mut(),
            u_interchange_scene_node_get_specialized_types: std::ptr::null_mut(),
            u_interchange_scene_node_get_specialized_type_count: std::ptr::null_mut(),
            u_interchange_scene_node_get_specialized_type: std::ptr::null_mut(),
            u_interchange_scene_node_get_slot_material_dependency_uid: std::ptr::null_mut(),
            u_interchange_scene_node_get_slot_material_dependencies: std::ptr::null_mut(),
            u_interchange_scene_node_get_morph_target_curve_weights: std::ptr::null_mut(),
            u_interchange_scene_node_get_global_bind_pose_reference_for_mesh_uid: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_time_zero_local_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_time_zero_global_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_pivot_node_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_local_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_is_scene_root: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_has_bind_pose: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_global_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_global_matrix_for_t0_rebinding: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_geometric_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_component_visibility: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_bind_pose_local_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_bind_pose_global_transform: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_asset_instance_uid: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_animation_asset_uid_to_play: std::ptr::null_mut(),
            u_interchange_scene_node_get_custom_actor_visibility: std::ptr::null_mut(),
            u_interchange_scene_node_get_component_uids: std::ptr::null_mut(),
            u_interchange_scene_node_get_animation_curve_type_for_curve_name: std::ptr::null_mut(),
            u_interchange_scene_node_add_specialized_type: std::ptr::null_mut(),
            u_interchange_scene_node_add_component_uid: std::ptr::null_mut(),
            u_interchange_shader_ports_api_make_input_value_key: std::ptr::null_mut(),
            u_interchange_shader_ports_api_make_input_parameter_key: std::ptr::null_mut(),
            u_interchange_shader_ports_api_make_input_name: std::ptr::null_mut(),
            u_interchange_shader_ports_api_make_input_connection_key: std::ptr::null_mut(),
            u_interchange_shader_ports_api_is_a_parameter: std::ptr::null_mut(),
            u_interchange_shader_ports_api_is_an_input: std::ptr::null_mut(),
            u_interchange_shader_ports_api_has_parameter: std::ptr::null_mut(),
            u_interchange_shader_ports_api_has_input: std::ptr::null_mut(),
            u_interchange_shader_ports_api_get_input_connection: std::ptr::null_mut(),
            u_interchange_shader_ports_api_gather_inputs: std::ptr::null_mut(),
            u_interchange_shader_ports_api_disconnect_input_from_output_node: std::ptr::null_mut(),
            u_interchange_shader_ports_api_disconnect_input: std::ptr::null_mut(),
            u_interchange_shader_ports_api_connect_ouput_to_input_by_name: std::ptr::null_mut(),
            u_interchange_shader_ports_api_connect_ouput_to_input_by_index: std::ptr::null_mut(),
            u_interchange_shader_ports_api_connect_default_ouput_to_input: std::ptr::null_mut(),
            u_interchange_function_call_shader_node_set_custom_material_function: std::ptr::null_mut(),
            u_interchange_function_call_shader_node_get_custom_material_function: std::ptr::null_mut(),
            u_interchange_shader_graph_node_set_custom_two_sided_transmission: std::ptr::null_mut(),
            u_interchange_shader_graph_node_set_custom_two_sided: std::ptr::null_mut(),
            u_interchange_shader_graph_node_set_custom_screen_space_reflections: std::ptr::null_mut(),
            u_interchange_shader_graph_node_set_custom_opacity_mask_clip_value: std::ptr::null_mut(),
            u_interchange_shader_graph_node_set_custom_is_a_shader_function: std::ptr::null_mut(),
            u_interchange_shader_graph_node_set_custom_displacement_center_mode: std::ptr::null_mut(),
            u_interchange_shader_graph_node_set_custom_blend_mode: std::ptr::null_mut(),
            u_interchange_shader_graph_node_get_custom_two_sided_transmission: std::ptr::null_mut(),
            u_interchange_shader_graph_node_get_custom_two_sided: std::ptr::null_mut(),
            u_interchange_shader_graph_node_get_custom_screen_space_reflections: std::ptr::null_mut(),
            u_interchange_shader_graph_node_get_custom_opacity_mask_clip_value: std::ptr::null_mut(),
            u_interchange_shader_graph_node_get_custom_is_a_shader_function: std::ptr::null_mut(),
            u_interchange_shader_graph_node_get_custom_displacement_center_mode: std::ptr::null_mut(),
            u_interchange_shader_graph_node_get_custom_blend_mode: std::ptr::null_mut(),
            u_interchange_specular_profile_node_set_custom_texture: std::ptr::null_mut(),
            u_interchange_specular_profile_node_set_custom_format: std::ptr::null_mut(),
            u_interchange_specular_profile_node_get_custom_texture: std::ptr::null_mut(),
            u_interchange_specular_profile_node_get_custom_format: std::ptr::null_mut(),
            u_interchange_texture2_d_node_set_force_long_lat_cubemap: std::ptr::null_mut(),
            u_interchange_texture2_d_node_set_custom_wrap_v: std::ptr::null_mut(),
            u_interchange_texture2_d_node_set_custom_wrap_u: std::ptr::null_mut(),
            u_interchange_texture2_d_node_get_source_blocks: std::ptr::null_mut(),
            u_interchange_texture2_d_node_get_force_long_lat_cubemap: std::ptr::null_mut(),
            u_interchange_texture2_d_node_get_custom_wrap_v: std::ptr::null_mut(),
            u_interchange_texture2_d_node_get_custom_wrap_u: std::ptr::null_mut(),
            u_interchange_variant_set_node_set_custom_variants_payload_key: std::ptr::null_mut(),
            u_interchange_variant_set_node_set_custom_display_text: std::ptr::null_mut(),
            u_interchange_variant_set_node_remove_custom_dependency_uid: std::ptr::null_mut(),
            u_interchange_variant_set_node_get_custom_variants_payload_key: std::ptr::null_mut(),
            u_interchange_variant_set_node_get_custom_display_text: std::ptr::null_mut(),
            u_interchange_variant_set_node_get_custom_dependency_uids: std::ptr::null_mut(),
            u_interchange_variant_set_node_get_custom_dependency_uid_count: std::ptr::null_mut(),
            u_interchange_variant_set_node_get_custom_dependency_uid: std::ptr::null_mut(),
            u_interchange_variant_set_node_add_custom_dependency_uid: std::ptr::null_mut(),
            u_interchange_scene_variant_sets_node_remove_custom_variant_set_uid: std::ptr::null_mut(),
            u_interchange_scene_variant_sets_node_get_custom_variant_set_uids: std::ptr::null_mut(),
            u_interchange_scene_variant_sets_node_get_custom_variant_set_uid_count: std::ptr::null_mut(),
            u_interchange_scene_variant_sets_node_get_custom_variant_set_uid: std::ptr::null_mut(),
            u_interchange_scene_variant_sets_node_add_custom_variant_set_uid: std::ptr::null_mut(),
            u_interchange_volume_node_set_custom_file_name: std::ptr::null_mut(),
            u_interchange_volume_node_set_custom_animation_id: std::ptr::null_mut(),
            u_interchange_volume_node_remove_custom_grid_dependency: std::ptr::null_mut(),
            u_interchange_volume_node_remove_custom_frame_index_in_animation: std::ptr::null_mut(),
            u_interchange_volume_node_get_custom_grid_dependency: std::ptr::null_mut(),
            u_interchange_volume_node_get_custom_grid_dependecies_count: std::ptr::null_mut(),
            u_interchange_volume_node_get_custom_grid_dependecies: std::ptr::null_mut(),
            u_interchange_volume_node_get_custom_frame_indices_in_animation: std::ptr::null_mut(),
            u_interchange_volume_node_get_custom_frame_index_in_animation: std::ptr::null_mut(),
            u_interchange_volume_node_get_custom_file_name: std::ptr::null_mut(),
            u_interchange_volume_node_get_custom_animation_id: std::ptr::null_mut(),
            u_interchange_volume_node_add_custom_grid_dependency: std::ptr::null_mut(),
            u_interchange_volume_node_add_custom_frame_index_in_animation: std::ptr::null_mut(),
            u_interchange_volume_grid_node_set_custom_num_components: std::ptr::null_mut(),
            u_interchange_volume_grid_node_set_custom_grid_transform: std::ptr::null_mut(),
            u_interchange_volume_grid_node_set_custom_grid_active_dimensions: std::ptr::null_mut(),
            u_interchange_volume_grid_node_set_custom_grid_active_aabb_min: std::ptr::null_mut(),
            u_interchange_volume_grid_node_set_custom_grid_active_aabb_max: std::ptr::null_mut(),
            u_interchange_volume_grid_node_set_custom_element_type: std::ptr::null_mut(),
            u_interchange_volume_grid_node_get_custom_num_components: std::ptr::null_mut(),
            u_interchange_volume_grid_node_get_custom_grid_transform: std::ptr::null_mut(),
            u_interchange_volume_grid_node_get_custom_grid_active_dimensions: std::ptr::null_mut(),
            u_interchange_volume_grid_node_get_custom_grid_active_aabb_min: std::ptr::null_mut(),
            u_interchange_volume_grid_node_get_custom_grid_active_aabb_max: std::ptr::null_mut(),
            u_interchange_volume_grid_node_get_custom_element_type: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangePhysicalCameraNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSensorWidth"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_set_custom_sensor_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSensorHeight"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_set_custom_sensor_height,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFocalLength"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_set_custom_focal_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomEnableDepthOfField"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_set_custom_enable_depth_of_field,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSensorWidth"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_get_custom_sensor_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSensorHeight"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_get_custom_sensor_height,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFocalLength"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_get_custom_focal_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomEnableDepthOfField"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_physical_camera_node_get_custom_enable_depth_of_field,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeStandardCameraNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomWidth"),
            &raw mut __FUNCTION_PTRS.u_interchange_standard_camera_node_set_custom_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomProjectionMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_set_custom_projection_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomNearClipPlane"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_set_custom_near_clip_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFieldOfView"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_set_custom_field_of_view,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFarClipPlane"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_set_custom_far_clip_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAspectRatio"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_set_custom_aspect_ratio,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomWidth"),
            &raw mut __FUNCTION_PTRS.u_interchange_standard_camera_node_get_custom_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomProjectionMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_get_custom_projection_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomNearClipPlane"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_get_custom_near_clip_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFieldOfView"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_get_custom_field_of_view,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFarClipPlane"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_get_custom_far_clip_plane,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAspectRatio"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_standard_camera_node_get_custom_aspect_ratio,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeShaderNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomShaderType"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_node_set_custom_shader_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomShaderType"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_node_get_custom_shader_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddStringInput"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_node_add_string_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddLinearColorInput"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_node_add_linear_color_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFloatInput"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_node_add_float_input,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeDecalNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSortOrder"),
            &raw mut __FUNCTION_PTRS.u_interchange_decal_node_set_custom_sort_order,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomDecalSize"),
            &raw mut __FUNCTION_PTRS.u_interchange_decal_node_set_custom_decal_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomDecalMaterialPathName"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_decal_node_set_custom_decal_material_path_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSortOrder"),
            &raw mut __FUNCTION_PTRS.u_interchange_decal_node_get_custom_sort_order,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDecalSize"),
            &raw mut __FUNCTION_PTRS.u_interchange_decal_node_get_custom_decal_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDecalMaterialPathName"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_decal_node_get_custom_decal_material_path_name,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeTextureNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPayLoadKey"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture_node_set_pay_load_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSRGB"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture_node_set_custom_srgb,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFilter"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture_node_set_custom_filter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomColorSpace"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture_node_set_custom_color_space,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustombFlipGreenChannel"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_texture_node_set_customb_flip_green_channel,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSRGB"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture_node_get_custom_srgb,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFilter"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture_node_get_custom_filter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomColorSpace"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture_node_get_custom_color_space,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustombFlipGreenChannel"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_texture_node_get_customb_flip_green_channel,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeAnimationTrackSetNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFrameRate"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_node_set_custom_frame_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCustomAnimationTrackUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_node_remove_custom_animation_track_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFrameRate"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_node_get_custom_frame_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationTrackUids"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_node_get_custom_animation_track_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationTrackUidCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_node_get_custom_animation_track_uid_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationTrackUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_node_get_custom_animation_track_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCustomAnimationTrackUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_node_add_custom_animation_track_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeAnimationTrackBaseNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomCompletionMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_base_node_set_custom_completion_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomCompletionMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_base_node_get_custom_completion_mode,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeAnimationTrackSetInstanceNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTrackSetDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_set_custom_track_set_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTimeScale"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_set_custom_time_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_set_custom_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomDuration"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_set_custom_duration,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTrackSetDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_get_custom_track_set_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTimeScale"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_get_custom_time_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_get_custom_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDuration"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_set_instance_node_get_custom_duration,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeAnimationTrackNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomPropertyTrack"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_set_custom_property_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFrameCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_set_custom_frame_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimationPayloadKey"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_set_custom_animation_payload_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomActorDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_set_custom_actor_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomPropertyTrack"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_get_custom_property_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFrameCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_get_custom_frame_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationPayloadKey"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_get_custom_animation_payload_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomActorDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_animation_track_node_get_custom_actor_dependency_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeTransformAnimationTrackNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomUsedChannels"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_transform_animation_track_node_set_custom_used_channels,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomUsedChannels"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_transform_animation_track_node_get_custom_used_channels,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSkeletalAnimationTrackNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceTimelineAnimationStopTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_stop_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceTimelineAnimationStartTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_start_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSkeletonNodeUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_custom_skeleton_node_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimationStopTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_custom_animation_stop_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimationStartTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_custom_animation_start_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimationSampleRate"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_custom_animation_sample_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimationPayloadKeyForSceneNodeUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_scene_node_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimationPayloadKeyForMorphTargetNodeUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_morph_target_node_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsNodeAnimatedWithBakedCurve"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_is_node_animated_with_baked_curve,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSceneNodeAnimationPayloadKeys"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_scene_node_animation_payload_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetNodeAnimationPayloadKeys"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_morph_target_node_animation_payload_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceTimelineAnimationStopTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_stop_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceTimelineAnimationStartTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_start_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSkeletonNodeUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_custom_skeleton_node_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationStopTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_custom_animation_stop_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationStartTime"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_custom_animation_start_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationSampleRate"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_skeletal_animation_track_node_get_custom_animation_sample_rate,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeGroomNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPayloadKey"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_set_payload_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomStartFrame"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_set_custom_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomNumFrames"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_set_custom_num_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomGroomCacheAttributes"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_groom_node_set_custom_groom_cache_attributes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFrameRate"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_set_custom_frame_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomEndFrame"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_set_custom_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomStartFrame"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_get_custom_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomNumFrames"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_get_custom_num_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGroomCacheAttributes"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_groom_node_get_custom_groom_cache_attributes,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFrameRate"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_get_custom_frame_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomEndFrame"),
            &raw mut __FUNCTION_PTRS.u_interchange_groom_node_get_custom_end_frame,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeBaseLightNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomUseTemperature"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_base_light_node_set_custom_use_temperature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTemperature"),
            &raw mut __FUNCTION_PTRS.u_interchange_base_light_node_set_custom_temperature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomLightColor"),
            &raw mut __FUNCTION_PTRS.u_interchange_base_light_node_set_custom_light_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomIntensity"),
            &raw mut __FUNCTION_PTRS.u_interchange_base_light_node_set_custom_intensity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomUseTemperature"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_base_light_node_get_custom_use_temperature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTemperature"),
            &raw mut __FUNCTION_PTRS.u_interchange_base_light_node_get_custom_temperature,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomLightColor"),
            &raw mut __FUNCTION_PTRS.u_interchange_base_light_node_get_custom_light_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomIntensity"),
            &raw mut __FUNCTION_PTRS.u_interchange_base_light_node_get_custom_intensity,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeLightNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomUseIESBrightness"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_light_node_set_custom_use_ies_brightness,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomRotation"),
            &raw mut __FUNCTION_PTRS.u_interchange_light_node_set_custom_rotation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomIntensityUnits"),
            &raw mut __FUNCTION_PTRS.u_interchange_light_node_set_custom_intensity_units,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomIESTexture"),
            &raw mut __FUNCTION_PTRS.u_interchange_light_node_set_custom_ies_texture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomIESBrightnessScale"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_light_node_set_custom_ies_brightness_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAttenuationRadius"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_light_node_set_custom_attenuation_radius,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomUseIESBrightness"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_light_node_get_custom_use_ies_brightness,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomRotation"),
            &raw mut __FUNCTION_PTRS.u_interchange_light_node_get_custom_rotation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomIntensityUnits"),
            &raw mut __FUNCTION_PTRS.u_interchange_light_node_get_custom_intensity_units,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomIESTexture"),
            &raw mut __FUNCTION_PTRS.u_interchange_light_node_get_custom_ies_texture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomIESBrightnessScale"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_light_node_get_custom_ies_brightness_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAttenuationRadius"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_light_node_get_custom_attenuation_radius,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangePointLightNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomUseInverseSquaredFalloff"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_point_light_node_set_custom_use_inverse_squared_falloff,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomLightFalloffExponent"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_point_light_node_set_custom_light_falloff_exponent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomUseInverseSquaredFalloff"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_point_light_node_get_custom_use_inverse_squared_falloff,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomLightFalloffExponent"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_point_light_node_get_custom_light_falloff_exponent,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSpotLightNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomOuterConeAngle"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_spot_light_node_set_custom_outer_cone_angle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomInnerConeAngle"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_spot_light_node_set_custom_inner_cone_angle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomOuterConeAngle"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_spot_light_node_get_custom_outer_cone_angle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomInnerConeAngle"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_spot_light_node_get_custom_inner_cone_angle,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeRectLightNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceWidth"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_rect_light_node_set_custom_source_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceHeight"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_rect_light_node_set_custom_source_height,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceWidth"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_rect_light_node_get_custom_source_width,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceHeight"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_rect_light_node_get_custom_source_height,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSkyLightNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomSourceType"),
            &raw mut __FUNCTION_PTRS.u_interchange_sky_light_node_set_custom_source_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomCubemapDependency"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_sky_light_node_set_custom_cubemap_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomSourceType"),
            &raw mut __FUNCTION_PTRS.u_interchange_sky_light_node_get_custom_source_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomCubemapDependency"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_sky_light_node_get_custom_cubemap_dependency,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeMaterialInstanceNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomParent"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_set_custom_parent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomBlendMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_set_custom_blend_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_get_vector_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTextureParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_get_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetStaticSwitchParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_get_static_switch_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetScalarParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_get_scalar_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomParent"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_get_custom_parent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBlendMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_get_custom_blend_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVectorParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_add_vector_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTextureParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_add_texture_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddStaticSwitchParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_add_static_switch_parameter_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddScalarParameterValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_instance_node_add_scalar_parameter_value,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeMaterialReferenceNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomContentPath"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_reference_node_set_custom_content_path,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomContentPath"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_material_reference_node_get_custom_content_path,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeMeshLODContainerNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ResetMeshLODNodeUids"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_lod_container_node_reset_mesh_lod_node_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMeshLODNodeUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_lod_container_node_remove_mesh_lod_node_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMeshLODNodeUids"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_lod_container_node_get_mesh_lod_node_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMeshLODNodeUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_lod_container_node_add_mesh_lod_node_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeMeshNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSlotMaterialDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_set_slot_material_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkinnedMesh"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_skinned_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSkeletonDependencyUid"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_skeleton_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSceneInstanceUid"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_scene_instance_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetPayLoadKey"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_pay_load_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMorphTargetName"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_morph_target_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMorphTargetDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_set_morph_target_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMorphTarget"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_morph_target,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomVertexCount"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_vertex_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomUVCount"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_uv_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomPolygonCount"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_polygon_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHasVertexTangent"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_set_custom_has_vertex_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHasVertexNormal"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_has_vertex_normal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHasVertexColor"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_has_vertex_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHasVertexBinormal"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_set_custom_has_vertex_binormal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHasSmoothGroup"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_has_smooth_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomCollisionType"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_collision_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomBoundingBox"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_set_custom_bounding_box,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSlotMaterialDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_remove_slot_material_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSkeletonDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_remove_skeleton_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSceneInstanceUid"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_remove_scene_instance_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMorphTargetDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_remove_morph_target_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAssemblyPartDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_remove_assembly_part_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSkinnedMesh"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_is_skinned_mesh,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsMorphTarget"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_is_morph_target,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlotMaterialDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_slot_material_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlotMaterialDependencies"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_slot_material_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletonDependency"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_skeleton_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletonDependencies"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_skeleton_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSkeletonDependeciesCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_skeleton_dependecies_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSceneInstanceUidsCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_scene_instance_uids_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSceneInstanceUids"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_scene_instance_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSceneInstanceUid"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_scene_instance_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetName"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_morph_target_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetDependency"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_morph_target_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetDependencies"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_morph_target_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetDependeciesCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_morph_target_dependecies_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomVertexCount"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_vertex_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomUVCount"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_uv_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomPolygonCount"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_polygon_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomHasVertexTangent"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_custom_has_vertex_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomHasVertexNormal"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_has_vertex_normal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomHasVertexColor"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_has_vertex_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomHasVertexBinormal"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_custom_has_vertex_binormal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomHasSmoothGroup"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_has_smooth_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomCollisionType"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_collision_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBoundingBox"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_custom_bounding_box,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssemblyPartDependency"),
            &raw mut __FUNCTION_PTRS.u_interchange_mesh_node_get_assembly_part_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssemblyPartDependenciesCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_assembly_part_dependencies_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAssemblyPartDependencies"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_get_assembly_part_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAssemblyPartDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_mesh_node_add_assembly_part_dependency_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeGeometryCacheNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_set_custom_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHasConstantTopology"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_set_custom_has_constant_topology,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFrameRate"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_set_custom_frame_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomEndFrame"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_set_custom_end_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomStartFrame"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_get_custom_start_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomHasConstantTopology"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_get_custom_has_constant_topology,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFrameRate"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_get_custom_frame_rate,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomEndFrame"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_geometry_cache_node_get_custom_end_frame,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSceneComponentNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomLocalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_component_node_set_custom_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomComponentVisibility"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_component_node_set_custom_component_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetParentSceneNodeAndTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_component_node_get_parent_scene_node_and_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomLocalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_component_node_get_custom_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomComponentVisibility"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_component_node_get_custom_component_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetComponentUids"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_component_node_get_component_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddComponentUid"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_component_node_add_component_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeInstancedStaticMeshComponentNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomInstancedAssetUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_static_mesh_component_node_set_custom_instanced_asset_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInstanceTransforms"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_static_mesh_component_node_get_instance_transforms,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomInstancedAssetUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_static_mesh_component_node_get_custom_instanced_asset_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInstanceTransforms"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_static_mesh_component_node_add_instance_transforms,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddInstanceTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_instanced_static_mesh_component_node_add_instance_transform,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSceneNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetSlotMaterialDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_slot_material_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetMorphTargetCurveWeight"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_morph_target_curve_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetGlobalBindPoseReferenceForMeshUIDs"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_global_bind_pose_reference_for_mesh_ui_ds,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTimeZeroLocalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_time_zero_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomPivotNodeTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_pivot_node_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomLocalTransform"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_set_custom_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomIsSceneRoot"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_set_custom_is_scene_root,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomHasBindPose"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_set_custom_has_bind_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomGlobalMatrixForT0Rebinding"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_global_matrix_for_t0_rebinding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomGeometricTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_geometric_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomComponentVisibility"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_component_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomBindPoseLocalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_bind_pose_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAssetInstanceUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_asset_instance_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimationAssetUidToPlay"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_custom_animation_asset_uid_to_play,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomActorVisibility"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_set_custom_actor_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimationCurveTypeForCurveName"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_set_animation_curve_type_for_curve_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSpecializedType"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_remove_specialized_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveSlotMaterialDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_remove_slot_material_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsSpecializedTypeContains"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_is_specialized_type_contains,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSpecializedTypes"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_specialized_types,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSpecializedTypeCount"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_specialized_type_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSpecializedType"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_specialized_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlotMaterialDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_slot_material_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSlotMaterialDependencies"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_slot_material_dependencies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMorphTargetCurveWeights"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_morph_target_curve_weights,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetGlobalBindPoseReferenceForMeshUID"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_global_bind_pose_reference_for_mesh_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTimeZeroLocalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_time_zero_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTimeZeroGlobalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_time_zero_global_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomPivotNodeTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_pivot_node_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomLocalTransform"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_custom_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomIsSceneRoot"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_custom_is_scene_root,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomHasBindPose"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_custom_has_bind_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGlobalTransform"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_custom_global_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGlobalMatrixForT0Rebinding"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_global_matrix_for_t0_rebinding,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGeometricTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_geometric_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomComponentVisibility"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_component_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBindPoseLocalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_bind_pose_local_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBindPoseGlobalTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_bind_pose_global_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAssetInstanceUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_asset_instance_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationAssetUidToPlay"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_custom_animation_asset_uid_to_play,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomActorVisibility"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_custom_actor_visibility,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetComponentUids"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_get_component_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationCurveTypeForCurveName"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_node_get_animation_curve_type_for_curve_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddSpecializedType"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_add_specialized_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddComponentUid"),
            &raw mut __FUNCTION_PTRS.u_interchange_scene_node_add_component_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeShaderPortsAPI::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeInputValueKey"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_make_input_value_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeInputParameterKey"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_ports_api_make_input_parameter_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeInputName"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_make_input_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("MakeInputConnectionKey"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_ports_api_make_input_connection_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAParameter"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_is_a_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsAnInput"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_is_an_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasParameter"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_has_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("HasInput"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_has_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetInputConnection"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_get_input_connection,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GatherInputs"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_gather_inputs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectInputFromOutputNode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_ports_api_disconnect_input_from_output_node,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DisconnectInput"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_ports_api_disconnect_input,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectOuputToInputByName"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_ports_api_connect_ouput_to_input_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectOuputToInputByIndex"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_ports_api_connect_ouput_to_input_by_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ConnectDefaultOuputToInput"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_ports_api_connect_default_ouput_to_input,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeFunctionCallShaderNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomMaterialFunction"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_function_call_shader_node_set_custom_material_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomMaterialFunction"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_function_call_shader_node_get_custom_material_function,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeShaderGraphNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTwoSidedTransmission"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_set_custom_two_sided_transmission,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTwoSided"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_graph_node_set_custom_two_sided,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomScreenSpaceReflections"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_set_custom_screen_space_reflections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomOpacityMaskClipValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_set_custom_opacity_mask_clip_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomIsAShaderFunction"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_set_custom_is_a_shader_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomDisplacementCenterMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_set_custom_displacement_center_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomBlendMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_set_custom_blend_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTwoSidedTransmission"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_get_custom_two_sided_transmission,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTwoSided"),
            &raw mut __FUNCTION_PTRS.u_interchange_shader_graph_node_get_custom_two_sided,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomScreenSpaceReflections"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_get_custom_screen_space_reflections,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomOpacityMaskClipValue"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_get_custom_opacity_mask_clip_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomIsAShaderFunction"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_get_custom_is_a_shader_function,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDisplacementCenterMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_get_custom_displacement_center_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomBlendMode"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_shader_graph_node_get_custom_blend_mode,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSpecularProfileNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomTexture"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_specular_profile_node_set_custom_texture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFormat"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_specular_profile_node_set_custom_format,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomTexture"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_specular_profile_node_get_custom_texture,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFormat"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_specular_profile_node_get_custom_format,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeTexture2DNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetForceLongLatCubemap"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_texture2_d_node_set_force_long_lat_cubemap,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomWrapV"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture2_d_node_set_custom_wrap_v,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomWrapU"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture2_d_node_set_custom_wrap_u,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSourceBlocks"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture2_d_node_get_source_blocks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetForceLongLatCubemap"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_texture2_d_node_get_force_long_lat_cubemap,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomWrapV"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture2_d_node_get_custom_wrap_v,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomWrapU"),
            &raw mut __FUNCTION_PTRS.u_interchange_texture2_d_node_get_custom_wrap_u,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeVariantSetNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomVariantsPayloadKey"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_set_custom_variants_payload_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomDisplayText"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_set_custom_display_text,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCustomDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_remove_custom_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomVariantsPayloadKey"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_get_custom_variants_payload_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDisplayText"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_get_custom_display_text,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDependencyUids"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_get_custom_dependency_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDependencyUidCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_get_custom_dependency_uid_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_get_custom_dependency_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCustomDependencyUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_variant_set_node_add_custom_dependency_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeSceneVariantSetsNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCustomVariantSetUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_variant_sets_node_remove_custom_variant_set_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomVariantSetUids"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_variant_sets_node_get_custom_variant_set_uids,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomVariantSetUidCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_variant_sets_node_get_custom_variant_set_uid_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomVariantSetUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_variant_sets_node_get_custom_variant_set_uid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCustomVariantSetUid"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_scene_variant_sets_node_add_custom_variant_set_uid,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeVolumeNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomFileName"),
            &raw mut __FUNCTION_PTRS.u_interchange_volume_node_set_custom_file_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomAnimationID"),
            &raw mut __FUNCTION_PTRS.u_interchange_volume_node_set_custom_animation_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCustomGridDependency"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_node_remove_custom_grid_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCustomFrameIndexInAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_node_remove_custom_frame_index_in_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGridDependency"),
            &raw mut __FUNCTION_PTRS.u_interchange_volume_node_get_custom_grid_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGridDependeciesCount"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_node_get_custom_grid_dependecies_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGridDependecies"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_node_get_custom_grid_dependecies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFrameIndicesInAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_node_get_custom_frame_indices_in_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFrameIndexInAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_node_get_custom_frame_index_in_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomFileName"),
            &raw mut __FUNCTION_PTRS.u_interchange_volume_node_get_custom_file_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomAnimationID"),
            &raw mut __FUNCTION_PTRS.u_interchange_volume_node_get_custom_animation_id,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCustomGridDependency"),
            &raw mut __FUNCTION_PTRS.u_interchange_volume_node_add_custom_grid_dependency,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCustomFrameIndexInAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_node_add_custom_frame_index_in_animation,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeVolumeGridNode::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomNumComponents"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_set_custom_num_components,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomGridTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_set_custom_grid_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomGridActiveDimensions"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_set_custom_grid_active_dimensions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomGridActiveAABBMin"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_set_custom_grid_active_aabb_min,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomGridActiveAABBMax"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_set_custom_grid_active_aabb_max,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCustomElementType"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_set_custom_element_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomNumComponents"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_get_custom_num_components,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGridTransform"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_get_custom_grid_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGridActiveDimensions"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_get_custom_grid_active_dimensions,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGridActiveAABBMin"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_get_custom_grid_active_aabb_min,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomGridActiveAABBMax"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_get_custom_grid_active_aabb_max,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCustomElementType"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_volume_grid_node_get_custom_element_type,
        );
    }
}
#[repr(C, align(8))]
pub struct FInterchangeAnimationPayLoadKey {
    pub unique_id: FString,
    pub ty: EInterchangeAnimationPayLoadType,
}
impl FInterchangeAnimationPayLoadKey {}
#[repr(C, align(8))]
pub struct FInterchangeGroomPayloadKey {
    pub unique_id: FString,
    pub ty: EInterchangeGroomPayLoadType,
    pub frame_number: i32,
}
impl FInterchangeGroomPayloadKey {}
#[repr(C, align(8))]
pub struct FInterchangeMeshPayLoadKey {
    pub unique_id: FString,
    pub ty: EInterchangeMeshPayLoadType,
    pub frame_number: i32,
}
impl FInterchangeMeshPayLoadKey {}
#[repr(C, align(8))]
pub struct UInterchangePhysicalCameraNode {
    __padding_end: [u8; 176],
}
impl UInterchangePhysicalCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePhysicalCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_sensor_width(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_sensor_width,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_sensor_width,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_sensor_height(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_sensor_height,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_sensor_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_focal_length(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_focal_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_focal_length,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_enable_depth_of_field(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_enable_depth_of_field,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_set_custom_enable_depth_of_field,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_sensor_width(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_sensor_width,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_sensor_width,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_sensor_height(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_sensor_height,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_sensor_height,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_focal_length(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_focal_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_focal_length,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_enable_depth_of_field(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_enable_depth_of_field,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_physical_camera_node_get_custom_enable_depth_of_field,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeStandardCameraNode {
    __padding_end: [u8; 208],
}
impl UInterchangeStandardCameraNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeStandardCameraNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_width(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_width,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_width,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_projection_mode(
        &mut self,
        attribute_value: &EInterchangeCameraProjectionType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_projection_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeCameraProjectionType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_projection_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_near_clip_plane(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_near_clip_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_near_clip_plane,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_field_of_view(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_field_of_view,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_field_of_view,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_far_clip_plane(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_far_clip_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_far_clip_plane,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_aspect_ratio(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_aspect_ratio,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_set_custom_aspect_ratio,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_width(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_width,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_width,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_projection_mode(
        &self,
        attribute_value: &mut EInterchangeCameraProjectionType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_projection_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeCameraProjectionType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_projection_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<EInterchangeCameraProjectionType>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_near_clip_plane(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_near_clip_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_near_clip_plane,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_field_of_view(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_field_of_view,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_field_of_view,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_far_clip_plane(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_far_clip_plane,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_far_clip_plane,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_aspect_ratio(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_aspect_ratio,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_standard_camera_node_get_custom_aspect_ratio,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeShaderNode {
    __padding_end: [u8; 128],
}
impl UInterchangeShaderNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeShaderNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_shader_type(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_set_custom_shader_type,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_set_custom_shader_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_shader_type(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_get_custom_shader_type,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_get_custom_shader_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_string_input(
        &mut self,
        input_name: FString,
        attribute_value: FString,
        b_is_a_parameter: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_add_string_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_parameter,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_add_string_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn add_linear_color_input(
        &mut self,
        input_name: FString,
        attribute_value: &crate::bindings::core_u_object::FLinearColor,
        b_is_a_parameter: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_add_linear_color_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_parameter,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_add_linear_color_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn add_float_input(
        &mut self,
        input_name: FString,
        attribute_value: &f32,
        b_is_a_parameter: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_add_float_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_a_parameter,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_node_add_float_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeDecalMaterialNode {
    __padding_end: [u8; 160],
}
impl UInterchangeDecalMaterialNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeDecalMaterialNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeDecalNode {
    __padding_end: [u8; 160],
}
impl UInterchangeDecalNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeDecalNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_sort_order(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_set_custom_sort_order,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_set_custom_sort_order,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_decal_size(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_set_custom_decal_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_set_custom_decal_size,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn set_custom_decal_material_path_name(
        &mut self,
        attribute_value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_set_custom_decal_material_path_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_set_custom_decal_material_path_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_sort_order(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_get_custom_sort_order,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_get_custom_sort_order,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_decal_size(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_get_custom_decal_size,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_get_custom_decal_size,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FVector>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_custom_decal_material_path_name(
        &self,
        attribute_value: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_get_custom_decal_material_path_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_decal_node_get_custom_decal_material_path_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTextureNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTextureNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_pay_load_key(&mut self, payload_key: FString) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_pay_load_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_pay_load_key,
                __buffer,
            )
        };
    }
    pub fn set_custom_srgb(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_custom_srgb,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_custom_srgb,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_filter(
        &mut self,
        attribute_value: &EInterchangeTextureFilterMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_custom_filter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureFilterMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_custom_filter,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_color_space(
        &mut self,
        attribute_value: &EInterchangeTextureColorSpace,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_custom_color_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureColorSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_custom_color_space,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_customb_flip_green_channel(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_customb_flip_green_channel,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_set_customb_flip_green_channel,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_srgb(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_custom_srgb,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_custom_srgb,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_filter(
        &self,
        attribute_value: &mut EInterchangeTextureFilterMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_custom_filter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureFilterMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_custom_filter,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<EInterchangeTextureFilterMode>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_color_space(
        &self,
        attribute_value: &mut EInterchangeTextureColorSpace,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_custom_color_space,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureColorSpace>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_custom_color_space,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<EInterchangeTextureColorSpace>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_customb_flip_green_channel(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_customb_flip_green_channel,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture_node_get_customb_flip_green_channel,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTexture2DArrayNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTexture2DArrayNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTexture2DArrayNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTextureCubeArrayNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureCubeArrayNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTextureCubeArrayNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTextureCubeNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureCubeNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTextureCubeNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTextureLightProfileNode {
    __padding_end: [u8; 176],
}
impl UInterchangeTextureLightProfileNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTextureLightProfileNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeVolumeTextureNode {
    __padding_end: [u8; 176],
}
impl UInterchangeVolumeTextureNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeVolumeTextureNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackSetNode {
    __padding_end: [u8; 160],
}
impl UInterchangeAnimationTrackSetNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeAnimationTrackSetNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_frame_rate(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_set_custom_frame_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_set_custom_frame_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn remove_custom_animation_track_uid(
        &mut self,
        animation_track_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_remove_custom_animation_track_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_track_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_remove_custom_animation_track_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_frame_rate(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_frame_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_frame_rate,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_animation_track_uids(
        &self,
        out_animation_track_uids: &mut TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_animation_track_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_animation_track_uids,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_animation_track_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_animation_track_uids);
        }
    }
    pub fn get_custom_animation_track_uid_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_animation_track_uid_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_animation_track_uid_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_custom_animation_track_uid(
        &self,
        index: i32,
        out_animation_track_uid: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_animation_track_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_animation_track_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_get_custom_animation_track_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_animation_track_uid);
        }
    }
    pub fn add_custom_animation_track_uid(
        &mut self,
        animation_track_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_add_custom_animation_track_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_track_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_node_add_custom_animation_track_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackBaseNode {
    __padding_end: [u8; 128],
}
impl UInterchangeAnimationTrackBaseNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeAnimationTrackBaseNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_completion_mode(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_base_node_set_custom_completion_mode,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_base_node_set_custom_completion_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_completion_mode(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_base_node_get_custom_completion_mode,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_base_node_get_custom_completion_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackSetInstanceNode {
    __padding_end: [u8; 192],
}
impl UInterchangeAnimationTrackSetInstanceNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeAnimationTrackSetInstanceNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_track_set_dependency_uid(
        &mut self,
        attribute_value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_track_set_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_track_set_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_time_scale(&mut self, attribute_value: &f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_time_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_time_scale,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_start_frame(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_start_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_duration(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_duration,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_set_custom_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_track_set_dependency_uid(
        &self,
        attribute_value: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_track_set_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_track_set_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_time_scale(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_time_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_time_scale,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_start_frame(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_start_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_start_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_duration(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_duration,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_set_instance_node_get_custom_duration,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeAnimationTrackNode {
    __padding_end: [u8; 224],
}
impl UInterchangeAnimationTrackNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeAnimationTrackNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_property_track(
        &mut self,
        property_track: EInterchangePropertyTracks,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_property_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &property_track,
                __buffer.add(0).cast::<EInterchangePropertyTracks>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_property_track,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_frame_count(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_frame_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_frame_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_animation_payload_key(
        &mut self,
        in_unique_id: FString,
        in_type: &EInterchangeAnimationPayLoadType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_animation_payload_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_unique_id,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_type,
                __buffer.add(16).cast::<EInterchangeAnimationPayLoadType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_animation_payload_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn set_custom_actor_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_actor_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_set_custom_actor_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_property_track(
        &self,
        property_track: &mut EInterchangePropertyTracks,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_property_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                property_track,
                __buffer.add(0).cast::<EInterchangePropertyTracks>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_property_track,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EInterchangePropertyTracks>().swap(property_track);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_frame_count(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_frame_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_frame_count,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_animation_payload_key(
        &self,
        animation_pay_load_key: &mut FInterchangeAnimationPayLoadKey,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_animation_payload_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_pay_load_key,
                __buffer.add(0).cast::<FInterchangeAnimationPayLoadKey>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_animation_payload_key,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<FInterchangeAnimationPayLoadKey>()
                .swap(animation_pay_load_key);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_custom_actor_dependency_uid(&self, dependency_uid: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_actor_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_animation_track_node_get_custom_actor_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(dependency_uid);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTransformAnimationTrackNode {
    __padding_end: [u8; 240],
}
impl UInterchangeTransformAnimationTrackNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTransformAnimationTrackNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_used_channels(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_transform_animation_track_node_set_custom_used_channels,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_transform_animation_track_node_set_custom_used_channels,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_used_channels(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_transform_animation_track_node_get_custom_used_channels,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_transform_animation_track_node_get_custom_used_channels,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSkeletalAnimationTrackNode {
    __padding_end: [u8; 704],
}
impl UInterchangeSkeletalAnimationTrackNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSkeletalAnimationTrackNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_source_timeline_animation_stop_time(
        &mut self,
        stop_time: &f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_stop_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(stop_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_stop_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_source_timeline_animation_start_time(
        &mut self,
        start_time: &f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_start_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(start_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_source_timeline_animation_start_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_skeleton_node_uid(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_skeleton_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_skeleton_node_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_animation_stop_time(&mut self, stop_time: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_animation_stop_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(stop_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_animation_stop_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_animation_start_time(&mut self, start_time: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_animation_start_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(start_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_animation_start_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_animation_sample_rate(&mut self, sample_rate: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_animation_sample_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(sample_rate, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_custom_animation_sample_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_animation_payload_key_for_scene_node_uid(
        &mut self,
        scene_node_uid: FString,
        in_unique_id: FString,
        in_type: &EInterchangeAnimationPayLoadType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_scene_node_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scene_node_uid,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_unique_id,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_type,
                __buffer.add(32).cast::<EInterchangeAnimationPayLoadType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_scene_node_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn set_animation_payload_key_for_morph_target_node_uid(
        &mut self,
        morph_target_node_uid: FString,
        in_unique_id: FString,
        in_type: &EInterchangeAnimationPayLoadType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<34>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_morph_target_node_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &morph_target_node_uid,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &in_unique_id,
                __buffer.add(16).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_type,
                __buffer.add(32).cast::<EInterchangeAnimationPayLoadType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_set_animation_payload_key_for_morph_target_node_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(33).cast::<bool>().read() }
    }
    pub fn is_node_animated_with_baked_curve(&self, scene_node_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_is_node_animated_with_baked_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &scene_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_is_node_animated_with_baked_curve,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_scene_node_animation_payload_keys(
        &self,
        out_scene_node_animation_payload_key_uids: &mut TMap<FString, FString>,
        out_scene_node_animation_payload_key_types: &mut TMap<FString, u8>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_scene_node_animation_payload_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_scene_node_animation_payload_key_uids,
                __buffer.add(0).cast::<TMap<FString, FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_scene_node_animation_payload_key_types,
                __buffer.add(80).cast::<TMap<FString, u8>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_scene_node_animation_payload_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TMap<FString, FString>>()
                .swap(out_scene_node_animation_payload_key_uids);
        }
        unsafe {
            __buffer
                .add(80)
                .cast::<TMap<FString, u8>>()
                .swap(out_scene_node_animation_payload_key_types);
        }
    }
    pub fn get_morph_target_node_animation_payload_keys(
        &self,
        out_morph_target_node_animation_payload_key_uids: &mut TMap<FString, FString>,
        out_morph_target_node_animation_payload_key_types: &mut TMap<FString, u8>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<160>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_morph_target_node_animation_payload_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_morph_target_node_animation_payload_key_uids,
                __buffer.add(0).cast::<TMap<FString, FString>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_morph_target_node_animation_payload_key_types,
                __buffer.add(80).cast::<TMap<FString, u8>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_morph_target_node_animation_payload_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TMap<FString, FString>>()
                .swap(out_morph_target_node_animation_payload_key_uids);
        }
        unsafe {
            __buffer
                .add(80)
                .cast::<TMap<FString, u8>>()
                .swap(out_morph_target_node_animation_payload_key_types);
        }
    }
    pub fn get_custom_source_timeline_animation_stop_time(
        &self,
        stop_time: &mut f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_stop_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(stop_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_stop_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(stop_time);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_source_timeline_animation_start_time(
        &self,
        start_time: &mut f64,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_start_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(start_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_source_timeline_animation_start_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(start_time);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_skeleton_node_uid(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_skeleton_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_skeleton_node_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_animation_stop_time(&self, stop_time: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_animation_stop_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(stop_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_animation_stop_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(stop_time);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_animation_start_time(&self, start_time: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_animation_start_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(start_time, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_animation_start_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(start_time);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_animation_sample_rate(&self, sample_rate: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_animation_sample_rate,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(sample_rate, __buffer.add(0).cast::<f64>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_skeletal_animation_track_node_get_custom_animation_sample_rate,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(sample_rate);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeAudioSoundWaveNode {
    __padding_end: [u8; 112],
}
impl UInterchangeAudioSoundWaveNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeAudioSoundWaveNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGroomNode {
    __padding_end: [u8; 192],
}
impl UInterchangeGroomNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGroomNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_payload_key(
        &mut self,
        payload_key: FString,
        pay_load_type: EInterchangeGroomPayLoadType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_payload_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pay_load_type,
                __buffer.add(16).cast::<EInterchangeGroomPayLoadType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_payload_key,
                __buffer,
            )
        };
    }
    pub fn set_custom_start_frame(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_start_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_num_frames(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_num_frames,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_num_frames,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_groom_cache_attributes(
        &mut self,
        attribute_value: &EInterchangeGroomCacheAttributes,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_groom_cache_attributes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeGroomCacheAttributes>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_groom_cache_attributes,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_frame_rate(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_frame_rate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_frame_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_end_frame(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_end_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_set_custom_end_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_start_frame(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_start_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_start_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_num_frames(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_num_frames,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_num_frames,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_groom_cache_attributes(
        &self,
        attribute_value: &mut EInterchangeGroomCacheAttributes,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_groom_cache_attributes,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeGroomCacheAttributes>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_groom_cache_attributes,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<EInterchangeGroomCacheAttributes>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_frame_rate(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_frame_rate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_frame_rate,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_end_frame(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_end_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_groom_node_get_custom_end_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeBaseLightNode {
    __padding_end: [u8; 176],
}
impl UInterchangeBaseLightNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeBaseLightNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_use_temperature(&mut self, attribute_value: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_use_temperature,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_use_temperature,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_temperature(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_temperature,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_temperature,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_light_color(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_light_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_light_color,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_intensity(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_intensity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_set_custom_intensity,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_use_temperature(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_use_temperature,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_use_temperature,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_temperature(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_temperature,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_temperature,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_light_color(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_light_color,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_light_color,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_intensity(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_intensity,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_base_light_node_get_custom_intensity,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeLightNode {
    __padding_end: [u8; 272],
}
impl UInterchangeLightNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeLightNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_use_ies_brightness(
        &mut self,
        attribute_value: &bool,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<3>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_use_ies_brightness,
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
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_use_ies_brightness,
                __buffer,
            )
        };
        unsafe { __buffer.add(2).cast::<bool>().read() }
    }
    pub fn set_custom_rotation(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FRotator,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<26>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_rotation,
                __buffer,
            )
        };
        unsafe { __buffer.add(25).cast::<bool>().read() }
    }
    pub fn set_custom_intensity_units(
        &mut self,
        attribute_value: &EInterchangeLightUnits,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_intensity_units,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeLightUnits>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_intensity_units,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_ies_texture(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_ies_texture,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_ies_texture,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_ies_brightness_scale(
        &mut self,
        attribute_value: &f32,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_ies_brightness_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_ies_brightness_scale,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn set_custom_attenuation_radius(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_attenuation_radius,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_set_custom_attenuation_radius,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_use_ies_brightness(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_use_ies_brightness,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_use_ies_brightness,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_rotation(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FRotator,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_rotation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FRotator>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_rotation,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FRotator>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn get_custom_intensity_units(
        &self,
        attribute_value: &mut EInterchangeLightUnits,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_intensity_units,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeLightUnits>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_intensity_units,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EInterchangeLightUnits>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_ies_texture(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_ies_texture,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_ies_texture,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_ies_brightness_scale(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_ies_brightness_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_ies_brightness_scale,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_attenuation_radius(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_attenuation_radius,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_light_node_get_custom_attenuation_radius,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangePointLightNode {
    __padding_end: [u8; 304],
}
impl UInterchangePointLightNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangePointLightNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_use_inverse_squared_falloff(
        &mut self,
        attribute_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_set_custom_use_inverse_squared_falloff,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_set_custom_use_inverse_squared_falloff,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_light_falloff_exponent(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_set_custom_light_falloff_exponent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_set_custom_light_falloff_exponent,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_use_inverse_squared_falloff(
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_get_custom_use_inverse_squared_falloff,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_get_custom_use_inverse_squared_falloff,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_light_falloff_exponent(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_get_custom_light_falloff_exponent,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_point_light_node_get_custom_light_falloff_exponent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSpotLightNode {
    __padding_end: [u8; 336],
}
impl UInterchangeSpotLightNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSpotLightNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_outer_cone_angle(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_set_custom_outer_cone_angle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_set_custom_outer_cone_angle,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_inner_cone_angle(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_set_custom_inner_cone_angle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_set_custom_inner_cone_angle,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_outer_cone_angle(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_get_custom_outer_cone_angle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_get_custom_outer_cone_angle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_inner_cone_angle(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_get_custom_inner_cone_angle,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_spot_light_node_get_custom_inner_cone_angle,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeRectLightNode {
    __padding_end: [u8; 304],
}
impl UInterchangeRectLightNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeRectLightNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_source_width(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_set_custom_source_width,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_set_custom_source_width,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_source_height(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_set_custom_source_height,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_set_custom_source_height,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_source_width(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_get_custom_source_width,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_get_custom_source_width,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_source_height(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_get_custom_source_height,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_rect_light_node_get_custom_source_height,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeDirectionalLightNode {
    __padding_end: [u8; 176],
}
impl UInterchangeDirectionalLightNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeDirectionalLightNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSkyLightNode {
    __padding_end: [u8; 208],
}
impl UInterchangeSkyLightNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSkyLightNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_source_type(
        &mut self,
        source_type: EInterchangeSkyLightSourceType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_set_custom_source_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_type,
                __buffer.add(0).cast::<EInterchangeSkyLightSourceType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_set_custom_source_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_cubemap_dependency(
        &mut self,
        texture_cube_node_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_set_custom_cubemap_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture_cube_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_set_custom_cubemap_dependency,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_source_type(
        &self,
        source_type: &mut EInterchangeSkyLightSourceType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_get_custom_source_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                source_type,
                __buffer.add(0).cast::<EInterchangeSkyLightSourceType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_get_custom_source_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EInterchangeSkyLightSourceType>().swap(source_type);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_cubemap_dependency(
        &self,
        texture_cube_node_uid: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_get_custom_cubemap_dependency,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                texture_cube_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_sky_light_node_get_custom_cubemap_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(texture_cube_node_uid);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeMaterialInstanceNode {
    __padding_end: [u8; 144],
}
impl UInterchangeMaterialInstanceNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeMaterialInstanceNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_parent(&self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_set_custom_parent,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_set_custom_parent,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_blend_mode(&mut self, attribute_value: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_set_custom_blend_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_set_custom_blend_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_vector_parameter_value(
        &self,
        parameter_name: FString,
        attribute_value: &mut crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FLinearColor>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_texture_parameter_value(
        &self,
        parameter_name: FString,
        attribute_value: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_static_switch_parameter_value(
        &self,
        parameter_name: FString,
        attribute_value: &mut bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn get_scalar_parameter_value(
        &self,
        parameter_name: FString,
        attribute_value: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_custom_parent(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_custom_parent,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_custom_parent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_blend_mode(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_custom_blend_mode,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_get_custom_blend_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn add_vector_parameter_value(
        &mut self,
        parameter_name: FString,
        attribute_value: &crate::bindings::core_u_object::FLinearColor,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_vector_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_vector_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_texture_parameter_value(
        &mut self,
        parameter_name: FString,
        attribute_value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_texture_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_texture_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn add_static_switch_parameter_value(
        &mut self,
        parameter_name: FString,
        attribute_value: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_static_switch_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn add_scalar_parameter_value(
        &mut self,
        parameter_name: FString,
        attribute_value: f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &parameter_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(16).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_instance_node_add_scalar_parameter_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeMaterialReferenceNode {
    __padding_end: [u8; 128],
}
impl UInterchangeMaterialReferenceNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeMaterialReferenceNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_content_path(&self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_reference_node_set_custom_content_path,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_reference_node_set_custom_content_path,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_content_path(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_reference_node_get_custom_content_path,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_material_reference_node_get_custom_content_path,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeMeshLODContainerNode {
    __padding_end: [u8; 144],
}
impl UInterchangeMeshLODContainerNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeMeshLODContainerNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn reset_mesh_lod_node_uids(&mut self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_reset_mesh_lod_node_uids,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_reset_mesh_lod_node_uids,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn remove_mesh_lod_node_uid(&mut self, mesh_lod_node_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_remove_mesh_lod_node_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_lod_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_remove_mesh_lod_node_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_mesh_lod_node_uids(&self, out_mesh_lod_node_uid: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_get_mesh_lod_node_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_mesh_lod_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_get_mesh_lod_node_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_mesh_lod_node_uid);
        }
    }
    pub fn add_mesh_lod_node_uid(&mut self, mesh_lod_node_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_add_mesh_lod_node_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_lod_node_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_lod_container_node_add_mesh_lod_node_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeMeshNode {
    __padding_end: [u8; 552],
}
impl UInterchangeMeshNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeMeshNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_slot_material_dependency_uid(
        &mut self,
        slot_name: FString,
        material_dependency_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_skinned_mesh(&mut self, b_is_skinned_mesh: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_skinned_mesh,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_skinned_mesh,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_skinned_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_skeleton_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_skeleton_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_skeleton_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_scene_instance_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_scene_instance_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_scene_instance_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_pay_load_key(
        &mut self,
        pay_load_key: FString,
        pay_load_type: &EInterchangeMeshPayLoadType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_pay_load_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pay_load_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                pay_load_type,
                __buffer.add(16).cast::<EInterchangeMeshPayLoadType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_pay_load_key,
                __buffer,
            )
        };
    }
    pub fn set_morph_target_name(&mut self, morph_target_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_morph_target_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &morph_target_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_morph_target_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_morph_target_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_morph_target_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_morph_target_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_morph_target(&mut self, b_is_morph_target: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_morph_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_is_morph_target,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_morph_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_vertex_count(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_vertex_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_vertex_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_uv_count(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_uv_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_uv_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_polygon_count(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_polygon_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_polygon_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_has_vertex_tangent(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_tangent,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_tangent,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_has_vertex_normal(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_normal,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_normal,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_has_vertex_color(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_color,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_color,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_has_vertex_binormal(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_binormal,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_vertex_binormal,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_has_smooth_group(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_smooth_group,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_has_smooth_group,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_collision_type(
        &mut self,
        attribute_value: EInterchangeMeshCollision,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_collision_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
                __buffer.add(0).cast::<EInterchangeMeshCollision>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_collision_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_bounding_box(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FBox,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_bounding_box,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_set_custom_bounding_box,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn remove_slot_material_dependency_uid(&mut self, slot_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_skeleton_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_skeleton_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_skeleton_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_scene_instance_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_scene_instance_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_scene_instance_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_morph_target_dependency_uid(
        &mut self,
        dependency_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_morph_target_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_morph_target_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_assembly_part_dependency_uid(
        &mut self,
        dependency_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_assembly_part_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_remove_assembly_part_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_skinned_mesh(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_is_skinned_mesh,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_is_skinned_mesh,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn is_morph_target(&self) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<1>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_is_morph_target,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_is_morph_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<bool>().read() }
    }
    pub fn get_slot_material_dependency_uid(
        &self,
        slot_name: FString,
        out_material_dependency: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_material_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(out_material_dependency);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_slot_material_dependencies(
        &self,
        out_material_dependencies: &mut TMap<FString, FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_slot_material_dependencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_material_dependencies,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_slot_material_dependencies,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TMap<FString, FString>>()
                .swap(out_material_dependencies);
        }
    }
    pub fn get_skeleton_dependency(&self, index: i32, out_dependency: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_skeleton_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_skeleton_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency);
        }
    }
    pub fn get_skeleton_dependencies(&self, out_dependencies: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_skeleton_dependencies,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_skeleton_dependencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependencies);
        }
    }
    pub fn get_skeleton_dependecies_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_skeleton_dependecies_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_skeleton_dependecies_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_scene_instance_uids_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_scene_instance_uids_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_scene_instance_uids_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_scene_instance_uids(&self, out_dependencies: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_scene_instance_uids,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_scene_instance_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependencies);
        }
    }
    pub fn get_scene_instance_uid(&self, index: i32, out_dependency: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_scene_instance_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_scene_instance_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency);
        }
    }
    pub fn get_morph_target_name(&self, out_morph_target_name: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_morph_target_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(out_morph_target_name);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_morph_target_dependency(&self, index: i32, out_dependency: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency);
        }
    }
    pub fn get_morph_target_dependencies(&self, out_dependencies: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_dependencies,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_dependencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependencies);
        }
    }
    pub fn get_morph_target_dependecies_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_dependecies_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_morph_target_dependecies_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_custom_vertex_count(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_vertex_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_vertex_count,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_uv_count(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_uv_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_uv_count,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_polygon_count(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_polygon_count,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_polygon_count,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_has_vertex_tangent(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_tangent,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_tangent,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_has_vertex_normal(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_normal,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_normal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_has_vertex_color(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_color,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_color,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_has_vertex_binormal(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_binormal,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_vertex_binormal,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_has_smooth_group(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_smooth_group,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_has_smooth_group,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_collision_type(
        &self,
        attribute_value: &mut EInterchangeMeshCollision,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_collision_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeMeshCollision>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_collision_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EInterchangeMeshCollision>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_bounding_box(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FBox,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_bounding_box,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_custom_bounding_box,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FBox>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn get_assembly_part_dependency(
        &self,
        index: i32,
        out_dependency: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_assembly_part_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_assembly_part_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency);
        }
    }
    pub fn get_assembly_part_dependencies_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_assembly_part_dependencies_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_assembly_part_dependencies_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_assembly_part_dependencies(
        &self,
        out_dependencies: &mut TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_assembly_part_dependencies,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_get_assembly_part_dependencies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependencies);
        }
    }
    pub fn add_assembly_part_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_add_assembly_part_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_mesh_node_add_assembly_part_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeGeometryCacheNode {
    __padding_end: [u8; 616],
}
impl UInterchangeGeometryCacheNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeGeometryCacheNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_start_frame(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_start_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_start_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_has_constant_topology(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_has_constant_topology,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_has_constant_topology,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_frame_rate(&mut self, attribute_value: &f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_frame_rate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_frame_rate,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn set_custom_end_frame(&mut self, attribute_value: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_end_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_set_custom_end_frame,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_start_frame(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_start_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_start_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_has_constant_topology(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_has_constant_topology,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_has_constant_topology,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_frame_rate(&self, attribute_value: &mut f64) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_frame_rate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_frame_rate,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f64>().swap(attribute_value);
        }
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_custom_end_frame(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_end_frame,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_geometry_cache_node_get_custom_end_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct UInterchangeSceneComponentNode {
    __padding_end: [u8; 304],
}
impl UInterchangeSceneComponentNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSceneComponentNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_local_transform(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_set_custom_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_set_custom_local_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn set_custom_component_visibility(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_set_custom_component_visibility,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_set_custom_component_visibility,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_parent_scene_node_and_transform(
        &self,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        scene_node_transform: &mut crate::bindings::core_u_object::FTransform,
        b_force_recache: bool,
    ) -> UPtr<UInterchangeSceneNode> {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_parent_scene_node_and_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                scene_node_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_recache,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_parent_scene_node_and_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(scene_node_transform);
        }
        unsafe { __buffer.add(120).cast::<UPtr<UInterchangeSceneNode>>().read() }
    }
    pub fn get_custom_local_transform(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_custom_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_custom_local_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_component_visibility(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_custom_component_visibility,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_custom_component_visibility,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_component_uids(&self, out_component_uids: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_component_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_component_uids,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_get_component_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_component_uids);
        }
    }
    pub fn add_component_uid(&mut self, component_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_add_component_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_component_node_add_component_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(16))]
pub struct UInterchangeInstancedStaticMeshComponentNode {
    __padding_end: [u8; 352],
}
impl UInterchangeInstancedStaticMeshComponentNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeInstancedStaticMeshComponentNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_instanced_asset_uid(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_set_custom_instanced_asset_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_set_custom_instanced_asset_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_instance_transforms(
        &self,
        out_instance_transforms: &mut TArray<crate::bindings::core_u_object::FTransform>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_get_instance_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_instance_transforms,
                __buffer
                    .add(0)
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_get_instance_transforms,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .swap(out_instance_transforms);
        }
    }
    pub fn get_custom_instanced_asset_uid(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_get_custom_instanced_asset_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_get_custom_instanced_asset_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_instance_transforms(
        &mut self,
        in_instance_transforms: &TArray<crate::bindings::core_u_object::FTransform>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_add_instance_transforms,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_instance_transforms,
                __buffer
                    .add(0)
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_add_instance_transforms,
                __buffer,
            )
        };
    }
    pub fn add_instance_transform(
        &mut self,
        instance_transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<96>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_add_instance_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                instance_transform,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_instanced_static_mesh_component_node_add_instance_transform,
                __buffer,
            )
        };
    }
}
#[repr(C, align(16))]
pub struct UInterchangeSceneNode {
    __padding_end: [u8; 1248],
}
impl UInterchangeSceneNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSceneNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_slot_material_dependency_uid(
        &mut self,
        slot_name: FString,
        material_dependency_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &material_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn set_morph_target_curve_weight(
        &mut self,
        morph_target_name: FString,
        weight: &f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_morph_target_curve_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &morph_target_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(weight, __buffer.add(16).cast::<f32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_morph_target_curve_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn set_global_bind_pose_reference_for_mesh_ui_ds(
        &mut self,
        global_bind_pose_reference_for_mesh_ui_ds: &TMap<
            FString,
            crate::bindings::core_u_object::FMatrix,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_global_bind_pose_reference_for_mesh_ui_ds,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                global_bind_pose_reference_for_mesh_ui_ds,
                __buffer
                    .add(0)
                    .cast::<TMap<FString, crate::bindings::core_u_object::FMatrix>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_global_bind_pose_reference_for_mesh_ui_ds,
                __buffer,
            )
        };
    }
    pub fn set_custom_time_zero_local_transform(
        &mut self,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        attribute_value: &crate::bindings::core_u_object::FTransform,
        b_reset_cache: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_time_zero_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_cache,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_time_zero_local_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(113).cast::<bool>().read() }
    }
    pub fn set_custom_pivot_node_transform(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_pivot_node_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_pivot_node_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn set_custom_local_transform(
        &mut self,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        attribute_value: &crate::bindings::core_u_object::FTransform,
        b_reset_cache: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_cache,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_local_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(113).cast::<bool>().read() }
    }
    pub fn set_custom_is_scene_root(&mut self, is_scene_root: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_is_scene_root,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                is_scene_root,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_is_scene_root,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_has_bind_pose(&mut self, b_has_bind_pose: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_has_bind_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_has_bind_pose,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_has_bind_pose,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_global_matrix_for_t0_rebinding(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FMatrix,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<129>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_global_matrix_for_t0_rebinding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_global_matrix_for_t0_rebinding,
                __buffer,
            )
        };
        unsafe { __buffer.add(128).cast::<bool>().read() }
    }
    pub fn set_custom_geometric_transform(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_geometric_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_geometric_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn set_custom_component_visibility(&mut self, b_in_is_visible: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_component_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_visible,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_component_visibility,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_bind_pose_local_transform(
        &mut self,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        attribute_value: &crate::bindings::core_u_object::FTransform,
        b_reset_cache: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<114>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_bind_pose_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_reset_cache,
                __buffer.add(112).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_bind_pose_local_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(113).cast::<bool>().read() }
    }
    pub fn set_custom_asset_instance_uid(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_asset_instance_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_asset_instance_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_animation_asset_uid_to_play(
        &mut self,
        attribute_value: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_animation_asset_uid_to_play,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_animation_asset_uid_to_play,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_actor_visibility(&mut self, b_in_is_visible: bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_actor_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_in_is_visible,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_custom_actor_visibility,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_animation_curve_type_for_curve_name(
        &mut self,
        curve_name: FString,
        animation_curve_type: &EInterchangeAnimationPayLoadType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_animation_curve_type_for_curve_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_curve_type,
                __buffer.add(16).cast::<EInterchangeAnimationPayLoadType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_set_animation_curve_type_for_curve_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn remove_specialized_type(&mut self, specialized_type: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_remove_specialized_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &specialized_type,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_remove_specialized_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_slot_material_dependency_uid(&mut self, slot_name: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_remove_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_remove_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_specialized_type_contains(&self, specialized_type: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_is_specialized_type_contains,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &specialized_type,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_is_specialized_type_contains,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_specialized_types(&self, out_specialized_types: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_specialized_types,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_specialized_types,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_specialized_types,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_specialized_types);
        }
    }
    pub fn get_specialized_type_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_specialized_type_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_specialized_type_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_specialized_type(&self, index: i32, out_specialized_type: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_specialized_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_specialized_type,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_specialized_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_specialized_type);
        }
    }
    pub fn get_slot_material_dependency_uid(
        &self,
        slot_name: FString,
        out_material_dependency: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &slot_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_material_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_slot_material_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<FString>().swap(out_material_dependency);
        }
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn get_slot_material_dependencies(
        &self,
        out_material_dependencies: &mut TMap<FString, FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_slot_material_dependencies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_material_dependencies,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_slot_material_dependencies,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TMap<FString, FString>>()
                .swap(out_material_dependencies);
        }
    }
    pub fn get_morph_target_curve_weights(
        &self,
        out_morph_target_curve_weights: &mut TMap<FString, f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_morph_target_curve_weights,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_morph_target_curve_weights,
                __buffer.add(0).cast::<TMap<FString, f32>>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_morph_target_curve_weights,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<TMap<FString, f32>>()
                .swap(out_morph_target_curve_weights);
        }
    }
    pub fn get_global_bind_pose_reference_for_mesh_uid(
        &self,
        mesh_uid: FString,
        global_bind_pose_reference: &mut crate::bindings::core_u_object::FMatrix,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<145>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_global_bind_pose_reference_for_mesh_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &mesh_uid,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                global_bind_pose_reference,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_global_bind_pose_reference_for_mesh_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<crate::bindings::core_u_object::FMatrix>()
                .swap(global_bind_pose_reference);
        }
        unsafe { __buffer.add(144).cast::<bool>().read() }
    }
    pub fn get_custom_time_zero_local_transform(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_time_zero_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_time_zero_local_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_time_zero_global_transform(
        &self,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        global_offset_transform: &crate::bindings::core_u_object::FTransform,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
        b_force_recache: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<210>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_time_zero_global_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                global_offset_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(112).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_recache,
                __buffer.add(208).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_time_zero_global_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(112)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(209).cast::<bool>().read() }
    }
    pub fn get_custom_pivot_node_transform(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_pivot_node_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_pivot_node_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_local_transform(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_local_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_is_scene_root(&self, is_scene_root: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_is_scene_root,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                is_scene_root,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_is_scene_root,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(is_scene_root);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_has_bind_pose(&self, b_has_bind_pose: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_has_bind_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_has_bind_pose,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_has_bind_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(b_has_bind_pose);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_global_transform(
        &self,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        global_offset_transform: &crate::bindings::core_u_object::FTransform,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
        b_force_recache: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<210>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_global_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                global_offset_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(112).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_recache,
                __buffer.add(208).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_global_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(112)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(209).cast::<bool>().read() }
    }
    pub fn get_custom_global_matrix_for_t0_rebinding(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FMatrix,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<129>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_global_matrix_for_t0_rebinding,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FMatrix>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_global_matrix_for_t0_rebinding,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FMatrix>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(128).cast::<bool>().read() }
    }
    pub fn get_custom_geometric_transform(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_geometric_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_geometric_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_component_visibility(&self, b_out_is_visible: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_component_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_visible,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_component_visibility,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(b_out_is_visible);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_bind_pose_local_transform(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_bind_pose_local_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_bind_pose_local_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_bind_pose_global_transform(
        &self,
        base_node_container: UPtr<
            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
        >,
        global_offset_transform: &crate::bindings::core_u_object::FTransform,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
        b_force_recache: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<210>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_bind_pose_global_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &base_node_container,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<
                            crate::bindings::interchange_core::UInterchangeBaseNodeContainer,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                global_offset_transform,
                __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(112).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_force_recache,
                __buffer.add(208).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_bind_pose_global_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(112)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(209).cast::<bool>().read() }
    }
    pub fn get_custom_asset_instance_uid(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_asset_instance_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_asset_instance_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_animation_asset_uid_to_play(
        &self,
        attribute_value: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_animation_asset_uid_to_play,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_animation_asset_uid_to_play,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_actor_visibility(&self, b_out_is_visible: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_actor_visibility,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                b_out_is_visible,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_custom_actor_visibility,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(b_out_is_visible);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_component_uids(&self, out_component_uids: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_component_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_component_uids,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_component_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_component_uids);
        }
    }
    pub fn get_animation_curve_type_for_curve_name(
        &self,
        curve_name: FString,
        out_curve_animation_type: &mut EInterchangeAnimationPayLoadType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<18>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_animation_curve_type_for_curve_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_curve_animation_type,
                __buffer.add(16).cast::<EInterchangeAnimationPayLoadType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_get_animation_curve_type_for_curve_name,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<EInterchangeAnimationPayLoadType>()
                .swap(out_curve_animation_type);
        }
        unsafe { __buffer.add(17).cast::<bool>().read() }
    }
    pub fn add_specialized_type(&mut self, specialized_type: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_add_specialized_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &specialized_type,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_add_specialized_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_component_uid(&mut self, component_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_add_component_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &component_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_node_add_component_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeShaderPortsAPI {
    __padding_end: [u8; 48],
}
impl UInterchangeShaderPortsAPI {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeShaderPortsAPI")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn make_input_value_key(input_name: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_value_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_value_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn make_input_parameter_key(input_name: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_parameter_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_parameter_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn make_input_name(input_key: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn make_input_connection_key(input_name: FString) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_connection_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_make_input_connection_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<FString>().read() }
    }
    pub fn is_a_parameter(attribute_key: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_is_a_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_is_a_parameter,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn is_an_input(attribute_key: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_is_an_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_key,
                __buffer.add(0).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_is_an_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn has_parameter(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        in_input_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_has_parameter,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_has_parameter,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn has_input(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        in_input_name: &FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_has_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                in_input_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_has_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_input_connection(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        input_name: FString,
        out_expression_uid: &mut FString,
        output_name: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_get_input_connection,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_expression_uid,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_get_input_connection,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<FString>().swap(out_expression_uid);
        }
        unsafe {
            __buffer.add(40).cast::<FString>().swap(output_name);
        }
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn gather_inputs(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        out_input_names: &mut TArray<FString>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_gather_inputs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_input_names,
                __buffer.add(8).cast::<TArray<FString>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_gather_inputs,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FString>>().swap(out_input_names);
        }
    }
    pub fn disconnect_input_from_output_node(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        input_name: FString,
        out_expression_uid: &mut FString,
        output_name: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_disconnect_input_from_output_node,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_expression_uid,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                output_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_disconnect_input_from_output_node,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<FString>().swap(out_expression_uid);
        }
        unsafe {
            __buffer.add(40).cast::<FString>().swap(output_name);
        }
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn disconnect_input(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        input_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_disconnect_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_disconnect_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<bool>().read() }
    }
    pub fn connect_ouput_to_input_by_name(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        input_name: FString,
        expression_uid: FString,
        output_name: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<57>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_connect_ouput_to_input_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression_uid,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_name,
                __buffer.add(40).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_connect_ouput_to_input_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(56).cast::<bool>().read() }
    }
    pub fn connect_ouput_to_input_by_index(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        input_name: FString,
        expression_uid: FString,
        output_index: i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<45>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_connect_ouput_to_input_by_index,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression_uid,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &output_index,
                __buffer.add(40).cast::<i32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_connect_ouput_to_input_by_index,
                __buffer,
            )
        };
        unsafe { __buffer.add(44).cast::<bool>().read() }
    }
    pub fn connect_default_ouput_to_input(
        interchange_node: UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
        input_name: FString,
        expression_uid: FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<41>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_connect_default_ouput_to_input,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interchange_node,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangeBaseNode>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &input_name,
                __buffer.add(8).cast::<FString>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &expression_uid,
                __buffer.add(24).cast::<FString>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_nodes::UInterchangeShaderPortsAPI::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_ports_api_connect_default_ouput_to_input,
                __buffer,
            )
        };
        unsafe { __buffer.add(40).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeFunctionCallShaderNode {
    __padding_end: [u8; 144],
}
impl UInterchangeFunctionCallShaderNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeFunctionCallShaderNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_material_function(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_function_call_shader_node_set_custom_material_function,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_function_call_shader_node_set_custom_material_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_material_function(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_function_call_shader_node_get_custom_material_function,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_function_call_shader_node_get_custom_material_function,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeShaderGraphNode {
    __padding_end: [u8; 240],
}
impl UInterchangeShaderGraphNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeShaderGraphNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_two_sided_transmission(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_two_sided_transmission,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_two_sided_transmission,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_two_sided(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_two_sided,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_two_sided,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_screen_space_reflections(
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_screen_space_reflections,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_screen_space_reflections,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_opacity_mask_clip_value(
        &mut self,
        attribute_value: &f32,
        b_add_apply_delegate: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<6>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_opacity_mask_clip_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_add_apply_delegate,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_opacity_mask_clip_value,
                __buffer,
            )
        };
        unsafe { __buffer.add(5).cast::<bool>().read() }
    }
    pub fn set_custom_is_a_shader_function(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_is_a_shader_function,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_is_a_shader_function,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_displacement_center_mode(&mut self, attribute_value: f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_displacement_center_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_displacement_center_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_blend_mode(&mut self, attribute_value: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_blend_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_set_custom_blend_mode,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_two_sided_transmission(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_two_sided_transmission,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_two_sided_transmission,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_two_sided(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_two_sided,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_two_sided,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_screen_space_reflections(
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_screen_space_reflections,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_screen_space_reflections,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_opacity_mask_clip_value(&self, attribute_value: &mut f32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_opacity_mask_clip_value,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_opacity_mask_clip_value,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_is_a_shader_function(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_is_a_shader_function,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_is_a_shader_function,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_displacement_center_mode(
        &self,
        attribute_value: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_displacement_center_mode,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_displacement_center_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<f32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_blend_mode(&self, attribute_value: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_blend_mode,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_shader_graph_node_get_custom_blend_mode,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(attribute_value);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSpecularProfileNode {
    __padding_end: [u8; 144],
}
impl UInterchangeSpecularProfileNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSpecularProfileNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_texture(&mut self, texture_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_set_custom_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &texture_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_set_custom_texture,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_format(&mut self, format: u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_set_custom_format,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&format, __buffer.add(0).cast::<u8>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_set_custom_format,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_texture(&self, texture_uid: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_get_custom_texture,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                texture_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_get_custom_texture,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(texture_uid);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_format(&self, format: &mut u8) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_get_custom_format,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(format, __buffer.add(0).cast::<u8>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_specular_profile_node_get_custom_format,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<u8>().swap(format);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTexture2DNode {
    __padding_end: [u8; 344],
}
impl UInterchangeTexture2DNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTexture2DNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_force_long_lat_cubemap(&mut self, attribute_value: &bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_set_force_long_lat_cubemap,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_set_force_long_lat_cubemap,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_wrap_v(
        &mut self,
        attribute_value: &EInterchangeTextureWrapMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_set_custom_wrap_v,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureWrapMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_set_custom_wrap_v,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn set_custom_wrap_u(
        &mut self,
        attribute_value: &EInterchangeTextureWrapMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_set_custom_wrap_u,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureWrapMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_set_custom_wrap_u,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_source_blocks(&self) -> TMap<i32, FString> {
        let mut __stack = crate::core_data::StackAlloc::<80>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_source_blocks,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_source_blocks,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<TMap<i32, FString>>().read() }
    }
    pub fn get_force_long_lat_cubemap(&self, attribute_value: &mut bool) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_force_long_lat_cubemap,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_force_long_lat_cubemap,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<bool>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_wrap_v(
        &self,
        attribute_value: &mut EInterchangeTextureWrapMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_custom_wrap_v,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureWrapMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_custom_wrap_v,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EInterchangeTextureWrapMode>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_wrap_u(
        &self,
        attribute_value: &mut EInterchangeTextureWrapMode,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_custom_wrap_u,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EInterchangeTextureWrapMode>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_texture2_d_node_get_custom_wrap_u,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EInterchangeTextureWrapMode>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTextureBlurNode {
    __padding_end: [u8; 344],
}
impl UInterchangeTextureBlurNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTextureBlurNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeVariantSetNode {
    __padding_end: [u8; 176],
}
impl UInterchangeVariantSetNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeVariantSetNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_variants_payload_key(&mut self, payload_key: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_set_custom_variants_payload_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &payload_key,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_set_custom_variants_payload_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_display_text(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_set_custom_display_text,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_set_custom_display_text,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_custom_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_remove_custom_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_remove_custom_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_variants_payload_key(&self, payload_key: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_variants_payload_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                payload_key,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_variants_payload_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(payload_key);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_display_text(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_display_text,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_display_text,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_dependency_uids(&self, out_dependency_uids: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_dependency_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dependency_uids,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_dependency_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependency_uids);
        }
    }
    pub fn get_custom_dependency_uid_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_dependency_uid_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_dependency_uid_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_custom_dependency_uid(
        &self,
        index: i32,
        out_dependency_uid: &mut FString,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_get_custom_dependency_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency_uid);
        }
    }
    pub fn add_custom_dependency_uid(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_add_custom_dependency_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_variant_set_node_add_custom_dependency_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeSceneVariantSetsNode {
    __padding_end: [u8; 144],
}
impl UInterchangeSceneVariantSetsNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeSceneVariantSetsNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn remove_custom_variant_set_uid(&mut self, variant_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_remove_custom_variant_set_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_remove_custom_variant_set_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_variant_set_uids(&self, out_variant_uids: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_get_custom_variant_set_uids,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_variant_uids,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_get_custom_variant_set_uids,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_variant_uids);
        }
    }
    pub fn get_custom_variant_set_uid_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_get_custom_variant_set_uid_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_get_custom_variant_set_uid_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_custom_variant_set_uid(&self, index: i32, out_variant_uid: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_get_custom_variant_set_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_variant_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_get_custom_variant_set_uid,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_variant_uid);
        }
    }
    pub fn add_custom_variant_set_uid(&mut self, variant_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_add_custom_variant_set_uid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variant_uid,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_scene_variant_sets_node_add_custom_variant_set_uid,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeVolumeNode {
    __padding_end: [u8; 208],
}
impl UInterchangeVolumeNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeVolumeNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_file_name(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_set_custom_file_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_set_custom_file_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn set_custom_animation_id(&mut self, attribute_value: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_set_custom_animation_id,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_set_custom_animation_id,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_custom_grid_dependency(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_remove_custom_grid_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_remove_custom_grid_dependency,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn remove_custom_frame_index_in_animation(&mut self, index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_remove_custom_frame_index_in_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_remove_custom_frame_index_in_animation,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_grid_dependency(&self, index: i32, out_dependency: &mut FString) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_grid_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_grid_dependency,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FString>().swap(out_dependency);
        }
    }
    pub fn get_custom_grid_dependecies_count(&self) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<4>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_grid_dependecies_count,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_grid_dependecies_count,
                __buffer,
            )
        };
        unsafe { __buffer.add(0).cast::<i32>().read() }
    }
    pub fn get_custom_grid_dependecies(&self, out_dependencies: &mut TArray<FString>) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_grid_dependecies,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_grid_dependecies,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<FString>>().swap(out_dependencies);
        }
    }
    pub fn get_custom_frame_indices_in_animation(
        &self,
        out_animation_indices: &mut TArray<i32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_frame_indices_in_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_animation_indices,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_frame_indices_in_animation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<TArray<i32>>().swap(out_animation_indices);
        }
    }
    pub fn get_custom_frame_index_in_animation(
        &self,
        index_index: i32,
        out_index: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_frame_index_in_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &index_index,
                __buffer.add(0).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(out_index, __buffer.add(4).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_frame_index_in_animation,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(4).cast::<i32>().swap(out_index);
        }
    }
    pub fn get_custom_file_name(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_file_name,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_file_name,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn get_custom_animation_id(&self, attribute_value: &mut FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_animation_id,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_get_custom_animation_id,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FString>().swap(attribute_value);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_custom_grid_dependency(&mut self, dependency_uid: FString) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_add_custom_grid_dependency,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_add_custom_grid_dependency,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_custom_frame_index_in_animation(&mut self, index: i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_add_custom_frame_index_in_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(&index, __buffer.add(0).cast::<i32>(), 1);
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_node_add_custom_frame_index_in_animation,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeVolumeGridNode {
    __padding_end: [u8; 208],
}
impl UInterchangeVolumeGridNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeVolumeGridNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_custom_num_components(&mut self, num_components: &i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_num_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                num_components,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_num_components,
                __buffer,
            )
        };
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn set_custom_grid_transform(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_transform,
                __buffer,
            )
        };
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn set_custom_grid_active_dimensions(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FIntVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_active_dimensions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FIntVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_active_dimensions,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_custom_grid_active_aabb_min(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FIntVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_active_aabb_min,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FIntVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_active_aabb_min,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_custom_grid_active_aabb_max(
        &mut self,
        attribute_value: &crate::bindings::core_u_object::FIntVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_active_aabb_max,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FIntVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_grid_active_aabb_max,
                __buffer,
            )
        };
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn set_custom_element_type(
        &mut self,
        attribute_value: &EVolumeGridElementType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_element_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EVolumeGridElementType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_set_custom_element_type,
                __buffer,
            )
        };
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
    pub fn get_custom_num_components(&self, num_components: &mut i32) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<5>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_num_components,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                num_components,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_num_components,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<i32>().swap(num_components);
        }
        unsafe { __buffer.add(4).cast::<bool>().read() }
    }
    pub fn get_custom_grid_transform(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FTransform,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<97>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
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
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(96).cast::<bool>().read() }
    }
    pub fn get_custom_grid_active_dimensions(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FIntVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_active_dimensions,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FIntVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_active_dimensions,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FIntVector>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_custom_grid_active_aabb_min(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FIntVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_active_aabb_min,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FIntVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_active_aabb_min,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FIntVector>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_custom_grid_active_aabb_max(
        &self,
        attribute_value: &mut crate::bindings::core_u_object::FIntVector,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_active_aabb_max,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<crate::bindings::core_u_object::FIntVector>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_grid_active_aabb_max,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(0)
                .cast::<crate::bindings::core_u_object::FIntVector>()
                .swap(attribute_value);
        }
        unsafe { __buffer.add(12).cast::<bool>().read() }
    }
    pub fn get_custom_element_type(
        &self,
        attribute_value: &mut EVolumeGridElementType,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<2>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_element_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_value,
                __buffer.add(0).cast::<EVolumeGridElementType>(),
                1,
            );
        }
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_nodes::__FUNCTION_PTRS
                    .u_interchange_volume_grid_node_get_custom_element_type,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<EVolumeGridElementType>().swap(attribute_value);
        }
        unsafe { __buffer.add(1).cast::<bool>().read() }
    }
}
#[repr(transparent)]
pub struct EInterchangeAnimationPayLoadType(pub u8);
impl EInterchangeAnimationPayLoadType {
    pub const NONE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        0,
    );
    pub const CURVE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        1,
    );
    pub const MORPHTARGETCURVE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        2,
    );
    pub const STEPCURVE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        3,
    );
    pub const BAKED: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        4,
    );
    pub const MORPHTARGETCURVEWEIGHTINSTANCE: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        5,
    );
    pub const GEOMETRY_CACHE_TRANSFORM: EInterchangeAnimationPayLoadType = EInterchangeAnimationPayLoadType(
        6,
    );
}
#[repr(transparent)]
pub struct EInterchangeGroomPayLoadType(pub u8);
impl EInterchangeGroomPayLoadType {
    pub const STATIC: EInterchangeGroomPayLoadType = EInterchangeGroomPayLoadType(0);
    pub const ANIMATED: EInterchangeGroomPayLoadType = EInterchangeGroomPayLoadType(1);
}
#[repr(transparent)]
pub struct EInterchangeMeshPayLoadType(pub u8);
impl EInterchangeMeshPayLoadType {
    pub const NONE: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(0);
    pub const STATIC: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(1);
    pub const SKELETAL: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(2);
    pub const MORPHTARGET: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(3);
    pub const ANIMATED: EInterchangeMeshPayLoadType = EInterchangeMeshPayLoadType(4);
}
#[repr(transparent)]
pub struct EInterchangeCameraProjectionType(pub u8);
impl EInterchangeCameraProjectionType {
    pub const PERSPECTIVE: EInterchangeCameraProjectionType = EInterchangeCameraProjectionType(
        0,
    );
    pub const ORTHOGRAPHIC: EInterchangeCameraProjectionType = EInterchangeCameraProjectionType(
        1,
    );
}
#[repr(transparent)]
pub struct EInterchangeTextureColorSpace(pub u8);
impl EInterchangeTextureColorSpace {
    pub const TCS_NONE: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(0);
    pub const TCS_S_RGB: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        1,
    );
    pub const TCS_REC2020: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        2,
    );
    pub const TCS_ACESAP0: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        3,
    );
    pub const TCS_ACESAP1: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        4,
    );
    pub const TCS_P3DCI: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        5,
    );
    pub const TCS_P3D65: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        6,
    );
    pub const TCS_RED_WIDE_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        7,
    );
    pub const TCS_SONY_S_GAMUT3: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        8,
    );
    pub const TCS_SONY_S_GAMUT3_CINE: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        9,
    );
    pub const TCS_ALEXA_WIDE_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        10,
    );
    pub const TCS_CANON_CINEMA_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        11,
    );
    pub const TCS_GO_PRO_PROTUNE_NATIVE: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        12,
    );
    pub const TCS_PANASONIC_V_GAMUT: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        13,
    );
    pub const TCS_CUSTOM: EInterchangeTextureColorSpace = EInterchangeTextureColorSpace(
        99,
    );
}
#[repr(transparent)]
pub struct EInterchangeTextureFilterMode(pub u8);
impl EInterchangeTextureFilterMode {
    pub const NEAREST: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(0);
    pub const BILINEAR: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(1);
    pub const TRILINEAR: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(
        2,
    );
    pub const DEFAULT: EInterchangeTextureFilterMode = EInterchangeTextureFilterMode(3);
}
#[repr(transparent)]
pub struct EInterchangePropertyTracks(pub i32);
impl EInterchangePropertyTracks {
    pub const AFFECT_DISTANCE_FIELD_LIGHTING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        0,
    );
    pub const AFFECT_DYNAMIC_INDIRECT_LIGHTING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        1,
    );
    pub const AFFECT_INDIRECT_LIGHTING_WHILE_HIDDEN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        2,
    );
    pub const AUTO_ACTIVATE: EInterchangePropertyTracks = EInterchangePropertyTracks(3);
    pub const BODY_INSTANCE_ANGULAR_DAMPING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        4,
    );
    pub const BODY_INSTANCEB_ENABLE_GRAVITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        5,
    );
    pub const BODY_INSTANCEB_NOTIFY_RIGID_BODY_COLLISION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        6,
    );
    pub const BODY_INSTANCEB_SIMULATE_PHYSICS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        7,
    );
    pub const BODY_INSTANCEB_UPDATE_KINEMATIC_FROM_SIMULATION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        8,
    );
    pub const BODY_INSTANCEB_USE_CCD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        9,
    );
    pub const BODY_INSTANCE_LINEAR_DAMPING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        10,
    );
    pub const BODY_INSTANCE_MASS_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        11,
    );
    pub const BOUNDS_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(12);
    pub const CAST_CONTACT_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        13,
    );
    pub const CAST_HIDDEN_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        14,
    );
    pub const CAST_INSET_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        15,
    );
    pub const CAST_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(16);
    pub const CUSTOM_DEPTH_STENCIL_VALUE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        17,
    );
    pub const CUSTOM_DEPTH_STENCIL_WRITE_MASK: EInterchangePropertyTracks = EInterchangePropertyTracks(
        18,
    );
    pub const DEFAULT_UP_VECTOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        19,
    );
    pub const DRAW_DEBUG: EInterchangePropertyTracks = EInterchangePropertyTracks(20);
    pub const EMISSIVE_LIGHT_SOURCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        21,
    );
    pub const EXCLUDE_FROM_LIGHT_ATTACHMENT_GROUP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        22,
    );
    pub const HIDDEN_IN_GAME: EInterchangePropertyTracks = EInterchangePropertyTracks(
        23,
    );
    pub const HIDDEN_IN_SCENE_CAPTURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        24,
    );
    pub const HOLDOUT: EInterchangePropertyTracks = EInterchangePropertyTracks(25);
    pub const LIGHT_ATTACHMENTS_AS_GROUP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        26,
    );
    pub const MOBILITY: EInterchangePropertyTracks = EInterchangePropertyTracks(27);
    pub const ONLY_OWNER_SEE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        28,
    );
    pub const OWNER_NO_SEE: EInterchangePropertyTracks = EInterchangePropertyTracks(29);
    pub const RECEIVES_DECALS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        30,
    );
    pub const RENDER_CUSTOM_DEPTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        31,
    );
    pub const RENDER_IN_DEPTH_PASS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        32,
    );
    pub const RENDER_IN_MAIN_PASS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        33,
    );
    pub const SINGLE_SAMPLE_SHADOW_FROM_STATIONARY_LIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        34,
    );
    pub const TRANSLUCENCY_SORT_DISTANCE_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        35,
    );
    pub const VISIBLE_IN_RAY_TRACING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        36,
    );
    pub const VISIBLE_IN_SCENE_CAPTURE_ONLY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        37,
    );
    pub const ACTOR_HIDDEN_IN_GAME: EInterchangePropertyTracks = EInterchangePropertyTracks(
        38,
    );
    pub const LIGHT_AFFECT_GLOBAL_ILLUMINATION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        39,
    );
    pub const LIGHT_AFFECT_REFLECTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        40,
    );
    pub const LIGHT_AFFECT_TRANSLUCENT_LIGHTING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        41,
    );
    pub const LIGHT_ATMOSPHERE_SUN_DISK_COLOR_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        42,
    );
    pub const LIGHT_ATMOSPHERE_SUN_LIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        43,
    );
    pub const LIGHT_ATMOSPHERE_SUN_LIGHT_INDEX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        44,
    );
    pub const LIGHT_ATTENUATION_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        45,
    );
    pub const LIGHT_BARN_DOOR_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        46,
    );
    pub const LIGHT_BARN_DOOR_LENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        47,
    );
    pub const LIGHT_BLOOM_MAX_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        48,
    );
    pub const LIGHT_BLOOM_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        49,
    );
    pub const LIGHT_BLOOM_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        50,
    );
    pub const LIGHT_BLOOM_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        51,
    );
    pub const LIGHT_CASCADE_DISTRIBUTION_EXPONENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        52,
    );
    pub const LIGHT_CASCADE_TRANSITION_FRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        53,
    );
    pub const LIGHT_CAST_DEEP_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        54,
    );
    pub const LIGHT_CAST_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        55,
    );
    pub const LIGHT_CAST_VOLUMETRIC_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        56,
    );
    pub const LIGHT_CLOUD_AMBIENT_OCCLUSION_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        57,
    );
    pub const LIGHT_CLOUD_SCATTERED_LUMINANCE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        58,
    );
    pub const LIGHT_CLOUD_SHADOW_ON_ATMOSPHERE_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        59,
    );
    pub const LIGHT_CLOUD_SHADOW_ON_SURFACE_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        60,
    );
    pub const LIGHT_CLOUD_SHADOW_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        61,
    );
    pub const LIGHT_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(62);
    pub const LIGHT_DYNAMIC_SHADOW_CASCADES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        63,
    );
    pub const LIGHT_DYNAMIC_SHADOW_DISTANCE_MOVABLE_LIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        64,
    );
    pub const LIGHT_DYNAMIC_SHADOW_DISTANCE_STATIONARY_LIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        65,
    );
    pub const LIGHT_ENABLE_LIGHT_SHAFT_BLOOM: EInterchangePropertyTracks = EInterchangePropertyTracks(
        66,
    );
    pub const LIGHT_ENABLE_LIGHT_SHAFT_OCCLUSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        67,
    );
    pub const LIGHT_FALLOFF_EXPONENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        68,
    );
    pub const LIGHT_FORCE_CACHED_SHADOWS_FOR_MOVABLE_PRIMITIVES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        69,
    );
    pub const LIGHT_FORWARD_SHADING_PRIORITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        70,
    );
    pub const LIGHT_FUNCTION_FADE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        71,
    );
    pub const LIGHT_FUNCTION_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        72,
    );
    pub const LIGHT_IES_BRIGHTNESS_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        73,
    );
    pub const LIGHT_INDIRECT_LIGHTING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        74,
    );
    pub const LIGHT_INNER_CONE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        75,
    );
    pub const LIGHT_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        76,
    );
    pub const LIGHT_INTENSITY_UNITS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        77,
    );
    pub const LIGHT_INVERSE_EXPOSURE_BLEND: EInterchangePropertyTracks = EInterchangePropertyTracks(
        78,
    );
    pub const LIGHT_LOWER_HEMISPHERE_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        79,
    );
    pub const LIGHTMASS_SETTINGS_LIGHT_SOURCE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        80,
    );
    pub const LIGHT_MIN_OCCLUSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        81,
    );
    pub const LIGHT_MODULATED_SHADOW_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        82,
    );
    pub const LIGHT_OCCLUSION_DEPTH_RANGE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        83,
    );
    pub const LIGHT_OCCLUSION_EXPONENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        84,
    );
    pub const LIGHT_OCCLUSION_MASK_DARKNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        85,
    );
    pub const LIGHT_OCCLUSION_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        86,
    );
    pub const LIGHT_OUTER_CONE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        87,
    );
    pub const LIGHT_SAMPLES_PER_PIXEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        88,
    );
    pub const LIGHT_SHADOW_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        89,
    );
    pub const LIGHT_SHADOW_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        90,
    );
    pub const LIGHT_SHADOW_CASCADE_BIAS_DISTRIBUTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        91,
    );
    pub const LIGHT_SHADOW_DISTANCE_FADEOUT_FRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        92,
    );
    pub const LIGHT_SHADOW_SLOPE_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        93,
    );
    pub const LIGHT_SHADOW_SOURCE_ANGLE_FACTOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        94,
    );
    pub const LIGHT_SHAFT_OVERRIDE_DIRECTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        95,
    );
    pub const LIGHT_SOFT_SOURCE_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        96,
    );
    pub const LIGHT_SOURCE_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        97,
    );
    pub const LIGHT_SOURCE_CUBEMAP_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        98,
    );
    pub const LIGHT_SOURCE_HEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        99,
    );
    pub const LIGHT_SOURCE_LENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        100,
    );
    pub const LIGHT_SOURCE_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        101,
    );
    pub const LIGHT_SOURCE_SOFT_ANGLE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        102,
    );
    pub const LIGHT_SOURCE_WIDTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        103,
    );
    pub const LIGHT_SPECULAR_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        104,
    );
    pub const LIGHT_DIFFUSE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        105,
    );
    pub const LIGHT_TEMPERATURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        106,
    );
    pub const LIGHT_TRANSMISSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        107,
    );
    pub const LIGHT_USE_IES_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        108,
    );
    pub const LIGHT_USE_INVERSE_SQUARED_FALLOFF: EInterchangePropertyTracks = EInterchangePropertyTracks(
        109,
    );
    pub const LIGHT_USE_TEMPERATURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        110,
    );
    pub const LIGHT_VOLUMETRIC_SCATTERING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        111,
    );
    pub const CAMERA_ASPECT_RATIO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        112,
    );
    pub const CAMERA_ASPECT_RATIO_AXIS_CONSTRAINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        113,
    );
    pub const CAMERA_AUTO_CALCULATE_ORTHO_PLANES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        114,
    );
    pub const CAMERA_AUTO_PLANE_SHIFT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        115,
    );
    pub const CAMERA_CONSTRAIN_ASPECT_RATIO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        116,
    );
    pub const CAMERA_CURRENT_APERTURE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        117,
    );
    pub const CAMERA_CURRENT_FOCAL_LENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        118,
    );
    pub const CAMERA_CUSTOM_NEAR_CLIPPING_PLANE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        119,
    );
    pub const CAMERA_FIELD_OF_VIEW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        120,
    );
    pub const CAMERA_FILMBACK_SENSOR_ASPECT_RATIO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        121,
    );
    pub const CAMERA_FILMBACK_SENSOR_HEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        122,
    );
    pub const CAMERA_FILMBACK_SENSOR_WIDTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        123,
    );
    pub const CAMERA_FOCUS_SETTINGS_FOCUS_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        124,
    );
    pub const CAMERA_FOCUS_SETTINGS_MANUAL_FOCUS_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        125,
    );
    pub const CAMERA_FOCUS_SETTINGS_TRACKING_FOCUS_SETTINGS_RELATIVE_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        126,
    );
    pub const CAMERA_ORTHO_FAR_CLIP_PLANE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        127,
    );
    pub const CAMERA_ORTHO_NEAR_CLIP_PLANE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        128,
    );
    pub const CAMERA_ORTHO_WIDTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        129,
    );
    pub const CAMERA_OVERRIDE_ASPECT_RATIO_AXIS_CONSTRAINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        130,
    );
    pub const CAMERA_POST_PROCESS_BLEND_WEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        131,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_CUBEMAP_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        132,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_CUBEMAP_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        133,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        134,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_FADE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        135,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_FADE_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        136,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        137,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_MIP_BLEND: EInterchangePropertyTracks = EInterchangePropertyTracks(
        138,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_MIP_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        139,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_MIP_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        140,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_POWER: EInterchangePropertyTracks = EInterchangePropertyTracks(
        141,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        142,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        143,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_STATIC_FRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        144,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AMBIENT_OCCLUSION_TEMPORAL_BLEND_WEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        145,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        146,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_HIGH_PERCENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        147,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_LOW_PERCENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        148,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_MAX_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        149,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_MIN_BRIGHTNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        150,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_SPEED_DOWN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        151,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_AUTO_EXPOSURE_SPEED_UP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        152,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM1_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        153,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM1_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        154,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM2_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        155,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM2_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        156,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM3_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        157,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM3_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        158,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM4_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        159,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM4_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        160,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM5_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        161,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM5_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        162,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM6_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        163,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM6_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        164,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_BUFFER_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        165,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_CENTER_UV: EInterchangePropertyTracks = EInterchangePropertyTracks(
        166,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_PRE_FILTER_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        167,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_PRE_FILTER_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        168,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_PRE_FILTER_MULT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        169,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_SCATTER_DISPERSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        170,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_CONVOLUTION_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        171,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_DIRT_MASK_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        172,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_DIRT_MASK_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        173,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        174,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_SIZE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        175,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLOOM_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        176,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_BLUE_CORRECTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        177,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_CHROMATIC_ABERRATION_START_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        178,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST: EInterchangePropertyTracks = EInterchangePropertyTracks(
        179,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        180,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        181,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CONTRAST_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        182,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CORRECTION_HIGHLIGHTS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        183,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CORRECTION_HIGHLIGHTS_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        184,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_CORRECTION_SHADOWS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        185,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        186,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        187,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        188,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAIN_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        189,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA: EInterchangePropertyTracks = EInterchangePropertyTracks(
        190,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        191,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        192,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GAMMA_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        193,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_GRADING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        194,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        195,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        196,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        197,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_OFFSET_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        198,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        199,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        200,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        201,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_COLOR_SATURATION_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        202,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_BLADE_COUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        203,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_DEPTH_BLUR_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        204,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_DEPTH_BLUR_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        205,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FAR_BLUR_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        206,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FAR_TRANSITION_REGION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        207,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FOCAL_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        208,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FOCAL_REGION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        209,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_FSTOP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        210,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_MIN_FSTOP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        211,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_NEAR_BLUR_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        212,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_NEAR_TRANSITION_REGION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        213,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_OCCLUSION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        214,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        215,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_SKY_FOCUS_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        216,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_USE_HAIR_DEPTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        217,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DEPTH_OF_FIELD_VIGNETTE_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        218,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_DYNAMIC_GLOBAL_ILLUMINATION_METHOD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        219,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_EXPAND_GAMUT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        220,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_BLACK_CLIP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        221,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_HIGHLIGHTS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        222,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_HIGHLIGHTS_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        223,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        224,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY_HIGHLIGHTS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        225,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY_MIDTONES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        226,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_INTENSITY_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        227,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_SHADOWS_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        228,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_GRAIN_TEXEL_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        229,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_SHOULDER: EInterchangePropertyTracks = EInterchangePropertyTracks(
        230,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_SLOPE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        231,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_TOE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        232,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_FILM_WHITE_CLIP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        233,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_HISTOGRAM_LOG_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        234,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_HISTOGRAM_LOG_MIN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        235,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_INDIRECT_LIGHTING_COLOR: EInterchangePropertyTracks = EInterchangePropertyTracks(
        236,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_INDIRECT_LIGHTING_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        237,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_BOKEH_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        238,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        239,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        240,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LENS_FLARE_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        241,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_BLURRED_LUMINANCE_BLEND: EInterchangePropertyTracks = EInterchangePropertyTracks(
        242,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_BLURRED_LUMINANCE_KERNEL_SIZE_PERCENT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        243,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_DETAIL_STRENGTH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        244,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_HIGHLIGHT_CONTRAST_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        245,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_HIGHLIGHT_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        246,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_MIDDLE_GREY_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        247,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_SHADOW_CONTRAST_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        248,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LOCAL_EXPOSURE_SHADOW_THRESHOLD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        249,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_DIFFUSE_COLOR_BOOST: EInterchangePropertyTracks = EInterchangePropertyTracks(
        250,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FINAL_GATHER_LIGHTING_UPDATE_SPEED: EInterchangePropertyTracks = EInterchangePropertyTracks(
        251,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FINAL_GATHER_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        252,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FINAL_GATHER_SCREEN_TRACES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        253,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FRONT_LAYER_TRANSLUCENCY_REFLECTIONS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        254,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_FULL_SKYLIGHT_LEAKING_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        255,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_REFLECTION_BOUNCES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        256,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_REFRACTION_BOUNCES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        257,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_ROUGHNESS_TO_TRACE_REFLECTIONS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        258,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_MAX_TRACE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        259,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_RAY_LIGHTING_MODE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        260,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_REFLECTION_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        261,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_REFLECTIONS_SCREEN_TRACES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        262,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_DETAIL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        263,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_LIGHTING_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        264,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_LIGHTING_UPDATE_SPEED: EInterchangePropertyTracks = EInterchangePropertyTracks(
        265,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SCENE_VIEW_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        266,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SKYLIGHT_LEAKING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        267,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_LUMEN_SKYLIGHT_LEAKING_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        268,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_MOTION_BLUR_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        269,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_MOTION_BLUR_MAX: EInterchangePropertyTracks = EInterchangePropertyTracks(
        270,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_MOTION_BLUR_PER_OBJECT_SIZE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        271,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_PATH_TRACING_MAX_BOUNCES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        272,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_PATH_TRACING_MAX_PATH_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        273,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO: EInterchangePropertyTracks = EInterchangePropertyTracks(
        274,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        275,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO_RADIUS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        276,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_AO_SAMPLES_PER_PIXEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        277,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_MAX_ROUGHNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        278,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_REFRACTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        279,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_REFRACTION_RAYS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        280,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_SAMPLES_PER_PIXEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        281,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_RAY_TRACING_TRANSLUCENCY_SHADOWS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        282,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_REFLECTION_METHOD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        283,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCENE_COLOR_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        284,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCENE_FRINGE_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        285,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCREEN_SPACE_REFLECTION_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        286,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCREEN_SPACE_REFLECTION_MAX_ROUGHNESS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        287,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SCREEN_SPACE_REFLECTION_QUALITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        288,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_SHARPEN: EInterchangePropertyTracks = EInterchangePropertyTracks(
        289,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_TEMPERATURE_TYPE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        290,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_TONE_CURVE_AMOUNT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        291,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_TRANSLUCENCY_TYPE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        292,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_VIGNETTE_INTENSITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        293,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_WHITE_TEMP: EInterchangePropertyTracks = EInterchangePropertyTracks(
        294,
    );
    pub const CAMERA_POST_PROCESS_SETTINGS_WHITE_TINT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        295,
    );
    pub const CAMERA_PROJECTION_MODE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        296,
    );
    pub const CAMERA_UPDATE_ORTHO_PLANES: EInterchangePropertyTracks = EInterchangePropertyTracks(
        297,
    );
    pub const CAMERA_USE_CAMERA_HEIGHT_AS_VIEW_TARGET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        298,
    );
    pub const CAMERA_USE_FIELD_OF_VIEW_FOR_LOD: EInterchangePropertyTracks = EInterchangePropertyTracks(
        299,
    );
    pub const MESH_OVERLAY_MATERIAL_MAX_DRAW_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        300,
    );
    pub const SKINNED_MESH_CAPSULE_INDIRECT_SHADOW_MIN_VISIBILITY: EInterchangePropertyTracks = EInterchangePropertyTracks(
        301,
    );
    pub const SKINNED_MESH_CAST_CAPSULE_DIRECT_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        302,
    );
    pub const SKINNED_MESH_CAST_CAPSULE_INDIRECT_SHADOW: EInterchangePropertyTracks = EInterchangePropertyTracks(
        303,
    );
    pub const SKINNED_MESH_RENDER_STATIC: EInterchangePropertyTracks = EInterchangePropertyTracks(
        304,
    );
    pub const SKINNED_MESH_VISIBILITY_BASED_ANIM_TICK_OPTION: EInterchangePropertyTracks = EInterchangePropertyTracks(
        305,
    );
    pub const SKELETAL_MESH: EInterchangePropertyTracks = EInterchangePropertyTracks(
        306,
    );
    pub const SKELETAL_MESH_ALLOW_CLOTH_ACTORS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        307,
    );
    pub const SKELETAL_MESH_ANIMATION_MODE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        308,
    );
    pub const SKELETAL_MESH_CLOTH_BLEND_WEIGHT: EInterchangePropertyTracks = EInterchangePropertyTracks(
        309,
    );
    pub const SKELETAL_MESH_CLOTH_MAX_DISTANCE_SCALE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        310,
    );
    pub const STATIC_MESH_DISTANCE_FIELD_SELF_SHADOW_BIAS: EInterchangePropertyTracks = EInterchangePropertyTracks(
        311,
    );
    pub const STATIC_MESH_EVALUATE_WORLD_POSITION_OFFSET: EInterchangePropertyTracks = EInterchangePropertyTracks(
        312,
    );
    pub const STATIC_MESH_EVALUATE_WORLD_POSITION_OFFSET_IN_RAY_TRACING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        313,
    );
    pub const STATIC_MESH_FORCED_LOD_MODEL: EInterchangePropertyTracks = EInterchangePropertyTracks(
        314,
    );
    pub const STATIC_MESH_REVERSE_CULLING: EInterchangePropertyTracks = EInterchangePropertyTracks(
        315,
    );
    pub const STATIC_MESH_WORLD_POSITION_OFFSET_DISABLE_DISTANCE: EInterchangePropertyTracks = EInterchangePropertyTracks(
        316,
    );
    pub const STATIC_MESH: EInterchangePropertyTracks = EInterchangePropertyTracks(317);
    pub const HETEROGENEOUS_VOLUME_FRAME: EInterchangePropertyTracks = EInterchangePropertyTracks(
        318,
    );
    pub const NONE: EInterchangePropertyTracks = EInterchangePropertyTracks(-1);
    pub const VISIBILITY: EInterchangePropertyTracks = EInterchangePropertyTracks(38);
}
#[repr(transparent)]
pub struct EInterchangeGroomCacheAttributes(pub u8);
impl EInterchangeGroomCacheAttributes {
    pub const NONE: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        0,
    );
    pub const POSITION: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        1,
    );
    pub const WIDTH: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        2,
    );
    pub const COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        4,
    );
    pub const POSITION_WIDTH: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        3,
    );
    pub const POSITION_COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        5,
    );
    pub const WIDTH_COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        6,
    );
    pub const POSITION_WIDTH_COLOR: EInterchangeGroomCacheAttributes = EInterchangeGroomCacheAttributes(
        7,
    );
}
#[repr(transparent)]
pub struct EInterchangeLightUnits(pub u8);
impl EInterchangeLightUnits {
    pub const UNITLESS: EInterchangeLightUnits = EInterchangeLightUnits(0);
    pub const CANDELAS: EInterchangeLightUnits = EInterchangeLightUnits(1);
    pub const LUMENS: EInterchangeLightUnits = EInterchangeLightUnits(2);
    pub const EV: EInterchangeLightUnits = EInterchangeLightUnits(3);
}
#[repr(transparent)]
pub struct EInterchangeSkyLightSourceType(pub u8);
impl EInterchangeSkyLightSourceType {
    pub const CAPTURED_SCENE: EInterchangeSkyLightSourceType = EInterchangeSkyLightSourceType(
        0,
    );
    pub const SPECIFIED_CUBEMAP: EInterchangeSkyLightSourceType = EInterchangeSkyLightSourceType(
        1,
    );
}
#[repr(transparent)]
pub struct EInterchangeMeshCollision(pub u8);
impl EInterchangeMeshCollision {
    pub const BOX: EInterchangeMeshCollision = EInterchangeMeshCollision(0);
    pub const SPHERE: EInterchangeMeshCollision = EInterchangeMeshCollision(1);
    pub const CAPSULE: EInterchangeMeshCollision = EInterchangeMeshCollision(2);
    pub const CONVEX10_DOP_X: EInterchangeMeshCollision = EInterchangeMeshCollision(3);
    pub const CONVEX10_DOP_Y: EInterchangeMeshCollision = EInterchangeMeshCollision(4);
    pub const CONVEX10_DOP_Z: EInterchangeMeshCollision = EInterchangeMeshCollision(5);
    pub const CONVEX18_DOP: EInterchangeMeshCollision = EInterchangeMeshCollision(6);
    pub const CONVEX26_DOP: EInterchangeMeshCollision = EInterchangeMeshCollision(7);
    pub const NONE: EInterchangeMeshCollision = EInterchangeMeshCollision(255);
}
#[repr(transparent)]
pub struct EInterchangeTextureWrapMode(pub u8);
impl EInterchangeTextureWrapMode {
    pub const WRAP: EInterchangeTextureWrapMode = EInterchangeTextureWrapMode(0);
    pub const CLAMP: EInterchangeTextureWrapMode = EInterchangeTextureWrapMode(1);
    pub const MIRROR: EInterchangeTextureWrapMode = EInterchangeTextureWrapMode(2);
}
#[repr(transparent)]
pub struct EVolumeGridElementType(pub u8);
impl EVolumeGridElementType {
    pub const UNKNOWN: EVolumeGridElementType = EVolumeGridElementType(0);
    pub const HALF: EVolumeGridElementType = EVolumeGridElementType(1);
    pub const FLOAT: EVolumeGridElementType = EVolumeGridElementType(2);
    pub const DOUBLE: EVolumeGridElementType = EVolumeGridElementType(3);
}
#[repr(transparent)]
pub struct EInterchangeMotionVectorsHandling(pub u8);
impl EInterchangeMotionVectorsHandling {
    pub const NO_MOTION_VECTORS: EInterchangeMotionVectorsHandling = EInterchangeMotionVectorsHandling(
        0,
    );
    pub const IMPORT_VELOCITIES_AS_MOTION_VECTORS: EInterchangeMotionVectorsHandling = EInterchangeMotionVectorsHandling(
        1,
    );
    pub const CALCULATE_MOTION_VECTORS_DURING_IMPORT: EInterchangeMotionVectorsHandling = EInterchangeMotionVectorsHandling(
        2,
    );
}
