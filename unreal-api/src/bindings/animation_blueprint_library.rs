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
    pub u_animation_attribute_blueprint_library_set_attribute_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_attribute_blueprint_library_set_attribute_key: *mut crate::ffi::UFunctionOpague,
    pub u_animation_attribute_blueprint_library_get_attribute_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_attribute_blueprint_library_get_attribute_key: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_variable_frame_stripping_settings: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_root_motion_lock_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_root_motion_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_rate_scale: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_is_root_motion_lock_forced: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_curve_meta_data_morph_target: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_curve_meta_data_material: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_curve_compression_settings: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_bone_compression_settings: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_animation_interpolation_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_additive_base_pose_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_set_additive_animation_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_replace_anim_notify_states: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_replace_anim_notifies: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_rename_curve_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_virtual_bones: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_virtual_bone: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_meta_data_of_class: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_curve_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_curve: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_bone_selective_animation: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_bone_animation: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_animation_sync_markers_by_track: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_animation_sync_markers_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_animation_notify_track: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_animation_notify_events_by_track: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_animation_notify_events_by_name: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_all_virtual_bones: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_all_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_all_curve_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_all_bone_animation: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_all_animation_sync_markers: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_remove_all_animation_notify_tracks: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_is_valid_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_is_valid_raw_animation_track_name: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_is_valid_anim_notify_track_name: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_is_valid_animation_sync_marker_name: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_is_root_motion_lock_forced: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_is_root_motion_enabled: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_vector_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_variable_frame_stripping_settings: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_unique_marker_names: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_transformation_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_time_at_frame: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_sequence_length: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_root_motion_lock_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_raw_track_scale_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_raw_track_rotation_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_raw_track_position_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_raw_track_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_rate_scale: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_num_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_num_frames: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_num_curve_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_nodes_of_class: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_montage_slot_names: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_meta_data_of_class: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_frame_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_float_value_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_float_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_curve_meta_data_names: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_curve_meta_data_morph_target: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_curve_meta_data_material: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_curve_compression_settings: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_bone_poses_for_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_bone_poses_for_frame: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_bone_pose_for_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_bone_pose_for_frame: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_bone_compression_settings: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_anim_notify_event_trigger_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_anim_notify_event_duration: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_track_names: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_sync_markers_for_track: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_sync_markers: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_notify_track_names: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_notify_events_for_track: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_notify_events: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_notify_event_names: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_interpolation_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_graphs: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_animation_curve_names: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_additive_base_pose_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_get_additive_animation_type: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_find_bone_path_to_root: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_finalize_bone_animation: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_extract_root_track_transform: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_evaluate_root_bone_timecode_subframe_attribute_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_evaluate_root_bone_timecode_attributes_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_evaluate_bone_timecode_and_slate_attributes_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_does_curve_exist: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_does_bone_name_exist: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_copy_anim_notifies_from_sequence: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_copy_animation_curve_names_to_skeleton: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_contains_meta_data_of_class: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_virtual_bone: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_vector_curve_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_vector_curve_key: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_transformation_curve_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_transformation_curve_key: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_node_asset_override: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_meta_data_object: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_float_curve_keys: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_float_curve_key: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_curve_meta_data: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_curve: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_animation_sync_marker: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_animation_notify_track: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_animation_notify_state_event_object: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_animation_notify_state_event: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_animation_notify_event_object: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_animation_notify_event_from_source: *mut crate::ffi::UFunctionOpague,
    pub u_animation_blueprint_library_add_animation_notify_event: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_set_bone_pose: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_is_valid: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_socket_pose: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_socket_names: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_relative_transform: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_relative_to_ref_pose_transform: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_ref_pose_relative_transform: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_reference_pose: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_ref_bone_pose: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_curve_weight: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_curve_names: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_bone_pose: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_bone_names: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_anim_pose_at_time: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_get_anim_pose_at_frame: *mut crate::ffi::UFunctionOpague,
    pub u_anim_pose_extensions_evaluate_animation_blueprint_with_input_pose: *mut crate::ffi::UFunctionOpague,
}
impl FunctionPtrs {
    pub const fn empty() -> Self {
        Self {
            u_animation_attribute_blueprint_library_set_attribute_keys: std::ptr::null_mut(),
            u_animation_attribute_blueprint_library_set_attribute_key: std::ptr::null_mut(),
            u_animation_attribute_blueprint_library_get_attribute_keys: std::ptr::null_mut(),
            u_animation_attribute_blueprint_library_get_attribute_key: std::ptr::null_mut(),
            u_animation_blueprint_library_set_variable_frame_stripping_settings: std::ptr::null_mut(),
            u_animation_blueprint_library_set_root_motion_lock_type: std::ptr::null_mut(),
            u_animation_blueprint_library_set_root_motion_enabled: std::ptr::null_mut(),
            u_animation_blueprint_library_set_rate_scale: std::ptr::null_mut(),
            u_animation_blueprint_library_set_is_root_motion_lock_forced: std::ptr::null_mut(),
            u_animation_blueprint_library_set_curve_meta_data_morph_target: std::ptr::null_mut(),
            u_animation_blueprint_library_set_curve_meta_data_material: std::ptr::null_mut(),
            u_animation_blueprint_library_set_curve_compression_settings: std::ptr::null_mut(),
            u_animation_blueprint_library_set_bone_compression_settings: std::ptr::null_mut(),
            u_animation_blueprint_library_set_animation_interpolation_type: std::ptr::null_mut(),
            u_animation_blueprint_library_set_additive_base_pose_type: std::ptr::null_mut(),
            u_animation_blueprint_library_set_additive_animation_type: std::ptr::null_mut(),
            u_animation_blueprint_library_replace_anim_notify_states: std::ptr::null_mut(),
            u_animation_blueprint_library_replace_anim_notifies: std::ptr::null_mut(),
            u_animation_blueprint_library_rename_curve_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_virtual_bones: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_virtual_bone: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_meta_data_of_class: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_curve_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_curve: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_bone_selective_animation: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_bone_animation: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_animation_sync_markers_by_track: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_animation_sync_markers_by_name: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_animation_notify_track: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_animation_notify_events_by_track: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_animation_notify_events_by_name: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_all_virtual_bones: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_all_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_all_curve_data: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_all_bone_animation: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_all_animation_sync_markers: std::ptr::null_mut(),
            u_animation_blueprint_library_remove_all_animation_notify_tracks: std::ptr::null_mut(),
            u_animation_blueprint_library_is_valid_time: std::ptr::null_mut(),
            u_animation_blueprint_library_is_valid_raw_animation_track_name: std::ptr::null_mut(),
            u_animation_blueprint_library_is_valid_anim_notify_track_name: std::ptr::null_mut(),
            u_animation_blueprint_library_is_valid_animation_sync_marker_name: std::ptr::null_mut(),
            u_animation_blueprint_library_is_root_motion_lock_forced: std::ptr::null_mut(),
            u_animation_blueprint_library_is_root_motion_enabled: std::ptr::null_mut(),
            u_animation_blueprint_library_get_vector_keys: std::ptr::null_mut(),
            u_animation_blueprint_library_get_variable_frame_stripping_settings: std::ptr::null_mut(),
            u_animation_blueprint_library_get_unique_marker_names: std::ptr::null_mut(),
            u_animation_blueprint_library_get_transformation_keys: std::ptr::null_mut(),
            u_animation_blueprint_library_get_time_at_frame: std::ptr::null_mut(),
            u_animation_blueprint_library_get_sequence_length: std::ptr::null_mut(),
            u_animation_blueprint_library_get_root_motion_lock_type: std::ptr::null_mut(),
            u_animation_blueprint_library_get_raw_track_scale_data: std::ptr::null_mut(),
            u_animation_blueprint_library_get_raw_track_rotation_data: std::ptr::null_mut(),
            u_animation_blueprint_library_get_raw_track_position_data: std::ptr::null_mut(),
            u_animation_blueprint_library_get_raw_track_data: std::ptr::null_mut(),
            u_animation_blueprint_library_get_rate_scale: std::ptr::null_mut(),
            u_animation_blueprint_library_get_num_keys: std::ptr::null_mut(),
            u_animation_blueprint_library_get_num_frames: std::ptr::null_mut(),
            u_animation_blueprint_library_get_num_curve_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_get_nodes_of_class: std::ptr::null_mut(),
            u_animation_blueprint_library_get_montage_slot_names: std::ptr::null_mut(),
            u_animation_blueprint_library_get_meta_data_of_class: std::ptr::null_mut(),
            u_animation_blueprint_library_get_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_get_frame_at_time: std::ptr::null_mut(),
            u_animation_blueprint_library_get_float_value_at_time: std::ptr::null_mut(),
            u_animation_blueprint_library_get_float_keys: std::ptr::null_mut(),
            u_animation_blueprint_library_get_curve_meta_data_names: std::ptr::null_mut(),
            u_animation_blueprint_library_get_curve_meta_data_morph_target: std::ptr::null_mut(),
            u_animation_blueprint_library_get_curve_meta_data_material: std::ptr::null_mut(),
            u_animation_blueprint_library_get_curve_compression_settings: std::ptr::null_mut(),
            u_animation_blueprint_library_get_bone_poses_for_time: std::ptr::null_mut(),
            u_animation_blueprint_library_get_bone_poses_for_frame: std::ptr::null_mut(),
            u_animation_blueprint_library_get_bone_pose_for_time: std::ptr::null_mut(),
            u_animation_blueprint_library_get_bone_pose_for_frame: std::ptr::null_mut(),
            u_animation_blueprint_library_get_bone_compression_settings: std::ptr::null_mut(),
            u_animation_blueprint_library_get_anim_notify_event_trigger_time: std::ptr::null_mut(),
            u_animation_blueprint_library_get_anim_notify_event_duration: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_track_names: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_sync_markers_for_track: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_sync_markers: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_notify_track_names: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_notify_events_for_track: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_notify_events: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_notify_event_names: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_interpolation_type: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_graphs: std::ptr::null_mut(),
            u_animation_blueprint_library_get_animation_curve_names: std::ptr::null_mut(),
            u_animation_blueprint_library_get_additive_base_pose_type: std::ptr::null_mut(),
            u_animation_blueprint_library_get_additive_animation_type: std::ptr::null_mut(),
            u_animation_blueprint_library_find_bone_path_to_root: std::ptr::null_mut(),
            u_animation_blueprint_library_finalize_bone_animation: std::ptr::null_mut(),
            u_animation_blueprint_library_extract_root_track_transform: std::ptr::null_mut(),
            u_animation_blueprint_library_evaluate_root_bone_timecode_subframe_attribute_at_time: std::ptr::null_mut(),
            u_animation_blueprint_library_evaluate_root_bone_timecode_attributes_at_time: std::ptr::null_mut(),
            u_animation_blueprint_library_evaluate_bone_timecode_and_slate_attributes_at_time: std::ptr::null_mut(),
            u_animation_blueprint_library_does_curve_exist: std::ptr::null_mut(),
            u_animation_blueprint_library_does_bone_name_exist: std::ptr::null_mut(),
            u_animation_blueprint_library_copy_anim_notifies_from_sequence: std::ptr::null_mut(),
            u_animation_blueprint_library_copy_animation_curve_names_to_skeleton: std::ptr::null_mut(),
            u_animation_blueprint_library_contains_meta_data_of_class: std::ptr::null_mut(),
            u_animation_blueprint_library_add_virtual_bone: std::ptr::null_mut(),
            u_animation_blueprint_library_add_vector_curve_keys: std::ptr::null_mut(),
            u_animation_blueprint_library_add_vector_curve_key: std::ptr::null_mut(),
            u_animation_blueprint_library_add_transformation_curve_keys: std::ptr::null_mut(),
            u_animation_blueprint_library_add_transformation_curve_key: std::ptr::null_mut(),
            u_animation_blueprint_library_add_node_asset_override: std::ptr::null_mut(),
            u_animation_blueprint_library_add_meta_data_object: std::ptr::null_mut(),
            u_animation_blueprint_library_add_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_add_float_curve_keys: std::ptr::null_mut(),
            u_animation_blueprint_library_add_float_curve_key: std::ptr::null_mut(),
            u_animation_blueprint_library_add_curve_meta_data: std::ptr::null_mut(),
            u_animation_blueprint_library_add_curve: std::ptr::null_mut(),
            u_animation_blueprint_library_add_animation_sync_marker: std::ptr::null_mut(),
            u_animation_blueprint_library_add_animation_notify_track: std::ptr::null_mut(),
            u_animation_blueprint_library_add_animation_notify_state_event_object: std::ptr::null_mut(),
            u_animation_blueprint_library_add_animation_notify_state_event: std::ptr::null_mut(),
            u_animation_blueprint_library_add_animation_notify_event_object: std::ptr::null_mut(),
            u_animation_blueprint_library_add_animation_notify_event_from_source: std::ptr::null_mut(),
            u_animation_blueprint_library_add_animation_notify_event: std::ptr::null_mut(),
            u_anim_pose_extensions_set_bone_pose: std::ptr::null_mut(),
            u_anim_pose_extensions_is_valid: std::ptr::null_mut(),
            u_anim_pose_extensions_get_socket_pose: std::ptr::null_mut(),
            u_anim_pose_extensions_get_socket_names: std::ptr::null_mut(),
            u_anim_pose_extensions_get_relative_transform: std::ptr::null_mut(),
            u_anim_pose_extensions_get_relative_to_ref_pose_transform: std::ptr::null_mut(),
            u_anim_pose_extensions_get_ref_pose_relative_transform: std::ptr::null_mut(),
            u_anim_pose_extensions_get_reference_pose: std::ptr::null_mut(),
            u_anim_pose_extensions_get_ref_bone_pose: std::ptr::null_mut(),
            u_anim_pose_extensions_get_curve_weight: std::ptr::null_mut(),
            u_anim_pose_extensions_get_curve_names: std::ptr::null_mut(),
            u_anim_pose_extensions_get_bone_pose: std::ptr::null_mut(),
            u_anim_pose_extensions_get_bone_names: std::ptr::null_mut(),
            u_anim_pose_extensions_get_anim_pose_at_time: std::ptr::null_mut(),
            u_anim_pose_extensions_get_anim_pose_at_frame: std::ptr::null_mut(),
            u_anim_pose_extensions_evaluate_animation_blueprint_with_input_pose: std::ptr::null_mut(),
        }
    }
}
pub fn initialize() {
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationAttributeBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAttributeKeys"),
            &raw mut __FUNCTION_PTRS
                .u_animation_attribute_blueprint_library_set_attribute_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAttributeKey"),
            &raw mut __FUNCTION_PTRS
                .u_animation_attribute_blueprint_library_set_attribute_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAttributeKeys"),
            &raw mut __FUNCTION_PTRS
                .u_animation_attribute_blueprint_library_get_attribute_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAttributeKey"),
            &raw mut __FUNCTION_PTRS
                .u_animation_attribute_blueprint_library_get_attribute_key,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimationBlueprintLibrary::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetVariableFrameStrippingSettings"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_variable_frame_stripping_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRootMotionLockType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_root_motion_lock_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRootMotionEnabled"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_root_motion_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetRateScale"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_set_rate_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetIsRootMotionLockForced"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_is_root_motion_lock_forced,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurveMetaDataMorphTarget"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_curve_meta_data_morph_target,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurveMetaDataMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_curve_meta_data_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetCurveCompressionSettings"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_curve_compression_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBoneCompressionSettings"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_bone_compression_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAnimationInterpolationType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_animation_interpolation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAdditiveBasePoseType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_additive_base_pose_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetAdditiveAnimationType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_set_additive_animation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceAnimNotifyStates"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_replace_anim_notify_states,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ReplaceAnimNotifies"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_replace_anim_notifies,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RenameCurveMetaData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_rename_curve_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveVirtualBones"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_virtual_bones,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveVirtualBone"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_virtual_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMetaDataOfClass"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_meta_data_of_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveMetaData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCurveMetaData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_curve_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveCurve"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_curve,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBoneSelectiveAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_bone_selective_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveBoneAnimation"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_bone_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAnimationSyncMarkersByTrack"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_animation_sync_markers_by_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAnimationSyncMarkersByName"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_animation_sync_markers_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAnimationNotifyTrack"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_animation_notify_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAnimationNotifyEventsByTrack"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_animation_notify_events_by_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAnimationNotifyEventsByName"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_animation_notify_events_by_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllVirtualBones"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_all_virtual_bones,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllMetaData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_all_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllCurveData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_remove_all_curve_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllBoneAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_all_bone_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllAnimationSyncMarkers"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_all_animation_sync_markers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("RemoveAllAnimationNotifyTracks"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_remove_all_animation_notify_tracks,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidTime"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_is_valid_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidRawAnimationTrackName"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_is_valid_raw_animation_track_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidAnimNotifyTrackName"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_is_valid_anim_notify_track_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValidAnimationSyncMarkerName"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_is_valid_animation_sync_marker_name,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRootMotionLockForced"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_is_root_motion_lock_forced,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsRootMotionEnabled"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_is_root_motion_enabled,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVectorKeys"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_vector_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetVariableFrameStrippingSettings"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_variable_frame_stripping_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetUniqueMarkerNames"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_unique_marker_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTransformationKeys"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_transformation_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetTimeAtFrame"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_time_at_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSequenceLength"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_sequence_length,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRootMotionLockType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_root_motion_lock_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRawTrackScaleData"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_raw_track_scale_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRawTrackRotationData"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_raw_track_rotation_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRawTrackPositionData"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_raw_track_position_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRawTrackData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_raw_track_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRateScale"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_rate_scale,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumKeys"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_num_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumFrames"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_num_frames,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNumCurveMetaData"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_num_curve_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetNodesOfClass"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_nodes_of_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMontageSlotNames"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_montage_slot_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaDataOfClass"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_meta_data_of_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetMetaData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFrameAtTime"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_frame_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloatValueAtTime"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_float_value_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetFloatKeys"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_float_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveMetaDataNames"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_curve_meta_data_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveMetaDataMorphTarget"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_curve_meta_data_morph_target,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveMetaDataMaterial"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_curve_meta_data_material,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveCompressionSettings"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_curve_compression_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBonePosesForTime"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_bone_poses_for_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBonePosesForFrame"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_bone_poses_for_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBonePoseForTime"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_bone_pose_for_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBonePoseForFrame"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_bone_pose_for_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneCompressionSettings"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_bone_compression_settings,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimNotifyEventTriggerTime"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_anim_notify_event_trigger_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimNotifyEventDuration"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_anim_notify_event_duration,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationTrackNames"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_track_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationSyncMarkersForTrack"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_sync_markers_for_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationSyncMarkers"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_sync_markers,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationNotifyTrackNames"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_notify_track_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationNotifyEventsForTrack"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_notify_events_for_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationNotifyEvents"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_notify_events,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationNotifyEventNames"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_notify_event_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationInterpolationType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_interpolation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationGraphs"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_get_animation_graphs,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimationCurveNames"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_animation_curve_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAdditiveBasePoseType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_additive_base_pose_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAdditiveAnimationType"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_get_additive_animation_type,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FindBonePathToRoot"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_find_bone_path_to_root,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("FinalizeBoneAnimation"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_finalize_bone_animation,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ExtractRootTrackTransform"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_extract_root_track_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateRootBoneTimecodeSubframeAttributeAtTime"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_evaluate_root_bone_timecode_subframe_attribute_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateRootBoneTimecodeAttributesAtTime"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_evaluate_root_bone_timecode_attributes_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateBoneTimecodeAndSlateAttributesAtTime"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_evaluate_bone_timecode_and_slate_attributes_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesCurveExist"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_does_curve_exist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("DoesBoneNameExist"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_does_bone_name_exist,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyAnimNotifiesFromSequence"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_copy_anim_notifies_from_sequence,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("CopyAnimationCurveNamesToSkeleton"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_copy_animation_curve_names_to_skeleton,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("ContainsMetaDataOfClass"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_contains_meta_data_of_class,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVirtualBone"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_virtual_bone,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVectorCurveKeys"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_vector_curve_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddVectorCurveKey"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_vector_curve_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTransformationCurveKeys"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_transformation_curve_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddTransformationCurveKey"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_transformation_curve_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddNodeAssetOverride"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_node_asset_override,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMetaDataObject"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_meta_data_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddMetaData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFloatCurveKeys"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_float_curve_keys,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddFloatCurveKey"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_float_curve_key,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCurveMetaData"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_curve_meta_data,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddCurve"),
            &raw mut __FUNCTION_PTRS.u_animation_blueprint_library_add_curve,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationSyncMarker"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_animation_sync_marker,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationNotifyTrack"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_animation_notify_track,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationNotifyStateEventObject"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_animation_notify_state_event_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationNotifyStateEvent"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_animation_notify_state_event,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationNotifyEventObject"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_animation_notify_event_object,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationNotifyEventFromSource"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_animation_notify_event_from_source,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("AddAnimationNotifyEvent"),
            &raw mut __FUNCTION_PTRS
                .u_animation_blueprint_library_add_animation_notify_event,
        );
    }
    unsafe {
        let bindings = crate::module::bindings();
        let class_ptr = UAnimPoseExtensions::static_class();
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("SetBonePose"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_set_bone_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("IsValid"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_is_valid,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSocketPose"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_socket_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetSocketNames"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_socket_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRelativeTransform"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_relative_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRelativeToRefPoseTransform"),
            &raw mut __FUNCTION_PTRS
                .u_anim_pose_extensions_get_relative_to_ref_pose_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRefPoseRelativeTransform"),
            &raw mut __FUNCTION_PTRS
                .u_anim_pose_extensions_get_ref_pose_relative_transform,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetReferencePose"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_reference_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetRefBonePose"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_ref_bone_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveWeight"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_curve_weight,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetCurveNames"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_curve_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBonePose"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_bone_pose,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetBoneNames"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_bone_names,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimPoseAtTime"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_anim_pose_at_time,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("GetAnimPoseAtFrame"),
            &raw mut __FUNCTION_PTRS.u_anim_pose_extensions_get_anim_pose_at_frame,
        );
        (bindings
            .core_fns
            .find_function_by_name)(
            class_ptr,
            unreal_ffi::Utf8Str::from("EvaluateAnimationBlueprintWithInputPose"),
            &raw mut __FUNCTION_PTRS
                .u_anim_pose_extensions_evaluate_animation_blueprint_with_input_pose,
        );
    }
}
#[repr(C, align(8))]
pub struct FAnimPoseEvaluationOptions {
    pub evaluation_type: EAnimDataEvalType,
    pub b_should_retarget: bool,
    pub b_extract_root_motion: bool,
    pub b_incorporate_root_motion_into_pose: bool,
    pub optional_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    pub b_retrieve_additive_as_full_pose: bool,
    pub b_evaluate_curves: bool,
}
impl FAnimPoseEvaluationOptions {}
#[repr(C, align(8))]
pub struct FAnimPose {
    pub(crate) __padding_end: [u8; 192],
}
impl FAnimPose {}
#[repr(C, align(8))]
pub struct UAnimationAttributeBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAnimationAttributeBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationAttributeBlueprintLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_attribute_keys(
        animation_data_controller: TScriptInterface<
            crate::bindings::engine::UAnimationDataController,
        >,
        attribute_identifier: &crate::bindings::engine::FAnimationAttributeIdentifier,
        times: &TArray<f32>,
        values: &TArray<i32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<129>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_set_attribute_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_data_controller,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::engine::UAnimationDataController,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_identifier,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::engine::FAnimationAttributeIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                times,
                __buffer.add(96).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                values,
                __buffer.add(112).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationAttributeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_set_attribute_keys,
                __buffer,
            )
        };
        unsafe { __buffer.add(128).cast::<bool>().read() }
    }
    pub fn set_attribute_key(
        animation_data_controller: TScriptInterface<
            crate::bindings::engine::UAnimationDataController,
        >,
        attribute_identifier: &crate::bindings::engine::FAnimationAttributeIdentifier,
        time: f32,
        value: &i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_set_attribute_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_data_controller,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<
                            crate::bindings::engine::UAnimationDataController,
                        >,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_identifier,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::engine::FAnimationAttributeIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(96).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(100).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationAttributeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_set_attribute_key,
                __buffer,
            )
        };
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
    pub fn get_attribute_keys(
        animation_data_model: TScriptInterface<
            crate::bindings::engine::UAnimationDataModel,
        >,
        attribute_identifier: &crate::bindings::engine::FAnimationAttributeIdentifier,
        out_times: &mut TArray<f32>,
        values: &mut TArray<i32>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<129>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_get_attribute_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_data_model,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<crate::bindings::engine::UAnimationDataModel>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_identifier,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::engine::FAnimationAttributeIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_times,
                __buffer.add(96).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                values,
                __buffer.add(112).cast::<TArray<i32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationAttributeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_get_attribute_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(96).cast::<TArray<f32>>().swap(out_times);
        }
        unsafe {
            __buffer.add(112).cast::<TArray<i32>>().swap(values);
        }
        unsafe { __buffer.add(128).cast::<bool>().read() }
    }
    pub fn get_attribute_key(
        animation_data_model: TScriptInterface<
            crate::bindings::engine::UAnimationDataModel,
        >,
        attribute_identifier: &crate::bindings::engine::FAnimationAttributeIdentifier,
        time: f32,
        value: &mut i32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<105>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_get_attribute_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_data_model,
                __buffer
                    .add(0)
                    .cast::<
                        TScriptInterface<crate::bindings::engine::UAnimationDataModel>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                attribute_identifier,
                __buffer
                    .add(16)
                    .cast::<crate::bindings::engine::FAnimationAttributeIdentifier>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(96).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(value, __buffer.add(100).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationAttributeBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_attribute_blueprint_library_get_attribute_key,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(100).cast::<i32>().swap(value);
        }
        unsafe { __buffer.add(104).cast::<bool>().read() }
    }
}
#[repr(C, align(8))]
pub struct UAnimationBlueprintLibrary {
    __padding_end: [u8; 48],
}
impl UAnimationBlueprintLibrary {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimationBlueprintLibrary")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_variable_frame_stripping_settings(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        variable_frame_stripping_settings: UPtr<
            crate::bindings::engine::UVariableFrameStrippingSettings,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_variable_frame_stripping_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &variable_frame_stripping_settings,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::UVariableFrameStrippingSettings>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_variable_frame_stripping_settings,
                __buffer,
            )
        };
    }
    pub fn set_root_motion_lock_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        root_motion_lock_type: crate::bindings::engine::ERootMotionRootLock,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_root_motion_lock_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &root_motion_lock_type,
                __buffer.add(8).cast::<crate::bindings::engine::ERootMotionRootLock>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_root_motion_lock_type,
                __buffer,
            )
        };
    }
    pub fn set_root_motion_enabled(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        b_enabled: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_root_motion_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_enabled, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_root_motion_enabled,
                __buffer,
            )
        };
    }
    pub fn set_rate_scale(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        rate_scale: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_rate_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&rate_scale, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_rate_scale,
                __buffer,
            )
        };
    }
    pub fn set_is_root_motion_lock_forced(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        b_forced: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_is_root_motion_lock_forced,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&b_forced, __buffer.add(8).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_is_root_motion_lock_forced,
                __buffer,
            )
        };
    }
    pub fn set_curve_meta_data_morph_target(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        curve_name: FName,
        b_override_morph_target: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_curve_meta_data_morph_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override_morph_target,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_curve_meta_data_morph_target,
                __buffer,
            )
        };
    }
    pub fn set_curve_meta_data_material(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        curve_name: FName,
        b_override_material: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_curve_meta_data_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_override_material,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_curve_meta_data_material,
                __buffer,
            )
        };
    }
    pub fn set_curve_compression_settings(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        compression_settings: UPtr<
            crate::bindings::engine::UAnimCurveCompressionSettings,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_curve_compression_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &compression_settings,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::UAnimCurveCompressionSettings>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_curve_compression_settings,
                __buffer,
            )
        };
    }
    pub fn set_bone_compression_settings(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        compression_settings: UPtr<crate::bindings::engine::UAnimBoneCompressionSettings>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_bone_compression_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &compression_settings,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::UAnimBoneCompressionSettings>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_bone_compression_settings,
                __buffer,
            )
        };
    }
    pub fn set_animation_interpolation_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        interpolation_type: crate::bindings::engine::EAnimInterpolationType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_animation_interpolation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &interpolation_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EAnimInterpolationType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_animation_interpolation_type,
                __buffer,
            )
        };
    }
    pub fn set_additive_base_pose_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        additive_base_pose_type: crate::bindings::engine::EAdditiveBasePoseType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_additive_base_pose_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &additive_base_pose_type,
                __buffer.add(8).cast::<crate::bindings::engine::EAdditiveBasePoseType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_additive_base_pose_type,
                __buffer,
            )
        };
    }
    pub fn set_additive_animation_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        additive_animation_type: crate::bindings::engine::EAdditiveAnimationType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_additive_animation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &additive_animation_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EAdditiveAnimationType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_set_additive_animation_type,
                __buffer,
            )
        };
    }
    pub fn replace_anim_notify_states(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        old_notify_class: TSubclassOf<crate::bindings::engine::UAnimNotifyState>,
        new_notify_class: TSubclassOf<crate::bindings::engine::UAnimNotifyState>,
        on_notify_state_replaced: FReplaceAnimNotifyStates_OnNotifyStateReplaced,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_replace_anim_notify_states,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_notify_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimNotifyState>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_notify_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimNotifyState>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_notify_state_replaced,
                __buffer
                    .add(24)
                    .cast::<FReplaceAnimNotifyStates_OnNotifyStateReplaced>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_replace_anim_notify_states,
                __buffer,
            )
        };
    }
    pub fn replace_anim_notifies(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        old_notify_class: TSubclassOf<crate::bindings::engine::UAnimNotify>,
        new_notify_class: TSubclassOf<crate::bindings::engine::UAnimNotify>,
        on_notify_replaced: FReplaceAnimNotifies_OnNotifyReplaced,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_replace_anim_notifies,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_notify_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimNotify>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_notify_class,
                __buffer
                    .add(16)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimNotify>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &on_notify_replaced,
                __buffer.add(24).cast::<FReplaceAnimNotifies_OnNotifyReplaced>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_replace_anim_notifies,
                __buffer,
            )
        };
    }
    pub fn rename_curve_meta_data(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        old_name: FName,
        new_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_rename_curve_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&old_name, __buffer.add(8).cast::<FName>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_rename_curve_meta_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(32).cast::<bool>().read() }
    }
    pub fn remove_virtual_bones(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        virtual_bone_names: TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_virtual_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &virtual_bone_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_virtual_bones,
                __buffer,
            )
        };
    }
    pub fn remove_virtual_bone(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        virtual_bone_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_virtual_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &virtual_bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_virtual_bone,
                __buffer,
            )
        };
    }
    pub fn remove_meta_data_of_class(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        meta_data_class: TSubclassOf<crate::bindings::engine::UAnimMetaData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_meta_data_of_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimMetaData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_meta_data_of_class,
                __buffer,
            )
        };
    }
    pub fn remove_meta_data(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        meta_data_object: UPtr<crate::bindings::engine::UAnimMetaData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_object,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimMetaData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_meta_data,
                __buffer,
            )
        };
    }
    pub fn remove_curve_meta_data(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        curve_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_curve_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_curve_meta_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn remove_curve(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        b_remove_name_from_skeleton: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_remove_name_from_skeleton,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_curve,
                __buffer,
            )
        };
    }
    pub fn remove_bone_selective_animation(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        bone_name: FName,
        children_excluded: TArray<FName>,
        b_include_children: bool,
        b_exclude_recursively: bool,
        b_finalize: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<43>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_bone_selective_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &children_excluded,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_children,
                __buffer.add(40).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_exclude_recursively,
                __buffer.add(41).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_finalize,
                __buffer.add(42).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_bone_selective_animation,
                __buffer,
            )
        };
    }
    pub fn remove_bone_animation(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        bone_name: FName,
        b_include_children: bool,
        b_finalize: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_bone_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_children,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_finalize,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_bone_animation,
                __buffer,
            )
        };
    }
    pub fn remove_animation_sync_markers_by_track(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        notify_track_name: FName,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_sync_markers_by_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_sync_markers_by_track,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn remove_animation_sync_markers_by_name(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        marker_name: FName,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_sync_markers_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &marker_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_sync_markers_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn remove_animation_notify_track(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_track_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<20>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_notify_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_notify_track,
                __buffer,
            )
        };
    }
    pub fn remove_animation_notify_events_by_track(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_track_name: FName,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_notify_events_by_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_notify_events_by_track,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn remove_animation_notify_events_by_name(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_name: FName,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_notify_events_by_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_animation_notify_events_by_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<i32>().read() }
    }
    pub fn remove_all_virtual_bones(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_virtual_bones,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_virtual_bones,
                __buffer,
            )
        };
    }
    pub fn remove_all_meta_data(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_meta_data,
                __buffer,
            )
        };
    }
    pub fn remove_all_curve_data(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_curve_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_curve_data,
                __buffer,
            )
        };
    }
    pub fn remove_all_bone_animation(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_bone_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_bone_animation,
                __buffer,
            )
        };
    }
    pub fn remove_all_animation_sync_markers(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_animation_sync_markers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_animation_sync_markers,
                __buffer,
            )
        };
    }
    pub fn remove_all_animation_notify_tracks(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_animation_notify_tracks,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_remove_all_animation_notify_tracks,
                __buffer,
            )
        };
    }
    pub fn is_valid_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        time: f32,
        is_valid: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<13>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(is_valid, __buffer.add(12).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<bool>().swap(is_valid);
        }
    }
    pub fn is_valid_raw_animation_track_name(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        track_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_raw_animation_track_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_raw_animation_track_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn is_valid_anim_notify_track_name(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_track_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_anim_notify_track_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_anim_notify_track_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn is_valid_animation_sync_marker_name(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        marker_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_animation_sync_marker_name,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &marker_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_valid_animation_sync_marker_name,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn is_root_motion_lock_forced(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_root_motion_lock_forced,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_root_motion_lock_forced,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn is_root_motion_enabled(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_root_motion_enabled,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_is_root_motion_enabled,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<bool>().read() }
    }
    pub fn get_vector_keys(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        times: &mut TArray<f32>,
        values: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_vector_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                times,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                values,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_vector_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<f32>>().swap(times);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(values);
        }
    }
    pub fn get_variable_frame_stripping_settings(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        variable_frame_stripping_settings: &mut UPtr<
            crate::bindings::engine::UVariableFrameStrippingSettings,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_variable_frame_stripping_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                variable_frame_stripping_settings,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::UVariableFrameStrippingSettings>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_variable_frame_stripping_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UVariableFrameStrippingSettings>>()
                .swap(variable_frame_stripping_settings);
        }
    }
    pub fn get_unique_marker_names(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        marker_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_unique_marker_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                marker_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_unique_marker_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(marker_names);
        }
    }
    pub fn get_transformation_keys(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        times: &mut TArray<f32>,
        values: &mut TArray<crate::bindings::core_u_object::FTransform>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_transformation_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                times,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                values,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_transformation_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<f32>>().swap(times);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .swap(values);
        }
    }
    pub fn get_time_at_frame(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        frame: i32,
        time: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_time_at_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&frame, __buffer.add(8).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(time, __buffer.add(12).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_time_at_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<f32>().swap(time);
        }
    }
    pub fn get_sequence_length(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        length: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_sequence_length,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(length, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_sequence_length,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<f32>().swap(length);
        }
    }
    pub fn get_root_motion_lock_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        lock_type: &mut crate::bindings::engine::ERootMotionRootLock,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_root_motion_lock_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                lock_type,
                __buffer.add(8).cast::<crate::bindings::engine::ERootMotionRootLock>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_root_motion_lock_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::engine::ERootMotionRootLock>()
                .swap(lock_type);
        }
    }
    pub fn get_raw_track_scale_data(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        track_name: FName,
        scale_data: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_scale_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                scale_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_scale_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(scale_data);
        }
    }
    pub fn get_raw_track_rotation_data(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        track_name: FName,
        rotation_data: &mut TArray<crate::bindings::core_u_object::FQuat>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_rotation_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation_data,
                __buffer.add(24).cast::<TArray<crate::bindings::core_u_object::FQuat>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_rotation_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FQuat>>()
                .swap(rotation_data);
        }
    }
    pub fn get_raw_track_position_data(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        track_name: FName,
        position_data: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_position_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position_data,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_position_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(position_data);
        }
    }
    pub fn get_raw_track_data(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        track_name: FName,
        position_keys: &mut TArray<crate::bindings::core_u_object::FVector>,
        rotation_keys: &mut TArray<crate::bindings::core_u_object::FQuat>,
        scaling_keys: &mut TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<72>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                position_keys,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                rotation_keys,
                __buffer.add(40).cast::<TArray<crate::bindings::core_u_object::FQuat>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                scaling_keys,
                __buffer
                    .add(56)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_raw_track_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(position_keys);
        }
        unsafe {
            __buffer
                .add(40)
                .cast::<TArray<crate::bindings::core_u_object::FQuat>>()
                .swap(rotation_keys);
        }
        unsafe {
            __buffer
                .add(56)
                .cast::<TArray<crate::bindings::core_u_object::FVector>>()
                .swap(scaling_keys);
        }
    }
    pub fn get_rate_scale(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        rate_scale: &mut f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_rate_scale,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(rate_scale, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_rate_scale,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<f32>().swap(rate_scale);
        }
    }
    pub fn get_num_keys(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        num_keys: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_num_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(num_keys, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_num_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(num_keys);
        }
    }
    pub fn get_num_frames(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        num_frames: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_num_frames,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(num_frames, __buffer.add(8).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_num_frames,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<i32>().swap(num_frames);
        }
    }
    pub fn get_num_curve_meta_data(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
    ) -> i32 {
        let mut __stack = crate::core_data::StackAlloc::<12>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_num_curve_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_num_curve_meta_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(8).cast::<i32>().read() }
    }
    pub fn get_nodes_of_class(
        animation_blueprint: UPtr<crate::bindings::engine::UAnimBlueprint>,
        node_class: TSubclassOf<crate::bindings::anim_graph::UAnimGraphNode_Base>,
        graph_nodes: &mut TArray<UPtr<crate::bindings::anim_graph::UAnimGraphNode_Base>>,
        b_include_child_classes: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<33>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_nodes_of_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &node_class,
                __buffer
                    .add(8)
                    .cast::<
                        TSubclassOf<crate::bindings::anim_graph::UAnimGraphNode_Base>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                graph_nodes,
                __buffer
                    .add(16)
                    .cast::<
                        TArray<UPtr<crate::bindings::anim_graph::UAnimGraphNode_Base>>,
                    >(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_include_child_classes,
                __buffer.add(32).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_nodes_of_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::anim_graph::UAnimGraphNode_Base>>>()
                .swap(graph_nodes);
        }
    }
    pub fn get_montage_slot_names(
        animation_montage: UPtr<crate::bindings::engine::UAnimMontage>,
        slot_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_montage_slot_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_montage,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimMontage>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                slot_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_montage_slot_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(slot_names);
        }
    }
    pub fn get_meta_data_of_class(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        meta_data_class: TSubclassOf<crate::bindings::engine::UAnimMetaData>,
        meta_data_of_class: &mut TArray<UPtr<crate::bindings::engine::UAnimMetaData>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_meta_data_of_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimMetaData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                meta_data_of_class,
                __buffer
                    .add(16)
                    .cast::<TArray<UPtr<crate::bindings::engine::UAnimMetaData>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_meta_data_of_class,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<TArray<UPtr<crate::bindings::engine::UAnimMetaData>>>()
                .swap(meta_data_of_class);
        }
    }
    pub fn get_meta_data(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        meta_data: &mut TArray<UPtr<crate::bindings::engine::UAnimMetaData>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                meta_data,
                __buffer
                    .add(8)
                    .cast::<TArray<UPtr<crate::bindings::engine::UAnimMetaData>>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_meta_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::engine::UAnimMetaData>>>()
                .swap(meta_data);
        }
    }
    pub fn get_frame_at_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        time: f32,
        frame: &mut i32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_frame_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(frame, __buffer.add(12).cast::<i32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_frame_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<i32>().swap(frame);
        }
    }
    pub fn get_float_value_at_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        time: f32,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_float_value_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(20).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_float_value_at_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(24).cast::<f32>().read() }
    }
    pub fn get_float_keys(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        times: &mut TArray<f32>,
        values: &mut TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_float_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                times,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                values,
                __buffer.add(40).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_float_keys,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<f32>>().swap(times);
        }
        unsafe {
            __buffer.add(40).cast::<TArray<f32>>().swap(values);
        }
    }
    pub fn get_curve_meta_data_names(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        out_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_meta_data_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_meta_data_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(out_names);
        }
    }
    pub fn get_curve_meta_data_morph_target(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        curve_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_meta_data_morph_target,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_meta_data_morph_target,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_curve_meta_data_material(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        curve_name: FName,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_meta_data_material,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_meta_data_material,
                __buffer,
            )
        };
        unsafe { __buffer.add(20).cast::<bool>().read() }
    }
    pub fn get_curve_compression_settings(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        compression_settings: &mut UPtr<
            crate::bindings::engine::UAnimCurveCompressionSettings,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_compression_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                compression_settings,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::UAnimCurveCompressionSettings>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_curve_compression_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UAnimCurveCompressionSettings>>()
                .swap(compression_settings);
        }
    }
    pub fn get_bone_poses_for_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        bone_names: TArray<FName>,
        time: f32,
        b_extract_root_motion: bool,
        poses: &mut TArray<crate::bindings::core_u_object::FTransform>,
        preview_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_poses_for_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extract_root_motion,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                poses,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_mesh,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_poses_for_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .swap(poses);
        }
    }
    pub fn get_bone_poses_for_frame(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        bone_names: TArray<FName>,
        frame: i32,
        b_extract_root_motion: bool,
        poses: &mut TArray<crate::bindings::core_u_object::FTransform>,
        preview_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_poses_for_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&frame, __buffer.add(24).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extract_root_motion,
                __buffer.add(28).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                poses,
                __buffer
                    .add(32)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &preview_mesh,
                __buffer.add(48).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_poses_for_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<TArray<crate::bindings::core_u_object::FTransform>>()
                .swap(poses);
        }
    }
    pub fn get_bone_pose_for_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        bone_name: FName,
        time: f32,
        b_extract_root_motion: bool,
        pose: &mut crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_pose_for_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extract_root_motion,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                pose,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_pose_for_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(pose);
        }
    }
    pub fn get_bone_pose_for_frame(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        bone_name: FName,
        frame: i32,
        b_extract_root_motion: bool,
        pose: &mut crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_pose_for_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&frame, __buffer.add(20).cast::<i32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_extract_root_motion,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                pose,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_pose_for_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(32)
                .cast::<crate::bindings::core_u_object::FTransform>()
                .swap(pose);
        }
    }
    pub fn get_bone_compression_settings(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        compression_settings: &mut UPtr<
            crate::bindings::engine::UAnimBoneCompressionSettings,
        >,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_compression_settings,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                compression_settings,
                __buffer
                    .add(8)
                    .cast::<
                        UPtr<crate::bindings::engine::UAnimBoneCompressionSettings>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_bone_compression_settings,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<UPtr<crate::bindings::engine::UAnimBoneCompressionSettings>>()
                .swap(compression_settings);
        }
    }
    pub fn get_anim_notify_event_trigger_time(
        notify_event: &crate::bindings::engine::FAnimNotifyEvent,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<228>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_anim_notify_event_trigger_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                notify_event,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNotifyEvent>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_anim_notify_event_trigger_time,
                __buffer,
            )
        };
        unsafe { __buffer.add(224).cast::<f32>().read() }
    }
    pub fn get_anim_notify_event_duration(
        notify_event: &crate::bindings::engine::FAnimNotifyEvent,
    ) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<228>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_anim_notify_event_duration,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                notify_event,
                __buffer.add(0).cast::<crate::bindings::engine::FAnimNotifyEvent>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_anim_notify_event_duration,
                __buffer,
            )
        };
        unsafe { __buffer.add(224).cast::<f32>().read() }
    }
    pub fn get_animation_track_names(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        track_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_track_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                track_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_track_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(track_names);
        }
    }
    pub fn get_animation_sync_markers_for_track(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        notify_track_name: FName,
        markers: &mut TArray<crate::bindings::engine::FAnimSyncMarker>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_sync_markers_for_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                markers,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::engine::FAnimSyncMarker>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_sync_markers_for_track,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::engine::FAnimSyncMarker>>()
                .swap(markers);
        }
    }
    pub fn get_animation_sync_markers(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        markers: &mut TArray<crate::bindings::engine::FAnimSyncMarker>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_sync_markers,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                markers,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::engine::FAnimSyncMarker>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_sync_markers,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::engine::FAnimSyncMarker>>()
                .swap(markers);
        }
    }
    pub fn get_animation_notify_track_names(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        track_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_track_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                track_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_track_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(track_names);
        }
    }
    pub fn get_animation_notify_events_for_track(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_track_name: FName,
        events: &mut TArray<crate::bindings::engine::FAnimNotifyEvent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_events_for_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                events,
                __buffer
                    .add(24)
                    .cast::<TArray<crate::bindings::engine::FAnimNotifyEvent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_events_for_track,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(24)
                .cast::<TArray<crate::bindings::engine::FAnimNotifyEvent>>()
                .swap(events);
        }
    }
    pub fn get_animation_notify_events(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_events: &mut TArray<crate::bindings::engine::FAnimNotifyEvent>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_events,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                notify_events,
                __buffer
                    .add(8)
                    .cast::<TArray<crate::bindings::engine::FAnimNotifyEvent>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_events,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<crate::bindings::engine::FAnimNotifyEvent>>()
                .swap(notify_events);
        }
    }
    pub fn get_animation_notify_event_names(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        event_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_event_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                event_names,
                __buffer.add(8).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_notify_event_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<TArray<FName>>().swap(event_names);
        }
    }
    pub fn get_animation_interpolation_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        interpolation_type: &mut crate::bindings::engine::EAnimInterpolationType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_interpolation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                interpolation_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EAnimInterpolationType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_interpolation_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::engine::EAnimInterpolationType>()
                .swap(interpolation_type);
        }
    }
    pub fn get_animation_graphs(
        animation_blueprint: UPtr<crate::bindings::engine::UAnimBlueprint>,
        animation_graphs: &mut TArray<UPtr<crate::bindings::anim_graph::UAnimationGraph>>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_graphs,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                animation_graphs,
                __buffer
                    .add(8)
                    .cast::<
                        TArray<UPtr<crate::bindings::anim_graph::UAnimationGraph>>,
                    >(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_graphs,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<TArray<UPtr<crate::bindings::anim_graph::UAnimationGraph>>>()
                .swap(animation_graphs);
        }
    }
    pub fn get_animation_curve_names(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_type: crate::bindings::engine::ERawCurveTrackTypes,
        curve_names: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<32>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_curve_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_type,
                __buffer.add(8).cast::<crate::bindings::engine::ERawCurveTrackTypes>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                curve_names,
                __buffer.add(16).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_animation_curve_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<TArray<FName>>().swap(curve_names);
        }
    }
    pub fn get_additive_base_pose_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        additive_base_pose_type: &mut crate::bindings::engine::EAdditiveBasePoseType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_additive_base_pose_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                additive_base_pose_type,
                __buffer.add(8).cast::<crate::bindings::engine::EAdditiveBasePoseType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_additive_base_pose_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::engine::EAdditiveBasePoseType>()
                .swap(additive_base_pose_type);
        }
    }
    pub fn get_additive_animation_type(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        additive_animation_type: &mut crate::bindings::engine::EAdditiveAnimationType,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<9>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_additive_animation_type,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                additive_animation_type,
                __buffer
                    .add(8)
                    .cast::<crate::bindings::engine::EAdditiveAnimationType>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_get_additive_animation_type,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(8)
                .cast::<crate::bindings::engine::EAdditiveAnimationType>()
                .swap(additive_animation_type);
        }
    }
    pub fn find_bone_path_to_root(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        bone_name: FName,
        bone_path: &mut TArray<FName>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_find_bone_path_to_root,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                bone_path,
                __buffer.add(24).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_find_bone_path_to_root,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(24).cast::<TArray<FName>>().swap(bone_path);
        }
    }
    pub fn finalize_bone_animation(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<8>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_finalize_bone_animation,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_finalize_bone_animation,
                __buffer,
            )
        };
    }
    pub fn extract_root_track_transform(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        time: f32,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<112>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_extract_root_track_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(8).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_extract_root_track_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(16).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn evaluate_root_bone_timecode_subframe_attribute_at_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        eval_time: f32,
        out_subframe: &mut f32,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_evaluate_root_bone_timecode_subframe_attribute_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&eval_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_subframe,
                __buffer.add(12).cast::<f32>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_evaluate_root_bone_timecode_subframe_attribute_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(12).cast::<f32>().swap(out_subframe);
        }
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn evaluate_root_bone_timecode_attributes_at_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        eval_time: f32,
        out_qualified_frame_time: &mut crate::bindings::core_u_object::FQualifiedFrameTime,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<29>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_evaluate_root_bone_timecode_attributes_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&eval_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_qualified_frame_time,
                __buffer
                    .add(12)
                    .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_evaluate_root_bone_timecode_attributes_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(12)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .swap(out_qualified_frame_time);
        }
        unsafe { __buffer.add(28).cast::<bool>().read() }
    }
    pub fn evaluate_bone_timecode_and_slate_attributes_at_time(
        bone_name: FName,
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        eval_time: f32,
        out_qualified_frame_time: &mut crate::bindings::core_u_object::FQualifiedFrameTime,
        slate: &mut FString,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<65>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_evaluate_bone_timecode_and_slate_attributes_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(0).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&eval_time, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_qualified_frame_time,
                __buffer
                    .add(28)
                    .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(slate, __buffer.add(48).cast::<FString>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_evaluate_bone_timecode_and_slate_attributes_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(28)
                .cast::<crate::bindings::core_u_object::FQualifiedFrameTime>()
                .swap(out_qualified_frame_time);
        }
        unsafe {
            __buffer.add(48).cast::<FString>().swap(slate);
        }
        unsafe { __buffer.add(64).cast::<bool>().read() }
    }
    pub fn does_curve_exist(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        curve_type: crate::bindings::engine::ERawCurveTrackTypes,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_does_curve_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_type,
                __buffer.add(20).cast::<crate::bindings::engine::ERawCurveTrackTypes>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_does_curve_exist,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn does_bone_name_exist(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        bone_name: FName,
        b_exists: &mut bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<21>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_does_bone_name_exist,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b_exists, __buffer.add(20).cast::<bool>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_does_bone_name_exist,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(20).cast::<bool>().swap(b_exists);
        }
    }
    pub fn copy_anim_notifies_from_sequence(
        source_animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        destination_animation_sequence_base: UPtr<
            crate::bindings::engine::UAnimSequenceBase,
        >,
        b_delete_existing_notifies: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_copy_anim_notifies_from_sequence,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &destination_animation_sequence_base,
                __buffer
                    .add(8)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_delete_existing_notifies,
                __buffer.add(16).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_copy_anim_notifies_from_sequence,
                __buffer,
            )
        };
    }
    pub fn copy_animation_curve_names_to_skeleton(
        old_skeleton: UPtr<crate::bindings::engine::USkeleton>,
        new_skeleton: UPtr<crate::bindings::engine::USkeleton>,
        sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_type: crate::bindings::engine::ERawCurveTrackTypes,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_copy_animation_curve_names_to_skeleton,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &old_skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &new_skeleton,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &sequence_base,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_type,
                __buffer.add(24).cast::<crate::bindings::engine::ERawCurveTrackTypes>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_copy_animation_curve_names_to_skeleton,
                __buffer,
            )
        };
    }
    pub fn contains_meta_data_of_class(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        meta_data_class: TSubclassOf<crate::bindings::engine::UAnimMetaData>,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<17>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_contains_meta_data_of_class,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimMetaData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_contains_meta_data_of_class,
                __buffer,
            )
        };
        unsafe { __buffer.add(16).cast::<bool>().read() }
    }
    pub fn add_virtual_bone(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        source_bone_name: FName,
        target_bone_name: FName,
        virtual_bone_name: &mut FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<44>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_virtual_bone,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &source_bone_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_bone_name,
                __buffer.add(20).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                virtual_bone_name,
                __buffer.add(32).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_virtual_bone,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<FName>().swap(virtual_bone_name);
        }
    }
    pub fn add_vector_curve_keys(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        times: &TArray<f32>,
        vectors: &TArray<crate::bindings::core_u_object::FVector>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_vector_curve_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                times,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                vectors,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FVector>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_vector_curve_keys,
                __buffer,
            )
        };
    }
    pub fn add_vector_curve_key(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        time: f32,
        vector: crate::bindings::core_u_object::FVector,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_vector_curve_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &vector,
                __buffer.add(24).cast::<crate::bindings::core_u_object::FVector>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_vector_curve_key,
                __buffer,
            )
        };
    }
    pub fn add_transformation_curve_keys(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        times: &TArray<f32>,
        transforms: &TArray<crate::bindings::core_u_object::FTransform>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_transformation_curve_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                times,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                transforms,
                __buffer
                    .add(40)
                    .cast::<TArray<crate::bindings::core_u_object::FTransform>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_transformation_curve_keys,
                __buffer,
            )
        };
    }
    pub fn add_transformation_curve_key(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        time: f32,
        transform: &crate::bindings::core_u_object::FTransform,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<128>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_transformation_curve_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                transform,
                __buffer.add(32).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_transformation_curve_key,
                __buffer,
            )
        };
    }
    pub fn add_node_asset_override(
        anim_blueprint: UPtr<crate::bindings::engine::UAnimBlueprint>,
        target: UPtr<crate::bindings::engine::UAnimationAsset>,
        override_: UPtr<crate::bindings::engine::UAnimationAsset>,
        b_print_applied_overrides: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<25>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_node_asset_override,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &anim_blueprint,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &override_,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_print_applied_overrides,
                __buffer.add(24).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_node_asset_override,
                __buffer,
            )
        };
    }
    pub fn add_meta_data_object(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        meta_data_object: UPtr<crate::bindings::engine::UAnimMetaData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<16>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_meta_data_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_object,
                __buffer.add(8).cast::<UPtr<crate::bindings::engine::UAnimMetaData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_meta_data_object,
                __buffer,
            )
        };
    }
    pub fn add_meta_data(
        animation_asset: UPtr<crate::bindings::engine::UAnimationAsset>,
        meta_data_class: TSubclassOf<crate::bindings::engine::UAnimMetaData>,
        meta_data_instance: &mut UPtr<crate::bindings::engine::UAnimMetaData>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<24>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_asset,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimationAsset>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &meta_data_class,
                __buffer
                    .add(8)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimMetaData>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                meta_data_instance,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UAnimMetaData>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_meta_data,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(16)
                .cast::<UPtr<crate::bindings::engine::UAnimMetaData>>()
                .swap(meta_data_instance);
        }
    }
    pub fn add_float_curve_keys(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        times: &TArray<f32>,
        values: &TArray<f32>,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<56>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_float_curve_keys,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                times,
                __buffer.add(24).cast::<TArray<f32>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                values,
                __buffer.add(40).cast::<TArray<f32>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_float_curve_keys,
                __buffer,
            )
        };
    }
    pub fn add_float_curve_key(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        time: f32,
        value: f32,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<28>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_float_curve_key,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&value, __buffer.add(24).cast::<f32>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_float_curve_key,
                __buffer,
            )
        };
    }
    pub fn add_curve_meta_data(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        curve_name: FName,
        b_transact: bool,
    ) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_curve_meta_data,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_transact,
                __buffer.add(20).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_curve_meta_data,
                __buffer,
            )
        };
        unsafe { __buffer.add(21).cast::<bool>().read() }
    }
    pub fn add_curve(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        curve_name: FName,
        curve_type: crate::bindings::engine::ERawCurveTrackTypes,
        b_meta_data_curve: bool,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<22>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_curve,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &curve_type,
                __buffer.add(20).cast::<crate::bindings::engine::ERawCurveTrackTypes>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &b_meta_data_curve,
                __buffer.add(21).cast::<bool>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_curve,
                __buffer,
            )
        };
    }
    pub fn add_animation_sync_marker(
        animation_sequence: UPtr<crate::bindings::engine::UAnimSequence>,
        marker_name: FName,
        time: f32,
        notify_track_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_sync_marker,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::UAnimSequence>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &marker_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(20).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_sync_marker,
                __buffer,
            )
        };
    }
    pub fn add_animation_notify_track(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_track_name: FName,
        track_color: crate::bindings::core_u_object::FLinearColor,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_track,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &track_color,
                __buffer.add(20).cast::<crate::bindings::core_u_object::FLinearColor>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_track,
                __buffer,
            )
        };
    }
    pub fn add_animation_notify_state_event_object(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        start_time: f32,
        duration: f32,
        notify_state: UPtr<crate::bindings::engine::UAnimNotifyState>,
        notify_track_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_state_event_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(12).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_state,
                __buffer
                    .add(16)
                    .cast::<UPtr<crate::bindings::engine::UAnimNotifyState>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_state_event_object,
                __buffer,
            )
        };
    }
    pub fn add_animation_notify_state_event(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_track_name: FName,
        start_time: f32,
        duration: f32,
        notify_state_class: TSubclassOf<crate::bindings::engine::UAnimNotifyState>,
    ) -> UPtr<crate::bindings::engine::UAnimNotifyState> {
        let mut __stack = crate::core_data::StackAlloc::<48>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_state_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_time,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&duration, __buffer.add(24).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_state_class,
                __buffer
                    .add(32)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimNotifyState>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_state_event,
                __buffer,
            )
        };
        unsafe {
            __buffer
                .add(40)
                .cast::<UPtr<crate::bindings::engine::UAnimNotifyState>>()
                .read()
        }
    }
    pub fn add_animation_notify_event_object(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        start_time: f32,
        notify: UPtr<crate::bindings::engine::UAnimNotify>,
        notify_track_name: FName,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<36>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_event_object,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify,
                __buffer.add(16).cast::<UPtr<crate::bindings::engine::UAnimNotify>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(24).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_event_object,
                __buffer,
            )
        };
    }
    pub fn add_animation_notify_event_from_source(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        start_time: f32,
        notify_event: crate::bindings::engine::FAnimNotifyEvent,
        notify_track_name: FName,
    ) -> UPtr<crate::bindings::engine::UAnimNotify> {
        let mut __stack = crate::core_data::StackAlloc::<264>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_event_from_source,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&start_time, __buffer.add(8).cast::<f32>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_event,
                __buffer.add(16).cast::<crate::bindings::engine::FAnimNotifyEvent>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(240).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_event_from_source,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(256).cast::<UPtr<crate::bindings::engine::UAnimNotify>>().read()
        }
    }
    pub fn add_animation_notify_event(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        notify_track_name: FName,
        start_time: f32,
        notify_class: TSubclassOf<crate::bindings::engine::UAnimNotify>,
    ) -> UPtr<crate::bindings::engine::UAnimNotify> {
        let mut __stack = crate::core_data::StackAlloc::<40>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_event,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_track_name,
                __buffer.add(8).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &start_time,
                __buffer.add(20).cast::<f32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &notify_class,
                __buffer
                    .add(24)
                    .cast::<TSubclassOf<crate::bindings::engine::UAnimNotify>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimationBlueprintLibrary::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_animation_blueprint_library_add_animation_notify_event,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(32).cast::<UPtr<crate::bindings::engine::UAnimNotify>>().read()
        }
    }
}
#[repr(C, align(8))]
pub struct UAnimPoseExtensions {
    __padding_end: [u8; 48],
}
impl UAnimPoseExtensions {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UAnimPoseExtensions")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
    pub fn set_bone_pose(
        pose: &mut FAnimPose,
        transform: crate::bindings::core_u_object::FTransform,
        bone_name: FName,
        space: EAnimPoseSpaces,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<301>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_set_bone_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &transform,
                __buffer.add(192).cast::<crate::bindings::core_u_object::FTransform>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(288).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(300).cast::<EAnimPoseSpaces>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_set_bone_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(0).cast::<FAnimPose>().swap(pose);
        }
    }
    pub fn is_valid(pose: &FAnimPose) -> bool {
        let mut __stack = crate::core_data::StackAlloc::<193>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_is_valid,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_is_valid,
                __buffer,
            )
        };
        unsafe { __buffer.add(192).cast::<bool>().read() }
    }
    pub fn get_socket_pose(
        pose: &FAnimPose,
        socket_name: FName,
        space: EAnimPoseSpaces,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_socket_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &socket_name,
                __buffer.add(192).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(204).cast::<EAnimPoseSpaces>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_socket_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_socket_names(pose: &FAnimPose, sockets: &mut TArray<FName>) {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_socket_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                sockets,
                __buffer.add(192).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_socket_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(192).cast::<TArray<FName>>().swap(sockets);
        }
    }
    pub fn get_relative_transform(
        pose: &FAnimPose,
        from_bone_name: FName,
        to_bone_name: FName,
        space: EAnimPoseSpaces,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_relative_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_bone_name,
                __buffer.add(192).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_bone_name,
                __buffer.add(204).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(216).cast::<EAnimPoseSpaces>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_relative_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(224).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_relative_to_ref_pose_transform(
        pose: &FAnimPose,
        bone_name: FName,
        space: EAnimPoseSpaces,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_relative_to_ref_pose_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(192).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(204).cast::<EAnimPoseSpaces>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_relative_to_ref_pose_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_ref_pose_relative_transform(
        pose: &FAnimPose,
        from_bone_name: FName,
        to_bone_name: FName,
        space: EAnimPoseSpaces,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<320>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_ref_pose_relative_transform,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &from_bone_name,
                __buffer.add(192).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &to_bone_name,
                __buffer.add(204).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(216).cast::<EAnimPoseSpaces>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_ref_pose_relative_transform,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(224).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_reference_pose(
        skeleton: UPtr<crate::bindings::engine::USkeleton>,
        out_pose: &mut FAnimPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<200>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_reference_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &skeleton,
                __buffer.add(0).cast::<UPtr<crate::bindings::engine::USkeleton>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pose,
                __buffer.add(8).cast::<FAnimPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_reference_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(8).cast::<FAnimPose>().swap(out_pose);
        }
    }
    pub fn get_ref_bone_pose(
        pose: &FAnimPose,
        bone_name: FName,
        space: EAnimPoseSpaces,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_ref_bone_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(192).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(204).cast::<EAnimPoseSpaces>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_ref_bone_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_curve_weight(pose: &FAnimPose, curve_name: &FName) -> f32 {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_curve_weight,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                curve_name,
                __buffer.add(192).cast::<FName>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_curve_weight,
                __buffer,
            )
        };
        unsafe { __buffer.add(204).cast::<f32>().read() }
    }
    pub fn get_curve_names(pose: &FAnimPose, curves: &mut TArray<FName>) {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_curve_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                curves,
                __buffer.add(192).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_curve_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(192).cast::<TArray<FName>>().swap(curves);
        }
    }
    pub fn get_bone_pose(
        pose: &FAnimPose,
        bone_name: FName,
        space: EAnimPoseSpaces,
    ) -> crate::bindings::core_u_object::FTransform {
        let mut __stack = crate::core_data::StackAlloc::<304>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_bone_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &bone_name,
                __buffer.add(192).cast::<FName>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &space,
                __buffer.add(204).cast::<EAnimPoseSpaces>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_bone_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<crate::bindings::core_u_object::FTransform>().read()
        }
    }
    pub fn get_bone_names(pose: &FAnimPose, bones: &mut TArray<FName>) {
        let mut __stack = crate::core_data::StackAlloc::<208>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_bone_names,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(0).cast::<FAnimPose>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                bones,
                __buffer.add(192).cast::<TArray<FName>>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_bone_names,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(192).cast::<TArray<FName>>().swap(bones);
        }
    }
    pub fn get_anim_pose_at_time(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        time: f64,
        evaluation_options: FAnimPoseEvaluationOptions,
        pose: &mut FAnimPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_anim_pose_at_time,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(&time, __buffer.add(8).cast::<f64>(), 1);
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_options,
                __buffer.add(16).cast::<FAnimPoseEvaluationOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(40).cast::<FAnimPose>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_anim_pose_at_time,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<FAnimPose>().swap(pose);
        }
    }
    pub fn get_anim_pose_at_frame(
        animation_sequence_base: UPtr<crate::bindings::engine::UAnimSequenceBase>,
        frame_index: i32,
        evaluation_options: FAnimPoseEvaluationOptions,
        pose: &mut FAnimPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<232>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_anim_pose_at_frame,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_sequence_base,
                __buffer
                    .add(0)
                    .cast::<UPtr<crate::bindings::engine::UAnimSequenceBase>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &frame_index,
                __buffer.add(8).cast::<i32>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &evaluation_options,
                __buffer.add(16).cast::<FAnimPoseEvaluationOptions>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(pose, __buffer.add(40).cast::<FAnimPose>(), 1);
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_get_anim_pose_at_frame,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(40).cast::<FAnimPose>().swap(pose);
        }
    }
    pub fn evaluate_animation_blueprint_with_input_pose(
        input_pose: &FAnimPose,
        target_skeletal_mesh: UPtr<crate::bindings::engine::USkeletalMesh>,
        animation_blueprint: UPtr<crate::bindings::engine::UAnimBlueprint>,
        out_pose: &mut FAnimPose,
    ) {
        let mut __stack = crate::core_data::StackAlloc::<400>::new();
        let __buffer = __stack.buffer_mut();
        let __bindings = crate::module::bindings();
        unsafe {
            (__bindings
                .core_fns
                .initialize_values_in_param_buffer)(
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_evaluate_animation_blueprint_with_input_pose,
                __buffer,
            )
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                input_pose,
                __buffer.add(0).cast::<FAnimPose>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &target_skeletal_mesh,
                __buffer.add(192).cast::<UPtr<crate::bindings::engine::USkeletalMesh>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                &animation_blueprint,
                __buffer
                    .add(200)
                    .cast::<UPtr<crate::bindings::engine::UAnimBlueprint>>(),
                1,
            );
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                out_pose,
                __buffer.add(208).cast::<FAnimPose>(),
                1,
            );
        }
        let __object_ptr = crate::bindings::animation_blueprint_library::UAnimPoseExtensions::cdo();
        unsafe {
            (__bindings
                .core_fns
                .process_event)(
                __object_ptr,
                crate::bindings::animation_blueprint_library::__FUNCTION_PTRS
                    .u_anim_pose_extensions_evaluate_animation_blueprint_with_input_pose,
                __buffer,
            )
        };
        unsafe {
            __buffer.add(208).cast::<FAnimPose>().swap(out_pose);
        }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_BaseAttributeActionNode {
    __padding_end: [u8; 368],
}
impl UK2Node_BaseAttributeActionNode {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_BaseAttributeActionNode")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_SetAttributeKeyGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_SetAttributeKeyGeneric {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_SetAttributeKeyGeneric")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_SetAttributeKeysGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_SetAttributeKeysGeneric {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_SetAttributeKeysGeneric")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_GetAttributeKeyGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_GetAttributeKeyGeneric {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_GetAttributeKeyGeneric")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct UK2Node_GetAttributeKeysGeneric {
    __padding_end: [u8; 368],
}
impl UK2Node_GetAttributeKeysGeneric {
    pub fn static_class() -> *mut crate::ffi::UObjectOpague {
        *crate::bindings::globals::CLASS_PTRS
            .wait()
            .name_to_ptr
            .get("UK2Node_GetAttributeKeysGeneric")
            .unwrap()
    }
    pub fn cdo() -> *mut crate::ffi::UObjectOpague {
        let class = Self::static_class();
        unsafe {
            let mut cdo = std::ptr::null_mut();
            (crate::module::bindings().core_fns.get_cdo_from_class)(class, &raw mut cdo);
            cdo
        }
    }
}
#[repr(C, align(8))]
pub struct FReplaceAnimNotifies_OnNotifyReplaced {
    _opague: [u8; 32],
}
#[repr(C, align(8))]
pub struct FReplaceAnimNotifyStates_OnNotifyStateReplaced {
    _opague: [u8; 32],
}
#[repr(transparent)]
pub struct EAnimDataEvalType(pub u8);
impl EAnimDataEvalType {
    pub const SOURCE: EAnimDataEvalType = EAnimDataEvalType(0);
    pub const RAW: EAnimDataEvalType = EAnimDataEvalType(1);
    pub const COMPRESSED: EAnimDataEvalType = EAnimDataEvalType(2);
}
#[repr(transparent)]
pub struct EAnimPoseSpaces(pub u8);
impl EAnimPoseSpaces {
    pub const LOCAL: EAnimPoseSpaces = EAnimPoseSpaces(0);
    pub const WORLD: EAnimPoseSpaces = EAnimPoseSpaces(1);
}
