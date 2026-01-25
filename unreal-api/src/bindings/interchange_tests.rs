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
    pub u_actor_import_test_functions_check_imported_actor_count: *mut crate::ffi::UFunctionOpague,
    pub u_actor_import_test_functions_check_component_property_value: *mut crate::ffi::UFunctionOpague,
    pub u_actor_import_test_functions_check_actor_property_value: *mut crate::ffi::UFunctionOpague,
    pub u_actor_import_test_functions_check_actor_class_count: *mut crate::ffi::UFunctionOpague,
    pub u_actor_import_test_functions_check_actor_class: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_imported_anim_sequence_count: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_curve_key_value: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_curve_key_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_curve_key_leave_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_curve_key_leave_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_curve_key_arrive_tangent_weight: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_curve_key_arrive_tangent: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_animation_length: *mut crate::ffi::UFunctionOpague,
    pub u_animation_import_test_functions_check_animation_frame_number: *mut crate::ffi::UFunctionOpague,
    pub u_asset_import_test_functions_check_object_path_has_substring: *mut crate::ffi::UFunctionOpague,
    pub u_asset_import_test_functions_check_metadata_value: *mut crate::ffi::UFunctionOpague,
    pub u_asset_import_test_functions_check_metadata_exist: *mut crate::ffi::UFunctionOpague,
    pub u_asset_import_test_functions_check_imported_metadata_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_result_import_test_functions_check_if_error_or_warning_was_generated: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_import_test_functions_check_sequence_length: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_import_test_functions_check_section_interpolation_mode: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_import_test_functions_check_section_count: *mut crate::ffi::UFunctionOpague,
    pub u_level_sequence_import_test_functions_check_level_sequence_count: *mut crate::ffi::UFunctionOpague,
    pub u_level_variant_sets_import_test_functions_check_variant_sets_count: *mut crate::ffi::UFunctionOpague,
    pub u_level_variant_sets_import_test_functions_check_variants_count: *mut crate::ffi::UFunctionOpague,
    pub u_level_variant_sets_import_test_functions_check_level_variant_sets_count: *mut crate::ffi::UFunctionOpague,
    pub u_level_variant_sets_import_test_functions_check_bindings_count: *mut crate::ffi::UFunctionOpague,
    pub u_light_import_test_functions_check_light_position: *mut crate::ffi::UFunctionOpague,
    pub u_light_import_test_functions_check_light_intensity: *mut crate::ffi::UFunctionOpague,
    pub u_light_import_test_functions_check_light_direction: *mut crate::ffi::UFunctionOpague,
    pub u_light_import_test_functions_check_light_color: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_vector_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_shading_model: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_scalar_parameter: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_opacity_mask_clip_value: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_is_two_sided: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_imported_material_instance_count: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_imported_material_count: *mut crate::ffi::UFunctionOpague,
    pub u_material_import_test_functions_check_blend_mode: *mut crate::ffi::UFunctionOpague,
    pub u_material_x_test_functions_check_input_connected: *mut crate::ffi::UFunctionOpague,
    pub u_material_x_test_functions_check_connected_input_count: *mut crate::ffi::UFunctionOpague,
    pub u_point_light_import_test_functions_check_light_falloff_exponent: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_vertex_index_position: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_vertex_index_normal: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_vertex_index_color: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_uv_channel_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_triangle_count_in_section: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_socket_name: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_socket_location: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_socket_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_skinned_vertex_count_for_bone: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_section_material_name: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_section_imported_material_slot_name: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_section_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_render_vertex_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_render_triangle_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_morph_target_name: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_morph_target_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_material_slot_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_lod_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_imported_skeletal_mesh_count: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_bone_position: *mut crate::ffi::UFunctionOpague,
    pub u_skeletal_mesh_import_test_functions_check_bone_count: *mut crate::ffi::UFunctionOpague,
    pub u_spot_light_import_test_functions_check_light_outer_cone_angle: *mut crate::ffi::UFunctionOpague,
    pub u_spot_light_import_test_functions_check_light_inner_cone_angle: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_vertex_index_position: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_vertex_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_uv_channel_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_triangle_count_in_polygon_group: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_triangle_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_that_mesh_has_quads_or_ngons: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_socket_name: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_socket_location: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_socket_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_simple_collision_primitive_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_section_material_name: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_section_material_index: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_section_imported_material_slot_name: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_section_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_render_vertex_index_normal: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_render_vertex_index_color: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_render_vertex_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_render_uv_channel_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_render_triangle_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_render_has_vertex_colors: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_polygon_group_imported_material_slot_name: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_polygon_group_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_polygon_count_in_polygon_group: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_polygon_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_nanite_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_material_slot_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_lod_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_imported_static_mesh_count: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_imported_material_slot_name: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_build_settings: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_bounding_box_size_less_than: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_bounding_box_size_greater_than: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_bounding_box_size: *mut crate::ffi::UFunctionOpague,
    pub u_static_mesh_import_test_functions_check_against_ground_truth: *mut crate::ffi::UFunctionOpague,
    pub u_texture_import_test_functions_check_texture_filter: *mut crate::ffi::UFunctionOpague,
    pub u_texture_import_test_functions_check_texture_address_z: *mut crate::ffi::UFunctionOpague,
    pub u_texture_import_test_functions_check_texture_address_y: *mut crate::ffi::UFunctionOpague,
    pub u_texture_import_test_functions_check_texture_address_x: *mut crate::ffi::UFunctionOpague,
    pub u_texture_import_test_functions_check_imported_texture_count: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_import_test_plan_run_this_test: *mut crate::ffi::UFunctionOpague,
    pub u_interchange_tests_blueprint_function_library_get_pipeline_properties_as_json: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_actor_import_test_functions_check_imported_actor_count: std::ptr::null_mut(),
            u_actor_import_test_functions_check_component_property_value: std::ptr::null_mut(),
            u_actor_import_test_functions_check_actor_property_value: std::ptr::null_mut(),
            u_actor_import_test_functions_check_actor_class_count: std::ptr::null_mut(),
            u_actor_import_test_functions_check_actor_class: std::ptr::null_mut(),
            u_animation_import_test_functions_check_imported_anim_sequence_count: std::ptr::null_mut(),
            u_animation_import_test_functions_check_curve_key_value: std::ptr::null_mut(),
            u_animation_import_test_functions_check_curve_key_time: std::ptr::null_mut(),
            u_animation_import_test_functions_check_curve_key_leave_tangent_weight: std::ptr::null_mut(),
            u_animation_import_test_functions_check_curve_key_leave_tangent: std::ptr::null_mut(),
            u_animation_import_test_functions_check_curve_key_arrive_tangent_weight: std::ptr::null_mut(),
            u_animation_import_test_functions_check_curve_key_arrive_tangent: std::ptr::null_mut(),
            u_animation_import_test_functions_check_animation_length: std::ptr::null_mut(),
            u_animation_import_test_functions_check_animation_frame_number: std::ptr::null_mut(),
            u_asset_import_test_functions_check_object_path_has_substring: std::ptr::null_mut(),
            u_asset_import_test_functions_check_metadata_value: std::ptr::null_mut(),
            u_asset_import_test_functions_check_metadata_exist: std::ptr::null_mut(),
            u_asset_import_test_functions_check_imported_metadata_count: std::ptr::null_mut(),
            u_interchange_result_import_test_functions_check_if_error_or_warning_was_generated: std::ptr::null_mut(),
            u_level_sequence_import_test_functions_check_sequence_length: std::ptr::null_mut(),
            u_level_sequence_import_test_functions_check_section_interpolation_mode: std::ptr::null_mut(),
            u_level_sequence_import_test_functions_check_section_count: std::ptr::null_mut(),
            u_level_sequence_import_test_functions_check_level_sequence_count: std::ptr::null_mut(),
            u_level_variant_sets_import_test_functions_check_variant_sets_count: std::ptr::null_mut(),
            u_level_variant_sets_import_test_functions_check_variants_count: std::ptr::null_mut(),
            u_level_variant_sets_import_test_functions_check_level_variant_sets_count: std::ptr::null_mut(),
            u_level_variant_sets_import_test_functions_check_bindings_count: std::ptr::null_mut(),
            u_light_import_test_functions_check_light_position: std::ptr::null_mut(),
            u_light_import_test_functions_check_light_intensity: std::ptr::null_mut(),
            u_light_import_test_functions_check_light_direction: std::ptr::null_mut(),
            u_light_import_test_functions_check_light_color: std::ptr::null_mut(),
            u_material_import_test_functions_check_vector_parameter: std::ptr::null_mut(),
            u_material_import_test_functions_check_shading_model: std::ptr::null_mut(),
            u_material_import_test_functions_check_scalar_parameter: std::ptr::null_mut(),
            u_material_import_test_functions_check_opacity_mask_clip_value: std::ptr::null_mut(),
            u_material_import_test_functions_check_is_two_sided: std::ptr::null_mut(),
            u_material_import_test_functions_check_imported_material_instance_count: std::ptr::null_mut(),
            u_material_import_test_functions_check_imported_material_count: std::ptr::null_mut(),
            u_material_import_test_functions_check_blend_mode: std::ptr::null_mut(),
            u_material_x_test_functions_check_input_connected: std::ptr::null_mut(),
            u_material_x_test_functions_check_connected_input_count: std::ptr::null_mut(),
            u_point_light_import_test_functions_check_light_falloff_exponent: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_vertex_index_position: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_vertex_index_normal: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_vertex_index_color: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_uv_channel_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_triangle_count_in_section: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_socket_name: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_socket_location: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_socket_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_skinned_vertex_count_for_bone: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_section_material_name: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_section_imported_material_slot_name: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_section_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_render_vertex_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_render_triangle_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_morph_target_name: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_morph_target_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_material_slot_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_lod_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_imported_skeletal_mesh_count: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_bone_position: std::ptr::null_mut(),
            u_skeletal_mesh_import_test_functions_check_bone_count: std::ptr::null_mut(),
            u_spot_light_import_test_functions_check_light_outer_cone_angle: std::ptr::null_mut(),
            u_spot_light_import_test_functions_check_light_inner_cone_angle: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_vertex_index_position: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_vertex_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_uv_channel_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_triangle_count_in_polygon_group: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_triangle_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_that_mesh_has_quads_or_ngons: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_socket_name: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_socket_location: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_socket_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_simple_collision_primitive_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_section_material_name: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_section_material_index: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_section_imported_material_slot_name: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_section_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_render_vertex_index_normal: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_render_vertex_index_color: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_render_vertex_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_render_uv_channel_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_render_triangle_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_render_has_vertex_colors: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_polygon_group_imported_material_slot_name: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_polygon_group_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_polygon_count_in_polygon_group: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_polygon_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_nanite_settings: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_material_slot_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_lod_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_imported_static_mesh_count: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_imported_material_slot_name: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_build_settings: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_bounding_box_size_less_than: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_bounding_box_size_greater_than: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_bounding_box_size: std::ptr::null_mut(),
            u_static_mesh_import_test_functions_check_against_ground_truth: std::ptr::null_mut(),
            u_texture_import_test_functions_check_texture_filter: std::ptr::null_mut(),
            u_texture_import_test_functions_check_texture_address_z: std::ptr::null_mut(),
            u_texture_import_test_functions_check_texture_address_y: std::ptr::null_mut(),
            u_texture_import_test_functions_check_texture_address_x: std::ptr::null_mut(),
            u_texture_import_test_functions_check_imported_texture_count: std::ptr::null_mut(),
            u_interchange_import_test_plan_run_this_test: std::ptr::null_mut(),
            u_interchange_tests_blueprint_function_library_get_pipeline_properties_as_json: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UActorImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedActorCount"),
            &raw mut __FUNCTION_PTRS
                .u_actor_import_test_functions_check_imported_actor_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckComponentPropertyValue"),
            &raw mut __FUNCTION_PTRS
                .u_actor_import_test_functions_check_component_property_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckActorPropertyValue"),
            &raw mut __FUNCTION_PTRS
                .u_actor_import_test_functions_check_actor_property_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckActorClassCount"),
            &raw mut __FUNCTION_PTRS
                .u_actor_import_test_functions_check_actor_class_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckActorClass"),
            &raw mut __FUNCTION_PTRS.u_actor_import_test_functions_check_actor_class,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedAnimSequenceCount"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_imported_anim_sequence_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckCurveKeyValue"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_curve_key_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckCurveKeyTime"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_curve_key_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckCurveKeyLeaveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_curve_key_leave_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckCurveKeyLeaveTangent"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_curve_key_leave_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckCurveKeyArriveTangentWeight"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_curve_key_arrive_tangent_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckCurveKeyArriveTangent"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_curve_key_arrive_tangent,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckAnimationLength"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_animation_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckAnimationFrameNumber"),
            &raw mut __FUNCTION_PTRS
                .u_animation_import_test_functions_check_animation_frame_number,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAssetImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckObjectPathHasSubstring"),
            &raw mut __FUNCTION_PTRS
                .u_asset_import_test_functions_check_object_path_has_substring,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMetadataValue"),
            &raw mut __FUNCTION_PTRS.u_asset_import_test_functions_check_metadata_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMetadataExist"),
            &raw mut __FUNCTION_PTRS.u_asset_import_test_functions_check_metadata_exist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedMetadataCount"),
            &raw mut __FUNCTION_PTRS
                .u_asset_import_test_functions_check_imported_metadata_count,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeResultImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckIfErrorOrWarningWasGenerated"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_result_import_test_functions_check_if_error_or_warning_was_generated,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelSequenceImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSequenceLength"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_import_test_functions_check_sequence_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionInterpolationMode"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_import_test_functions_check_section_interpolation_mode,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionCount"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_import_test_functions_check_section_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLevelSequenceCount"),
            &raw mut __FUNCTION_PTRS
                .u_level_sequence_import_test_functions_check_level_sequence_count,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULevelVariantSetsImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVariantSetsCount"),
            &raw mut __FUNCTION_PTRS
                .u_level_variant_sets_import_test_functions_check_variant_sets_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVariantsCount"),
            &raw mut __FUNCTION_PTRS
                .u_level_variant_sets_import_test_functions_check_variants_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLevelVariantSetsCount"),
            &raw mut __FUNCTION_PTRS
                .u_level_variant_sets_import_test_functions_check_level_variant_sets_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBindingsCount"),
            &raw mut __FUNCTION_PTRS
                .u_level_variant_sets_import_test_functions_check_bindings_count,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = ULightImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLightPosition"),
            &raw mut __FUNCTION_PTRS.u_light_import_test_functions_check_light_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLightIntensity"),
            &raw mut __FUNCTION_PTRS.u_light_import_test_functions_check_light_intensity,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLightDirection"),
            &raw mut __FUNCTION_PTRS.u_light_import_test_functions_check_light_direction,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLightColor"),
            &raw mut __FUNCTION_PTRS.u_light_import_test_functions_check_light_color,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMaterialImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVectorParameter"),
            &raw mut __FUNCTION_PTRS
                .u_material_import_test_functions_check_vector_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckShadingModel"),
            &raw mut __FUNCTION_PTRS.u_material_import_test_functions_check_shading_model,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckScalarParameter"),
            &raw mut __FUNCTION_PTRS
                .u_material_import_test_functions_check_scalar_parameter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckOpacityMaskClipValue"),
            &raw mut __FUNCTION_PTRS
                .u_material_import_test_functions_check_opacity_mask_clip_value,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckIsTwoSided"),
            &raw mut __FUNCTION_PTRS.u_material_import_test_functions_check_is_two_sided,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedMaterialInstanceCount"),
            &raw mut __FUNCTION_PTRS
                .u_material_import_test_functions_check_imported_material_instance_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedMaterialCount"),
            &raw mut __FUNCTION_PTRS
                .u_material_import_test_functions_check_imported_material_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBlendMode"),
            &raw mut __FUNCTION_PTRS.u_material_import_test_functions_check_blend_mode,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UMaterialXTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckInputConnected"),
            &raw mut __FUNCTION_PTRS.u_material_x_test_functions_check_input_connected,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckConnectedInputCount"),
            &raw mut __FUNCTION_PTRS
                .u_material_x_test_functions_check_connected_input_count,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UPointLightImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLightFalloffExponent"),
            &raw mut __FUNCTION_PTRS
                .u_point_light_import_test_functions_check_light_falloff_exponent,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USkeletalMeshImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVertexIndexPosition"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_vertex_index_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVertexIndexNormal"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_vertex_index_normal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVertexIndexColor"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_vertex_index_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckUVChannelCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_uv_channel_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTriangleCountInSection"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_triangle_count_in_section,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSocketName"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_socket_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSocketLocation"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_socket_location,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSocketCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_socket_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSkinnedVertexCountForBone"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_skinned_vertex_count_for_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionMaterialName"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_section_material_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionImportedMaterialSlotName"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_section_imported_material_slot_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_section_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderVertexCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_render_vertex_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderTriangleCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_render_triangle_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMorphTargetName"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_morph_target_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMorphTargetCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_morph_target_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMaterialSlotCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_material_slot_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLodCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_lod_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedSkeletalMeshCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_imported_skeletal_mesh_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBonePosition"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_bone_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBoneCount"),
            &raw mut __FUNCTION_PTRS
                .u_skeletal_mesh_import_test_functions_check_bone_count,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = USpotLightImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLightOuterConeAngle"),
            &raw mut __FUNCTION_PTRS
                .u_spot_light_import_test_functions_check_light_outer_cone_angle,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLightInnerConeAngle"),
            &raw mut __FUNCTION_PTRS
                .u_spot_light_import_test_functions_check_light_inner_cone_angle,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UStaticMeshImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVertexIndexPosition"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_vertex_index_position,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckVertexCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_vertex_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckUVChannelCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_uv_channel_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTriangleCountInPolygonGroup"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_triangle_count_in_polygon_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTriangleCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_triangle_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckThatMeshHasQuadsOrNgons"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_that_mesh_has_quads_or_ngons,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSocketName"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_socket_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSocketLocation"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_socket_location,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSocketCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_socket_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSimpleCollisionPrimitiveCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_simple_collision_primitive_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionMaterialName"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_section_material_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionMaterialIndex"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_section_material_index,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionImportedMaterialSlotName"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_section_imported_material_slot_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckSectionCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_section_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderVertexIndexNormal"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_render_vertex_index_normal,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderVertexIndexColor"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_render_vertex_index_color,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderVertexCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_render_vertex_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderUVChannelCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_render_uv_channel_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderTriangleCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_render_triangle_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckRenderHasVertexColors"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_render_has_vertex_colors,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckPolygonGroupImportedMaterialSlotName"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_polygon_group_imported_material_slot_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckPolygonGroupCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_polygon_group_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckPolygonCountInPolygonGroup"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_polygon_count_in_polygon_group,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckPolygonCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_polygon_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckNaniteSettings"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_nanite_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckMaterialSlotCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_material_slot_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckLodCount"),
            &raw mut __FUNCTION_PTRS.u_static_mesh_import_test_functions_check_lod_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedStaticMeshCount"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_imported_static_mesh_count,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedMaterialSlotName"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_imported_material_slot_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBuildSettings"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_build_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBoundingBoxSizeLessThan"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_bounding_box_size_less_than,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBoundingBoxSizeGreaterThan"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_bounding_box_size_greater_than,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckBoundingBoxSize"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_bounding_box_size,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckAgainstGroundTruth"),
            &raw mut __FUNCTION_PTRS
                .u_static_mesh_import_test_functions_check_against_ground_truth,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UTextureImportTestFunctions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTextureFilter"),
            &raw mut __FUNCTION_PTRS.u_texture_import_test_functions_check_texture_filter,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTextureAddressZ"),
            &raw mut __FUNCTION_PTRS
                .u_texture_import_test_functions_check_texture_address_z,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTextureAddressY"),
            &raw mut __FUNCTION_PTRS
                .u_texture_import_test_functions_check_texture_address_y,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckTextureAddressX"),
            &raw mut __FUNCTION_PTRS
                .u_texture_import_test_functions_check_texture_address_x,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CheckImportedTextureCount"),
            &raw mut __FUNCTION_PTRS
                .u_texture_import_test_functions_check_imported_texture_count,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeImportTestPlan::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RunThisTest"),
            &raw mut __FUNCTION_PTRS.u_interchange_import_test_plan_run_this_test,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UInterchangeTestsBlueprintFunctionLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetPipelinePropertiesAsJSON"),
            &raw mut __FUNCTION_PTRS
                .u_interchange_tests_blueprint_function_library_get_pipeline_properties_as_json,
        );
    }
}
#[repr(C, align(8))]
pub struct FInterchangeTestScreenshotParameters {
    pub(crate) __padding_end: [u8; 88],
}
impl FInterchangeTestScreenshotParameters {}
#[repr(C, align(8))]
pub struct FInterchangeTestPlanPipelineSettings {
    pub(crate) __padding_end: [u8; 24],
}
impl FInterchangeTestPlanPipelineSettings {}
#[repr(C, align(8))]
pub struct UInterchangeImportTestSettings {
    __padding_end: [u8; 80],
}
impl UInterchangeImportTestSettings {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeImportTestSettings")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UImportTestFunctionsBase {
    __padding_end: [u8; 48],
}
impl UImportTestFunctionsBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UImportTestFunctionsBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UActorImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UActorImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UActorImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimationImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UAnimationImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UAssetImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UAssetImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAssetImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeResultImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UInterchangeResultImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeResultImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelSequenceImportTestFunctions {
    __padding_end: [u8; 48],
}
impl ULevelSequenceImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelSequenceImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULevelVariantSetsImportTestFunctions {
    __padding_end: [u8; 48],
}
impl ULevelVariantSetsImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULevelVariantSetsImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct ULightImportTestFunctions {
    __padding_end: [u8; 48],
}
impl ULightImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("ULightImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UMaterialImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UMaterialXTestFunctions {
    __padding_end: [u8; 48],
}
impl UMaterialXTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UMaterialXTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UPointLightImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UPointLightImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UPointLightImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USkeletalMeshImportTestFunctions {
    __padding_end: [u8; 48],
}
impl USkeletalMeshImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USkeletalMeshImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct USpotLightImportTestFunctions {
    __padding_end: [u8; 48],
}
impl USpotLightImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("USpotLightImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UStaticMeshImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UStaticMeshImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UStaticMeshImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UTextureImportTestFunctions {
    __padding_end: [u8; 48],
}
impl UTextureImportTestFunctions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UTextureImportTestFunctions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeImportTestPlan {
    __padding_end: [u8; 160],
}
impl UInterchangeImportTestPlan {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeImportTestPlan")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn run_this_test(&mut self) {
        let mut __stack = crate::core_data::StackAlloc::<0>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_tests::__FUNCTION_PTRS
                    .u_interchange_import_test_plan_run_this_test,
                __buffer,
            )
        };
        let __object_ptr = self as *const _ as *mut std::ffi::c_void;
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_tests::__FUNCTION_PTRS
                    .u_interchange_import_test_plan_run_this_test,
                __buffer,
            )
        };
    }
}
#[repr(C, align(8))]
pub struct UInterchangeImportTestStepBase {
    __padding_end: [u8; 72],
}
impl UInterchangeImportTestStepBase {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeImportTestStepBase")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeImportTestStepImport {
    __padding_end: [u8; 280],
}
impl UInterchangeImportTestStepImport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeImportTestStepImport")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeImportTestStepReimport {
    __padding_end: [u8; 288],
}
impl UInterchangeImportTestStepReimport {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeImportTestStepReimport")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UInterchangeTestsBlueprintFunctionLibrary {
    __padding_end: [u8; 48],
}
impl UInterchangeTestsBlueprintFunctionLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UInterchangeTestsBlueprintFunctionLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn get_pipeline_properties_as_json(
        pipeline: UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
    ) -> FString {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::interchange_tests::__FUNCTION_PTRS
                    .u_interchange_tests_blueprint_function_library_get_pipeline_properties_as_json,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &pipeline,
                __buffer
                    .add(0)
                    .cast::<
                        UPtr<crate::bindings::interchange_core::UInterchangePipelineBase>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::interchange_tests::UInterchangeTestsBlueprintFunctionLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::interchange_tests::__FUNCTION_PTRS
                    .u_interchange_tests_blueprint_function_library_get_pipeline_properties_as_json,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<FString>().read() }
    }
}
